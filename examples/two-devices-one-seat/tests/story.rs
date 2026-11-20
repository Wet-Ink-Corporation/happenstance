//! The story, one claim at a time, against real files.
//!
//! Every test opens three SQLite files of its own in a fresh directory, so the
//! tests can run in parallel without meeting on a file. Nothing sleeps: an
//! in-process far side either answers or fails, so a sync is a bounded loop of
//! steps rather than a wait.
//!
//! # Why the tests live in `mod story`
//!
//! `xtask/src/proof.rs` matches fully-qualified test names out of `cargo test
//! -- --list`, and a bare top-level test lists without a module prefix and
//! would never match its row.

mod story {
    use std::collections::BTreeSet;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::{fs, io};

    use happenstance::Event;
    use happenstance_sync::runner::{Adjudicate, EdgeState, Phase};
    use happenstance_sync::{EventGroup, FailureClass};
    use two_devices_one_seat::{
        CAPACITY, Converged, Device, Link, Place, Refusal, RuleError, SaleId, Scenario, StoryError,
        tell,
    };

    /// A directory of this test's own, removed when the test ends.
    ///
    /// Named with the process id and the test's name, so two tests, or two
    /// runs, never share a file.
    struct Scratch {
        path: PathBuf,
    }

    impl Scratch {
        fn new(test: &str) -> io::Result<Self> {
            let path = std::env::temp_dir().join(format!(
                "happenstance-two-devices-test-{}-{test}",
                std::process::id()
            ));
            match fs::remove_dir_all(&path) {
                Err(error) if error.kind() != io::ErrorKind::NotFound => return Err(error),
                _ => {}
            }
            fs::create_dir_all(&path)?;
            Ok(Self { path })
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            if let Err(error) = fs::remove_dir_all(&self.path) {
                eprintln!("could not remove {}: {error}", self.path.display());
            }
        }
    }

    /// A show on central with two of its three seats sold there, and both
    /// devices synced down: one seat left, everywhere.
    async fn one_seat_left(dir: &Path) -> Result<Scenario, StoryError> {
        let mut scenario = Scenario::open(dir)?;
        scenario.schedule(CAPACITY).await?;
        scenario.sell(Place::Central, "Ada").await?;
        scenario.sell(Place::Central, "Brian").await?;
        scenario.sync(Device::Tablet).await?;
        scenario.sync(Device::Phone).await?;
        Ok(scenario)
    }

    /// Both devices sell the last seat offline, and then sync, `first` first.
    /// Central now holds four sales for three seats.
    async fn oversold(dir: &Path, first: Device) -> Result<Scenario, StoryError> {
        let mut scenario = one_seat_left(dir).await?;
        scenario.set_link(Device::Tablet, Link::Down);
        scenario.set_link(Device::Phone, Link::Down);
        scenario.sell(Place::Tablet, "Grace").await?;
        scenario.sell(Place::Phone, "Linus").await?;
        scenario.set_link(Device::Tablet, Link::Up);
        scenario.set_link(Device::Phone, Link::Up);
        scenario.sync(first).await?;
        scenario.sync(first.other()).await?;
        Ok(scenario)
    }

    #[tokio::test]
    async fn a_device_sells_a_seat_offline() {
        let dir = Scratch::new("offline-sale").expect("a scratch directory");
        let mut scenario = Scenario::open(dir.path()).expect("three fresh files");
        scenario.schedule(CAPACITY).await.expect("scheduled");
        scenario.sync(Device::Tablet).await.expect("synced down");

        scenario.set_link(Device::Tablet, Link::Down);
        let sale = scenario
            .sell(Place::Tablet, "Grace")
            .await
            .expect("the tablet decides alone, against what it holds");

        assert_eq!(sale.as_str(), "tablet-1");
        let tablet = scenario.ledger(Place::Tablet).await.expect("tablet fold");
        let central = scenario.ledger(Place::Central).await.expect("central fold");
        assert_eq!(tablet.held(), 1);
        assert_eq!(central.held(), 0, "nothing left the tablet");
    }

