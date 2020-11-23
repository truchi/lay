////////////////////////////////////////////////////////////////////////////////
// 🚨🚨🚨🚨🚨🚨🚨🚨 This file is @generated by build script. 🚨🚨🚨🚨🚨🚨🚨🚨 //
// 🚧🚧🚧🚧🚧🚧🚧🚧           ⛔ DO NOT MODIFY! ⛔           🚧🚧🚧🚧🚧🚧🚧🚧 //
////////////////////////////////////////////////////////////////////////////////

use crate::*;
pub use Border::*;

/// [`Border`](crate::Border) (`Circle`, `Frame`, `ResetBorder`).
///
/// Prints the corresponding CSI to the terminal when `Display`ed.
///
/// `Default`s to `Border::ResetBorder`, the unsetting CSI.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Border {
    Circle,
    Frame,
    ResetBorder,
}

/// Returns `Border::ResetBorder`.
impl Default for Border {
    /// Returns `Border::ResetBorder`.
    fn default() -> Self {
        Border::ResetBorder
    }
}

impl StylerIndex for Border {
    /// Returns `None`.
    fn get_foreground(&self) -> Option<Foreground> {
        None
    }

    /// Returns `None`.
    fn get_background(&self) -> Option<Background> {
        None
    }

    /// Returns `None`.
    fn get_weight(&self) -> Option<Weight> {
        None
    }

    /// Returns `None`.
    fn get_slant(&self) -> Option<Slant> {
        None
    }

    /// Returns `None`.
    fn get_underline(&self) -> Option<Underline> {
        None
    }

    /// Returns `None`.
    fn get_strike(&self) -> Option<Strike> {
        None
    }

    /// Returns `None`.
    fn get_overline(&self) -> Option<Overline> {
        None
    }

    /// Returns `None`.
    fn get_invert(&self) -> Option<Invert> {
        None
    }

    /// Returns `None`.
    fn get_blink(&self) -> Option<Blink> {
        None
    }

    /// Returns `Some(self)`.
    fn get_border(&self) -> Option<Border> {
        Some(*self)
    }
}

impl Styler for Border {
    type Output = Style;

    /// Returns a `Style` with `border` (self) and `foreground`.
    fn foreground(self, foreground: impl Into<Option<Foreground>>) -> Self::Output {
        Style {
            border: Some(self),
            foreground: foreground.into(),
            ..Style::NONE
        }
    }

    /// Returns a `Style` with `border` (self) and `background`.
    fn background(self, background: impl Into<Option<Background>>) -> Self::Output {
        Style {
            border: Some(self),
            background: background.into(),
            ..Style::NONE
        }
    }

    /// Returns a `Style` with `border` (self) and `weight`.
    fn weight(self, weight: impl Into<Option<Weight>>) -> Self::Output {
        Style {
            border: Some(self),
            weight: weight.into(),
            ..Style::NONE
        }
    }

    /// Returns a `Style` with `border` (self) and `slant`.
    fn slant(self, slant: impl Into<Option<Slant>>) -> Self::Output {
        Style {
            border: Some(self),
            slant: slant.into(),
            ..Style::NONE
        }
    }

    /// Returns a `Style` with `border` (self) and `underline`.
    fn underline(self, underline: impl Into<Option<Underline>>) -> Self::Output {
        Style {
            border: Some(self),
            underline: underline.into(),
            ..Style::NONE
        }
    }

    /// Returns a `Style` with `border` (self) and `strike`.
    fn strike(self, strike: impl Into<Option<Strike>>) -> Self::Output {
        Style {
            border: Some(self),
            strike: strike.into(),
            ..Style::NONE
        }
    }

    /// Returns a `Style` with `border` (self) and `overline`.
    fn overline(self, overline: impl Into<Option<Overline>>) -> Self::Output {
        Style {
            border: Some(self),
            overline: overline.into(),
            ..Style::NONE
        }
    }

    /// Returns a `Style` with `border` (self) and `invert`.
    fn invert(self, invert: impl Into<Option<Invert>>) -> Self::Output {
        Style {
            border: Some(self),
            invert: invert.into(),
            ..Style::NONE
        }
    }

    /// Returns a `Style` with `border` (self) and `blink`.
    fn blink(self, blink: impl Into<Option<Blink>>) -> Self::Output {
        Style {
            border: Some(self),
            blink: blink.into(),
            ..Style::NONE
        }
    }

    /// Returns a `Style` with `border`.
    fn border(self, border: impl Into<Option<Border>>) -> Self::Output {
        Style {
            border: border.into(),
            ..Style::NONE
        }
    }
}
