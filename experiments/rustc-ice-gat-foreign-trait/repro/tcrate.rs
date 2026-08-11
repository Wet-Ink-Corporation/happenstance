pub trait Store {
    type Batch<'a>
    where
        Self: 'a;

    fn commit(&self, batch: Self::Batch<'_>) -> impl Sized;
}