    #[tokio::test]
    async fn a_sync_without_signal_is_transient_and_loses_nothing() {
        let dir = Scratch::new("no-signal").expect("a scratch directory");
        let mut scenario = Scenario::open(dir.path()).expect("three fresh files");
        scenario.schedule(CAPACITY).await.expect("scheduled");
        scenario.sync(Device::Tablet).await.expect("synced down");
        scenario.set_link(Device::Tablet, Link::Down);
        scenario
            .sell(Place::Tablet, "Grace")
            .await
            .expect("sold offline");

        let refused = scenario.sync(Device::Tablet).await;

        let failure = match &refused {
            Err(error) => error.failure().expect("a sync failure carries its class"),
            Ok(synced) => panic!("a sync with no signal reported {synced:?}"),
        };
        assert_eq!(failure.class, FailureClass::Transient);
        assert_eq!(scenario.status(Device::Tablet).state, EdgeState::Running);

        scenario.set_link(Device::Tablet, Link::Up);
        scenario
            .sync(Device::Tablet)
            .await
            .expect("the signal is back");
        let central = scenario.ledger(Place::Central).await.expect("central fold");
        assert_eq!(central.held(), 1, "the failed attempt lost nothing");
    }

    /// The case the test above cannot reach: with no signal at all the step
    /// fails in its pull, before the device's sale is in any batch. Here the
    /// pull succeeds and only the push is lost, so a runner that dropped or
    /// confirmed the unpushed batch on failure would leave central short.
    #[tokio::test]
    async fn a_lost_push_is_transient_and_the_next_step_delivers_it() {
        let dir = Scratch::new("lost-push").expect("a scratch directory");
        let mut scenario = Scenario::open(dir.path()).expect("three fresh files");
        scenario.schedule(CAPACITY).await.expect("scheduled");
        scenario.sync(Device::Tablet).await.expect("synced down");
        scenario.set_link(Device::Tablet, Link::Down);
        scenario
            .sell(Place::Tablet, "Grace")
            .await
            .expect("sold offline");
        scenario
            .sell(Place::Central, "Ada")
            .await
            .expect("sold at central meanwhile");

        scenario.set_link(Device::Tablet, Link::PullOnly);
        let lost = scenario.sync(Device::Tablet).await;

        let failure = match &lost {
            Err(error) => error.failure().expect("a sync failure carries its phase"),
            Ok(synced) => panic!("a sync whose push was lost reported {synced:?}"),
        };
        assert_eq!(failure.phase, Phase::Push, "the pull got through");
        assert_eq!(failure.class, FailureClass::Transient);
        assert_eq!(scenario.status(Device::Tablet).state, EdgeState::Running);
        let tablet = scenario.ledger(Place::Tablet).await.expect("tablet fold");
        assert_eq!(tablet.held(), 2, "the pull landed central's sale first");
        let central = scenario.ledger(Place::Central).await.expect("central fold");
        assert_eq!(central.held(), 1, "the tablet's sale never left");

        scenario.set_link(Device::Tablet, Link::Up);
        let synced = scenario
            .sync(Device::Tablet)
            .await
            .expect("the push goes through");
        assert_eq!(synced.landed, 1, "the next step delivered the lost push");
        let central = scenario.ledger(Place::Central).await.expect("central fold");
        let sold: Vec<&str> = central.sales().map(|sale| sale.sale.as_str()).collect();
        assert_eq!(sold, ["central-1", "tablet-1"]);
    }

    #[tokio::test]
    async fn the_last_seat_is_refused_locally_once_sold() {
        let dir = Scratch::new("refused-locally").expect("a scratch directory");
        let scenario = one_seat_left(dir.path()).await.expect("one seat left");
        scenario
            .sell(Place::Central, "Cleo")
            .await
            .expect("the last seat");
        let before = scenario.log(Place::Central).await.expect("log").len();

        let refused = scenario.sell(Place::Central, "Dmitri").await;

        let refusal = refused.as_ref().err().and_then(StoryError::refusal);
        assert!(
            matches!(
                refusal,
                Some(Refusal::SoldOut {
                    capacity: 3,
                    held: 3
                })
            ),
            "the fourth sale was not refused as sold out: {refused:?}"
        );
        let after = scenario.log(Place::Central).await.expect("log").len();
        assert_eq!(after, before, "a refusal appends nothing");
    }

    #[tokio::test]
    async fn both_devices_sell_the_last_seat_and_central_holds_both() {
        let dir = Scratch::new("double-sale").expect("a scratch directory");
        let mut scenario = one_seat_left(dir.path()).await.expect("one seat left");
        scenario.set_link(Device::Tablet, Link::Down);
        scenario.set_link(Device::Phone, Link::Down);

        let tablet = scenario.sell(Place::Tablet, "Grace").await;
        let phone = scenario.sell(Place::Phone, "Linus").await;
        assert!(
            tablet.is_ok(),
            "the tablet's own rule allowed it: {tablet:?}"
        );
        assert!(phone.is_ok(), "the phone's own rule allowed it: {phone:?}");

        scenario.set_link(Device::Tablet, Link::Up);
        scenario.set_link(Device::Phone, Link::Up);
        scenario.sync(Device::Tablet).await.expect("tablet up");
        scenario.sync(Device::Phone).await.expect("phone up");

        let central = scenario.ledger(Place::Central).await.expect("central fold");
        // Counted as sales, not seats held: the push that oversold the show
        // landed with its refund in the same write, so both sales are in
        // central's log and one of them no longer holds a seat.
        assert_eq!(central.sales().count(), 4, "ingest never refuses (SY-1)");
        assert_eq!(central.capacity(), Some(CAPACITY));
        assert_eq!(central.held(), CAPACITY, "the oversold sale was refunded");
        assert_eq!(central.refunded().count(), 1, "exactly one refund");
    }

