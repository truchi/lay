////////////////////////////////////////////////////////////////////////////////
// 🚨🚨🚨🚨🚨🚨🚨🚨 This file is @generated by build script. 🚨🚨🚨🚨🚨🚨🚨🚨 //
// 🚧🚧🚧🚧🚧🚧🚧🚧           ⛔ DO NOT MODIFY! ⛔           🚧🚧🚧🚧🚧🚧🚧🚧 //
////////////////////////////////////////////////////////////////////////////////

use crate::*;
pub use Overline::*;

/// `Overline` (`Overlined`, `ResetOverline`).
///
/// Prints the corresponding CSI to the terminal when `Display`ed.  
/// `Default`s to `Overline::ResetOverline`, the unsetting CSI.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Overline {
    Overlined,
    ResetOverline,
}

/// Returns `Overline::ResetOverline`.
impl Default for Overline {
    /// Returns `Overline::ResetOverline`.
    fn default() -> Self {
        Overline::ResetOverline
    }
}

impl StylerIndex for Overline {
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
        None
    }

    fn get_strike(&self) -> Option<Strike> {
        None
    }

    fn get_overline(&self) -> Option<Overline> {
        Some(*self)
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

impl Styler for Overline {
    type Output = Style;

    fn foreground(self, foreground: impl Into<Option<Foreground>>) -> Self::Output {
        Style {
            foreground: foreground.into(),
            background: None,
            weight:     None,
            slant:      None,
            underline:  None,
            strike:     None,
            overline:   Some(self),
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
            underline:  None,
            strike:     None,
            overline:   Some(self),
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
            underline:  None,
            strike:     None,
            overline:   Some(self),
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
            underline:  None,
            strike:     None,
            overline:   Some(self),
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
            overline:   Some(self),
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
            underline:  None,
            strike:     strike.into(),
            overline:   Some(self),
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
            underline:  None,
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
            underline:  None,
            strike:     None,
            overline:   Some(self),
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
            underline:  None,
            strike:     None,
            overline:   Some(self),
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
            underline:  None,
            strike:     None,
            overline:   Some(self),
            invert:     None,
            blink:      None,
            border:     border.into(),
        }
    }
}
