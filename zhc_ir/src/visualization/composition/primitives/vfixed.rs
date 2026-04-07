use super::*;
use zhc_utils::graphics::{Color, Frame, Height, Remaining, Size, Taken};

macro_rules! vstack_fixed {
    ($name:ident, $n:literal, [$($etype:ident, $efield:ident),*]) => {
        /// Fixed vertical stack of exactly $n elements with spacing.
        pub struct $name<$($etype),*, C: Class = NoClass> {
            $(pub $efield: $etype,)*
            styler: Styler<C>,
            variable: VariableCell,
        }

        impl<$($etype),*, C: Class> $name<$($etype),*, C> {
            /// Creates a new vertical stack.
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

                let mut size = Size::ZERO.pad_top(style.padding);
                let mut has_content = false;
                $(
                    let child_size = self.$efield.get_size();
                    if child_size.height.0.0 > 0.0 {
                        if has_content {
                            size = size.pad_bottom(style.spacing);
                        }
                        size = size.stack_vertical(child_size);
                        #[allow(unused_assignments)]
                        { has_content = true; }
                    }
                )*
                size = size.pad_bottom(style.padding);
                self.variable.set_size(size);
            }

            fn solve_frame(&mut self, available: Frame) {
                let style = self.styler.get();
                let size = self.get_size();
                let frame = available.resize(&size, style.halign, style.valign);
                self.variable.set_frame(frame.clone());

                let mut remaining = frame.crop_top(Height(style.padding));
                let mut has_content = false;
                $(
                    let child_height = self.$efield.get_size().height;
                    if child_height.0.0 > 0.0 {
                        if has_content {
                            remaining = remaining.crop_top(Height(style.spacing));
                        }
                        let (Taken(child_frame), Remaining(new_remaining)) =
                            remaining.take_top(child_height);
                        self.$efield.solve_frame(child_frame);
                        remaining = new_remaining;
                        #[allow(unused_assignments)]
                        { has_content = true; }
                    } else {
                        // Zero-height child still needs a frame (collapsed)
                        self.$efield.solve_frame(remaining.take_top(Height::ZERO).0.0);
                    }
                )*
                let remaining = remaining.crop_top(Height(style.padding));
                remaining.assert_collapsed();
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

                // Collect non-zero-height child frames for separator rendering
                let child_frames: Vec<Frame> = vec![$(self.$efield.get_frame()),*]
                    .into_iter()
                    .filter(|f| f.size.height.0.0 > 0.0)
                    .collect();

                // Render separators between non-empty children if enabled
                if style.draw_separators && child_frames.len() > 1 {
                    for i in 0..child_frames.len() - 1 {
                        let sep_y = (child_frames[i].bottom_left().y.0 + child_frames[i + 1].top_left().y.0) / 2.0;
                        elements.push(SvgElement::Rect {
                            x: frame.position.x.0,
                            y: sep_y - style.border_width.0 / 2.0,
                            width: frame.size.width.0.0,
                            height: style.border_width.0,
                            rx: None,
                            fill: Some(style.border_color.to_string()),
                            stroke: None,
                            stroke_width: None,
                            class: None,
                            id: None,
                            data_val: None,
                        });
                    }
                }

                // Render children
                $(elements.extend(self.$efield.render());)*

                elements
            }
        }
    };
}

vstack_fixed!(V3, 3, [E1, e1, E2, e2, E3, e3]);
vstack_fixed!(V4, 4, [E1, e1, E2, e2, E3, e3, E4, e4]);
