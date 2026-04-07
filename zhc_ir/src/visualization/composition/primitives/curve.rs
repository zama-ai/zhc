use super::*;
use crate::ValId;
use crate::visualization::svg::PathCommand;
use zhc_utils::graphics::{Frame, Position, Size, Y};

/// A curve connecting points through waypoints.
/// Reads positions from watched cells at render time.
pub struct Curve<C: Class = CurveClass> {
    pub waypoints: Vec<VariableWatch>,
    val_id: Option<ValId>,
    styler: Styler<C>,
    variable: VariableCell,
}

impl<C: Class> Curve<C> {
    pub fn new(
        modifier: Option<StyleModifier>,
        waypoints: Vec<VariableWatch>,
        val_id: Option<ValId>,
    ) -> Self {
        Self {
            waypoints,
            val_id,
            styler: Styler::new(modifier),
            variable: VariableCell::fresh(),
        }
    }

    /// Returns positions by reading frames from watched cells.
    /// First waypoint uses bottom_center (source port).
    /// Last waypoint uses top_center (destination port).
    /// Intermediate waypoints use center (dummies).
    pub fn get_positions(&self) -> Vec<Position> {
        let len = self.waypoints.len();
        self.waypoints
            .iter()
            .enumerate()
            .map(|(i, watch)| {
                let frame = watch.get_frame();
                if i == 0 {
                    frame.bottom_center()
                } else if i == len - 1 {
                    frame.top_center()
                } else {
                    frame.center()
                }
            })
            .collect()
    }
}

impl<C: Class> SceneElement for Curve<C> {
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

impl<C: Class> Renderable for Curve<C> {
    fn render(&self) -> Vec<SvgElement> {
        let positions = self.get_positions();
        if positions.len() < 2 {
            return vec![];
        }

        let style = self.styler.get();
        let mut commands = Vec::new();
        commands.push(PathCommand::MoveTo(positions[0]));

        // Generate cubic bezier segments between consecutive waypoints
        for i in 0..positions.len() - 1 {
            let start = positions[i];
            let end = positions[i + 1];

            // Control points: vertical offset for smooth curves
            let dy = (end.y.0 - start.y.0) / 3.0;
            let cp1 = Position {
                x: start.x,
                y: Y::new(start.y.0 + dy),
            };
            let cp2 = Position {
                x: end.x,
                y: Y::new(end.y.0 - dy),
            };

            commands.push(PathCommand::CubicTo(cp1, cp2, end));
        }

        vec![
            // Invisible wider hit area for easier hover detection
            SvgElement::Path {
                commands: commands.clone(),
                fill: Some("none".into()),
                stroke: Some("transparent".into()),
                stroke_width: Some(12.0),
                class: Some("link-hitarea".into()),
                id: None,
                title: None,
                data_val: self.val_id,
            },
            // Visible curve
            SvgElement::Path {
                commands,
                fill: Some("none".into()),
                stroke: Some(style.border_color.to_string()),
                stroke_width: Some(style.border_width.0),
                class: Some("link".into()),
                id: None,
                title: None,
                data_val: self.val_id,
            },
        ]
    }
}
