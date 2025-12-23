//! Geometry types for 2D graphics and layout systems.
//!
//! This module provides a type-safe coordinate system built around distinct
//! types for measurements (`Thickness`, `Width`, `Height`), positions (`X`, `Y`, `Position`),
//! and regions (`Size`, `Frame`). The design prevents common errors like mixing
//! width and height values through the type system.
//!
//! The core measurement hierarchy starts with `Thickness` as the base unit for
//! non-negative distances, which then specializes into `Width` and `Height` for
//! dimensional measurements. Position types `X` and `Y` handle coordinates,
//! combining into `Position` for 2D points. `Size` pairs width and height,
//! while `Frame` combines position and size for complete rectangular regions.
//!
//! Frame operations support both "taking" (splitting into used and remaining areas)
//! and "cropping" (removing portions) patterns common in layout algorithms.
//! Size operations include padding methods and stacking for combining dimensions.
//! All arithmetic operations include bounds checking and validity assertions.

use std::{cmp::max, ops::{Add, Div, Mul, Neg, Sub}};

use crate::iter::CollectInSmallVec;

/// Font family identifier using static string references.
#[derive(Debug, Clone)]
pub struct Font(pub &'static str);

/// Font size measurement with text dimension calculation capabilities.
#[derive(Debug, Clone, PartialEq, PartialOrd, Copy)]
pub struct FontSize(pub f64, ());

impl Eq for FontSize {}

impl Ord for FontSize {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl FontSize {
    /// Zero font size constant.
    pub const ZERO: Self = FontSize(0., ());

    /// Creates a new font size from the given value.
    ///
    /// # Panics
    ///
    /// Panics if `raw` is not finite or is negative.
    pub const fn new(raw: f64) -> Self {
        let output = Self(raw, ());
        assert!(output.is_valid());
        output
    }

    const fn is_valid(&self) -> bool {
        self.0.is_finite() && self.0 >= 0.0
    }

    fn char_size(&self) -> Size {
        Size {
            width: Width::new(self.0 * 0.6),
            height: Height::new(self.0 * 1.2)
        }
    }

    /// Calculates the total size required to render the given text.
    pub fn get_text_size(&self, text: &str) -> Size {
        let lines = text.lines().cosvec();
        let max_width = lines.iter().map(|line| line.len()).max().unwrap_or(0);
        let n_lines = lines.len();
        let char_sz = self.char_size();
        Size {
            width: char_sz.width * max_width,
            height: char_sz.height * n_lines
        }
    }
}


/// Non-negative thickness measurement serving as the base unit for dimensions.
#[derive(Debug, Clone, PartialEq, PartialOrd, Copy)]
pub struct Thickness(pub f64, ());

impl Eq for Thickness {}

impl Ord for Thickness {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl Thickness {
    /// Zero thickness constant.
    pub const ZERO: Self = Thickness(0., ());

    /// Creates a new thickness from the given value.
    ///
    /// # Panics
    ///
    /// Panics if `raw` is not finite or is negative.
    pub const fn new(raw: f64) -> Self {
        let output = Self(raw, ());
        assert!(output.is_valid());
        output
    }

    const fn is_valid(&self) -> bool {
        self.0.is_finite() && self.0 >= 0.0
    }
}

impl Add for Thickness {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert!(self.is_valid() && rhs.is_valid());
        Thickness(self.0 + rhs.0, ())
    }
}

impl Sub for Thickness {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert!(self.is_valid() && rhs.is_valid());
        assert!(self >= rhs);
        Thickness(self.0 - rhs.0, ())
    }
}

impl Div<usize> for Thickness {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        assert!(self.is_valid());
        assert!(rhs != 0);
        Thickness(self.0 / rhs as f64, ())
    }
}

impl Mul<usize> for Thickness {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        assert!(self.is_valid());
        Thickness(self.0 * rhs as f64, ())
    }
}

/// Horizontal width measurement wrapping thickness with type safety.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Width(pub Thickness);

impl Width {
    /// Zero width constant.
    pub const ZERO: Self = Width(Thickness::ZERO);

    /// Creates a new width from the given value.
    ///
    /// # Panics
    ///
    /// Panics if `raw` is not finite or is negative.
    pub const fn new(raw: f64) -> Self {
        let output = Self(Thickness::new(raw));
        assert!(output.is_valid());
        output
    }

    const fn is_valid(&self) -> bool {
        self.0.is_valid()
    }
}

