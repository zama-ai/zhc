use super::Html;
use crate::visualization::svg::Svg;

/// Wraps an SVG in an HTML document with zoom/pan support.
pub fn wrap_svg(svg: Svg) -> Html {
    Html {
        title: "ZHC-Viewer".into(),
        svg,
        css: include_str!("style.css").into(),
        javascript: include_str!("script.js").into(),
    }
}
