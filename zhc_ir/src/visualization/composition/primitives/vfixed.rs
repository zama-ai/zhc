use super::*;
use std::marker::PhantomData;
use zhc_utils::graphics::{Frame, Height, Remaining, Size, Taken};

macro_rules! vstack_fixed {
    ($name:ident, $n:literal, [$($etype:ident, $efield:ident),*]) => {
        /// Fixed vertical stack of exactly $n elements with spacing.
        pub struct $name<$($etype: Element),*, C: Class = NoClass> {
            $(pub $efield: $etype,)*
            class: PhantomData<C>,
            variable: VariableCell,
        }

        impl<$($etype: Element),*, C: Class> $name<$($etype),*, C> {
            /// Creates a new vertical stack.
            pub fn new($($efield: $etype),*) -> Self {
                Self {
                    $($efield,)*
                    class: PhantomData,
                    variable: VariableCell::fresh(),
                }
            }
        }

        impl<$($etype: Element),*, C: Class> Element for $name<$($etype),*, C> {
            fn solve_size(&mut self, stylesheet: &StyleSheet) {
                let style = stylesheet.get::<C>();
                $(self.$efield.solve_size(stylesheet);)*

                let mut size = Size::ZERO.pad_top(style.padding);
                let mut first = true;
                $(
                    if !first {
                        size = size.pad_bottom(style.spacing);
                    }
                    size = size.stack_vertical(self.$efield.get_size());
                    first = false;
                )*
                size = size.pad_bottom(style.padding);
                self.variable.set_size(size);
            }

            fn solve_frame(&mut self, stylesheet: &StyleSheet, available: Frame) {
                let style = stylesheet.get::<C>();
                let size = self.get_size();
                let frame = available.resize(&size, style.halign, style.valign);
                self.variable.set_frame(frame.clone());

                let mut remaining = frame.crop_top(Height(style.padding));
                let mut first = true;
                $(
                    if !first {
                        remaining = remaining.crop_top(Height(style.spacing));
                    }
                    let (Taken(available), Remaining(new_remaining)) =
                        remaining.take_top(self.$efield.get_size().height);
                    self.$efield.solve_frame(stylesheet, available);
                    remaining = new_remaining;
                    #[allow(unused_assignments)]
                    {
                        first = false;
                    }
                )*
                let remaining = remaining.crop_top(Height(style.padding));
                remaining.assert_collapsed();
            }

            fn get_size(&self) -> Size {
                self.variable.get_size()
            }

            fn get_frame(&self) -> Frame {
                self.variable.get_frame()
            }

            fn get_variable_cell(&self) -> VariableCell {
                self.variable.clone()
            }
        }
    };
}

vstack_fixed!(V3, 3, [E1, e1, E2, e2, E3, e3]);
vstack_fixed!(V4, 4, [E1, e1, E2, e2, E3, e3, E4, e4]);