impl Add<Thickness> for Width {
    type Output = Self;

    fn add(self, rhs: Thickness) -> Self::Output {
        assert!(self.is_valid() && rhs.is_valid());
        Width(self.0 + rhs)
    }
}

impl Add<Width> for Width {
    type Output = Self;

    fn add(self, rhs: Width) -> Self::Output {
        assert!(self.is_valid() && rhs.is_valid());
        Width(self.0 + rhs.0)
    }
}

impl Sub<Width> for Width {
    type Output = Self;

    fn sub(self, rhs: Width) -> Self::Output {
        assert!(self.is_valid() && rhs.is_valid());
        assert!(self >= rhs);
        Width(self.0 - rhs.0)
    }
}

impl Mul<usize> for Width {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        assert!(self.is_valid());
        Width(self.0 * rhs)
    }
}

impl Div<usize> for Width {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        assert!(self.is_valid());
        assert!(rhs != 0);
        Width(self.0 / rhs)
    }
}

/// Vertical height measurement wrapping thickness with type safety.
#[derive(Debug, Clone, Copy,PartialEq, Eq, PartialOrd, Ord)]
pub struct Height(pub Thickness);

impl Height {
    /// Zero height constant.
    pub const ZERO: Self = Height(Thickness::ZERO);

    /// Creates a new height from the given value.
    ///
    /// # Panics
    ///
    /// Panics if `raw` is not finite or is negative.
    pub const fn new(raw: f64) -> Self {
        let output = Self(Thickness::new(raw));
        assert!(output.is_valid());
        output
    }

    const fn is_valid(&self) -> bool {
        self.0.is_valid()
    }
}

impl Add<Thickness> for Height {
    type Output = Self;

    fn add(self, rhs: Thickness) -> Self::Output {
        assert!(self.is_valid() && rhs.is_valid());
        Height(self.0 + rhs)
    }
}

impl Add<Height> for Height {
    type Output = Self;

    fn add(self, rhs: Height) -> Self::Output {
        assert!(self.is_valid() && rhs.is_valid());
        Height(self.0 + rhs.0)
    }
}

impl Sub<Height> for Height {
    type Output = Self;

    fn sub(self, rhs: Height) -> Self::Output {
        assert!(self.is_valid() && rhs.is_valid());
        assert!(self >= rhs);
        Height(self.0 - rhs.0)
    }
}

impl Mul<usize> for Height {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        assert!(self.is_valid());
        Height(self.0 * rhs)
    }
}

impl Div<usize> for Height {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        assert!(self.is_valid());
        assert!(rhs != 0);
        Height(self.0 / rhs)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Delta(pub f64);

impl Delta {
    pub fn abs(mut self) -> Self {
        self.0 = self.0.abs();
        self
    }
}

impl Div<usize> for Delta {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        Delta(self.0 / rhs as f64)
    }
}

impl Mul<usize> for Delta {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Delta(self.0 * rhs as f64)
    }
}

impl Div<f64> for Delta {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Delta(self.0 / rhs)
    }
}

impl Mul<f64> for Delta {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Delta(self.0 * rhs)
    }
}

impl Neg for Delta {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Delta(-self.0)
    }
}

/// Horizontal position coordinate with arithmetic operations for width and thickness.
#[derive(Debug, Clone, PartialEq, PartialOrd, Copy)]
pub struct X(pub f64, ());

impl Eq for X {}
impl Ord for X {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}
impl X {
    /// Zero X coordinate constant.
    pub const ZERO: Self = X(0., ());

    /// Creates a new X coordinate from the given value.
    ///
    /// # Panics
    ///
    /// Panics if `raw` is not finite or is negative.
    pub const fn new(raw: f64) -> Self {
        let output = Self(raw, ());
        assert!(output.is_valid());
        output
    }

    const fn is_valid(&self) -> bool {
        self.0.is_finite()
    }
}

impl Add<Width> for X {
    type Output = X;

    fn add(self, rhs: Width) -> Self::Output {
        assert!(self.is_valid() && rhs.is_valid());
        X(self.0 + rhs.0.0, ())
    }
}

impl Add<Thickness> for X {
    type Output = X;

    fn add(self, rhs: Thickness) -> Self::Output {
        assert!(self.is_valid() && rhs.is_valid());
        X(self.0 + rhs.0, ())
    }
}

impl Add<Delta> for X {
    type Output = X;

