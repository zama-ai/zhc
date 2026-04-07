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

use std::marker::PhantomData;

use zhc_utils::graphics::{Color, Font, FontSize, HAlign, Justify, Thickness, VAlign};

/// Marker trait for compile-time style class identification.
pub trait Class: 'static {
    const STYLE: Style = Style::DEFAULT;
}

/// Visual styling properties for UI components.
#[derive(Clone)]
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
    pub corner_radius: Thickness,
    pub halign: HAlign,
    pub valign: VAlign,
    pub hjustify: Justify,
    pub vjustify: Justify,
    /// When true, draw horizontal separators between children in vertical stacks.
    pub draw_separators: bool,
}

impl Style {
    /// Default style configuration for all components.
    pub const DEFAULT: Self = Style {
        font: Font("Courier"),
        font_size: FontSize::new(10.),
        font_halign: HAlign::Left,
        font_valign: VAlign::Center,
        font_color: Color::BLACK,
        padding: Thickness::new(2.),
        spacing: Thickness::new(2.),
        border_width: Thickness::new(0.2),
        border_color: Color::TRANSPARENT,
        fill_color: Color::TRANSPARENT,
        corner_radius: Thickness::ZERO,
        halign: HAlign::Center,
        valign: VAlign::Center,
        hjustify: Justify::Pack,
        vjustify: Justify::Pack,
        draw_separators: false,
    };

