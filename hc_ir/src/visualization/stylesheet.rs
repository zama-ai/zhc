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

use hc_utils::{
    FastMap,
    graphics::{Color, Font, FontSize, HAlign, Thickness, VAlign},
};
use std::any::TypeId;

/// Marker trait for compile-time style class identification.
pub trait Class: 'static {}

/// Visual styling properties for UI components.
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
        border_color: Color::BLACK,
        fill_color: Color::TRANSPARENT,
        halign: HAlign::Center,
        valign: VAlign::Center,
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

        output.insert::<InputPortClass>(Style {
            fill_color: Color::AQUAMARINE,
            padding: Thickness::new(2.),
            ..Default::default()
        });
        output.insert::<InputsClass>(Style {
            fill_color: Color::AQUA,
            spacing: Thickness::new(10.),
            ..Default::default()
        });
        output.insert::<BodyClass>(Style {
            padding: Thickness::new(4.),
            ..Default::default()
        });
        output.insert::<OutputPortClass>(Style {
            fill_color: Color::AQUAMARINE,
            padding: Thickness::new(2.),
            ..Default::default()
        });
        output.insert::<OutputsClass>(Style {
            fill_color: Color::AQUA,
            padding: Thickness::new(2.),
            ..Default::default()
        });
        output.insert::<InputOperationClass>(Style {
            fill_color: Color::SEASHELL,
            valign: VAlign::Top,
            ..Default::default()
        });
        output.insert::<OperationClass>(Style {
            fill_color: Color::ALICEBLUE,
            valign: VAlign::Top,
            ..Default::default()
        });
        output.insert::<EffectOperationClass>(Style {
            fill_color: Color::HONEYDEW,
            valign: VAlign::Top,
            ..Default::default()
        });
        output.insert::<HoleClass>(Style {
            padding: Thickness::new(50.),
            ..Default::default()
        });
        output.insert::<LayerClass>(Style {
            padding: Thickness::new(10.),
            spacing: Thickness::new(10.),
            ..Default::default()
        });
        output.insert::<VerticesClass>(Style {
            padding: Thickness::new(100.),
            spacing: Thickness::new(50.),
            ..Default::default()
        });
        output.insert::<LinkClass>(Style {
            border_width: Thickness::new(1.),
            border_color: Color::GRAY,
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

pub struct InputPortClass;
impl Class for InputPortClass {}

pub struct InputsClass;
impl Class for InputsClass {}

pub struct BodyClass;
impl Class for BodyClass {}

pub struct OutputPortClass;
impl Class for OutputPortClass {}

pub struct OutputsClass;
impl Class for OutputsClass {}

pub struct InputOperationClass;
impl Class for InputOperationClass {}

pub struct OperationClass;
impl Class for OperationClass {}

pub struct EffectOperationClass;
impl Class for EffectOperationClass {}

pub struct HoleClass;
impl Class for HoleClass {}

pub struct LayerClass;
impl Class for LayerClass {}

pub struct VerticesClass;
impl Class for VerticesClass {}

pub struct LinkClass;
impl Class for LinkClass {}
