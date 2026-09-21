# Wave `2026-09-21-intake` — corpus match

What already exists in `.kb/`, what the one incoming claim-set could have merged into, and the
score behind every disposition. Read this before `02-placement-and-adjudication.md`: the
placements there are defensible only because of what this file establishes — and this wave more
than any before it, because of **what the tree already carries**.

This is the corpus's **thirteenth wave**, and its smallest: one staged file, four claims. That
does not make it easy. Its whole weight sits on a single question the corpus has circled twice
and never answered — *what does a wave do when an accepted decision's own falsifier has fired?* —
and on the mechanical consequence of answering it, which is that `KbFrontmatter` has no way to
say "superseded in part."

Its hard jobs are four:

1. **Honouring the intake's `should` in substance while declining it in mechanism.** The file
   says the new atom *"should carry `supersedes: [kb-decision-0022]` **scoped to §11**."* The
   frontmatter key carries no scope, and `decisions/README.md` attaches a mechanical consequence
   to it: setting it obliges flipping `kb-decision-0022` to `status: superseded` +
   `superseded_by`. That flip would deauthorise §4, §6, §7, §9, §10 and §15 — every one of which
   the same file says *stands unchanged*. `02`, Adjudication 1: the corpus's own
   partial-supersession convention (`kb-decision-0007` → 0006, `kb-decision-0031` → 0007) wins,
   and it is `supersedes: null` with the scope in prose. Carried to `unresolved[]` as a residual,
   not as a doubt about the call.
2. **Splitting the evidence off the conclusion, against a sibling that scores 66.**
   `kb-reference-busy-timeout-margin-001` is the obvious owner by lexical score and by
   directory, and it is the wrong one: it is a **snapshot of 2026-09-03**, of a different
   instrument, against a budget this wave moves. `reference/README.md`'s dating rule exists for
   exactly this. `02`, Adjudication 2.
3. **Amending two open questions without closing either.** One of
   `adr-0022-falsifiers-have-fired`'s three findings is decided and two are not;
   `no-fixture-tolerance-for-transient-contention`'s blocking instrument now exists and its
   question does not move. Both stay `status: accepted`. `02`, Adjudications 3 and 4.
4. **Recording what the tree carries that the digest did not extract.** The intake says the
   instrument demonstrates *"both affected rules reject a merely-contended store — each on a
   different assertion."* The test file carries a stronger finding than that, and it bears
   directly on the open question it is filed against. `02`, Adjudication 4.

## The corpus, as verified

```
ls .kb/decisions      →  64 atoms — 62 accepted, 2 superseded (0002, 0021)
                         numbered 0001–0025 and 0029–0064, plus SD-0001, SD-0002, standalone-svg
                         0026–0028 reserved by subject (RUNBOOK.md); 0065+ free on disk
ls .kb/open-questions →  77 atoms, authority_tier: note — 63 accepted, 14 superseded
ls .kb/reference      →  25 · playbooks 11 · concepts 3 · maps 3 · governance 2 · design 2 · product 0
ls references/adr     →  33 records — highest 0063; NO 0064 record, and none is owed by convention
git log --oneline -1  →  f89e184, "The busy timeout is fifteen seconds — ADR-0022 §11
                         superseded in part (#13)"
```

**The tree has already moved, and this wave is the record catching up.** That is unusual enough
to state before anything else, because it inverts the provenance problem the twelfth wave had.
Verified at this worktree's `HEAD`:

- `crates/happenstance-sqlite/src/connection.rs:112` reads
  `pub const BUSY_TIMEOUT_MS: u64 = 15_000;`, with a fifty-line doc comment carrying the sweep
  table and the CF-33 argument.
- `crates/happenstance-sqlite/README.md:128` already documents `busy_timeout` as `15,000` ms,
  *"finite **and** generous, and both halves are load-bearing."*
- `experiments/busy-timeout-margin/results/adapter-cap-sweep.md` exists, with method, conditions,
  the sweep, the duration rows and its own *"What this does not show."*
- `happenstance_testkit::FaultyStore::contend_next` exists
  (`crates/happenstance-testkit/src/faulty.rs:333`, `:429`), and
  `crates/happenstance-testkit/tests/contended_store_instruments.rs` carries nine tests against
  it.

So no claim in this wave is a decision recorded ahead of the tree it binds. Every one is a
decision recorded *behind* it — the code, the rustdoc and the adapter README all say fifteen
seconds, and `.kb/` still says five in the only place it is allowed to: `kb-decision-0022`'s
immutable summary.

**One check that could have made this wave much harder, and came back clean.** `grep -n
"busy_timeout\|BUSY_TIMEOUT\|5,000 ms" spec/SPECIFICATION.md` returns **nothing**. No clause —
`[FROZEN]` or otherwise — quotes the busy timeout, so nothing here is a frozen-clause edit and
`cargo xtask spec-trace` has no stake in this wave. Contrast §8/§16, where the same open question
records that repairing ES-27's `Rejects:` prose *is* a `[FROZEN]` clause edit; that is one of the
two findings this wave leaves open, and the difference is exactly why.

