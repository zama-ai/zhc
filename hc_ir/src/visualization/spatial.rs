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
    Dialect, IR,
    visualization::{
        layout::{CoordinatesSpec, Link, Vertex},
        stylesheet::{
            BodyClass, EffectOperationClass, HoleClass, InputOperationClass, InputPortClass,
            InputsClass, LayerClass, OperationClass, OutputPortClass, OutputsClass, VerticesClass,
        },
    },
};
use hc_utils::{graphics::*, iter::Separate, small::SmallVec};
use hc_utils_macro::fsm;
use std::{marker::PhantomData};

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

macro_rules! vstack_fixed {
    ($name:ident, $n:literal, [$($etype:ident, $efield:ident),*]) => {
        /// Fixed vertical stack of exactly $n elements with spacing.
        pub struct $name<$($etype: Element),*, C: Class = NoClass> {
            $(pub $efield: $etype,)*
            class: PhantomData<C>,
            solution: Solution,
        }

        impl<$($etype: Element),*, C: Class> $name<$($etype),*, C> {
            /// Creates a new vertical stack.
            pub fn new($($efield: $etype),*) -> Self {
                Self {
                    $($efield,)*
                    class: PhantomData,
                    solution: Solution::Fresh,
                }
            }
        }

        impl<$($etype: Element),*, C: Class> Element for $name<$($etype),*, C> {
            fn solve_size(&mut self, stylesheet: &StyleSheet) {
                let style = stylesheet.get::<C>();
                $(self.$efield.solve_size(stylesheet);)*

                let mut size = Size::ZERO.pad_top(style.padding);
                let mut first = true;
                $(
                    if !first {
                        size = size.pad_bottom(style.spacing);
                    }
                    size = size.stack_vertical(self.$efield.get_size());
                    first = false;
                )*
                size = size.pad_bottom(style.padding);
                self.solution.set_size(size);
            }

            fn solve_frame(&mut self, stylesheet: &StyleSheet, available: Frame) {
                let style = stylesheet.get::<C>();
                let size = self.get_size();
                let frame = available.resize(&size, style.halign, style.valign);
                self.solution.set_frame(frame.clone());

                let mut remaining = frame.crop_top(Height(style.padding));
                let mut first = true;
                $(
                    if !first {
                        remaining = remaining.crop_top(Height(style.spacing));
                    }
                    let (Taken(available), Remaining(new_remaining)) =
                        remaining.take_top(self.$efield.get_size().height);
                    self.$efield.solve_frame(stylesheet, available);
                    remaining = new_remaining;
                    #[allow(unused_assignments)]
                    {
                        first = false;
                    }
                )*
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
    };
}

vstack_fixed!(V2, 2, [E1, e1, E2, e2]);
vstack_fixed!(V3, 3, [E1, e1, E2, e2, E3, e3]);

macro_rules! discriminated_union {
    ($name:ident, [$($etype:ident),*]) => {
        /// Discriminated union for runtime polymorphism.
        pub enum $name<$($etype: Element),*> {
            $($etype($etype),)*
        }

        impl<$($etype: Element),*> Element for $name<$($etype),*> {
            fn solve_size(&mut self, stylesheet: &StyleSheet) {
                match self {
                    $($name::$etype(e) => e.solve_size(stylesheet),)*
                }
            }

            fn solve_frame(&mut self, stylesheet: &StyleSheet, available: Frame) {
                match self {
                    $($name::$etype(e) => e.solve_frame(stylesheet, available),)*
                }
            }

            fn get_size(&self) -> Size {
                match self {
                    $($name::$etype(e) => e.get_size(),)*
                }
            }

            fn get_frame(&self) -> Frame {
                match self {
                    $($name::$etype(e) => e.get_frame(),)*
                }
            }
        }
    };
}

discriminated_union!(D4, [E1, E2, E3, E4]);

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

pub type InputOperation = V2<Body, Outputs, InputOperationClass>;

/// Complete operation node with inputs, body, and outputs arranged vertically.
pub type Operation = V3<Inputs, Body, Outputs, OperationClass>;

pub type EffectOperation = V2<Inputs, Body, EffectOperationClass>;

/// Empty placeholder element for missing nodes.
pub type Hole = Empty<HoleClass>;

pub type Node = D4<InputOperation, Operation, EffectOperation, Hole>;

/// Horizontal row of nodes forming a diagram layer.
pub type Layer = HStack<Node, LayerClass>;

pub type Vertices = VStack<Layer, VerticesClass>;

pub struct Path {
    pub control_points: SmallVec<Position>,
    pub value: String,
}

pub struct Diagram {
    pub vertices: Vertices,
    pub paths: Vec<Path>,
}

/// Converts a layout specification into a fully positioned diagram element.
pub fn layout_to_diagram<D: Dialect>(
    ir: &IR<D>,
    layout: &layout::Layout,
    stylesheet: &StyleSheet,
) -> Diagram {
    let mut vertices = gen_vertices(ir, layout.iter_vertices());
    vertices.solve_size(stylesheet);
    let size = vertices.get_size();
    vertices.solve_frame(
        stylesheet,
        Frame {
            position: Position::ORIGIN,
            size,
        },
    );

    let links = layout
        .iter_links()
        .map(|Link { value, path, .. }| {
            let control_points = path
                .iter()
                .map(|coord| match coord.spec {
                    CoordinatesSpec::OpArg(arg_i) => {
                        let layer = &vertices.content[coord.layer as usize];
                        let node = &layer.content[coord.node as usize];
                        let arg = match node {
                            D4::E2(op) => &op.e1.content[arg_i as usize],
                            D4::E3(eff) => &eff.e1.content[arg_i as usize],
                            _ => unreachable!()
                        };
                        arg.get_frame().center()
                    }
                    CoordinatesSpec::OpRet(ret_i) => {
                        let layer = &vertices.content[coord.layer as usize];
                        let node = &layer.content[coord.node as usize];
                        let ret = match node {
                            D4::E1(inp) => &inp.e2.content[ret_i as usize],
                            D4::E2(op) => &op.e3.content[ret_i as usize],
                            _ => unreachable!()
                        };
                        ret.get_frame().center()
                    }
                    CoordinatesSpec::Val => {
                        let layer = &vertices.content[coord.layer as usize];
                        let node = &layer.content[coord.node as usize];
                        let D4::E4(val) = node else { unreachable!() };
                        val.get_frame().center()
                    }
                })
                .collect();
            Path {
                control_points,
                value: format!("{:?}", value),
            }
        })
        .collect();

    Diagram {
        vertices,
        paths: links,
    }
}

fn gen_vertices<'a, D: Dialect>(
    ir: &IR<D>,
    diag: impl Iterator<Item = impl Iterator<Item = &'a Vertex>>,
) -> Vertices {
    Vertices::new(diag.map(|l| gen_layer(ir, l)).collect())
}

fn gen_layer<'a, D: Dialect>(ir: &IR<D>, lay: impl Iterator<Item = &'a Vertex>) -> Layer {
    Layer::new(lay.map(|v| gen_node(ir, v)).collect())
}

