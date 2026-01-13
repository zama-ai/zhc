//! Layout engine for diagram elements with CSS-like styling and constraint solving.
//!
//! This module provides a complete layout system for rendering hierarchical diagrams.
//! The core architecture revolves around the `Element` trait, which defines a two-phase
//! layout process: size calculation followed by frame positioning.
//!
//! ## Layout Process
//!
//! Elements progress through three states tracked by `Solution`:
//! - Fresh: newly created, no layout computed
//! - Sized: intrinsic size calculated based on content and styling
//! - Framed: positioned within available space with final bounds
//!
//! The layout containers (`VStack`, `HStack`, `V3`) implement automatic spacing
//! and positioning of child elements. Each element can be styled through the
//! `StyleSheet` system using CSS-like classes for typography, spacing, and alignment.
//!
//! ## Element Hierarchy
//!
//! Primitive elements (`TextBox`, `Empty`) compute their own intrinsic sizes,
//! while container elements (`VStack`, `HStack`, `V3`) aggregate child layouts.
//! The `D2` enum allows runtime polymorphism between different element types.
//!
//! Type aliases provide semantic naming for diagram components: `Operation` combines
//! inputs, body, and outputs in a vertical layout, while `Layer` and `Diagram`
//! create the overall hierarchical structure.

use crate::{
    Dialect,
    visualization::{layout::CoordinatesSpec, stylesheet::{
        BodyClass, HoleClass, InputPortClass, InputsClass, LayerClass, OperationClass, OutputPortClass, OutputsClass, VerticesClass
    }},
};
use hc_utils::{graphics::*, iter::Separate, small::SmallVec};
use hc_utils_macro::fsm;
use std::marker::PhantomData;

use super::layout;
use super::stylesheet::{Class, NoClass, StyleSheet};

/// Tracks the layout computation state of an element through the sizing and positioning phases.
#[fsm]
pub enum Solution {
    Fresh,
    Sized { size: Size },
    Framed { size: Size, frame: Frame },
}

impl Solution {
    /// Transitions from Fresh to Sized state with the computed size.
    pub fn set_size(&mut self, size: Size) {
        self.transition(|old| {
            let Solution::Fresh = old else { panic!() };
            Solution::Sized { size }
        });
    }

    /// Transitions from Sized to Framed state with the final positioned frame.
    pub fn set_frame(&mut self, frame: Frame) {
        self.transition(|old| {
            let Solution::Sized { size } = old else {
                panic!()
            };
            Solution::Framed { size, frame }
        });
    }

    /// Returns the computed size after the sizing phase.
    pub fn get_size(&self) -> Size {
        match self {
            Self::Sized { size, .. } | Self::Framed { size, .. } => size.clone(),
            _ => panic!(),
        }
    }

    /// Returns the final positioned frame after the positioning phase.
    pub fn get_frame(&self) -> Frame {
        match self {
            Self::Framed { frame, .. } => frame.clone(),
            _ => panic!(),
        }
    }
}

/// Defines the two-phase layout protocol for all renderable diagram elements.
pub trait Element {
    /// Computes the element's intrinsic size based on content and stylesheet.
    fn solve_size(&mut self, stylesheet: &StyleSheet);

    /// Positions the element within the available frame using stylesheet alignment rules.
    fn solve_frame(&mut self, stylesheet: &StyleSheet, available: Frame);

    /// Returns the computed size from the sizing phase.
    fn get_size(&self) -> Size;

    /// Returns the final positioned frame from the positioning phase.
    fn get_frame(&self) -> Frame;
}

/// Text element that renders string content with typography styling.
pub struct TextBox<C: Class = NoClass> {
    pub content: String,
    class: PhantomData<C>,
    solution: Solution,
}

impl<C: Class> TextBox<C> {
    /// Creates a new text box with the given content string.
    pub fn new(content: String) -> Self {
        Self {
            content,
            class: PhantomData,
            solution: Solution::Fresh,
        }
    }
}

impl<C: Class> Element for TextBox<C> {
    fn solve_size(&mut self, stylesheet: &StyleSheet) {
        let style = stylesheet.get::<C>();
        let size = style
            .font_size
            .get_text_size(&self.content)
            .pad(style.padding);
        self.solution.set_size(size);
    }

    fn solve_frame(&mut self, stylesheet: &StyleSheet, available: Frame) {
        let style = stylesheet.get::<C>();
        let frame = available.resize(&self.get_size(), style.halign, style.valign);
        self.solution.set_frame(frame);
    }

