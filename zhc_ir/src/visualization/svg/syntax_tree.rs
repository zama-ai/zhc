use crate::ValId;
use zhc_utils::graphics::{HAlign, Position, VAlign};

#[derive(Debug, Clone)]
pub struct Svg {
    pub width: f64,
    pub height: f64,
    pub elements: Vec<SvgElement>,
}

#[derive(Debug, Clone)]
pub enum SvgElement {
    Rect {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        fill: Option<String>,
        stroke: Option<String>,
        stroke_width: Option<f64>,
        class: Option<String>,
        id: Option<String>,
        data_val: Option<ValId>,
    },
    Text {
        x: f64,
        y: f64,
        content: String,
        font_size: f64,
        font_family: Option<String>,
        fill: Option<String>,
        text_anchor: TextAnchor,
        dominant_baseline: DominantBaseline,
        class: Option<String>,
        id: Option<String>,
    },
    Path {
        commands: Vec<PathCommand>,
        fill: Option<String>,
        stroke: Option<String>,
        stroke_width: Option<f64>,
        class: Option<String>,
        id: Option<String>,
        title: Option<String>,
        data_val: Option<ValId>,
    },
    Group {
        elements: Vec<SvgElement>,
        transform: Option<String>,
        id: Option<String>,
        class: Option<String>,
    },
}

#[derive(Debug, Clone)]
pub enum PathCommand {
    MoveTo(Position),
    CubicTo(Position, Position, Position),
    EllipticalArc {
        rx: f64,
        ry: f64,
        x_axis_rotation: f64,
        large_arc: bool,
        sweep: bool,
        end: Position,
    },
    ClosePath,
}

#[derive(Debug, Clone)]
pub enum TextAnchor {
    Start,
    Middle,
    End,
}

impl From<HAlign> for TextAnchor {
    fn from(halign: HAlign) -> Self {
        match halign {
            HAlign::Left => TextAnchor::Start,
            HAlign::Center => TextAnchor::Middle,
            HAlign::Right => TextAnchor::End,
        }
    }
}

#[derive(Debug, Clone)]
pub enum DominantBaseline {
    Auto,
    Middle,
    Hanging,
}

impl From<VAlign> for DominantBaseline {
    fn from(valign: VAlign) -> Self {
        match valign {
            VAlign::Top => DominantBaseline::Hanging,
            VAlign::Center => DominantBaseline::Middle,
            VAlign::Bottom => DominantBaseline::Auto,
        }
    }
}

impl std::fmt::Display for Svg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#,
            self.width, self.height
        )?;

        for element in &self.elements {
            write!(f, "{}", element)?;
        }

        writeln!(f, "</svg>")
    }
}

