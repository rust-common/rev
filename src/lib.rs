pub struct RevIntoIter<T: IntoIterator + Copy>
    where T::IntoIter: DoubleEndedIterator
{
    original: T
}

pub trait Rev: IntoIterator + Copy
    where Self::IntoIter: DoubleEndedIterator
{
    /// ```
    /// use rev::Rev;
    /// let a = [1, 2, 3];
    /// let x = a.rev();
    /// let d: Vec<i32> = x.into_iter().map(|&x| x).collect();
    /// assert_eq!(d, vec!(3, 2, 1));
    /// ```
    fn rev(self) -> RevIntoIter<Self> {
        RevIntoIter { original: self }
    }
}

impl<T: IntoIterator + Copy> Rev for T
    where T::IntoIter: DoubleEndedIterator
{
}

impl<T: IntoIterator + Copy> IntoIterator for RevIntoIter<T>
    where T::IntoIter: DoubleEndedIterator
{
    type Item = T::Item;
    type IntoIter = std::iter::Rev<T::IntoIter>;
    fn into_iter(self) -> Self::IntoIter {
        self.original.into_iter().rev()
    }
}

impl<T: IntoIterator + Copy> Copy for RevIntoIter<T>
    where T::IntoIter: DoubleEndedIterator
{
}

impl<T: IntoIterator + Copy> Clone for RevIntoIter<T>
    where T::IntoIter: DoubleEndedIterator
{
    fn clone(&self) -> Self {
        Self { original: self.original }
    }
}