    /// Central's rule, asked again about everything central holds, answers
    /// nothing, and a second sync of both devices writes nothing. Rejects a
    /// rule that refunds a sale it has already judged when that sale is
    /// delivered again, which the runner's skip would otherwise hide.
    #[tokio::test]
    async fn adjudication_is_idempotent() {
        let dir = Scratch::new("idempotent").expect("a scratch directory");
        let mut scenario = oversold(dir.path(), Device::Tablet)
            .await
            .expect("oversold");
        let central = scenario.ledger(Place::Central).await.expect("central fold");
        assert_eq!(
            central.refunded().count(),
            1,
            "the push landed with its refund"
        );
        let before = scenario.log(Place::Central).await.expect("log").len();

        let answered = scenario.judge_again().await.expect("judged again");
        scenario.sync(Device::Tablet).await.expect("tablet again");
        scenario.sync(Device::Phone).await.expect("phone again");

        assert_eq!(
            answered, 0,
            "the rule answers a judged history with nothing"
        );
        let after = scenario.log(Place::Central).await.expect("log").len();
        assert_eq!(after, before);
    }

    #[tokio::test]
    async fn the_refunded_sale_is_the_last_to_reach_central() {
        for (first, refunded) in [(Device::Tablet, "phone-1"), (Device::Phone, "tablet-1")] {
            let dir = Scratch::new(&format!("last-to-reach-{first}")).expect("a scratch dir");
            let scenario = oversold(dir.path(), first).await.expect("oversold");

            let central = scenario.ledger(Place::Central).await.expect("central fold");
            let names: Vec<&str> = central.refunded().map(SaleId::as_str).collect();
            assert_eq!(names, [refunded], "{first} reached central first");
            assert_eq!(central.held(), CAPACITY);
        }
    }

    /// A central rule that judges nothing: every sale lands unanswered.
    #[derive(Debug, Clone, Copy)]
    struct NoRule;

    impl Adjudicate for NoRule {
        type Error = RuleError;

        async fn compensation(
            &self,
            _group: &EventGroup,
            _earlier: &[EventGroup],
        ) -> Result<Vec<Event>, RuleError> {
            Ok(Vec::new())
        }
    }

    /// The contrast: the same steps, with central's peer judging by a rule
    /// that answers nothing. The oversell is real, and only the rule answers it.
    #[tokio::test]
    async fn without_adjudication_the_show_stays_oversold() {
        let dir = Scratch::new("no-adjudication").expect("a scratch directory");
        let mut scenario = Scenario::judged_by(dir.path(), |_, _| NoRule).expect("three files");
        scenario.schedule(CAPACITY).await.expect("scheduled");
        scenario.sell(Place::Central, "Ada").await.expect("sold");
        scenario.sell(Place::Central, "Brian").await.expect("sold");
        for device in [Device::Tablet, Device::Phone] {
            scenario.sync(device).await.expect("synced down");
            scenario.set_link(device, Link::Down);
        }
        scenario.sell(Place::Tablet, "Grace").await.expect("sold");
        scenario.sell(Place::Phone, "Linus").await.expect("sold");
        for device in [Device::Tablet, Device::Phone, Device::Tablet] {
            scenario.set_link(device, Link::Up);
            scenario.sync(device).await.expect("synced");
        }

        scenario.project().await.expect("projected");

        for place in Place::ALL {
            let view = scenario.view(place).expect("the read model");
            assert_eq!(view.held(), 4, "{place} holds the oversell");
            assert_eq!(view.refunds(), 0, "{place} holds no refund");
        }
    }

