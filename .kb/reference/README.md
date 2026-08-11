# Reference layer — measurements, censuses and pointers

What was true, when it was measured, and where the evidence is: `reference` atoms carrying
`authority_tier: note`. A reference atom binds nothing. Its job is to be *citable* — so that a
decision can rest on a number without restating how the number was obtained.

## What belongs here

- a **measurement** — what was run, on what, on what date, and what it returned;
- a **census** — a count of some state of the corpus at a commit, with the tool that produced
  it, so a later count is comparable;
- a **pointer** — an index into evidence that lives somewhere else and should not be copied.

The pointer case is the one most often got wrong. When the evidence is large and already
written down, the atom names it and says what it establishes; it does not reproduce it. Two
copies of a measurement is two things to update and one that quietly goes stale, and the stale
one is the one that gets cited.

## The dating rule

**Every atom here is a statement about a moment.** Say which moment. A measurement without a
commit sha or a date is not a weaker reference, it is a false one: it reads as current, it is
cited as current, and nothing in the repository can detect that it stopped being true.

Put the sha or the date in the body, not only in `last_reviewed` — `last_reviewed` says when
someone looked at the atom, which is a different fact from when the measurement was taken.

## What does not belong here

**A conclusion drawn from the measurement.** The number goes here; what the project decided
because of it is a `decision` atom that cites this one. Keeping them apart is what lets a
decision be superseded without invalidating the evidence underneath it, and what lets the same
measurement support a second decision later.

**Anything still being edited.** A reference atom is a snapshot. If the thing it describes is
expected to change, what belongs here is the census taken at a stated point, not a mirror that
someone is expected to keep current — nobody will, and a mirror nobody maintains is worse than
no mirror because it looks maintained.

## Why this layer exists

Decisions in this repository are required to rest on evidence rather than argument. That only
works if the evidence outlives the pass that produced it and can be cited by name. Without a
home, a measurement lives in the commit message of the change it justified — findable only by
someone who already knows it exists, which is exactly the person who does not need it.
