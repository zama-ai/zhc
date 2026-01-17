use std::path::Path;
use crate::{Dialect, IR};

mod layout;
mod spatial;
mod svg;
mod stylesheet;


pub fn draw_ir<D: Dialect>(ir: &IR<D>, path: impl AsRef<Path>) {
    let stylesheet = stylesheet::StyleSheet::new();
    let layout = layout::Layout::from_ir(&ir);
    let diagram = spatial::layout_to_diagram(&ir, &layout, &stylesheet);
    let svg = svg::diagram_to_svg(&diagram, &stylesheet);
    let svg_content = format!("{}", svg);
    std::fs::write(path, svg_content).expect("Failed to write SVG file");
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::{tests::gen_complex_ir};

    #[test]
    fn test_visualization() {
        let ir = gen_complex_ir().unwrap();
        draw_ir(&ir, "test_output.svg");
    }
}