    #[tokio::test]
    async fn the_story_converges() {
        let dir = Scratch::new("converges").expect("a scratch directory");

        let converged: Converged = tell(dir.path(), &mut Vec::new())
            .await
            .expect("the story runs");

        let central = &converged.central;
        assert_eq!(central.ids.len(), 6, "1 scheduled, 4 sold, 1 refunded");
        for (place, replica) in converged.replicas() {
            assert_eq!(replica.ids, central.ids, "{place} holds the same events");
            let authors: BTreeSet<_> = replica.refunds.iter().map(|id| id.store()).collect();
            assert_eq!(replica.refunds.len(), 1, "{place}: exactly one refund");
            assert_eq!(
                authors,
                BTreeSet::from([central.store]),
                "{place}: central wrote it (SY-7)"
            );
            assert_eq!(replica.view.held(), CAPACITY, "{place}: seats held");
            assert_eq!(replica.view, central.view, "{place}: the same read model");
        }
    }

    #[tokio::test]
    async fn the_phone_is_refused_after_converging() {
        let dir = Scratch::new("refused-after").expect("a scratch directory");
        tell(dir.path(), &mut Vec::new())
            .await
            .expect("the story runs");

        let scenario = Scenario::open(dir.path()).expect("the same three files");
        scenario.set_link(Device::Phone, Link::Down);
        let refused = scenario.sell(Place::Phone, "Margaret").await;

        assert!(
            matches!(
                refused.as_ref().err().and_then(StoryError::refusal),
                Some(Refusal::SoldOut { .. })
            ),
            "the phone sold a seat it does not have: {refused:?}"
        );
    }

    /// The section markers, in the order the story needs them.
    const MARKERS: [&str; 10] = [
        "== the show ==",
        "== the signal drops ==",
        "== both devices sell the last seat ==",
        "== the signal returns ==",
        "== central sees the whole history ==",
        "== central keeps its rule ==",
        "== everyone converges ==",
        "== the same view, three places ==",
        "== every handle dropped; reading the files back ==",
        "== the phone tries again ==",
    ];

    /// Runs the binary and returns everything it printed.
    fn transcript() -> String {
        let output = Command::new(env!("CARGO_BIN_EXE_two-devices-one-seat"))
            .output()
            .expect("the binary should be runnable");
        assert!(
            output.status.success(),
            "the binary exited with {}\n--- stdout ---\n{}\n--- stderr ---\n{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );
        String::from_utf8(output.stdout).expect("the transcript should be UTF-8")
    }

    #[test]
    fn the_binary_tells_the_story() {
        let transcript = transcript();

        let mut searched_from = 0;
        for marker in MARKERS {
            let found = transcript
                .get(searched_from..)
                .and_then(|rest| rest.find(marker))
                .unwrap_or_else(|| panic!("`{marker}` is missing or out of order\n{transcript}"));
            searched_from += found + marker.len();
        }

        let view = transcript
            .split("== the same view, three places ==")
            .nth(1)
            .and_then(|rest| rest.split("\n\n").next())
            .expect("the view section");
        for place in Place::ALL {
            let line = view
                .lines()
                .find(|line| line.trim_start().starts_with(&place.to_string()))
                .unwrap_or_else(|| panic!("no line for {place}\n{transcript}"));
            for claim in ["3 of 3 seats held", "4 sales", "1 refund"] {
                assert_eq!(
                    line.matches(claim).count(),
                    1,
                    "{place}: `{claim}` in {line}"
                );
            }
        }

        assert_eq!(transcript, self::transcript(), "the transcript is stable");
    }

    #[test]
    fn the_program_uses_the_published_path() {
        let sources = [
            include_str!("../src/lib.rs"),
            include_str!("../src/main.rs"),
            include_str!("../src/domain.rs"),
            include_str!("../src/domain/decide.rs"),
            include_str!("../src/rule.rs"),
            include_str!("../src/uplink.rs"),
            include_str!("../src/timeline.rs"),
            include_str!("../src/scenario.rs"),
            include_str!("../src/error.rs"),
            include_str!("../src/readback.rs"),
            include_str!("../src/story.rs"),
        ]
        .concat();
        for needle in [
            "commit(",
            "run_projection(",
            "serve_sync_adjudicated(",
            ".with_adjudicator(",
            "RemoteSyncPeer::new(",
            "Edge::new(",
            "impl ConvergentProjection for Timeline",
        ] {
            assert!(sources.contains(needle), "src/ never calls `{needle}`");
        }
        for forbidden in ["MemorySyncPeer", "MemoryIngestStore"] {
            assert!(!sources.contains(forbidden), "src/ names `{forbidden}`");
        }
        let manifest = include_str!("../Cargo.toml");
        assert!(
            !manifest.contains("\"memory\""),
            "the manifest turns on a memory feature"
        );
    }
}