| Layer | Atoms | Bearing on this wave |
| --- | --- | --- |
| `decisions/` | **64** (62 accepted) | **One is added** — ADR-0065. **No accepted body is edited and no `status` is flipped** — seventh wave running. `kb-decision-0022` is superseded **in part**, which under this corpus's own convention is a prose scope on the *new* atom and no frontmatter change on the old one |
| `open-questions/` | 77 | **Three receive a dated merge**; none is added, none is resolved, none changes `status`. Two of the three are the atoms the intake names by path |
| `reference/` | 25 | **One is added** — the adapter cap sweep. No existing reference atom is extended; the dating rule, ninth wave running |
| `playbooks/`, `concepts/`, `governance/`, `design/`, `product/` | 11, 3, 2, 2, 0 | No bearing. The unbounded-handler argument is restated *inside* the decision rather than harvested as a playbook, because it is a rejected alternative and `decisions/README.md` says rejected alternatives travel with the decision |
| `maps/` | 3 | `decision-map` one row plus a wave section; `domain-map` a reference bullet and a decision annotation; `open-questions-index` three annotations |

For authority purposes:

- **Sixty-two accepted decision atoms exist and the intake names two of them** —
  `kb-decision-0022` (the subject) and `kb-decision-0064` (the measurement host, via the related
  list). **Neither body is edited and neither `status` is flipped.**
- **Zero full decision supersessions.** Seventh wave running. This is the corpus's **third**
  partial supersession and its first at *section* grain rather than at half-a-document grain.
- **One governance atom is load-bearing without being touched.**
  `kb-governance-what-may-refute-a-finding-001` sets the bar an empirical counter-claim must
  clear — *"a refutation is itself a finding and is held to the same standard of evidence, so an
  empirical counter-claim owes a measurement rather than a quotation."* The intake clears it:
  24 launches across three cells, a stated worst-case configuration, a published restore trap,
  and its own negative control in the parallelism table. That is why Claim 1 is adjudicated
  rather than deferred.

## Provenance check

| Thing the intake asserts | Verified at `HEAD` | Where |
| --- | --- | --- |
| `BUSY_TIMEOUT_MS` moves `5_000` → `15_000` | **yes**, already landed | `crates/happenstance-sqlite/src/connection.rs:112` |
| 5,000 ms → 7 red of 8; 15,000 → 0 of 16; 30,000 → 0 of 8 | **yes** | `experiments/busy-timeout-margin/results/adapter-cap-sweep.md`, "The sweep" |
| Passing-run durations 4.91–5.65 s at 5 s vs 4.99–5.40 s at 15 s | **yes** | same file, "What the raise costs when nothing is contended" |
| The sweep closes caveat 1 of the experiment README | **yes**, the page says so in terms and quotes the caveat | same file, "This is not `run.sh`" |
| `FaultyStore::contend_next` refuses *n* appends as `AppendError::Store(FaultyStoreError::Contended)` | **yes** | `crates/happenstance-testkit/src/faulty.rs:333`, `:429`, `:556` |
| Both affected rules reject a merely-contended store | **yes**, and the file carries more than this — see Adjudication 4 | `tests/contended_store_instruments.rs:346` |
| §11's falsifier text at `0022:614-616` | **yes** | `references/adr/0022-append-condition-strategy.md` |
| No SPECIFICATION clause quotes the busy timeout | **yes** — `grep` returns nothing | `spec/SPECIFICATION.md` |
| `.kb/open-questions/adr-0022-falsifiers-have-fired.md` exists, `status: accepted` | **yes** | `kb-open-question-adr-0022-falsifiers-fired-001` |
| `.kb/open-questions/no-fixture-tolerance-for-transient-contention.md` exists, `status: accepted` | **yes** | `kb-open-question-testkit-contention-tolerance-001` |
| `kb-governance-what-may-refute-a-finding-001` exists | **yes** | `.kb/governance/what-may-refute-a-finding.md` |
| ADR record `references/adr/0065-*.md` | **no** — none exists, and none is authored by this wave | `unresolved[]` item 2 |

## Merge candidates, scored

Scores are the digest's lexical figure, then this phase's **placement** score — how well the
candidate could actually *own* the claim, which is the number the disposition turns on. A high
lexical score against a candidate that cannot own the claim is the failure mode this table
exists to catch, and Claim 1 is that failure mode twice over.

### Claim 1 — the busy timeout is fifteen seconds (decision)

