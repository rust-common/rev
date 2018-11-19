
pub struct Rev<T>
where
    T: IntoIterator + Copy,
    T::IntoIter: DoubleEndedIterator,
{
    original: T
}

impl<T> Rev<T>
where
    T: IntoIterator + Copy,
    T::IntoIter: DoubleEndedIterator
{
    pub fn new(v: T) -> Self { Self { original: v } }
}

impl<T> IntoIterator for Rev<T>
where
    T: IntoIterator + Copy,
    T::IntoIter: DoubleEndedIterator
{
    type Item = T::Item;
    type IntoIter = std::iter::Rev<T::IntoIter>;
    fn into_iter(self) -> Self::IntoIter {
        self.original.into_iter().rev()
    }
}

impl<T> Copy for Rev<T>
where
    T: IntoIterator + Copy,
    T::IntoIter: DoubleEndedIterator
{
}

impl<T> Clone for Rev<T>
where
    T: IntoIterator + Copy,
    T::IntoIter: DoubleEndedIterator
{
    fn clone(&self) -> Self {
        Self::new(self.original)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let a = [1, 2, 3];
        let x = Rev::new(&a);
        let d: Vec<i32> = x.into_iter().map(|&x| x).collect();
        assert_eq!(d, vec!(3, 2, 1));
    }
}
