use super::*;
use zhc_utils::graphics::{Color, Frame, Size};

macro_rules! zstack_fixed {
    ($name:ident, $n:literal, [$($etype:ident, $efield:ident),*]) => {
        /// Fixed overlay of exactly $n elements sharing the same frame.
        pub struct $name<$($etype),*, C: Class = NoClass> {
            $(pub $efield: $etype,)*
            styler: Styler<C>,
            variable: VariableCell,
        }

        impl<$($etype),*, C: Class> $name<$($etype),*, C> {
            /// Creates a new overlay.
            pub fn new(modifier: Option<StyleModifier>, $($efield: $etype),*) -> Self {
                Self {
                    $($efield,)*
                    styler: Styler::new(modifier),
                    variable: VariableCell::fresh(),
                }
            }
        }

        impl<$($etype),*, C: Class> SceneElement for $name<$($etype),*, C> {
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

        impl<$($etype: SceneSolver),*, C: Class> SceneSolver for $name<$($etype),*, C> {
            fn solve_size(&mut self) {
                let style = self.styler.get();
                $(self.$efield.solve_size();)*
                let size = Size::ZERO
                    $(.union(self.$efield.get_size()))*
                    .pad(style.padding);
                self.variable.set_size(size);
            }

            fn solve_frame(&mut self, available: Frame) {
                let style = self.styler.get();
                let size = self.get_size();
                let frame = available.resize(&size, style.halign, style.valign);
                self.variable.set_frame(frame.clone());
                let inner = frame.crop_around(style.padding);
                $(self.$efield.solve_frame(inner.clone());)*
            }
        }

        impl<$($etype: Renderable),*, C: Class> Renderable for $name<$($etype),*, C> {
            fn render(&self) -> Vec<SvgElement> {
                let style = self.styler.get();
                let frame = self.get_frame();
                let mut elements = Vec::new();

                // Background rect if visible
                if style.fill_color != Color::TRANSPARENT || style.border_color != Color::TRANSPARENT {
                    elements.push(SvgElement::Rect {
                        x: frame.position.x.0,
                        y: frame.position.y.0,
                        width: frame.size.width.0.0,
                        height: frame.size.height.0.0,
                        rx: (style.corner_radius.0 > 0.0).then_some(style.corner_radius.0),
                        fill: Some(style.fill_color.to_string()),
                        stroke: Some(style.border_color.to_string()),
                        stroke_width: Some(style.border_width.0),
                        class: None,
                        id: None,
                        data_val: None,
                    });
                }

                // Render children (overlaid)
                $(elements.extend(self.$efield.render());)*

                elements
            }
        }
    };
}

zstack_fixed!(Z2, 2, [E1, e1, E2, e2]);
