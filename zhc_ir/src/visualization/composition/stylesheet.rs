//! Type-safe styling system for UI components using compile-time class markers.
//!
//! This module provides a CSS-like styling approach where UI components are associated
//! with style classes at compile time. The `Class` trait serves as a marker for style
//! categories, while `Style` defines the visual properties (fonts, alignment, spacing).
//!
//! The `StyleSheet` acts as a type-indexed map, storing one `Style` per class type.
//! Classes are zero-sized marker types that represent different UI component categories
//! like input ports, output ports, diagram bodies, etc. The system uses `TypeId`
//! internally to map from class types to their corresponding styles.
//!
//! Style lookup is performed via `StyleSheet::get<C>()` where `C` is a class type,
//! providing compile-time safety and avoiding string-based lookups. All predefined
//! classes start with default styling that can be customized by inserting new styles.

use std::any::TypeId;
use zhc_utils::{
    FastMap,
    graphics::{Color, Font, FontSize, HAlign, Justify, Thickness, VAlign},
};

/// Marker trait for compile-time style class identification.
pub trait Class: 'static {}

/// Visual styling properties for UI components.
#[allow(unused)]
pub struct Style {
    pub font: Font,
    pub font_size: FontSize,
    pub font_halign: HAlign,
    pub font_valign: VAlign,
    pub font_color: Color,
    pub padding: Thickness,
    pub spacing: Thickness,
    pub border_width: Thickness,
    pub border_color: Color,
    pub fill_color: Color,
    pub halign: HAlign,
    pub valign: VAlign,
    pub hjustify: Justify,
    pub vjustify: Justify,
}

impl Style {
    /// Default style configuration for all components.
    pub const DEFAULT: Self = Style {
        font: Font("Courier"),
        font_size: FontSize::new(10.),
        font_halign: HAlign::Left,
        font_valign: VAlign::Top,
        font_color: Color::BLACK,
        padding: Thickness::new(2.),
        spacing: Thickness::new(2.),
        border_width: Thickness::new(0.1),
        border_color: Color::TRANSPARENT,
        fill_color: Color::TRANSPARENT,
        halign: HAlign::Center,
        valign: VAlign::Center,
        hjustify: Justify::Pack,
        vjustify: Justify::Pack,
    };
}

