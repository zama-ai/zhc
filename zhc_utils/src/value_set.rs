use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ValueSet {
    set: u128,
    n_bits: u8,
}

impl ValueSet {
    pub fn new_empty(n_bits: u8) -> Self {
        assert!(n_bits <= 7);
        ValueSet { set: 0, n_bits }
    }

    pub fn new_full(n_bits: u8) -> Self {
        assert!(n_bits <= 7);
        let shift = 1u32 << n_bits;
        let mask = if shift == 128 { u128::MAX } else { (1u128 << shift) - 1 };
        ValueSet { set: mask, n_bits }
    }

    pub fn from_single(n_bits: u8, value: u8) -> Self {
        let mut vs = Self::new_empty(n_bits);
        vs.insert(value);
        vs
    }

    pub fn n_bits(&self) -> u8 {
        self.n_bits
    }

    pub fn domain_max(&self) -> u8 {
        ((1u16 << self.n_bits) - 1) as u8
    }

    pub fn insert(&mut self, value: u8) {
        debug_assert!(value <= self.domain_max());
        self.set |= 1u128 << value;
    }

    pub fn remove(&mut self, value: u8) {
        debug_assert!(value <= self.domain_max());
        self.set &= !(1u128 << value);
    }

    pub fn contains(&self, value: u8) -> bool {
        (self.set >> value) & 1 != 0
    }

    pub fn is_empty(&self) -> bool {
        self.set == 0
    }

    pub fn count(&self) -> u32 {
        self.set.count_ones()
    }

    pub fn min(&self) -> Option<u8> {
        if self.is_empty() {
            None
        } else {
            Some(self.set.trailing_zeros() as u8)
        }
    }

    pub fn max(&self) -> Option<u8> {
        if self.is_empty() {
            None
        } else {
            Some(127 - self.set.leading_zeros() as u8)
        }
    }

    pub fn iter_values(&self) -> impl Iterator<Item = u8> {
        let set = self.set;
        (0..=self.domain_max()).filter(move |&v| (set >> v) & 1 != 0)
    }

    pub fn union(self, other: Self) -> Self {
        debug_assert_eq!(self.n_bits, other.n_bits);
        ValueSet { set: self.set | other.set, n_bits: self.n_bits }
    }

    pub fn intersection(self, other: Self) -> Self {
        debug_assert_eq!(self.n_bits, other.n_bits);
        ValueSet { set: self.set & other.set, n_bits: self.n_bits }
    }
}

