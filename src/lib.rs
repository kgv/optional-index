#![no_std]

/// Optional immutable index.
pub trait OptionalIndex<I>
where
    I: ?Sized,
{
    type Output: Clone + ?Sized;

    fn optional_index(&self, index: I) -> Option<&Self::Output>;
}

/// Optional mutable index.
pub trait OptionalIndexMut<I>: OptionalIndex<I>
where
    I: ?Sized,
{
    fn optional_index_mut(&mut self, index: I) -> Option<&mut Self::Output>;
}

impl<T, I> OptionalIndex<I> for &T
where
    T: OptionalIndex<I>,
{
    type Output = <T as OptionalIndex<I>>::Output;

    fn optional_index(&self, index: I) -> Option<&Self::Output> {
        (*self).optional_index(index)
    }
}

impl<T, I> OptionalIndex<I> for &mut T
where
    T: OptionalIndex<I>,
{
    type Output = <T as OptionalIndex<I>>::Output;

    fn optional_index(&self, index: I) -> Option<&Self::Output> {
        (**self).optional_index(index)
    }
}

impl<T, I> OptionalIndexMut<I> for &mut T
where
    T: OptionalIndexMut<I>,
{
    fn optional_index_mut(&mut self, index: I) -> Option<&mut Self::Output> {
        (*self).optional_index_mut(index)
    }
}

impl<T, I> OptionalIndex<I> for Option<T>
where
    T: OptionalIndex<I>,
{
    type Output = <T as OptionalIndex<I>>::Output;

    fn optional_index(&self, index: I) -> Option<&Self::Output> {
        self.as_ref().and_then(|t| t.optional_index(index))
    }
}

impl<T, I> OptionalIndexMut<I> for Option<T>
where
    T: OptionalIndexMut<I>,
{
    fn optional_index_mut(&mut self, index: I) -> Option<&mut Self::Output> {
        self.as_mut().and_then(|t| t.optional_index_mut(index))
    }
}
