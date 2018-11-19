pub struct RevIntoIter<T>
    where
        T: IntoIterator + Copy,
        T::IntoIter: DoubleEndedIterator,
{
    original: T
}

pub trait Rev: IntoIterator + Copy
    where Self::IntoIter: DoubleEndedIterator {
    fn rev(self) -> RevIntoIter<Self>;
}

impl<T> Rev for T
    where
        T: IntoIterator + Copy,
        T::IntoIter: DoubleEndedIterator,
{
    fn rev(self) -> RevIntoIter<Self> { RevIntoIter { original: self } }
}

impl<T> IntoIterator for RevIntoIter<T>
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

impl<T> Copy for RevIntoIter<T>
where
    T: IntoIterator + Copy,
    T::IntoIter: DoubleEndedIterator
{
}

impl<T> Clone for RevIntoIter<T>
where
    T: IntoIterator + Copy,
    T::IntoIter: DoubleEndedIterator
{
    fn clone(&self) -> Self {
        Self { original: self.original }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let a = [1, 2, 3];
        let x = a.rev();
        let d: Vec<i32> = x.into_iter().map(|&x| x).collect();
        assert_eq!(d, vec!(3, 2, 1));
    }
}
