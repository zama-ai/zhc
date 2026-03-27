use crate::visualization::svg::Svg;

/// An HTML document wrapping an SVG visualization.
#[derive(Debug, Clone)]
pub struct Html {
    /// Title shown in the browser tab
    pub title: String,
    /// The SVG content to embed
    pub svg: Svg,
    /// Additional CSS for the HTML wrapper
    pub css: String,
    /// JavaScript for zoom/pan and interactions
    pub javascript: String,
}

impl std::fmt::Display for Html {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "<!DOCTYPE html>")?;
        writeln!(f, "<html>")?;
        writeln!(f, "<head>")?;
        writeln!(f, "  <meta charset=\"UTF-8\">")?;
        writeln!(
            f,
            "  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">"
        )?;
        writeln!(f, "  <title>{}</title>", html_escape(&self.title))?;
        writeln!(f, "  <style>")?;
        writeln!(f, "{}", self.css)?;
        writeln!(f, "  </style>")?;
        writeln!(f, "</head>")?;
        writeln!(f, "<body>")?;
        writeln!(f, "  <div id=\"viewport\">")?;
        writeln!(f, "    <div id=\"canvas\">")?;

        // Emit SVG with viewBox and 100% dimensions
        write!(f, "{}", self.svg_with_viewbox())?;

        writeln!(f, "    </div>")?;
        writeln!(f, "  </div>")?;
        writeln!(f, "  <script>")?;
        writeln!(f, "{}", self.javascript)?;
        writeln!(f, "  </script>")?;
        writeln!(f, "</body>")?;
        writeln!(f, "</html>")
    }
}

impl Html {
    /// Renders the SVG with viewBox for proper scaling.
    fn svg_with_viewbox(&self) -> String {
        let svg = &self.svg;
        let mut output = String::new();

        // SVG opening tag with viewBox
        output.push_str(&format!(
            r#"<svg viewBox="0 0 {} {}" preserveAspectRatio="xMidYMid meet" xmlns="http://www.w3.org/2000/svg">"#,
            svg.width, svg.height
        ));
        output.push('\n');

        // Elements
        for element in &svg.elements {
            output.push_str(&format!("{}", element));
        }

        output.push_str("</svg>\n");
        output
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