impl std::fmt::Display for SvgElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SvgElement::Rect {
                x,
                y,
                width,
                height,
                fill,
                stroke,
                stroke_width,
                class,
                id,
                data_val,
            } => {
                write!(
                    f,
                    r#"  <rect x="{}" y="{}" width="{}" height="{}""#,
                    x, y, width, height
                )?;
                if let Some(id) = id {
                    write!(f, r#" id="{}""#, id)?;
                }
                if let Some(class) = class {
                    write!(f, r#" class="{}""#, class)?;
                }
                if let Some(fill) = fill {
                    write!(f, r#" fill="{}""#, fill)?;
                }
                if let Some(stroke) = stroke {
                    write!(f, r#" stroke="{}""#, stroke)?;
                }
                if let Some(stroke_width) = stroke_width {
                    write!(f, r#" stroke-width="{}""#, stroke_width)?;
                }
                if let Some(data_val) = data_val {
                    write!(f, r#" data-val="{}""#, data_val)?;
                }
                writeln!(f, " />")
            }
            SvgElement::Text {
                x,
                y,
                content,
                font_size,
                font_family,
                fill,
                text_anchor,
                dominant_baseline,
                class,
                id,
            } => {
                write!(
                    f,
                    r#"  <text x="{}" y="{}" font-size="{}" text-anchor="{}" dominant-baseline="{}""#,
                    x, y, font_size, text_anchor, dominant_baseline
                )?;
                if let Some(id) = id {
                    write!(f, r#" id="{}""#, id)?;
                }
                if let Some(class) = class {
                    write!(f, r#" class="{}""#, class)?;
                }
                if let Some(font_family) = font_family {
                    write!(f, r#" font-family="{}""#, font_family)?;
                }
                if let Some(fill) = fill {
                    write!(f, r#" fill="{}""#, fill)?;
                }
                writeln!(
                    f,
                    ">{}</text>",
                    content
                        .replace('&', "&amp;")
                        .replace('<', "&lt;")
                        .replace('>', "&gt;")
                        .replace('"', "&quot;")
                        .replace('\'', "&#39;")
                )
            }
            SvgElement::Path {
                commands,
                fill,
                stroke,
                stroke_width,
                class,
                id,
                title,
                data_val,
            } => {
                if title.is_some() {
                    writeln!(f, "  <g>")?;
                    write!(f, "    ")?;
                }
                write!(f, r#"<path d=""#)?;
                for command in commands {
                    write!(f, "{}", command)?;
                }
                write!(f, r#"""#)?;
                if let Some(id) = id {
                    write!(f, r#" id="{}""#, id)?;
                }
                if let Some(class) = class {
                    write!(f, r#" class="{}""#, class)?;
                }
                if let Some(fill) = fill {
                    write!(f, r#" fill="{}""#, fill)?;
                }
                if let Some(stroke) = stroke {
                    write!(f, r#" stroke="{}""#, stroke)?;
                }
                if let Some(stroke_width) = stroke_width {
                    write!(f, r#" stroke-width="{}""#, stroke_width)?;
                }
                if let Some(data_val) = data_val {
                    write!(f, r#" data-val="{}""#, data_val)?;
                }
                if title.is_some() {
                    writeln!(f, " >")?;
                    if let Some(title_text) = title {
                        writeln!(f, "      <title>{}</title>", title_text)?;
                    }
                    writeln!(f, "    </path>")?;
                    writeln!(f, "  </g>")
                } else {
                    writeln!(f, " />")
                }
            }
            SvgElement::Group {
                elements,
                transform,
                id,
                class,
            } => {
                // Skip empty groups that have no semantic purpose
                if elements.is_empty() && transform.is_none() && id.is_none() && class.is_none() {
                    return Ok(());
                }
                write!(f, "  <g")?;
                if let Some(id) = id {
                    write!(f, r#" id="{}""#, id)?;
                }
                if let Some(class) = class {
                    write!(f, r#" class="{}""#, class)?;
                }
                if let Some(transform) = transform {
                    write!(f, r#" transform="{}""#, transform)?;
                }
                writeln!(f, ">")?;
                for element in elements {
                    let element_str = format!("{}", element);
                    for line in element_str.lines() {
                        if !line.is_empty() {
                            writeln!(f, "  {}", line)?;
                        }
                    }
                }
                writeln!(f, "  </g>")
            }
        }
    }
}

impl std::fmt::Display for PathCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathCommand::MoveTo(pos) => write!(f, "M {} {} ", pos.x.0, pos.y.0),
            PathCommand::CubicTo(cp1, cp2, pos) => {
                write!(
                    f,
                    "C {} {} {} {} {} {} ",
                    cp1.x.0, cp1.y.0, cp2.x.0, cp2.y.0, pos.x.0, pos.y.0
                )
            }
            PathCommand::EllipticalArc {
                rx,
                ry,
                x_axis_rotation,
                large_arc,
                sweep,
                end,
            } => {
                write!(
                    f,
                    "A {} {} {} {} {} {} {} ",
                    rx,
                    ry,
                    x_axis_rotation,
                    if *large_arc { 1 } else { 0 },
                    if *sweep { 1 } else { 0 },
                    end.x.0,
                    end.y.0
                )
            }
            PathCommand::ClosePath => write!(f, "Z "),
        }
    }
}

impl std::fmt::Display for TextAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextAnchor::Start => write!(f, "start"),
            TextAnchor::Middle => write!(f, "middle"),
            TextAnchor::End => write!(f, "end"),
        }
    }
}

impl std::fmt::Display for DominantBaseline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DominantBaseline::Auto => write!(f, "auto"),
            DominantBaseline::Middle => write!(f, "middle"),
            DominantBaseline::Hanging => write!(f, "hanging"),
        }
    }
}