    fn get_size(&self) -> Size {
        self.solution.get_size()
    }

    fn get_frame(&self) -> Frame {
        self.solution.get_frame()
    }
}

/// Empty element that takes up space according to its padding but renders nothing.
pub struct Empty<C: Class = NoClass> {
    class: PhantomData<C>,
    solution: Solution,
}

impl<C: Class> Empty<C> {
    /// Creates a new empty element.
    pub fn new() -> Self {
        Self {
            class: PhantomData,
            solution: Solution::Fresh,
        }
    }
}

impl<C: Class> Element for Empty<C> {
    fn solve_size(&mut self, stylesheet: &StyleSheet) {
        let style = stylesheet.get::<C>();
        let size = Size::ZERO.pad(style.padding);
        self.solution.set_size(size);
    }

    fn solve_frame(&mut self, stylesheet: &StyleSheet, available: Frame) {
        let style = stylesheet.get::<C>();
        let frame = available.resize(&self.get_size(), style.halign, style.valign);
        self.solution.set_frame(frame);
    }

    fn get_size(&self) -> Size {
        self.solution.get_size()
    }

    fn get_frame(&self) -> Frame {
        self.solution.get_frame()
    }
}

enum Spaced<E> {
    Element(E),
    Space,
}

/// Vertical stack container that arranges child elements top to bottom with spacing.
pub struct VStack<E: Element, C: Class = NoClass> {
    pub content: Vec<E>,
    class: PhantomData<C>,
    solution: Solution,
}

impl<E: Element, C: Class> VStack<E, C> {
    /// Creates a new vertical stack with the given child elements.
    pub fn new(content: Vec<E>) -> Self {
        Self {
            content,
            class: PhantomData,
            solution: Solution::Fresh,
        }
    }
}

impl<E: Element, C: Class> Element for VStack<E, C> {
    fn solve_size(&mut self, stylesheet: &StyleSheet) {
        let style = stylesheet.get::<C>();
        let size = self
            .content
            .iter_mut()
            .map(Spaced::Element)
            .separate_with(|| Spaced::Space)
            .fold(Size::ZERO, |size, element| match element {
                Spaced::Element(element) => {
                    element.solve_size(stylesheet);
                    size.stack_vertical(element.get_size())
                }
                Spaced::Space => size.pad_bottom(style.spacing),
            })
            .pad(style.padding);
        self.solution.set_size(size);
    }

    fn solve_frame(&mut self, stylesheet: &StyleSheet, available: Frame) {
        let style = stylesheet.get::<C>();
        let size = self.get_size();
        let frame = available.resize(&size, style.halign, style.valign);
        self.solution.set_frame(frame.clone());

        let remaining = frame.crop_top(Height(style.padding));
        let remaining = self
            .content
            .iter_mut()
            .map(Spaced::Element)
            .separate_with(|| Spaced::Space)
            .fold(remaining, |available, element| match element {
                Spaced::Element(element) => {
                    let height = element.get_size().height;
                    let (Taken(available_to_element), Remaining(remaining)) =
                        available.take_top(height);
                    element.solve_frame(stylesheet, available_to_element);
                    remaining
                }
                Spaced::Space => available.crop_top(Height(style.spacing)),
            });
        let remaining = remaining.crop_top(Height(style.padding));

        assert!(remaining.is_collapsed());
    }

    fn get_size(&self) -> Size {
        self.solution.get_size()
    }

    fn get_frame(&self) -> Frame {
        self.solution.get_frame()
    }
}

/// Horizontal stack container that arranges child elements left to right with spacing.
pub struct HStack<E: Element, C: Class = NoClass> {
    pub content: Vec<E>,
    class: PhantomData<C>,
    solution: Solution,
}

impl<E: Element, C: Class> HStack<E, C> {
    /// Creates a new horizontal stack with the given child elements.
    pub fn new(content: Vec<E>) -> Self {
        Self {
            content,
            class: PhantomData,
            solution: Solution::Fresh,
        }
    }
}

impl<E: Element, C: Class> Element for HStack<E, C> {
    fn solve_size(&mut self, stylesheet: &StyleSheet) {
        let style = stylesheet.get::<C>();
        let size = self
            .content
            .iter_mut()
            .map(Spaced::Element)
            .separate_with(|| Spaced::Space)
            .fold(Size::ZERO, |size, element| match element {
                Spaced::Element(element) => {
                    element.solve_size(stylesheet);
                    size.stack_horizontal(element.get_size())
                }
                Spaced::Space => size.pad_right(style.spacing),
            })
            .pad(style.padding);
        self.solution.set_size(size);
    }

