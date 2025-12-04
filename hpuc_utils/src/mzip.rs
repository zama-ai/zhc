pub trait MultiZip {
    type Zipped;

    fn mzip(self) -> Self::Zipped;
}
macro_rules! impl_multizip {
    ($n:literal, $zip_struct:ident, ($($generic:ident),+), ($($field:tt),+), ($($param:ident),+)) => {
        pub struct $zip_struct<$($generic: Iterator),+>($($generic),+);

        impl<$($generic: Iterator),+> Iterator for $zip_struct<$($generic),+> {
            type Item = ($($generic::Item),+);

            fn next(&mut self) -> Option<Self::Item> {
                match ($(self.$field.next()),+) {
                    ($(Some($param)),+) => Some(($($param),+)),
                    _ => None
                }
            }
        }

        impl<$($generic: Iterator),+> MultiZip for ($($generic),+) {
            type Zipped = $zip_struct<$($generic),+>;

            fn mzip(self) -> Self::Zipped {
                $zip_struct($(self.$field),+)
            }
        }
    };
}

impl_multizip!(2, Zip2, (A, B), (0, 1), (a, b));
impl_multizip!(3, Zip3, (A, B, C), (0, 1, 2), (a, b, c));
impl_multizip!(4, Zip4, (A, B, C, D), (0, 1, 2, 3), (a, b, c, d));
impl_multizip!(5, Zip5, (A, B, C, D, E), (0, 1, 2, 3, 4), (a, b, c, d, e));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zip3_basic() {
        let iter1 = vec![1, 2, 3].into_iter();
        let iter2 = vec!['a', 'b', 'c'].into_iter();
        let iter3 = vec![true, false, true].into_iter();

        let zipped: Vec<_> = (iter1, iter2, iter3).mzip().collect();

        assert_eq!(zipped, vec![(1, 'a', true), (2, 'b', false), (3, 'c', true)]);
    }

    #[test]
    fn test_zip3_different_lengths() {
        let iter1 = vec![1, 2].into_iter();
        let iter2 = vec!['a', 'b', 'c'].into_iter();
        let iter3 = vec![true].into_iter();

        let zipped: Vec<_> = (iter1, iter2, iter3).mzip().collect();

        // Should stop at the shortest iterator
        assert_eq!(zipped, vec![(1, 'a', true)]);
    }

    #[test]
    fn test_zip3_empty() {
        let iter1 = Vec::<usize>::new().into_iter();
        let iter2 = vec!['a', 'b'].into_iter();
        let iter3 = vec![true, false].into_iter();

        let zipped: Vec<_> = (iter1, iter2, iter3).mzip().collect();

        assert_eq!(zipped, vec![]);
    }
}