    fn add(self, rhs: Delta) -> Self::Output {
        assert!(self.is_valid());
        X(self.0 + rhs.0, ())
    }
}

impl Sub<Delta> for X {
    type Output = X;

    fn sub(self, rhs: Delta) -> Self::Output {
        assert!(self.is_valid());
        X(self.0 - rhs.0, ())
    }
}


impl Sub<Width> for X {
    type Output = X;

    fn sub(self, rhs: Width) -> Self::Output {
        assert!(self.is_valid() && rhs.is_valid());
        assert!(self.0 >= rhs.0.0);
        X(self.0 - rhs.0.0, ())
    }
}

impl Sub<Thickness> for X {
    type Output = X;

    fn sub(self, rhs: Thickness) -> Self::Output {
        assert!(self.is_valid() && rhs.is_valid());
        assert!(self.0 >= rhs.0);
        X(self.0 - rhs.0, ())
    }
}

impl Sub<X> for X {
    type Output = Delta;

    fn sub(self, rhs: X) -> Self::Output {
        Delta(self.0 - rhs.0)
    }
}

/// Vertical position coordinate with arithmetic operations for height and thickness.
#[derive(Debug, Clone, PartialEq, PartialOrd, Copy)]
pub struct Y(pub f64, ());

impl Eq for Y {}
impl Ord for Y {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}
impl Y {
    /// Zero Y coordinate constant.
    pub const ZERO: Self = Y(0., ());

    /// Creates a new Y coordinate from the given value.
    ///
    /// # Panics
    ///
    /// Panics if `raw` is not finite or is negative.
    pub const fn new(raw: f64) -> Self {
        let output = Self(raw, ());
        assert!(output.is_valid());
        output
    }

    const fn is_valid(&self) -> bool {
        self.0.is_finite()
    }
}

impl Add<Height> for Y {
    type Output = Y;

    fn add(self, rhs: Height) -> Self::Output {
        assert!(self.is_valid() && rhs.is_valid());
        Y(self.0 + rhs.0.0, ())
    }
}

impl Add<Thickness> for Y {
    type Output = Y;

    fn add(self, rhs: Thickness) -> Self::Output {
        assert!(self.is_valid() && rhs.is_valid());
        Y(self.0 + rhs.0, ())
    }
}

impl Add<Delta> for Y {
    type Output = Y;

    fn add(self, rhs: Delta) -> Self::Output {
        assert!(self.is_valid());
        Y(self.0 + rhs.0, ())
    }
}

impl Sub<Delta> for Y {
    type Output = Y;

    fn sub(self, rhs: Delta) -> Self::Output {
        assert!(self.is_valid());
        Y(self.0 - rhs.0, ())
    }
}

impl Sub<Height> for Y {
    type Output = Y;

    fn sub(self, rhs: Height) -> Self::Output {
        assert!(self.is_valid() && rhs.is_valid());
        assert!(self.0 >= rhs.0.0);
        Y(self.0 - rhs.0.0, ())
    }
}

impl Sub<Thickness> for Y {
    type Output = Y;

    fn sub(self, rhs: Thickness) -> Self::Output {
        assert!(self.is_valid() && rhs.is_valid());
        assert!(self.0 >= rhs.0);
        Y(self.0 - rhs.0, ())
    }
}

impl Sub<Y> for Y {
    type Output = Delta;

    fn sub(self, rhs: Y) -> Self::Output {
        Delta(self.0 - rhs.0)
    }
}



#[derive(Debug, Clone, Copy)]
pub struct Motion {
    x: Delta,
    y: Delta
}

impl Mul<usize> for Motion {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Motion {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<usize> for Motion {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        Motion {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}


/// 2D position combining X and Y coordinates.
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: X,
    pub y: Y,
}

impl Position {
    /// Origin position at (0, 0).
    pub const ORIGIN: Self = Position {
        x: X::ZERO,
        y: Y::ZERO,
    };

    pub fn move_x(mut self, delta: Delta) -> Self {
        self.x = self.x + delta;
        self
    }

    pub fn move_y(mut self, delta: Delta) -> Self {
        self.y = self.y + delta;
        self
    }
}

impl Sub<Position> for Position {
    type Output = Motion;

