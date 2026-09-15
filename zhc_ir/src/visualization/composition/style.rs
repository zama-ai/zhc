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

/// Purely-decorative card outline; not part of the layout/box model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardShape {
    /// Plain rounded rect on all four corners.
    Rect,
    /// Ticket-stub zigzag on the top edge, square top corners.
    SawtoothTop,
    /// Ticket-stub zigzag on the bottom edge, square bottom corners.
    SawtoothBottom,
}

/// Paint used for the background of a styled element.
#[derive(Debug, Clone, PartialEq)]
pub enum Fill {
    /// A single, uniform color.
    Solid(Color),
    /// Diagonal, repeating stripes, one for each color in order.
    Stripes {
        colors: Vec<Color>,
        stripe_width: f64,
    },
}

impl Fill {
    /// Creates repeating diagonal stripes with a default width of 6 SVG units.
    pub fn stripes(colors: impl IntoIterator<Item = Color>) -> Self {
        Self::Stripes {
            colors: colors.into_iter().collect(),
            stripe_width: 6.0,
        }
    }

    /// Creates repeating diagonal stripes with a custom width in SVG units.
    pub fn stripes_with_width(colors: impl IntoIterator<Item = Color>, stripe_width: f64) -> Self {
        Self::Stripes {
            colors: colors.into_iter().collect(),
            stripe_width,
        }
    }
}

impl From<Color> for Fill {
    fn from(color: Color) -> Self {
        Self::Solid(color)
    }
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
    pub fill: Fill,
    pub corner_radius: Thickness,
    pub halign: HAlign,
    pub valign: VAlign,
    pub hjustify: Justify,
    pub vjustify: Justify,
    pub draw_separators: bool,
    pub accent_color: Color,
    pub card_shape: CardShape,
}