    fn solve_frame(&mut self, stylesheet: &StyleSheet, available: Frame) {
        let style = stylesheet.get::<C>();
        let size = self.get_size();
        let frame = available.resize(&size, style.halign, style.valign);
        self.solution.set_frame(frame.clone());

        let remaining = frame.crop_left(Width(style.padding));
        let remaining = self
            .content
            .iter_mut()
            .map(Spaced::Element)
            .separate_with(|| Spaced::Space)
            .fold(remaining, |available, e| match e {
                Spaced::Element(element) => {
                    let width = element.get_size().width;
                    let (Taken(available_to_element), Remaining(remaining)) =
                        available.take_left(width);
                    element.solve_frame(stylesheet, available_to_element);
                    remaining
                }
                Spaced::Space => available.crop_left(Width(style.spacing)),
            });
        let remaining = remaining.crop_left(Width(style.padding));

        assert!(remaining.is_collapsed());
    }

    fn get_size(&self) -> Size {
        self.solution.get_size()
    }

    fn get_frame(&self) -> Frame {
        self.solution.get_frame()
    }
}

/// Fixed vertical stack of exactly three elements with spacing.
pub struct V3<E1: Element, E2: Element, E3: Element, C: Class = NoClass> {
    pub e1: E1,
    pub e2: E2,
    pub e3: E3,
    class: PhantomData<C>,
    solution: Solution,
}

impl<E1: Element, E2: Element, E3: Element, C: Class> V3<E1, E2, E3, C> {
    /// Creates a new three-element vertical stack.
    pub fn new(e1: E1, e2: E2, e3: E3) -> Self {
        Self {
            e1,
            e2,
            e3,
            class: PhantomData,
            solution: Solution::Fresh,
        }
    }
}

impl<E1: Element, E2: Element, E3: Element, C: Class> Element for V3<E1, E2, E3, C> {
    fn solve_size(&mut self, stylesheet: &StyleSheet) {
        let style = stylesheet.get::<C>();
        self.e1.solve_size(stylesheet);
        self.e2.solve_size(stylesheet);
        self.e3.solve_size(stylesheet);
        let size = Size::ZERO
            .pad_top(style.padding)
            .stack_vertical(self.e1.get_size())
            .pad_bottom(style.spacing)
            .stack_vertical(self.e2.get_size())
            .pad_bottom(style.spacing)
            .stack_vertical(self.e3.get_size())
            .pad_bottom(style.padding);
        self.solution.set_size(size);
    }

    fn solve_frame(&mut self, stylesheet: &StyleSheet, available: Frame) {
        let style = stylesheet.get::<C>();
        let size = self.get_size();
        let frame = available.resize(&size, style.halign, style.valign);
        self.solution.set_frame(frame.clone());

        let remaining = frame.crop_top(Height(style.padding));
        let (Taken(e1_available), Remaining(remaining)) = remaining.take_top(self.e1.get_size().height);
        self.e1.solve_frame(stylesheet, e1_available);
        let remaining = remaining.crop_top(Height(style.spacing));
        let (Taken(e2_available), Remaining(remaining)) = remaining.take_top(self.e2.get_size().height);
        self.e2.solve_frame(stylesheet, e2_available);
        let remaining = remaining.crop_top(Height(style.spacing));
        let (Taken(e3_available), Remaining(remaining)) = remaining.take_top(self.e3.get_size().height);
        self.e3.solve_frame(stylesheet, e3_available);
        let remaining = remaining.crop_top(Height(style.padding));

        assert!(remaining.is_collapsed());
    }

    fn get_size(&self) -> Size {
        self.solution.get_size()
    }

    fn get_frame(&self) -> Frame {
        self.solution.get_frame()
    }
}

/// Discriminated union of two element types for runtime polymorphism.
pub enum D2<E1: Element, E2: Element> {
    E1(E1),
    E2(E2),
}

impl<E1: Element, E2: Element> Element for D2<E1, E2> {
    fn solve_size(&mut self, stylesheet: &StyleSheet) {
        match self {
            D2::E1(e) => e.solve_size(stylesheet),
            D2::E2(e) => e.solve_size(stylesheet),
        }
    }

    fn solve_frame(&mut self, stylesheet: &StyleSheet, available: Frame) {
        match self {
            D2::E1(e) => e.solve_frame(stylesheet, available),
            D2::E2(e) => e.solve_frame(stylesheet, available),
        }
    }