    fn sub(self, rhs: Position) -> Self::Output {
        Motion {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl Add<Motion> for Position {
    type Output = Position;

    fn add(self, rhs: Motion) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Motion> for Position {
    type Output = Position;

    fn sub(self, rhs: Motion) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

/// Dimensions combining width and height with padding and stacking operations.
#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: Width,
    pub height: Height,
}

impl Size {
    /// Zero size constant.
    pub const ZERO: Self = Size {
        width: Width::ZERO,
        height: Height::ZERO
    };

    /// Adds padding to all sides of the size.
    pub fn pad(self, padding: Thickness) -> Self {
        self.pad_horizontal(padding).pad_vertical(padding)
    }

    /// Adds padding to the left and right sides.
    pub fn pad_horizontal(self, padding: Thickness) -> Self {
        self.pad_left(padding).pad_right(padding)
    }

    /// Adds padding to the top and bottom sides.
    pub fn pad_vertical(self, padding: Thickness) -> Self {
        self.pad_top(padding).pad_bottom(padding)
    }

    /// Adds padding to the left side.
    pub fn pad_left(mut self, padding: Thickness) -> Self {
        self.width = self.width + padding;
        self
    }

    /// Adds padding to the right side.
    pub fn pad_right(mut self, padding: Thickness) -> Self {
        self.width = self.width + padding;
        self
    }

    /// Adds padding to the top side.
    pub fn pad_top(mut self, padding: Thickness) -> Self {
        self.height = self.height + padding;
        self
    }

    /// Adds padding to the bottom side.
    pub fn pad_bottom(mut self, padding: Thickness) -> Self {
        self.height = self.height + padding;
        self
    }

    /// Combines two sizes horizontally.
    ///
    /// The resulting size has the combined width of both sizes and the
    /// maximum height of the two.
    pub fn stack_horizontal(self, other: Self) -> Self {
        Self {
            width: self.width + other.width,
            height: max(self.height, other.height),
        }
    }

    /// Combines two sizes vertically.
    ///
    /// The resulting size has the combined height of both sizes and the
    /// maximum width of the two.
    pub fn stack_vertical(self, other: Self) -> Self {
        Self {
            height: self.height + other.height,
            width: max(self.width, other.width),
        }
    }
}

impl Mul<usize> for Size {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Size {
            width: self.width * rhs,
            height: self.height * rhs,
        }
    }
}

impl Div<usize> for Size {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        Size {
            width: self.width / rhs,
            height: self.height / rhs
        }
    }
}

/// Rectangular region combining position and size with take/crop operations for layout.
#[derive(Debug, Clone)]
pub struct Frame {
    pub position: Position,
    pub size: Size
}

/// Wrapper for a frame that was taken from another frame.
pub struct Taken(pub Frame);

/// Wrapper for the remaining frame after a take operation.
pub struct Remaining(pub Frame);

impl Frame {
    /// Checks if the frame has zero width or height.
    pub fn is_collapsed(&self) -> bool {
        self.size.height.0 == Thickness::ZERO || self.size.width.0 == Thickness::ZERO
    }

    /// Returns the top-left corner position.
    pub fn top_left(&self) -> Position {
        Position {
            x: self.position.x,
            y: self.position.y,
        }
    }

    /// Returns the top-right corner position.
    pub fn top_right(&self) -> Position {
        Position {
            x: self.position.x + self.size.width,
            y: self.position.y,
        }
    }

    /// Returns the bottom-left corner position.
    pub fn bottom_left(&self) -> Position {
        Position {
            x: self.position.x,
            y: self.position.y + self.size.height,
        }
    }

    /// Returns the bottom-right corner position.
    pub fn bottom_right(&self) -> Position {
        Position {
            x: self.position.x + self.size.width,
            y: self.position.y + self.size.height,
        }
    }

    /// Returns the center position.
    pub fn center(&self) -> Position {
        Position {
            x: self.position.x + self.size.width / 2,
            y: self.position.y + self.size.height / 2,
        }
    }

    /// Removes and returns a frame from the top with the specified `height`.
    ///
    /// # Panics
    ///
    /// Panics if `height` is greater than the current frame height.
    pub fn take_top(self, height: Height) -> (Taken, Remaining) {
        let taken = Taken(Frame {
            position: self.position.clone(),
            size: Size {
                width: self.size.width,
                height,
            },
        });

        let residual = Remaining(Frame {
            position: Position {
                x: self.position.x,
                y: self.position.y + height,
            },
            size: Size {
                width: self.size.width,
                height: self.size.height - height,
            },
        });

        (taken, residual)
    }

    /// Removes and returns a frame from the bottom with the specified `height`.
    ///
    /// # Panics
    ///
    /// Panics if `height` is greater than the current frame height.
    pub fn take_bottom(self, height: Height) -> (Taken, Remaining) {
        let taken = Taken(Frame {
            position: Position {
                x: self.position.x,
                y: self.position.y + self.size.height - height,
            },
            size: Size {
                width: self.size.width,
                height,
            },
        });

        let residual = Remaining(Frame {
            position: self.position.clone(),
            size: Size {
                width: self.size.width,
                height: self.size.height - height,
            },
        });

        (taken, residual)
    }

    /// Removes and returns a frame from the left with the specified `width`.
    ///
    /// # Panics
    ///
    /// Panics if `width` is greater than the current frame width.
    pub fn take_left(self, width: Width) -> (Taken, Remaining) {
        let taken = Taken(Frame {
            position: self.position.clone(),
            size: Size {
                width,
                height: self.size.height,
            },
        });

        let residual = Remaining(Frame {
            position: Position {
                x: self.position.x + width,
                y: self.position.y,
            },
            size: Size {
                width: self.size.width - width,
                height: self.size.height,
            },
        });

        (taken, residual)
    }

    /// Removes and returns a frame from the right with the specified `width`.
    ///
    /// # Panics
    ///
    /// Panics if `width` is greater than the current frame width.
    pub fn take_right(self, width: Width) -> (Taken, Remaining) {
        let taken = Taken(Frame {
            position: Position {
                x: self.position.x + self.size.width - width,
                y: self.position.y,
            },
            size: Size {
                width,
                height: self.size.height,
            },
        });

        let residual = Remaining(Frame {
            position: self.position.clone(),
            size: Size {
                width: self.size.width - width,
                height: self.size.height,
            },
        });

        (taken, residual)
    }

    /// Crops the frame from the top by the specified `height` and returns the modified frame.
    ///
    /// # Panics
    ///
    /// Panics if `height` is greater than the current frame height.
    pub fn crop_top(mut self, height: Height) -> Frame {
        self.position.y = self.position.y + height;
        self.size.height = self.size.height - height;
        self
    }

    /// Crops the frame from the bottom by the specified `height` and returns the modified frame.
    ///
    /// # Panics
    ///
    /// Panics if `height` is greater than the current frame height.
    pub fn crop_bottom(mut self, height: Height) -> Frame {
        self.size.height = self.size.height - height;
        self
    }

    /// Crops the frame from the left by the specified `width` and returns the modified frame.
    ///
    /// # Panics
    ///
    /// Panics if `width` is greater than the current frame width.
    pub fn crop_left(mut self, width: Width) -> Frame {
        self.position.x = self.position.x + width;
        self.size.width = self.size.width - width;
        self
    }

    /// Crops the frame from the right by the specified `width` and returns the modified frame.
    ///
    /// # Panics
    ///
    /// Panics if `width` is greater than the current frame width.
    pub fn crop_right(mut self, width: Width) -> Frame {
        self.size.width = self.size.width - width;
        self
    }

    /// Resizes the frame horizontally to the specified `width` using the given alignment.
    ///
    /// # Panics
    ///
    /// Panics if `width` is greater than the current frame width.
    pub fn resize_horizontal(self, width: Width, align: HAlign) -> Frame {
        assert!(width <= self.size.width);
        let x_offset = match align {
            HAlign::Left => Width::ZERO,
            HAlign::Center => (self.size.width - width) / 2,
            HAlign::Right => self.size.width - width,
        };

        Frame {
            position: Position {
                x: self.position.x + x_offset,
                y: self.position.y,
            },
            size: Size {
                width,
                height: self.size.height,
            },
        }
    }

    /// Resizes the frame vertically to the specified `height` using the given alignment.
    ///
    /// # Panics
    ///
    /// Panics if `height` is greater than the current frame height.
    pub fn resize_vertical(self, height: Height, align: VAlign) -> Frame {
        assert!(height <= self.size.height);
        let y_offset = match align {
            VAlign::Top => Height::ZERO,
            VAlign::Center => (self.size.height - height) / 2,
            VAlign::Bottom => self.size.height - height,
        };

        Frame {
            position: Position {
                x: self.position.x,
                y: self.position.y + y_offset,
            },
            size: Size {
                width: self.size.width,
                height,
            },
        }
    }

    /// Resizes the frame to the specified `size` using the given alignments.
    ///
    /// # Panics
    ///
    /// Panics if the new size is larger than the current frame size in either dimension.
    pub fn resize(self, size: &Size, halign: HAlign, valign: VAlign) -> Frame {
        self.resize_horizontal(size.width, halign).resize_vertical(size.height, valign)
    }

    /// Consumes the frame and returns a collapsed frame at the same position.
    pub fn consume(self) -> Frame {
        Frame {
            position: self.position,
            size: Size {
                width: Width::ZERO,
                height: Height::ZERO,
            },
        }
    }
}

/// Horizontal alignment options for positioning within frames.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HAlign {
    Left,
    Center,
    Right
}

/// Vertical alignment options for positioning within frames.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VAlign {
    Top,
    Center,
    Bottom
}

/// RGBA color representation with standard color constants and hex formatting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    /// Red component (0-255)
    pub r: u8,
    /// Green component (0-255)
    pub g: u8,
    /// Blue component (0-255)
    pub b: u8,
    /// Alpha (transparency) component (0-255, where 0 is fully transparent and 255 is fully opaque)
    pub a: u8,
}

impl Color {
    /// Creates a new color with the specified RGBA values.
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Creates a new opaque color with the specified RGB values and full alpha.
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const TRANSPARENT: Self = Color::new(0, 0, 0, 0);
    pub const BLACK: Self = Color::rgb(0, 0, 0);
    pub const WHITE: Self = Color::rgb(255, 255, 255);
    pub const RED: Self = Color::rgb(255, 0, 0);
    pub const GREEN: Self = Color::rgb(0, 255, 0);
    pub const BLUE: Self = Color::rgb(0, 0, 255);
    pub const ALICEBLUE: Self = Color::rgb(240, 248, 255);
    pub const ANTIQUEWHITE: Self = Color::rgb(250, 235, 215);
    pub const AQUA: Self = Color::rgb(0, 255, 255);
    pub const AQUAMARINE: Self = Color::rgb(127, 255, 212);
    pub const AZURE: Self = Color::rgb(240, 255, 255);
    pub const BEIGE: Self = Color::rgb(245, 245, 220);
    pub const BISQUE: Self = Color::rgb(255, 228, 196);
    pub const BLANCHEDALMOND: Self = Color::rgb(255, 235, 205);
    pub const BLUEVIOLET: Self = Color::rgb(138, 43, 226);
    pub const BROWN: Self = Color::rgb(165, 42, 42);
    pub const BURLYWOOD: Self = Color::rgb(222, 184, 135);
    pub const CADETBLUE: Self = Color::rgb(95, 158, 160);
    pub const CHARTREUSE: Self = Color::rgb(127, 255, 0);
    pub const CHOCOLATE: Self = Color::rgb(210, 105, 30);
    pub const CORAL: Self = Color::rgb(255, 127, 80);
    pub const CORNFLOWERBLUE: Self = Color::rgb(100, 149, 237);
    pub const CORNSILK: Self = Color::rgb(255, 248, 220);
    pub const CRIMSON: Self = Color::rgb(220, 20, 60);
    pub const CYAN: Self = Color::rgb(0, 255, 255);
    pub const DARKBLUE: Self = Color::rgb(0, 0, 139);
    pub const DARKCYAN: Self = Color::rgb(0, 139, 139);
    pub const DARKGOLDENROD: Self = Color::rgb(184, 134, 11);
    pub const DARKGRAY: Self = Color::rgb(169, 169, 169);
    pub const DARKGREY: Self = Color::rgb(169, 169, 169);
    pub const DARKGREEN: Self = Color::rgb(0, 100, 0);
    pub const DARKKHAKI: Self = Color::rgb(189, 183, 107);
    pub const DARKMAGENTA: Self = Color::rgb(139, 0, 139);
    pub const DARKOLIVEGREEN: Self = Color::rgb(85, 107, 47);
    pub const DARKORANGE: Self = Color::rgb(255, 140, 0);
    pub const DARKORCHID: Self = Color::rgb(153, 50, 204);
    pub const DARKRED: Self = Color::rgb(139, 0, 0);
    pub const DARKSALMON: Self = Color::rgb(233, 150, 122);
    pub const DARKSEAGREEN: Self = Color::rgb(143, 188, 143);
    pub const DARKSLATEBLUE: Self = Color::rgb(72, 61, 139);
    pub const DARKSLATEGRAY: Self = Color::rgb(47, 79, 79);
    pub const DARKSLATEGREY: Self = Color::rgb(47, 79, 79);
    pub const DARKTURQUOISE: Self = Color::rgb(0, 206, 209);
    pub const DARKVIOLET: Self = Color::rgb(148, 0, 211);
    pub const DEEPPINK: Self = Color::rgb(255, 20, 147);
    pub const DEEPSKYBLUE: Self = Color::rgb(0, 191, 255);
    pub const DIMGRAY: Self = Color::rgb(105, 105, 105);
    pub const DIMGREY: Self = Color::rgb(105, 105, 105);
    pub const DODGERBLUE: Self = Color::rgb(30, 144, 255);
    pub const FIREBRICK: Self = Color::rgb(178, 34, 34);
    pub const FLORALWHITE: Self = Color::rgb(255, 250, 240);
    pub const FORESTGREEN: Self = Color::rgb(34, 139, 34);
    pub const FUCHSIA: Self = Color::rgb(255, 0, 255);
    pub const GAINSBORO: Self = Color::rgb(220, 220, 220);
    pub const GHOSTWHITE: Self = Color::rgb(248, 248, 255);
    pub const GOLD: Self = Color::rgb(255, 215, 0);
    pub const GOLDENROD: Self = Color::rgb(218, 165, 32);
    pub const GRAY: Self = Color::rgb(128, 128, 128);
    pub const GREY: Self = Color::rgb(128, 128, 128);
    pub const GREENYELLOW: Self = Color::rgb(173, 255, 47);
    pub const HONEYDEW: Self = Color::rgb(240, 255, 240);
    pub const HOTPINK: Self = Color::rgb(255, 105, 180);
    pub const INDIANRED: Self = Color::rgb(205, 92, 92);
    pub const INDIGO: Self = Color::rgb(75, 0, 130);
    pub const IVORY: Self = Color::rgb(255, 255, 240);
    pub const KHAKI: Self = Color::rgb(240, 230, 140);
    pub const LAVENDER: Self = Color::rgb(230, 230, 250);
    pub const LAVENDERBLUSH: Self = Color::rgb(255, 240, 245);
    pub const LAWNGREEN: Self = Color::rgb(124, 252, 0);
    pub const LEMONCHIFFON: Self = Color::rgb(255, 250, 205);
    pub const LIGHTBLUE: Self = Color::rgb(173, 216, 230);
    pub const LIGHTCORAL: Self = Color::rgb(240, 128, 128);
    pub const LIGHTCYAN: Self = Color::rgb(224, 255, 255);
    pub const LIGHTGOLDENRODYELLOW: Self = Color::rgb(250, 250, 210);
    pub const LIGHTGRAY: Self = Color::rgb(211, 211, 211);
    pub const LIGHTGREY: Self = Color::rgb(211, 211, 211);
    pub const LIGHTGREEN: Self = Color::rgb(144, 238, 144);
    pub const LIGHTPINK: Self = Color::rgb(255, 182, 193);
    pub const LIGHTSALMON: Self = Color::rgb(255, 160, 122);
    pub const LIGHTSEAGREEN: Self = Color::rgb(32, 178, 170);
    pub const LIGHTSKYBLUE: Self = Color::rgb(135, 206, 250);
    pub const LIGHTSLATEGRAY: Self = Color::rgb(119, 136, 153);
    pub const LIGHTSLATEGREY: Self = Color::rgb(119, 136, 153);
    pub const LIGHTSTEELBLUE: Self = Color::rgb(176, 196, 222);
    pub const LIGHTYELLOW: Self = Color::rgb(255, 255, 224);
    pub const LIME: Self = Color::rgb(0, 255, 0);
    pub const LIMEGREEN: Self = Color::rgb(50, 205, 50);
    pub const LINEN: Self = Color::rgb(250, 240, 230);
    pub const MAGENTA: Self = Color::rgb(255, 0, 255);
    pub const MAROON: Self = Color::rgb(128, 0, 0);
    pub const MEDIUMAQUAMARINE: Self = Color::rgb(102, 205, 170);
    pub const MEDIUMBLUE: Self = Color::rgb(0, 0, 205);
    pub const MEDIUMORCHID: Self = Color::rgb(186, 85, 211);
    pub const MEDIUMPURPLE: Self = Color::rgb(147, 112, 219);
    pub const MEDIUMSEAGREEN: Self = Color::rgb(60, 179, 113);
    pub const MEDIUMSLATEBLUE: Self = Color::rgb(123, 104, 238);
    pub const MEDIUMSPRINGGREEN: Self = Color::rgb(0, 250, 154);
    pub const MEDIUMTURQUOISE: Self = Color::rgb(72, 209, 204);
    pub const MEDIUMVIOLETRED: Self = Color::rgb(199, 21, 133);
    pub const MIDNIGHTBLUE: Self = Color::rgb(25, 25, 112);
    pub const MINTCREAM: Self = Color::rgb(245, 255, 250);
    pub const MISTYROSE: Self = Color::rgb(255, 228, 225);
    pub const MOCCASIN: Self = Color::rgb(255, 228, 181);
    pub const NAVAJOWHITE: Self = Color::rgb(255, 222, 173);
    pub const NAVY: Self = Color::rgb(0, 0, 128);
    pub const OLDLACE: Self = Color::rgb(253, 245, 230);
    pub const OLIVE: Self = Color::rgb(128, 128, 0);
    pub const OLIVEDRAB: Self = Color::rgb(107, 142, 35);
    pub const ORANGE: Self = Color::rgb(255, 165, 0);
    pub const ORANGERED: Self = Color::rgb(255, 69, 0);
    pub const ORCHID: Self = Color::rgb(218, 112, 214);
    pub const PALEGOLDENROD: Self = Color::rgb(238, 232, 170);
    pub const PALEGREEN: Self = Color::rgb(152, 251, 152);
    pub const PALETURQUOISE: Self = Color::rgb(175, 238, 238);
    pub const PALEVIOLETRED: Self = Color::rgb(219, 112, 147);
    pub const PAPAYAWHIP: Self = Color::rgb(255, 239, 213);
    pub const PEACHPUFF: Self = Color::rgb(255, 218, 185);
    pub const PERU: Self = Color::rgb(205, 133, 63);
    pub const PINK: Self = Color::rgb(255, 192, 203);
    pub const PLUM: Self = Color::rgb(221, 160, 221);
    pub const POWDERBLUE: Self = Color::rgb(176, 224, 230);
    pub const PURPLE: Self = Color::rgb(128, 0, 128);
    pub const ROSYBROWN: Self = Color::rgb(188, 143, 143);
    pub const ROYALBLUE: Self = Color::rgb(65, 105, 225);
    pub const SADDLEBROWN: Self = Color::rgb(139, 69, 19);
    pub const SALMON: Self = Color::rgb(250, 128, 114);
    pub const SANDYBROWN: Self = Color::rgb(244, 164, 96);
    pub const SEAGREEN: Self = Color::rgb(46, 139, 87);
    pub const SEASHELL: Self = Color::rgb(255, 245, 238);
    pub const SIENNA: Self = Color::rgb(160, 82, 45);
    pub const SILVER: Self = Color::rgb(192, 192, 192);
    pub const SKYBLUE: Self = Color::rgb(135, 206, 235);
    pub const SLATEBLUE: Self = Color::rgb(106, 90, 205);
    pub const SLATEGRAY: Self = Color::rgb(112, 128, 144);
    pub const SLATEGREY: Self = Color::rgb(112, 128, 144);
    pub const SNOW: Self = Color::rgb(255, 250, 250);
    pub const SPRINGGREEN: Self = Color::rgb(0, 255, 127);
    pub const STEELBLUE: Self = Color::rgb(70, 130, 180);
    pub const TAN: Self = Color::rgb(210, 180, 140);
    pub const TEAL: Self = Color::rgb(0, 128, 128);
    pub const THISTLE: Self = Color::rgb(216, 191, 216);
    pub const TOMATO: Self = Color::rgb(255, 99, 71);
    pub const TURQUOISE: Self = Color::rgb(64, 224, 208);
    pub const VIOLET: Self = Color::rgb(238, 130, 238);
    pub const WHEAT: Self = Color::rgb(245, 222, 179);
    pub const WHITESMOKE: Self = Color::rgb(245, 245, 245);
    pub const YELLOW: Self = Color::rgb(255, 255, 0);
    pub const YELLOWGREEN: Self = Color::rgb(154, 205, 50);
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.a == 255 {
            write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
        } else {
            write!(f, "#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a)
        }
    }
}