impl fmt::Debug for ValueSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = self.n_bits as u32;
        let total = 1u32 << n;
        let count = self.count();

        if f.alternate() {
            let width = 1u32 << ((n + 1) / 2);
            let height = 1u32 << (n / 2);
            for row in 0..height {
                if row > 0 {
                    writeln!(f)?;
                }
                for col in 0..width {
                    let val = row * width + col;
                    if val < total && self.contains(val as u8) {
                        write!(f, "\u{2593}\u{2593}")?;
                    } else {
                        write!(f, "\u{2591}\u{2591}")?;
                    }
                }
            }
            Ok(())
        } else {
            write!(f, "ValueSet({}b {}/{} ", n, count, total)?;
            for v in 0..total as u8 {
                if self.contains(v) {
                    write!(f, "\u{2593}")?;
                } else {
                    write!(f, "\u{2591}")?;
                }
            }
            write!(f, ")")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_set() {
        let vs = ValueSet::new_empty(3);
        assert!(vs.is_empty());
        assert_eq!(vs.count(), 0);
        assert_eq!(vs.min(), None);
        assert_eq!(vs.max(), None);
        assert_eq!(vs.iter_values().count(), 0);
    }

    #[test]
    fn full_set() {
        let vs = ValueSet::new_full(3);
        assert!(!vs.is_empty());
        assert_eq!(vs.count(), 8);
        assert_eq!(vs.min(), Some(0));
        assert_eq!(vs.max(), Some(7));
        assert_eq!(vs.domain_max(), 7);
        let vals: Vec<u8> = vs.iter_values().collect();
        assert_eq!(vals, (0..=7).collect::<Vec<_>>());
    }

    #[test]
    fn full_set_1bit() {
        let vs = ValueSet::new_full(1);
        assert_eq!(vs.count(), 2);
        assert_eq!(vs.domain_max(), 1);
        assert!(vs.contains(0));
        assert!(vs.contains(1));
    }

    #[test]
    fn full_set_7bit() {
        let vs = ValueSet::new_full(7);
        assert_eq!(vs.count(), 128);
        assert_eq!(vs.domain_max(), 127);
    }

    #[test]
    fn from_single() {
        let vs = ValueSet::from_single(4, 10);
        assert_eq!(vs.count(), 1);
        assert!(vs.contains(10));
        assert!(!vs.contains(0));
        assert_eq!(vs.min(), Some(10));
        assert_eq!(vs.max(), Some(10));
    }

    #[test]
    fn insert_remove() {
        let mut vs = ValueSet::new_empty(3);
        vs.insert(2);
        vs.insert(5);
        assert!(vs.contains(2));
        assert!(vs.contains(5));
        assert!(!vs.contains(3));
        assert_eq!(vs.count(), 2);

        vs.remove(2);
        assert!(!vs.contains(2));
        assert_eq!(vs.count(), 1);

        vs.remove(5);
        assert!(vs.is_empty());
    }

    #[test]
    fn min_max() {
        let mut vs = ValueSet::new_empty(4);
        vs.insert(3);
        vs.insert(9);
        vs.insert(1);
        assert_eq!(vs.min(), Some(1));
        assert_eq!(vs.max(), Some(9));
    }

    #[test]
    fn iter_values_order() {
        let mut vs = ValueSet::new_empty(4);
        vs.insert(7);
        vs.insert(2);
        vs.insert(12);
        let vals: Vec<u8> = vs.iter_values().collect();
        assert_eq!(vals, vec![2, 7, 12]);
    }

    #[test]
    fn union() {
        let a = ValueSet::from_single(3, 1);
        let b = ValueSet::from_single(3, 5);
        let u = a.union(b);
        assert_eq!(u.count(), 2);
        assert!(u.contains(1));
        assert!(u.contains(5));
    }

    #[test]
    fn intersection() {
        let mut a = ValueSet::new_empty(3);
        a.insert(1);
        a.insert(3);
        a.insert(5);
        let mut b = ValueSet::new_empty(3);
        b.insert(3);
        b.insert(5);
        b.insert(7);
        let i = a.intersection(b);
        assert_eq!(i.count(), 2);
        assert!(i.contains(3));
        assert!(i.contains(5));
        assert!(!i.contains(1));
        assert!(!i.contains(7));
    }

    #[test]
    fn intersection_disjoint() {
        let a = ValueSet::from_single(3, 0);
        let b = ValueSet::from_single(3, 7);
        let i = a.intersection(b);
        assert!(i.is_empty());
    }

    #[test]
    fn debug_compact() {
        assert_eq!(
            format!("{:?}", ValueSet::new_empty(3)),
            "ValueSet(3b 0/8 ░░░░░░░░)"
        );
        assert_eq!(
            format!("{:?}", ValueSet::new_full(3)),
            "ValueSet(3b 8/8 ▓▓▓▓▓▓▓▓)"
        );
        let mut vs = ValueSet::new_empty(4);
        for v in [0, 1, 3, 4, 5, 6, 7, 9, 11, 12] {
            vs.insert(v);
        }
        assert_eq!(
            format!("{:?}", vs),
            "ValueSet(4b 10/16 ▓▓░▓▓▓▓▓░▓░▓▓░░░)"
        );
    }

    #[test]
    fn debug_grid() {
        let mut vs = ValueSet::new_empty(4);
        for v in [0, 1, 3, 4, 5, 6, 7, 9, 11, 12] {
            vs.insert(v);
        }
        assert_eq!(
            format!("{:#?}", vs),
            "\
▓▓▓▓░░▓▓
▓▓▓▓▓▓▓▓
░░▓▓░░▓▓
▓▓░░░░░░"
        );
    }

    #[test]
    #[should_panic]
    fn new_empty_rejects_8bits() {
        ValueSet::new_empty(8);
    }

    #[test]
    #[should_panic]
    fn new_full_rejects_8bits() {
        ValueSet::new_full(8);
    }
}