impl Default for Style {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// Type-indexed collection of styles mapped to component classes.
pub struct StyleSheet(FastMap<TypeId, Style>);

impl StyleSheet {
    /// Creates a stylesheet with default styles for all predefined classes.
    pub fn new() -> Self {
        let mut output = StyleSheet(FastMap::new());
        output.insert::<NoClass>(Style::DEFAULT);

        output.insert::<OpInputPortClass>(Style {
            fill_color: Color::AQUAMARINE,
            border_color: Color::BLACK,
            padding: Thickness::new(2.),
            ..Default::default()
        });
        output.insert::<OpInputsClass>(Style {
            fill_color: Color::TRANSPARENT,
            border_color: Color::TRANSPARENT,
            padding: Thickness::new(2.),
            ..Default::default()
        });
        output.insert::<OpBodyClass>(Style {
            padding: Thickness::new(4.),
            ..Default::default()
        });
        output.insert::<OpCommentClass>(Style {
            padding: Thickness::new(4.),
            font_color: Color::GRAY,
            ..Default::default()
        });
        output.insert::<OpOutputPortClass>(Style {
            fill_color: Color::AQUAMARINE,
            border_color: Color::BLACK,
            padding: Thickness::new(2.),
            ..Default::default()
        });
        output.insert::<OpOutputsClass>(Style {
            fill_color: Color::TRANSPARENT,
            border_color: Color::TRANSPARENT,
            padding: Thickness::new(2.),
            ..Default::default()
        });
        output.insert::<InputOpClass>(Style {
            fill_color: Color::SEASHELL,
            valign: VAlign::Top,
            border_color: Color::BLACK,
            ..Default::default()
        });
        output.insert::<OpClass>(Style {
            fill_color: Color::ALICEBLUE,
            valign: VAlign::Top,
            border_color: Color::BLACK,
            ..Default::default()
        });
        output.insert::<EffectOpClass>(Style {
            fill_color: Color::HONEYDEW,
            valign: VAlign::Top,
            border_color: Color::BLACK,
            ..Default::default()
        });
        output.insert::<DummyClass>(Style {
            padding: Thickness::new(10.),
            ..Default::default()
        });
        output.insert::<LayerClass>(Style {
            padding: Thickness::new(0.),
            spacing: Thickness::new(10.),
            hjustify: Justify::Space,
            ..Default::default()
        });
        output.insert::<LayerSpacerClass>(Style {
            hjustify: Justify::Space,
            ..Default::default()
        });
        output.insert::<VerticesClass>(Style {
            padding: Thickness::new(10.),
            spacing: Thickness::new(10.),
            ..Default::default()
        });
        output.insert::<LinkClass>(Style {
            border_width: Thickness::new(1.),
            border_color: Color::GRAY,
            ..Default::default()
        });
        output.insert::<GroupClass>(Style {
            fill_color: Color::LAVENDER,
            border_color: Color::BLACK,
            padding: Thickness::new(4.),
            spacing: Thickness::new(2.),
            valign: VAlign::Top,
            ..Default::default()
        });
        output.insert::<GroupTitleClass>(Style {
            font_size: FontSize::new(8.),
            font_color: Color::GRAY,
            padding: Thickness::new(0.),
            spacing: Thickness::new(0.),
            ..Default::default()
        });
        output.insert::<GroupInputPortClass>(Style {
            fill_color: Color::LIGHTSKYBLUE,
            border_color: Color::BLACK,
            padding: Thickness::new(0.),
            spacing: Thickness::new(0.),
            ..Default::default()
        });
        output.insert::<GroupOutputPortClass>(Style {
            fill_color: Color::LIGHTSKYBLUE,
            border_color: Color::BLACK,
            padding: Thickness::new(0.),
            spacing: Thickness::new(0.),
            ..Default::default()
        });
        output.insert::<GroupInputsClass>(Style {
            fill_color: Color::TRANSPARENT,
            border_color: Color::BLACK,
            padding: Thickness::new(0.),
            spacing: Thickness::new(0.),
            hjustify: Justify::Space,
            ..Default::default()
        });
        output.insert::<GroupOutputsClass>(Style {
            fill_color: Color::TRANSPARENT,
            border_color: Color::BLACK,
            padding: Thickness::new(0.),
            spacing: Thickness::new(0.),
            hjustify: Justify::Space,
            ..Default::default()
        });
        output.insert::<GroupContentClass>(Style {
            padding: Thickness::new(0.),
            spacing: Thickness::new(0.),
            ..Default::default()
        });
        output
    }

    /// Associates a style with the specified class type.
    pub fn insert<C: Class>(&mut self, style: Style) {
        self.0.insert(TypeId::of::<C>(), style);
    }

    /// Retrieves the style for the specified class type.
    pub fn get<C: Class>(&self) -> &Style {
        self.0.get(&TypeId::of::<C>()).unwrap()
    }
}

/// Default styling class for unspecialized components.
pub struct NoClass;
impl Class for NoClass {}

pub struct OpInputPortClass;
impl Class for OpInputPortClass {}

pub struct OpInputsClass;
impl Class for OpInputsClass {}

pub struct OpBodyClass;
impl Class for OpBodyClass {}

pub struct OpCommentClass;
impl Class for OpCommentClass {}

pub struct OpOutputPortClass;
impl Class for OpOutputPortClass {}

pub struct OpOutputsClass;
impl Class for OpOutputsClass {}

pub struct InputOpClass;
impl Class for InputOpClass {}

pub struct OpClass;
impl Class for OpClass {}

pub struct EffectOpClass;
impl Class for EffectOpClass {}

pub struct DummyClass;
impl Class for DummyClass {}

pub struct LayerClass;
impl Class for LayerClass {}

pub struct LayerSpacerClass;
impl Class for LayerSpacerClass {}

pub struct VerticesClass;
impl Class for VerticesClass {}

pub struct LinkClass;
impl Class for LinkClass {}

pub struct GroupClass;
impl Class for GroupClass {}

pub struct GroupTitleClass;
impl Class for GroupTitleClass {}

pub struct GroupInputPortClass;
impl Class for GroupInputPortClass {}

pub struct GroupOutputPortClass;
impl Class for GroupOutputPortClass {}

pub struct GroupInputsClass;
impl Class for GroupInputsClass {}

pub struct GroupOutputsClass;
impl Class for GroupOutputsClass {}

pub struct GroupContentClass;
impl Class for GroupContentClass {}
