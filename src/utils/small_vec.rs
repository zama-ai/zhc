// For now I just do an alias, but I'll have to implement that correctly.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SmallVec<A>(pub Vec<A>);

impl<A> SmallVec<A> {
    pub fn as_slice(&self) -> &[A] {
        &self.0
    }

    pub fn push(&mut self, value: A) {
        self.0.push(value);
    }

    pub fn append(&mut self, other: &mut SmallVec<A>) {
        self.0.append(&mut other.0);
    }

    pub fn iter(&self) -> std::slice::Iter<'_, A> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, A> {
        self.0.iter_mut()
    }

    pub fn into_iter(self) -> std::vec::IntoIter<A> {
        self.0.into_iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<A> std::ops::Index<usize> for SmallVec<A> {
    type Output = A;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<A> std::ops::IndexMut<usize> for SmallVec<A> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<A> std::iter::FromIterator<A> for SmallVec<A> {
    fn from_iter<I: IntoIterator<Item = A>>(iter: I) -> Self {
        SmallVec(Vec::from_iter(iter))
    }
}

impl<A> std::iter::Extend<A> for SmallVec<A> {
    fn extend<I: IntoIterator<Item = A>>(&mut self, iter: I) {
        self.0.extend(iter);
    }
}

#[macro_export]
macro_rules! svec {
    () => {
        $crate::utils::SmallVec(vec![])
    };
    ($elem:expr; $n:expr) => {
        $crate::utils::SmallVec(vec![$elem; $n])
    };
    ($($x:expr),+ $(,)?) => {
        $crate::utils::SmallVec(vec![$($x),+])
    };
}
