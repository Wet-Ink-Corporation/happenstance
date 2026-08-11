pub struct Borrowing<'a>(&'a ());

impl tcrate::Store for Borrowing<'_> {
    type Batch<'a>
        = ()
    where
        Self: 'a;

    fn commit(&self, _batch: Self::Batch<'_>) -> impl Sized {}
}