    fn get_size(&self) -> Size {
        match self {
            D2::E1(e) => e.get_size(),
            D2::E2(e) => e.get_size(),
        }
    }

    fn get_frame(&self) -> Frame {
        match self {
            D2::E1(e) => e.get_frame(),
            D2::E2(e) => e.get_frame(),
        }
    }
}

/// Text element representing an operation input port.
pub type InputPort = TextBox<InputPortClass>;

/// Horizontal collection of input ports.
pub type Inputs = HStack<InputPort, InputsClass>;

/// Text element representing an operation body.
pub type Body = TextBox<BodyClass>;

/// Text element representing an operation output port.
pub type OutputPort = TextBox<OutputPortClass>;

/// Horizontal collection of output ports.
pub type Outputs = HStack<OutputPort, OutputsClass>;

/// Complete operation node with inputs, body, and outputs arranged vertically.
pub type Operation = V3<Inputs, Body, Outputs, OperationClass>;

/// Empty placeholder element for missing nodes.
pub type Hole = Empty<HoleClass>;

/// Either an operation or a placeholder hole.
pub type Node = D2<Operation, Hole>;

/// Horizontal row of nodes forming a diagram layer.
pub type Layer = HStack<Node, LayerClass>;

pub type Vertices = VStack<Layer, VerticesClass>;

pub struct Link {
    pub control_points: SmallVec<Position>,
    pub value: String
}

pub struct Diagram {
    pub vertices: Vertices,
    pub links: Vec<Link>
}

/// Converts a layout specification into a fully positioned diagram element.
pub fn layout_to_diagram<'ir, D: Dialect>(
    layout: &layout::Layout<'ir, D>,
    stylesheet: &StyleSheet,
) -> Diagram {
    let mut vertices = gen_vertices(layout.iter_vertices());
    vertices.solve_size(stylesheet);
    let size = vertices.get_size();
    vertices.solve_frame(
        stylesheet,
        Frame {
            position: Position::ORIGIN,
            size,
        },
    );

    let links = layout.iter_links()
        .map(|(link, value)| {
            let control_points = link.into_iter().map(|coord| {
                match coord.spec {
                    CoordinatesSpec::OpArg(arg_i) => {
                        let layer = &vertices.content[coord.layer as usize];
                        let node = &layer.content[coord.node as usize];
                        let D2::E1(op) = node else {unreachable!()};
                        let arg = &op.e1.content[arg_i as usize];
                        arg.get_frame().center()
                    },
                    CoordinatesSpec::OpRet(ret_i) => {
                        let layer = &vertices.content[coord.layer as usize];
                        let node = &layer.content[coord.node as usize];
                        let D2::E1(op) = node else {unreachable!()};
                        let ret = &op.e3.content[ret_i as usize];
                        ret.get_frame().center()
                    },
                    CoordinatesSpec::Val => {
                        let layer = &vertices.content[coord.layer as usize];
                        let node = &layer.content[coord.node as usize];
                        let D2::E2(val) = node else {unreachable!()};
                        val.get_frame().center()
                    },
                }
            })
            .collect();
            Link{
                control_points,
                value,
            }
        })
        .collect();


    Diagram {
        vertices,
        links
    }
}

fn gen_vertices<'a, 'ir: 'a, D: Dialect>(diag: impl Iterator<Item = impl Iterator<Item = &'a layout::Node<'ir, D>>>) -> Vertices {
    Vertices::new(diag.map(gen_layer).collect())
}

fn gen_layer<'a, 'ir: 'a, D: Dialect>(lay: impl Iterator<Item = &'a layout::Node<'ir, D>>) -> Layer {
    Layer::new(lay.map(gen_node).collect())
}

fn gen_node<'ir, D: Dialect>(inp: &layout::Node<'ir, D>) -> Node {
    match inp {
        layout::Node::Operation(op_ref) => Node::E1(Operation::new(
            Inputs::new(
                op_ref
                    .get_args_iter()
                    .map(|arg| format!("{}: {}", arg.to_string(), arg.get_type()))
                    .map(TextBox::new)
                    .collect(),
            ),
            Body::new(op_ref.to_string()),
            Outputs::new(
                op_ref
                    .get_returns_iter()
                    .map(|ret| format!("{}: {}", ret.to_string(), ret.get_type()))
                    .map(TextBox::new)
                    .collect(),
            ),
        )),
        layout::Node::Value(..) => Node::E2(Hole::new()),
    }
}
