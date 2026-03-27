use super::*;
use zhc_utils::graphics::{Frame, Size};

macro_rules! discriminated_union {
    ($name:ident, [$($etype:ident),*]) => {
        /// Discriminated union for runtime polymorphism.
        pub enum $name<$($etype: Element),*> {
            $($etype($etype),)*
        }

        impl<$($etype: Element),*> Element for $name<$($etype),*> {
            fn solve_size(&mut self, stylesheet: &StyleSheet) {
                match self {
                    $($name::$etype(e) => e.solve_size(stylesheet),)*
                }
            }

            fn solve_frame(&mut self, stylesheet: &StyleSheet, available: Frame) {
                match self {
                    $($name::$etype(e) => e.solve_frame(stylesheet, available),)*
                }
            }

            fn get_size(&self) -> Size {
                match self {
                    $($name::$etype(e) => e.get_size(),)*
                }
            }

            fn get_frame(&self) -> Frame {
                match self {
                    $($name::$etype(e) => e.get_frame(),)*
                }
            }

            fn get_variable_cell(&self) -> VariableCell {
                match self {
                    $($name::$etype(e) => e.get_variable_cell(),)*
                }
            }
        }
    };
}

discriminated_union!(D2, [E1, E2]);
discriminated_union!(D7, [E1, E2, E3, E4, E5, E6, E7]);
