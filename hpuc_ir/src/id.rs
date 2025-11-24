use std::ops::{Add, Sub};

use hpuc_utils::StoreIndex;

macro_rules! impl_index {
    ($name: ident, $raw: ident, $raw_type: ident) => {
        pub type $raw = $raw_type;

        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(pub(super) $raw);

        impl Add<$raw> for $name {
            type Output = $name;

            fn add(self, rhs: $raw) -> Self::Output {
                $name(self.0 + rhs)
            }
        }

        impl Sub<$raw> for $name {
            type Output = $name;

            fn sub(self, rhs: $raw) -> Self::Output {
                $name(self.0 - rhs)
            }
        }

        impl $name {
            pub fn range(start: $raw, end: $raw) -> impl Iterator<Item = $name> {
                (start..end).map(|a| $name(a))
            }
        }

        impl StoreIndex for $name {
            type Raw = $raw;
            fn as_usize(&self) -> usize {
                self.0 as usize
            }
            fn as_raw(&self) -> $raw {
                self.0
            }
            fn raw_from_usize(val: usize) -> $raw {
                val as $raw
            }
            fn from_usize(val: usize) -> $name {
                $name(val as $raw)
            }
        }

        impl From<$name> for usize {
            fn from(value: $name) -> Self {
                <$name as StoreIndex>::as_usize(&value)
            }
        }
    };
}

impl_index!(OpId, OpIdRaw, u16);
impl_index!(ValId, ValIdRaw, u16);
impl_index!(ValueNumber, ValueNumberRaw, u16);
