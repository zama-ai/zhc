use hpuc_utils::{
    graphics::{ Delta, HAlign, Position, VAlign},
    iter::{Slide, Slider},
};

use crate::visualization::{
    spatial::{self, Diagram, Element},
    stylesheet::{
        BodyClass, InputPortClass, InputsClass, LinkClass, OperationClass, OutputPortClass, OutputsClass, StyleSheet
    },
};

#[derive(Debug, Clone)]
pub struct Svg {
    pub width: f64,
    pub height: f64,
    pub elements: Vec<SvgElement>,
    pub css: Option<String>,
    pub javascript: Option<String>,
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

        if self.css.is_some() || self.javascript.is_some() {
            writeln!(f, "  <defs>")?;
            if let Some(css) = &self.css {
                writeln!(f, "    <style><![CDATA[{}]]></style>", css)?;
            }
            writeln!(f, "  </defs>")?;
        }

        for element in &self.elements {
            write!(f, "{}", element)?;
        }

        if let Some(js) = &self.javascript {
            writeln!(f, "  <script><![CDATA[{}]]></script>", js)?;
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

pub fn diagram_to_svg(diagram: &Diagram, stylesheet: &StyleSheet) -> Svg {
    let frame = diagram.vertices.get_frame();
    let vertices = gen_vertices(&diagram.vertices, stylesheet);
    let links = gen_links(&diagram.links, stylesheet);
    let css = Some(include_str!("style.css").into());
    let javascript = Some(include_str!("script.js").into());

    Svg {
        width: frame.size.width.0.0,
        height: frame.size.height.0.0,
        elements: [links, vertices ].into(),
        css,
        javascript,
    }
}

fn gen_links(links: &[spatial::Link], stylesheet: &StyleSheet) -> SvgElement {
    let mut elements = Vec::new();
    for link in links.iter() {
        elements.push(gen_link(link, stylesheet));
    }
    SvgElement::Group {
        elements,
        transform: None,
        id: Some("links".into()),
        class: None,
    }
}

fn gen_link(link: &spatial::Link, stylesheet: &StyleSheet) -> SvgElement {
    let magnitude = Delta(150.);
    let style = stylesheet.get::<LinkClass>();
    let path_commands = link
        .control_points
        .iter()
        .cloned()
        .slide::<2>()
        .filter_map(|window| match window {
            Slider::Prelude(w) if w.len() == 1 => {
                Some(PathCommand::MoveTo(w[0]))
            }
            Slider::Complete(w) => {
                Some(PathCommand::CubicTo(w[0].move_y(magnitude / 3), w[1].move_y(-magnitude/3), w[1]))
            }
            _ => None,
        })
        .collect();
    SvgElement::Path {
        commands: path_commands,
        fill: Some("transparent".into()),
        stroke: Some(style.border_color.to_string()),
        stroke_width: Some(style.border_width.0),
        class: Some("interactive-path".into()),
        id: None,
        title: Some(link.value.clone()),
    }
}
fn gen_vertices(diagram: &spatial::Vertices, stylesheet: &StyleSheet) -> SvgElement {
    let mut elements = Vec::new();
    for layer in &diagram.content {
        elements.push(gen_layer(layer, stylesheet));
    }
    SvgElement::Group {
        elements,
        transform: None,
        id: Some("vertices".into()),
        class: None,
    }
}

fn gen_layer(layer: &spatial::Layer, stylesheet: &StyleSheet) -> SvgElement {
    let mut elements = Vec::new();
    for node in &layer.content {
        elements.push(gen_node(node, stylesheet));
    }
    SvgElement::Group {
        elements,
        transform: None,
        id: None,
        class: None,
    }
}

fn gen_node(node: &spatial::Node, stylesheet: &StyleSheet) -> SvgElement {
    match node {
        spatial::Node::E1(operation) => gen_operation(operation, stylesheet),
        spatial::Node::E2(hole) => gen_hole(hole, stylesheet),
    }
}

fn gen_hole(_hole: &spatial::Hole, _stylesheet: &StyleSheet) -> SvgElement {
    SvgElement::Group {
        elements: vec![],
        transform: None,
        id: None,
        class: None,
    }
}

fn gen_operation(operation: &spatial::Operation, stylesheet: &StyleSheet) -> SvgElement {
    let frame = operation.get_frame();
    let mut elements = Vec::new();
    let style = stylesheet.get::<OperationClass>();
    elements.push(SvgElement::Rect {
        x: frame.position.x.0,
        y: frame.position.y.0,
        width: frame.size.width.0.0,
        height: frame.size.height.0.0,
        fill: Some(style.fill_color.to_string()),
        stroke: Some(style.border_color.to_string()),
        stroke_width: Some(style.border_width.0),
        class: None,
        id: None,
    });
    elements.push(gen_inputs(&operation.e1, stylesheet));
    elements.push(gen_body(&operation.e2, stylesheet));
    elements.push(gen_outputs(&operation.e3, stylesheet));
    SvgElement::Group {
        elements,
        transform: None,
        id: None,
        class: None,
    }
}

fn gen_outputs(outputs: &spatial::Outputs, stylesheet: &StyleSheet) -> SvgElement {
    let frame = outputs.get_frame();
    let mut elements = Vec::new();
    let style = stylesheet.get::<OutputsClass>();
    if !outputs.content.is_empty() {
        elements.push(SvgElement::Rect {
            x: frame.position.x.0,
            y: frame.position.y.0,
            width: frame.size.width.0.0,
            height: frame.size.height.0.0,
            fill: Some(style.fill_color.to_string()),
            stroke: Some(style.border_color.to_string()),
            stroke_width: Some(style.border_width.0),
            class: None,
            id: None,
        });
    }
    for output_port in &outputs.content {
        elements.push(gen_output_port(output_port, stylesheet));
    }
    SvgElement::Group {
        elements,
        transform: None,
        id: None,
        class: None,
    }
}

fn gen_output_port(output_port: &spatial::OutputPort, stylesheet: &StyleSheet) -> SvgElement {
    let frame = output_port.get_frame();
    let style = stylesheet.get::<OutputPortClass>();
    SvgElement::Group {
        elements: vec![
            SvgElement::Rect {
                x: frame.position.x.0,
                y: frame.position.y.0,
                width: frame.size.width.0.0,
                height: frame.size.height.0.0,
                fill: Some(style.fill_color.to_string()),
                stroke: Some(style.border_color.to_string()),
                stroke_width: Some(style.border_width.0),
                class: None,
                id: None,
            },
            SvgElement::Text {
                x: frame.position.x.0 + style.padding.0,
                y: frame.position.y.0 + style.padding.0,
                content: output_port.content.clone(),
                font_size: style.font_size.0,
                font_family: Some(style.font.0.into()),
                fill: Some(style.font_color.to_string()),
                text_anchor: style.font_halign.into(),
                dominant_baseline: style.font_valign.into(),
                class: None,
                id: None,
            },
        ],
        transform: None,
        id: None,
        class: None,
    }
}

fn gen_inputs(inputs: &spatial::Inputs, stylesheet: &StyleSheet) -> SvgElement {
    let frame = inputs.get_frame();
    let mut elements = Vec::new();
    let style = stylesheet.get::<InputsClass>();
    if !inputs.content.is_empty() {
        elements.push(SvgElement::Rect {
            x: frame.position.x.0,
            y: frame.position.y.0,
            width: frame.size.width.0.0,
            height: frame.size.height.0.0,
            fill: Some(style.fill_color.to_string()),
            stroke: Some(style.border_color.to_string()),
            stroke_width: Some(style.border_width.0),
            class: None,
            id: None,
        });
    }
    for input_port in &inputs.content {
        elements.push(gen_input_port(input_port, stylesheet));
    }
    SvgElement::Group {
        elements,
        transform: None,
        id: None,
        class: None,
    }
}

fn gen_input_port(input_port: &spatial::InputPort, stylesheet: &StyleSheet) -> SvgElement {
    let frame = input_port.get_frame();
    let style = stylesheet.get::<InputPortClass>();
    SvgElement::Group {
        elements: vec![
            SvgElement::Rect {
                x: frame.position.x.0,
                y: frame.position.y.0,
                width: frame.size.width.0.0,
                height: frame.size.height.0.0,
                fill: Some(style.fill_color.to_string()),
                stroke: Some(style.border_color.to_string()),
                stroke_width: Some(style.border_width.0),
                class: None,
                id: None,
            },
            SvgElement::Text {
                x: frame.position.x.0 + style.padding.0,
                y: frame.position.y.0 + style.padding.0,
                content: input_port.content.clone(),
                font_size: style.font_size.0,
                font_family: Some(style.font.0.into()),
                fill: Some(style.font_color.to_string()),
                text_anchor: style.font_halign.into(),
                dominant_baseline: style.font_valign.into(),
                class: None,
                id: None,
            },
        ],
        transform: None,
        id: None,
        class: None,
    }
}

fn gen_body(body: &spatial::Body, stylesheet: &StyleSheet) -> SvgElement {
    let frame = body.get_frame();
    let style = stylesheet.get::<BodyClass>();
    SvgElement::Group {
        elements: vec![
            SvgElement::Rect {
                x: frame.position.x.0,
                y: frame.position.y.0,
                width: frame.size.width.0.0,
                height: frame.size.height.0.0,
                fill: Some(style.fill_color.to_string()),
                stroke: Some(style.border_color.to_string()),
                stroke_width: Some(style.border_width.0),
                class: None,
                id: None,
            },
            SvgElement::Text {
                x: frame.position.x.0 + style.padding.0,
                y: frame.position.y.0 + style.padding.0,
                content: body.content.clone(),
                font_size: style.font_size.0,
                font_family: Some(style.font.0.into()),
                fill: Some(style.font_color.to_string()),
                text_anchor: style.font_halign.into(),
                dominant_baseline: style.font_valign.into(),
                class: None,
                id: None,
            },
        ],
        transform: None,
        id: None,
        class: None,
    }
}
