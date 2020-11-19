////////////////////////////////////////////////////////////////////////////////
// 🚨🚨🚨🚨🚨🚨🚨🚨 This file is @generated by build script. 🚨🚨🚨🚨🚨🚨🚨🚨 //
// 🚧🚧🚧🚧🚧🚧🚧🚧           ⛔ DO NOT MODIFY! ⛔           🚧🚧🚧🚧🚧🚧🚧🚧 //
////////////////////////////////////////////////////////////////////////////////

use crate::*;
pub use Blink::*;

/// `Blink` (`Slow`, `Fast`, `ResetBlink`).
///
/// Prints the corresponding CSI to the terminal when `Display`ed.  
/// `Default`s to `Blink::ResetBlink`, the unsetting CSI.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Blink {
    Slow,
    Fast,
    ResetBlink,
}

/// Returns `Blink::ResetBlink`.
impl Default for Blink {
    /// Returns `Blink::ResetBlink`.
    fn default() -> Self {
        Blink::ResetBlink
    }
}

impl StylerIndex for Blink {
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
        None
    }

    fn get_invert(&self) -> Option<Invert> {
        None
    }

    fn get_blink(&self) -> Option<Blink> {
        Some(*self)
    }

    fn get_border(&self) -> Option<Border> {
        None
    }
}

impl Styler for Blink {
    type Output = Style;

    fn foreground(self, foreground: impl Into<Option<Foreground>>) -> Self::Output {
        Style {
            foreground: foreground.into(),
            background: None,
            weight:     None,
            slant:      None,
            underline:  None,
            strike:     None,
            overline:   None,
            invert:     None,
            blink:      Some(self),
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
            overline:   None,
            invert:     None,
            blink:      Some(self),
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
            overline:   None,
            invert:     None,
            blink:      Some(self),
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
            overline:   None,
            invert:     None,
            blink:      Some(self),
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
            blink:      Some(self),
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
            overline:   None,
            invert:     None,
            blink:      Some(self),
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
            blink:      Some(self),
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
            overline:   None,
            invert:     invert.into(),
            blink:      Some(self),
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
            underline:  None,
            strike:     None,
            overline:   None,
            invert:     None,
            blink:      Some(self),
            border:     border.into(),
        }
    }
}