impl Style {
    /// Default style configuration for all components.
    pub const DEFAULT: Self = Style {
        font: Font("'JetBrains Mono', 'DejaVu Sans Mono', 'Liberation Mono', 'Courier', monospace"),
        font_size: FontSize::new(10.),
        font_halign: HAlign::Left,
        font_valign: VAlign::Center,
        font_color: Color::BLACK,
        padding: Thickness::new(2.),
        spacing: Thickness::new(2.),
        border_width: Thickness::new(0.2),
        border_color: Color::TRANSPARENT,
        fill: Fill::Solid(Color::TRANSPARENT),
        corner_radius: Thickness::ZERO,
        halign: HAlign::Center,
        valign: VAlign::Center,
        hjustify: Justify::Pack,
        vjustify: Justify::Pack,
        draw_separators: false,
        accent_color: Color::TRANSPARENT,
        card_shape: CardShape::Rect,
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
            fill: modifier.fill.unwrap_or(self.fill),
            corner_radius: modifier.corner_radius.unwrap_or(self.corner_radius),
            halign: modifier.halign.unwrap_or(self.halign),
            valign: modifier.valign.unwrap_or(self.valign),
            hjustify: modifier.hjustify.unwrap_or(self.hjustify),
            vjustify: modifier.vjustify.unwrap_or(self.vjustify),
            draw_separators: modifier.draw_separators.unwrap_or(self.draw_separators),
            accent_color: modifier.accent_color.unwrap_or(self.accent_color),
            card_shape: modifier.card_shape.unwrap_or(self.card_shape),
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
    pub fill: Option<Fill>,
    pub corner_radius: Option<Thickness>,
    pub halign: Option<HAlign>,
    pub valign: Option<VAlign>,
    pub hjustify: Option<Justify>,
    pub vjustify: Option<Justify>,
    pub draw_separators: Option<bool>,
    pub accent_color: Option<Color>,
    pub card_shape: Option<CardShape>,
}

impl Default for StyleModifier {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl StyleModifier {
    pub const DEFAULT: Self = StyleModifier {
        font: None,
        font_size: None,
        font_halign: None,
        font_valign: None,
        font_color: None,
        padding: None,
        spacing: None,
        border_width: None,
        border_color: None,
        fill: None,
        corner_radius: None,
        halign: None,
        valign: None,
        hjustify: None,
        vjustify: None,
        draw_separators: None,
        accent_color: None,
        card_shape: None,
    };
}

pub struct Styler<C: Class> {
    class: PhantomData<C>,
    modifier: StyleModifier,
}

impl<C: Class> Styler<C> {
    pub fn new(modifier: Option<StyleModifier>) -> Self {
        let modifier = modifier.unwrap_or(StyleModifier::DEFAULT);
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
        fill: Fill::Solid(Color::WHITE),
        border_width: Thickness::new(1.),
        border_color: Color::rgb(214, 218, 226),
        font_color: Color::rgb(43, 49, 58),
        padding: Thickness::new(2.),
        corner_radius: Thickness::new(5.),
        font_size: FontSize::new(10.),
        draw_separators: true,
        ..Style::DEFAULT
    };
}

pub struct OpPortTextClass;
impl Class for OpPortTextClass {
    const STYLE: Style = Style {
        font_color: Color::rgb(43, 49, 58),
        font_size: FontSize::new(10.),
        padding: Thickness::ZERO,
        ..Style::DEFAULT
    };
}

pub struct OpInputsClass;
impl Class for OpInputsClass {
    const STYLE: Style = Style {
        fill: Fill::Solid(Color::TRANSPARENT),
        border_color: Color::TRANSPARENT,
        padding: Thickness::new(4.),
        ..Style::DEFAULT
    };
}

pub struct OpBodyClass;
impl Class for OpBodyClass {
    const STYLE: Style = Style {
        padding: Thickness::new(4.),
        font_size: FontSize::new(10.),
        font_color: Color::rgb(27, 31, 38),
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
        fill: Fill::Solid(Color::WHITE),
        border_width: Thickness::new(1.),
        border_color: Color::rgb(214, 218, 226),
        font_color: Color::rgb(43, 49, 58),
        padding: Thickness::new(2.),
        corner_radius: Thickness::new(5.),
        font_size: FontSize::new(10.),
        draw_separators: true,
        ..Style::DEFAULT
    };
}

pub struct OpOutputsClass;
impl Class for OpOutputsClass {
    const STYLE: Style = Style {
        fill: Fill::Solid(Color::TRANSPARENT),
        border_color: Color::TRANSPARENT,
        padding: Thickness::new(4.),
        ..Style::DEFAULT
    };
}

pub struct InputOpClass;
impl Class for InputOpClass {
    const STYLE: Style = Style {
        fill: Fill::Solid(Color::WHITE),
        valign: VAlign::Top,
        border_color: Color::rgb(200, 207, 217),
        border_width: Thickness::new(1.),
        corner_radius: Thickness::new(6.),
        padding: Thickness::new(10.), // clears the rail (6px inset + 4px width)
        draw_separators: true,
        accent_color: Color::MEDIUMSEAGREEN,
        card_shape: CardShape::SawtoothTop,
        ..Style::DEFAULT
    };
}

pub struct OpClass;
impl Class for OpClass {
    const STYLE: Style = Style {
        fill: Fill::Solid(Color::WHITE),
        valign: VAlign::Top,
        border_color: Color::rgb(200, 207, 217),
        border_width: Thickness::new(1.),
        corner_radius: Thickness::new(6.),
        padding: Thickness::new(10.),
        draw_separators: true,
        accent_color: Color::CORNFLOWERBLUE,
        ..Style::DEFAULT
    };
}

pub struct EffectOpClass;
impl Class for EffectOpClass {
    const STYLE: Style = Style {
        fill: Fill::Solid(Color::WHITE),
        valign: VAlign::Top,
        border_color: Color::rgb(200, 207, 217),
        border_width: Thickness::new(1.),
        corner_radius: Thickness::new(6.),
        padding: Thickness::new(10.),
        draw_separators: true,
        accent_color: Color::DARKORANGE,
        card_shape: CardShape::SawtoothBottom,
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
        border_width: Thickness::new(2.),
        border_color: Color::rgb(174, 182, 192),
        ..Style::DEFAULT
    };
}

pub struct GroupClass;
impl Class for GroupClass {
    const STYLE: Style = Style {
        fill: Fill::Solid(Color::rgb(96, 120, 152).with_opacity(0.12)),
        corner_radius: Thickness::new(10.),
        padding: Thickness::new(4.),
        spacing: Thickness::new(2.),
        border_width: Thickness::new(1.),
        border_color: Color::rgb(213, 217, 225),
        valign: VAlign::Top,
        ..Style::DEFAULT
    };
}

pub struct GroupTitleClass;
impl Class for GroupTitleClass {
    const STYLE: Style = Style {
        font_size: FontSize::new(20.),
        font_color: Color::rgb(121, 130, 143),
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
        fill: Fill::Solid(Color::LIGHTSKYBLUE),
        border_color: Color::BLACK,
        padding: Thickness::new(0.),
        spacing: Thickness::new(0.),
        ..Style::DEFAULT
    };
}

pub struct GroupOutputPortClass;
impl Class for GroupOutputPortClass {
    const STYLE: Style = Style {
        fill: Fill::Solid(Color::LIGHTSKYBLUE),
        border_color: Color::BLACK,
        padding: Thickness::new(0.),
        spacing: Thickness::new(0.),
        ..Style::DEFAULT
    };
}

pub struct GroupInputsClass;
impl Class for GroupInputsClass {
    const STYLE: Style = Style {
        fill: Fill::Solid(Color::TRANSPARENT),
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
        fill: Fill::Solid(Color::TRANSPARENT),
        border_color: Color::BLACK,
        padding: Thickness::new(0.),
        spacing: Thickness::new(0.),
        hjustify: Justify::Space,
        ..Style::DEFAULT
    };
}