    pub fn modify(self, modifier: StyleModifier) -> Style {
        Style {
            font: modifier.font.unwrap_or(self.font),
            font_size: modifier.font_size.unwrap_or(self.font_size),
            font_halign: modifier.font_halign.unwrap_or(self.font_halign),
            font_valign: modifier.font_valign.unwrap_or(self.font_valign),
            font_color: modifier.font_color.unwrap_or(self.font_color),
            padding: modifier.padding.unwrap_or(self.padding),
            spacing: modifier.spacing.unwrap_or(self.spacing),
            border_width: modifier.border_width.unwrap_or(self.border_width),
            border_color: modifier.border_color.unwrap_or(self.border_color),
            fill_color: modifier.fill_color.unwrap_or(self.fill_color),
            corner_radius: modifier.corner_radius.unwrap_or(self.corner_radius),
            halign: modifier.halign.unwrap_or(self.halign),
            valign: modifier.valign.unwrap_or(self.valign),
            hjustify: modifier.hjustify.unwrap_or(self.hjustify),
            vjustify: modifier.vjustify.unwrap_or(self.vjustify),
            draw_separators: modifier.draw_separators.unwrap_or(self.draw_separators),
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[derive(Clone)]
pub struct StyleModifier {
    pub font: Option<Font>,
    pub font_size: Option<FontSize>,
    pub font_halign: Option<HAlign>,
    pub font_valign: Option<VAlign>,
    pub font_color: Option<Color>,
    pub padding: Option<Thickness>,
    pub spacing: Option<Thickness>,
    pub border_width: Option<Thickness>,
    pub border_color: Option<Color>,
    pub fill_color: Option<Color>,
    pub corner_radius: Option<Thickness>,
    pub halign: Option<HAlign>,
    pub valign: Option<VAlign>,
    pub hjustify: Option<Justify>,
    pub vjustify: Option<Justify>,
    pub draw_separators: Option<bool>,
}

impl StyleModifier {
    pub fn trivial() -> Self {
        StyleModifier {
            font: None,
            font_size: None,
            font_halign: None,
            font_valign: None,
            font_color: None,
            padding: None,
            spacing: None,
            border_width: None,
            border_color: None,
            fill_color: None,
            corner_radius: None,
            halign: None,
            valign: None,
            hjustify: None,
            vjustify: None,
            draw_separators: None,
        }
    }
}

pub struct Styler<C: Class> {
    class: PhantomData<C>,
    modifier: StyleModifier,
}

impl<C: Class> Styler<C> {
    pub fn new(modifier: Option<StyleModifier>) -> Self {
        let modifier = modifier.unwrap_or(StyleModifier::trivial());
        Styler {
            class: PhantomData,
            modifier,
        }
    }

    pub fn get(&self) -> Style {
        C::STYLE.clone().modify(self.modifier.clone())
    }
}

/// Default styling class for unspecialized components.
pub struct NoClass;
impl Class for NoClass {}

pub struct OpInputPortClass;
impl Class for OpInputPortClass {
    const STYLE: Style = Style {
        fill_color: Color::AQUAMARINE,
        border_color: Color::BLACK,
        padding: Thickness::new(2.),
        ..Style::DEFAULT
    };
}

pub struct OpInputsClass;
impl Class for OpInputsClass {
    const STYLE: Style = Style {
        fill_color: Color::TRANSPARENT,
        border_color: Color::TRANSPARENT,
        padding: Thickness::new(2.),
        ..Style::DEFAULT
    };
}

pub struct OpBodyClass;
impl Class for OpBodyClass {
    const STYLE: Style = Style {
        padding: Thickness::new(4.),
        ..Style::DEFAULT
    };
}

pub struct OpCommentClass;
impl Class for OpCommentClass {
    const STYLE: Style = Style {
        padding: Thickness::new(4.),
        font_color: Color::GRAY,
        ..Style::DEFAULT
    };
}

pub struct OpOutputPortClass;
impl Class for OpOutputPortClass {
    const STYLE: Style = Style {
        fill_color: Color::AQUAMARINE,
        border_color: Color::BLACK,
        padding: Thickness::new(2.),
        ..Style::DEFAULT
    };
}

pub struct OpOutputsClass;
impl Class for OpOutputsClass {
    const STYLE: Style = Style {
        fill_color: Color::TRANSPARENT,
        border_color: Color::TRANSPARENT,
        padding: Thickness::new(2.),
        ..Style::DEFAULT
    };
}

pub struct InputOpClass;
impl Class for InputOpClass {
    const STYLE: Style = Style {
        fill_color: Color::SEASHELL,
        valign: VAlign::Top,
        border_color: Color::GRAY,
        border_width: Thickness::new(0.7),
        corner_radius: Thickness::new(4.),
        draw_separators: true,
        ..Style::DEFAULT
    };
}

pub struct OpClass;
impl Class for OpClass {
    const STYLE: Style = Style {
        fill_color: Color::ALICEBLUE,
        valign: VAlign::Top,
        border_color: Color::GRAY,
        border_width: Thickness::new(0.7),
        corner_radius: Thickness::new(4.),
        draw_separators: true,
        ..Style::DEFAULT
    };
}

pub struct EffectOpClass;
impl Class for EffectOpClass {
    const STYLE: Style = Style {
        fill_color: Color::HONEYDEW,
        valign: VAlign::Top,
        border_color: Color::GRAY,
        border_width: Thickness::new(0.7),
        corner_radius: Thickness::new(4.),
        draw_separators: true,
        ..Style::DEFAULT
    };
}

pub struct DummyClass;
impl Class for DummyClass {
    const STYLE: Style = Style {
        padding: Thickness::new(10.),
        ..Style::DEFAULT
    };
}

pub struct LayerClass;
impl Class for LayerClass {
    const STYLE: Style = Style {
        padding: Thickness::new(0.),
        spacing: Thickness::new(10.),
        hjustify: Justify::Space,
        ..Style::DEFAULT
    };
}

pub struct LayerSpacerClass;
impl Class for LayerSpacerClass {
    const STYLE: Style = Style {
        hjustify: Justify::Space,
        padding: Thickness::new(0.),
        spacing: Thickness::new(0.),
        ..Style::DEFAULT
    };
}

pub struct LayersClass;
impl Class for LayersClass {
    const STYLE: Style = Style {
        padding: Thickness::new(10.),
        spacing: Thickness::new(0.),
        ..Style::DEFAULT
    };
}

pub struct CurveClass;
impl Class for CurveClass {
    const STYLE: Style = Style {
        border_width: Thickness::new(1.),
        border_color: Color::GRAY,
        ..Style::DEFAULT
    };
}

pub struct GroupClass;
impl Class for GroupClass {
    const STYLE: Style = Style {
        fill_color: Color::CORNFLOWERBLUE.with_opacity(0.2),
        corner_radius: Thickness::new(10.),
        padding: Thickness::new(4.),
        spacing: Thickness::new(2.),
        border_width: Thickness::new(1.),
        border_color: Color::CORNFLOWERBLUE.with_opacity(0.3),
        valign: VAlign::Top,
        ..Style::DEFAULT
    };
}

pub struct GroupTitleClass;
impl Class for GroupTitleClass {
    const STYLE: Style = Style {
        font_size: FontSize::new(8.),
        font_color: Color::BLACK.with_opacity(0.6),
        padding: Thickness::new(5.),
        spacing: Thickness::new(0.),
        font_halign: HAlign::Left,
        halign: HAlign::Left,
        ..Style::DEFAULT
    };
}

pub struct GroupInputPortClass;
impl Class for GroupInputPortClass {
    const STYLE: Style = Style {
        fill_color: Color::LIGHTSKYBLUE,
        border_color: Color::BLACK,
        padding: Thickness::new(0.),
        spacing: Thickness::new(0.),
        ..Style::DEFAULT
    };
}

pub struct GroupOutputPortClass;
impl Class for GroupOutputPortClass {
    const STYLE: Style = Style {
        fill_color: Color::LIGHTSKYBLUE,
        border_color: Color::BLACK,
        padding: Thickness::new(0.),
        spacing: Thickness::new(0.),
        ..Style::DEFAULT
    };
}

pub struct GroupInputsClass;
impl Class for GroupInputsClass {
    const STYLE: Style = Style {
        fill_color: Color::TRANSPARENT,
        border_color: Color::BLACK,
        padding: Thickness::new(0.),
        spacing: Thickness::new(0.),
        hjustify: Justify::Space,
        ..Style::DEFAULT
    };
}

pub struct GroupOutputsClass;
impl Class for GroupOutputsClass {
    const STYLE: Style = Style {
        fill_color: Color::TRANSPARENT,
        border_color: Color::BLACK,
        padding: Thickness::new(0.),
        spacing: Thickness::new(0.),
        hjustify: Justify::Space,
        ..Style::DEFAULT
    };
}