fn gen_node<D: Dialect>(ir: &IR<D>, inp: &Vertex) -> Node {
    match inp {
        layout::Vertex::Operation(opid) => {
            let op_ref = ir.get_op(*opid);
            if op_ref.is_input() {
                Node::E1(InputOperation::new(
                    Body::new(op_ref.operation.to_string()),
                    Outputs::new(
                        op_ref
                            .get_returns_iter()
                            .map(|ret| format!("{}: {}", ret.to_string(), ret.get_type()))
                            .map(TextBox::new)
                            .collect(),
                    ),
                ))
            } else if op_ref.is_effect() {
                Node::E3(EffectOperation::new(
                    Inputs::new(
                        op_ref
                            .get_args_iter()
                            .map(|arg| format!("{}: {}", arg.to_string(), arg.get_type()))
                            .map(TextBox::new)
                            .collect(),
                    ),
                    Body::new(op_ref.operation.to_string()),
                ))
            } else {
                Node::E2(Operation::new(
                    Inputs::new(
                        op_ref
                            .get_args_iter()
                            .map(|arg| format!("{}: {}", arg.to_string(), arg.get_type()))
                            .map(TextBox::new)
                            .collect(),
                    ),
                    Body::new(op_ref.operation.to_string()),
                    Outputs::new(
                        op_ref
                            .get_returns_iter()
                            .map(|ret| format!("{}: {}", ret.to_string(), ret.get_type()))
                            .map(TextBox::new)
                            .collect(),
                    ),
                ))
            }
        }
        layout::Vertex::Value(..) => Node::E4(Hole::new()),
    }
}
