use super::*;
use zhc_utils::graphics::{Frame, Size};

macro_rules! discriminated_union {
    ($name:ident, [$($etype:ident),*]) => {
        /// Discriminated union for runtime polymorphism.
        pub enum $name<$($etype),*> {
            $($etype($etype),)*
        }

        impl<$($etype: SceneElement),*> SceneElement for $name<$($etype),*> {
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

        impl<$($etype: SceneSolver),*> SceneSolver for $name<$($etype),*> {
            fn solve_size(&mut self) {
                match self {
                    $($name::$etype(e) => e.solve_size(),)*
                }
            }

            fn solve_frame(&mut self, available: Frame) {
                match self {
                    $($name::$etype(e) => e.solve_frame(available),)*
                }
            }
        }

        impl<$($etype: Renderable),*> Renderable for $name<$($etype),*> {
            fn render(&self) -> Vec<SvgElement> {
                match self {
                    $($name::$etype(e) => e.render(),)*
                }
            }
        }
    };
}

discriminated_union!(D2, [E1, E2]);
discriminated_union!(D7, [E1, E2, E3, E4, E5, E6, E7]);
