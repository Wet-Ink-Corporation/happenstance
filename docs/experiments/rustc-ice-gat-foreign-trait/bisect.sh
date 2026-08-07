#!/bin/sh
# Regenerates the bisection table in README.md.
#
# Compiles the reproduction and five variants, each removing exactly one
# ingredient, and classifies the outcome. The table is a result rather than a
# recollection: run this and it either still says what the README says or the
# compiler has changed under us, which is the thing worth knowing.
#
# Usage:  ./bisect.sh [rustc]        e.g.  ./bisect.sh 'rustup run 1.85 rustc'
set -eu

RUSTC="${1:-rustc}"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

echo "toolchain: $($RUSTC --version)"
echo

# $1 = label, $2 = key, $3 = trait crate source, $4 = impl crate source
probe() {
    label="$1" key="$2"
    printf '%s\n' "$3" > "$WORK/t_$key.rs"
    printf '%s\n' "$4" > "$WORK/i_$key.rs"

    # The first compilation can itself be the result: the single-crate variant
    # puts the trait and the impl in one file, and that is precisely the case
    # where rustc reports E0477 instead of crashing. Classify whichever
    # compilation produced output rather than assuming it is the second.
    out=$($RUSTC --edition 2024 --crate-type lib "$WORK/t_$key.rs" \
              -o "$WORK/libt_$key.rlib" 2>&1 || true)
    if [ -z "$out" ]; then
        # `-o` into the scratch directory, not merely `-L`: without it rustc
        # writes the impl crate's .rlib into the *current* directory, and a
        # script whose side effect is littering the repository is a script
        # nobody runs twice.
        out=$($RUSTC --edition 2024 --crate-type lib "$WORK/i_$key.rs" \
                  -o "$WORK/libi_$key.rlib" \
                  --extern tcrate="$WORK/libt_$key.rlib" -L "$WORK" 2>&1 || true)
    fi

    case $out in
        *'unexpectedly panicked'*) verdict='ICE' ;;
        *E0477*)                   verdict='clean E0477, no ICE' ;;
        *error*)                   verdict="other: $(printf '%s' "$out" | grep -m1 '^error')" ;;
        *)                         verdict='compiles clean' ;;
    esac
    printf '%-46s %s\n' "$label" "$verdict"
}

TRAIT_FULL='pub trait Store {
    type Batch<'"'"'a>
    where
        Self: '"'"'a;

    fn commit(&self, batch: Self::Batch<'"'"'_>) -> impl Sized;
}'
IMPL_FULL='pub struct Borrowing<'"'"'a>(&'"'"'a ());

impl tcrate::Store for Borrowing<'"'"'_> {
    type Batch<'"'"'a>
        = ()
    where
        Self: '"'"'a;

    fn commit(&self, _batch: Self::Batch<'"'"'_>) -> impl Sized {}
}'

probe 'the reproduction, unmodified' base "$TRAIT_FULL" "$IMPL_FULL"

# 1. The crate split. Both halves compiled as ONE crate; the impl crate is a
#    stub so the harness above still has two compilations to run.
probe 'remove the crate split' split \
  'pub trait Store { type Batch<'"'"'a> where Self: '"'"'a; fn commit(&self, batch: Self::Batch<'"'"'_>) -> impl Sized; }
pub struct Borrowing<'"'"'a>(&'"'"'a ());
impl Store for Borrowing<'"'"'_> { type Batch<'"'"'a> = () where Self: '"'"'a; fn commit(&self, _b: Self::Batch<'"'"'_>) -> impl Sized {} }' \
  'pub fn nothing() {}'

# 2. The GAT's `where Self: 'a` clause.
probe "remove \`where Self: 'a\`" wsl \
  'pub trait Store { type Batch<'"'"'a>; fn commit(&self, batch: Self::Batch<'"'"'_>) -> impl Sized; }' \
  'pub struct Borrowing<'"'"'a>(&'"'"'a ());
impl tcrate::Store for Borrowing<'"'"'_> { type Batch<'"'"'a> = (); fn commit(&self, _b: Self::Batch<'"'"'_>) -> impl Sized {} }'

# 3. The return-position impl Trait. This is the one `async fn` supplies.
probe 'remove the RPITIT return' rpitit \
  'pub trait Store { type Batch<'"'"'a> where Self: '"'"'a; fn commit(&self, batch: Self::Batch<'"'"'_>); }' \
  'pub struct Borrowing<'"'"'a>(&'"'"'a ());
impl tcrate::Store for Borrowing<'"'"'_> { type Batch<'"'"'a> = () where Self: '"'"'a; fn commit(&self, _b: Self::Batch<'"'"'_>) {} }'

# 4. The impl self type's lifetime.
probe "remove the impl self type's lifetime" static \
  "$TRAIT_FULL" \
  'pub struct Plain;
impl tcrate::Store for Plain { type Batch<'"'"'a> = () where Self: '"'"'a; fn commit(&self, _b: Self::Batch<'"'"'_>) -> impl Sized {} }'

# 5. The GAT's lifetime parameter.
probe "remove the GAT's lifetime parameter" gat \
  'pub trait Store { type Batch; fn commit(&self, batch: Self::Batch) -> impl Sized; }' \
  'pub struct Borrowing<'"'"'a>(&'"'"'a ());
impl tcrate::Store for Borrowing<'"'"'_> { type Batch = (); fn commit(&self, _b: Self::Batch) -> impl Sized {} }'
