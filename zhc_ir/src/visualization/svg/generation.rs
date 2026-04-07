use super::syntax_tree::*;

use crate::visualization::composition::SceneElement;

/// Renders a scene graph element to SVG.
pub fn draw(scene: &impl Renderable) -> Svg {
    let frame = scene.get_frame();
    Svg {
        width: frame.size.width.0.0,
        height: frame.size.height.0.0,
        elements: scene.render(),
    }
}

/// Trait for scene graph elements that can render to SVG.
pub trait Renderable: SceneElement {
    fn render(&self) -> Vec<SvgElement>;
}
