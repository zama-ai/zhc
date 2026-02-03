use std::{
    fmt::Display,
    ops::{Add, Sub},
};

use hc_utils::StoreIndex;

/// Generates a typed identifier with arithmetic operations and store indexing support.
///
/// Creates a strongly-typed wrapper around a raw numeric type that can be used
/// as an index into stores while preventing mixing of different ID types.
/// The generated type supports basic arithmetic operations and range generation.
macro_rules! impl_index {
    ($name: ident, $raw: ident, $raw_type: ident, $doc: expr) => {
        pub type $raw = $raw_type;

        #[doc = $doc]
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
            /// Creates an iterator over a range of identifiers from `start` to `end`.
            pub fn range(start: $raw, end: $raw) -> impl DoubleEndedIterator<Item = $name> {
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

impl_index!(
    OpId,
    OpIdRaw,
    u16,
    "Identifier for operations within an IR."
);
impl_index!(ValId, ValIdRaw, u16, "Identifier for values within an IR.");
impl_index!(
    ValueNumber,
    ValueNumberRaw,
    u16,
    "Identifier used in value numbering for optimization passes."
);

impl Display for ValId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            // Alternate is an inactive valid
            write!(f, "%_{}", self.0)
        } else {
            write!(f, "%{}", self.0)
        }
    }
}

impl Display for OpId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(width) = f.width() {
            write!(f, "@{:0width$}", self.0, width = width)
        } else {
            write!(f, "@{}", self.0)
        }
    }
}
