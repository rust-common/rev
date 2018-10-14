
pub struct Rev<T, I> 
where 
    T: IntoIterator<Item = I> + Copy,
    T::IntoIter: DoubleEndedIterator, 
{
    pub original: T
}

pub fn rev<T, I>(v: T) -> Rev<T, I> 
where 
    T: IntoIterator<Item = I> + Copy,
    T::IntoIter: DoubleEndedIterator
{
    Rev { original: v }
}

impl<T, I> IntoIterator for Rev<T, I> 
where 
    T: IntoIterator<Item = I> + Copy,
    T::IntoIter: DoubleEndedIterator 
{
    type Item = I;
    type IntoIter = std::iter::Rev<T::IntoIter>;
    fn into_iter(self) -> Self::IntoIter {
        self.original.into_iter().rev()
    }
}

impl<T, I> Copy for Rev<T, I> 
where
    T: IntoIterator<Item = I> + Copy,
    T::IntoIter: DoubleEndedIterator 
{
}

impl<T, I> Clone for Rev<T, I> 
where 
    T: IntoIterator<Item = I> + Copy,
    T::IntoIter: DoubleEndedIterator 
{
    fn clone(&self) -> Self {
        rev(self.original)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let a = [1, 2, 3];
        let x = rev(&a);
        let d: Vec<i32> = x.into_iter().map(|&x| x).collect();
        assert_eq!(d, vec!(3, 2, 1));
    }
}
