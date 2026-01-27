use std::ops::{Deref, DerefMut};

/// A smart pointer that tracks changes to a value when dropped.
pub struct ChangeGuard<'a, T: PartialEq> {
    value: &'a mut T,
    original: T,
    changed_flag: &'a mut bool,
}

impl<'a, T: PartialEq> ChangeGuard<'a, T> {
    /// Creates a new change guard that monitors modifications to the given value.
    pub fn new(value: &'a mut T, changed_flag: &'a mut bool) -> Self
    where
        T: Clone,
    {
        let original = value.clone();
        ChangeGuard {
            value,
            original,
            changed_flag,
        }
    }
}
impl<'a, T: PartialEq> Deref for ChangeGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<'a, T: PartialEq> DerefMut for ChangeGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value
    }
}

impl<'a, T> Drop for ChangeGuard<'a, T>
where
    T: PartialEq,
{
    fn drop(&mut self) {
        if *self.value != self.original {
            *self.changed_flag = true;
        }
    }
}
