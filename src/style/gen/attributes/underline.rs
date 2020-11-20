////////////////////////////////////////////////////////////////////////////////
// 🚨🚨🚨🚨🚨🚨🚨🚨 This file is @generated by build script. 🚨🚨🚨🚨🚨🚨🚨🚨 //
// 🚧🚧🚧🚧🚧🚧🚧🚧           ⛔ DO NOT MODIFY! ⛔           🚧🚧🚧🚧🚧🚧🚧🚧 //
////////////////////////////////////////////////////////////////////////////////

use crate::*;
pub use Underline::*;

/// [`Underline`](crate::Underline) (`Underlined`, `ResetUnderline`).
///
/// Prints the corresponding CSI to the terminal when `Display`ed.
///
/// `Default`s to `Underline::ResetUnderline`, the unsetting CSI.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Underline {
    Underlined,
    ResetUnderline,
}

/// Returns `Underline::ResetUnderline`.
impl Default for Underline {
    /// Returns `Underline::ResetUnderline`.
    fn default() -> Self {
        Underline::ResetUnderline
    }
}

impl StylerIndex for Underline {
    fn get_foreground(&self) -> Option<Foreground> {
        None
    }

    fn get_background(&self) -> Option<Background> {
        None
    }

    fn get_weight(&self) -> Option<Weight> {
        None
    }

    fn get_slant(&self) -> Option<Slant> {
        None
    }

    fn get_underline(&self) -> Option<Underline> {
        Some(*self)
    }

    fn get_strike(&self) -> Option<Strike> {
        None
    }

    fn get_overline(&self) -> Option<Overline> {
        None
    }

    fn get_invert(&self) -> Option<Invert> {
        None
    }

    fn get_blink(&self) -> Option<Blink> {
        None
    }

    fn get_border(&self) -> Option<Border> {
        None
    }
}

impl Styler for Underline {
    type Output = Style;

    fn foreground(self, foreground: impl Into<Option<Foreground>>) -> Self::Output {
        Style {
            foreground: foreground.into(),
            background: None,
            weight:     None,
            slant:      None,
            underline:  Some(self),
            strike:     None,
            overline:   None,
            invert:     None,
            blink:      None,
            border:     None,
        }
    }

    fn background(self, background: impl Into<Option<Background>>) -> Self::Output {
        Style {
            foreground: None,
            background: background.into(),
            weight:     None,
            slant:      None,
            underline:  Some(self),
            strike:     None,
            overline:   None,
            invert:     None,
            blink:      None,
            border:     None,
        }
    }

    fn weight(self, weight: impl Into<Option<Weight>>) -> Self::Output {
        Style {
            foreground: None,
            background: None,
            weight:     weight.into(),
            slant:      None,
            underline:  Some(self),
            strike:     None,
            overline:   None,
            invert:     None,
            blink:      None,
            border:     None,
        }
    }

    fn slant(self, slant: impl Into<Option<Slant>>) -> Self::Output {
        Style {
            foreground: None,
            background: None,
            weight:     None,
            slant:      slant.into(),
            underline:  Some(self),
            strike:     None,
            overline:   None,
            invert:     None,
            blink:      None,
            border:     None,
        }
    }

    fn underline(self, underline: impl Into<Option<Underline>>) -> Self::Output {
        Style {
            foreground: None,
            background: None,
            weight:     None,
            slant:      None,
            underline:  underline.into(),
            strike:     None,
            overline:   None,
            invert:     None,
            blink:      None,
            border:     None,
        }
    }

    fn strike(self, strike: impl Into<Option<Strike>>) -> Self::Output {
        Style {
            foreground: None,
            background: None,
            weight:     None,
            slant:      None,
            underline:  Some(self),
            strike:     strike.into(),
            overline:   None,
            invert:     None,
            blink:      None,
            border:     None,
        }
    }

    fn overline(self, overline: impl Into<Option<Overline>>) -> Self::Output {
        Style {
            foreground: None,
            background: None,
            weight:     None,
            slant:      None,
            underline:  Some(self),
            strike:     None,
            overline:   overline.into(),
            invert:     None,
            blink:      None,
            border:     None,
        }
    }

    fn invert(self, invert: impl Into<Option<Invert>>) -> Self::Output {
        Style {
            foreground: None,
            background: None,
            weight:     None,
            slant:      None,
            underline:  Some(self),
            strike:     None,
            overline:   None,
            invert:     invert.into(),
            blink:      None,
            border:     None,
        }
    }

    fn blink(self, blink: impl Into<Option<Blink>>) -> Self::Output {
        Style {
            foreground: None,
            background: None,
            weight:     None,
            slant:      None,
            underline:  Some(self),
            strike:     None,
            overline:   None,
            invert:     None,
            blink:      blink.into(),
            border:     None,
        }
    }

    fn border(self, border: impl Into<Option<Border>>) -> Self::Output {
        Style {
            foreground: None,
            background: None,
            weight:     None,
            slant:      None,
            underline:  Some(self),
            strike:     None,
            overline:   None,
            invert:     None,
            blink:      None,
            border:     border.into(),
        }
    }
}
