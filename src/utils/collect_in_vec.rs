pub trait CollectInVec<T> {
    fn covect(self) -> Vec<T>;
}

impl<T, I> CollectInVec<T> for I
where
    I: Iterator<Item = T>,
{
    fn covect(self) -> Vec<T> {
        self.collect()
    }
}