| Candidate | Lexical | Placement | Why it is or is not the owner |
| --- | ---: | ---: | --- |
| `kb-decision-0022` | 88 | **0** | The subject, and **immutable**. Rule 1 forecloses it absolutely: *"never edit an accepted decision's BODY to absorb a claim."* The atom itself concedes the point — `kb-open-question-adr-0022-falsifiers-fired-001` spends a paragraph on why recording "falsifier fired" inside 0022 would be *"that edit, made worse"* |
| `kb-reference-busy-timeout-margin-001` | 58 | 5 | A `reference` atom, `authority_tier: note`. `decisions/README.md`: a commitment is *"never a `playbook` or a `concept`, because those two carry no immutability"* — the same argument disqualifies a reference. It also binds nothing, and `BUSY_TIMEOUT_MS` is a shipped constant |
| `kb-open-question-adr-0022-falsifiers-fired-001` | 55 | 20 | This is where the claim *came from* and it is not where it goes. `open-questions/README.md`: *"A resolved decision wearing a question mark … filing it here strips it of its authority."* It receives the **amendment** (Claim 2), not the decision |
| `kb-decision-0007` (as precedent, not owner) | 15 | n/a | Cited by the digest for the partial-supersession *shape*, and it is the right precedent. `kb-decision-0031` is the stronger one — it states the consequence out loud: *"which is why `kb-decision-0007` stays accepted and `superseded_by` stays null"* |
| **new decision atom** | — | **95** | **Chosen.** `create_new`, `.kb/decisions/0065-adr-0022-s11-busy-timeout-is-fifteen-seconds.md` |

### Claim 2 — amend the falsifiers-fired open question

| Candidate | Lexical | Placement | Why |
| --- | ---: | ---: | --- |
| `kb-open-question-adr-0022-falsifiers-fired-001` | 96 | **96** | **Chosen.** `merge_existing`. Same atom, same three findings, and its *"What is not decided"* section states the exact fork this wave resolves one third of: *"which of three moves resolves it"*, plus a fourth reading — *"supersede it **in part** … narrower than superseding ADR-0022 and wider than ratifying it"* — which is precisely what ADR-0065 does, transposed from §8 to §11 |
| `kb-decision-0022` | 52 | 0 | Immutable. See above |
| a new open question | — | 5 | Nothing is newly unknown. Two findings stay unknown and they are already recorded here |

### Claim 3 — the contention-tolerance instrument now exists

| Candidate | Lexical | Placement | Why |
| --- | ---: | ---: | --- |
| `kb-open-question-testkit-contention-tolerance-001` | 93 | **93** | **Chosen.** `merge_existing`. Its own closing paragraph names the missing thing almost to the letter: *"The instrument is a decorator over `MemoryEventStore` whose `append` refuses the first `m` callers with a transient store error; it discriminates the two arms as well as gating them."* `SendFaultyStore::contend_next` is that decorator, and it has discriminated them |
| `kb-reference-busy-timeout-margin-001` | 38 | 10 | Cited by the question, not its owner; and a reference atom is a snapshot, not a tracker |
| `kb-decision-0042` (fixture capabilities land defaulted) | — | 0 | Accepted and immutable, and the question is *whether* to buy a tolerance, not where it would land — 0042 already answers the second |

### Claim 4 — the adapter cap sweep (reference)

| Candidate | Lexical | Placement | Why |
| --- | ---: | ---: | --- |
| `kb-reference-busy-timeout-margin-001` | 66 | **25** | **Rejected as owner**, and this is the wave's second-hardest call. Same experiment directory, adjacent subject, and `merge_existing` is the stated default for a candidate scoring this high. It loses on three independent grounds, in `02`, Adjudication 2 |
| `kb-decision-0022` | 42 | 0 | Immutable, and `decisions/README.md` puts evidence outside the decisions layer anyway |
| `kb-reference-one-connection-latency-001` | 20 | 10 | Sibling *pattern* — an adapter-level SQLite measurement in `reference/` — which is an argument for the shape of the new atom, not for merging into this one |
| **new reference atom** | — | **90** | **Chosen.** `create_new`, `.kb/reference/busy-timeout-adapter-cap-sweep-2026-09.md` |

### The fifth destination, which no claim asked for

`kb-open-question-adr-status-vocabulary-001` scores **0** lexically against every claim — the
intake never mentions it. It is nevertheless a destination, because Adjudication 1 produces the
exact artefact that atom is open about. Its body already reads: *"Two more are 'partly
superseded' — ADR-0005 and ADR-0006 — where marking the atom superseded would strip status from
a half that still binds, and marking it accepted hides that a half does not. … What is not
decided is whether that is the corpus's answer or a stopgap."* ADR-0065 is the third instance and
the first at section grain, and it is the first where a staged file **asked in writing for the
other spelling**. An open question about whether a convention is the answer or a stopgap is owed
its next instance. `02`, Adjudication 5, and it is the wave's one discretionary operation.

## What no candidate owns

Nothing in this wave is homeless. Four claims, four destinations, plus the discretionary fifth,
and the two operations that create rather than merge each carry a written reason in `02`. What
*is* left without an owner is listed in `unresolved[]`, and none of it is a claim from this file
— it is the residue the file names and declines: §9's twenty-line reproduction, §8/§16's
unfireable falsifier, and the corpus's inability to spell a scoped supersession in frontmatter.
