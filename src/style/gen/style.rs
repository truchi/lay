////////////////////////////////////////////////////////////////////////////////
// 🚨🚨🚨🚨🚨🚨🚨🚨 This file is @generated by build script. 🚨🚨🚨🚨🚨🚨🚨🚨 //
// 🚧🚧🚧🚧🚧🚧🚧🚧           ⛔ DO NOT MODIFY! ⛔           🚧🚧🚧🚧🚧🚧🚧🚧 //
////////////////////////////////////////////////////////////////////////////////

use crate::*;

/// [`Style`](crate::Style)s.
///
/// A straightforward implementation of [`Styler`](crate::Styler).
///
/// `Display`s the corresponding CSIs to the terminal.
///
/// `Default`s as an empty [`Style`](crate::Style) (all fields set to `None`).
#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct Style {
    pub foreground: Option<Foreground>,
    pub background: Option<Background>,
    pub weight:     Option<Weight>,
    pub slant:      Option<Slant>,
    pub underline:  Option<Underline>,
    pub strike:     Option<Strike>,
    pub overline:   Option<Overline>,
    pub invert:     Option<Invert>,
    pub blink:      Option<Blink>,
    pub border:     Option<Border>,
}

impl Style {
    /// A [`Style`](crate::Style) with fields set to `None`.
    pub const NONE: Self = Self {
        foreground: None,
        background: None,
        weight:     None,
        slant:      None,
        underline:  None,
        strike:     None,
        overline:   None,
        invert:     None,
        blink:      None,
        border:     None,
    };
    /// A [`Style`](crate::Style) with fields set to their reset value.
    pub const RESET: Self = Self {
        foreground: Some(Foreground(Color::ResetColor)),
        background: Some(Background(Color::ResetColor)),
        weight:     Some(Weight::ResetWeight),
        slant:      Some(Slant::ResetSlant),
        underline:  Some(Underline::ResetUnderline),
        strike:     Some(Strike::ResetStrike),
        overline:   Some(Overline::ResetOverline),
        invert:     Some(Invert::ResetInvert),
        blink:      Some(Blink::ResetBlink),
        border:     Some(Border::ResetBorder),
    };
}

/// `Display`s the corresponding CSIs to the terminal.
impl Display for Style {
    /// `Display`s the corresponding CSIs to the terminal.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some(foreground) = self.get_foreground() {
            Display::fmt(&foreground, f)?;
        }

        if let Some(background) = self.get_background() {
            Display::fmt(&background, f)?;
        }

        if let Some(weight) = self.get_weight() {
            Display::fmt(&weight, f)?;
        }

        if let Some(slant) = self.get_slant() {
            Display::fmt(&slant, f)?;
        }

        if let Some(underline) = self.get_underline() {
            Display::fmt(&underline, f)?;
        }

        if let Some(strike) = self.get_strike() {
            Display::fmt(&strike, f)?;
        }

        if let Some(overline) = self.get_overline() {
            Display::fmt(&overline, f)?;
        }

        if let Some(invert) = self.get_invert() {
            Display::fmt(&invert, f)?;
        }

        if let Some(blink) = self.get_blink() {
            Display::fmt(&blink, f)?;
        }

        if let Some(border) = self.get_border() {
            Display::fmt(&border, f)?;
        }

        Ok(())
    }
}

impl Debug for Style {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut tuple = f.debug_tuple("Style");
        if let Some(foreground) = self.get_foreground() {
            tuple.field(&foreground);
        }

        if let Some(background) = self.get_background() {
            tuple.field(&background);
        }

        if let Some(weight) = self.get_weight() {
            tuple.field(&weight);
        }

        if let Some(slant) = self.get_slant() {
            tuple.field(&slant);
        }

        if let Some(underline) = self.get_underline() {
            tuple.field(&underline);
        }

        if let Some(strike) = self.get_strike() {
            tuple.field(&strike);
        }

        if let Some(overline) = self.get_overline() {
            tuple.field(&overline);
        }

        if let Some(invert) = self.get_invert() {
            tuple.field(&invert);
        }

        if let Some(blink) = self.get_blink() {
            tuple.field(&blink);
        }

        if let Some(border) = self.get_border() {
            tuple.field(&border);
        }

        tuple.finish()
    }
}

// ========================================================================== //
// ========================================================================== //
//                                Conversions                                 //
// ========================================================================== //
// ========================================================================== //

/// Returns an empty [`Style`](crate::Style) with `Some(foreground)`.
impl From<Foreground> for Style {
    /// Returns an empty [`Style`](crate::Style) with `Some(foreground)`.
    fn from(foreground: Foreground) -> Self {
        Self {
            foreground: Some(foreground),
            background: None,
            weight:     None,
            slant:      None,
            underline:  None,
            strike:     None,
            overline:   None,
            invert:     None,
            blink:      None,
            border:     None,
        }
    }
}

/// Returns an empty [`Style`](crate::Style) with `Some(background)`.
impl From<Background> for Style {
    /// Returns an empty [`Style`](crate::Style) with `Some(background)`.
    fn from(background: Background) -> Self {
        Self {
            foreground: None,
            background: Some(background),
            weight:     None,
            slant:      None,
            underline:  None,
            strike:     None,
            overline:   None,
            invert:     None,
            blink:      None,
            border:     None,
        }
    }
}

/// Returns an empty [`Style`](crate::Style) with `Some(weight)`.
impl From<Weight> for Style {
    /// Returns an empty [`Style`](crate::Style) with `Some(weight)`.
    fn from(weight: Weight) -> Self {
        Self {
            foreground: None,
            background: None,
            weight:     Some(weight),
            slant:      None,
            underline:  None,
            strike:     None,
            overline:   None,
            invert:     None,
            blink:      None,
            border:     None,
        }
    }
}

/// Returns an empty [`Style`](crate::Style) with `Some(slant)`.
impl From<Slant> for Style {
    /// Returns an empty [`Style`](crate::Style) with `Some(slant)`.
    fn from(slant: Slant) -> Self {
        Self {
            foreground: None,
            background: None,
            weight:     None,
            slant:      Some(slant),
            underline:  None,
            strike:     None,
            overline:   None,
            invert:     None,
            blink:      None,
            border:     None,
        }
    }
}

/// Returns an empty [`Style`](crate::Style) with `Some(underline)`.
impl From<Underline> for Style {
    /// Returns an empty [`Style`](crate::Style) with `Some(underline)`.
    fn from(underline: Underline) -> Self {
        Self {
            foreground: None,
            background: None,
            weight:     None,
            slant:      None,
            underline:  Some(underline),
            strike:     None,
            overline:   None,
            invert:     None,
            blink:      None,
            border:     None,
        }
    }
}

/// Returns an empty [`Style`](crate::Style) with `Some(strike)`.
impl From<Strike> for Style {
    /// Returns an empty [`Style`](crate::Style) with `Some(strike)`.
    fn from(strike: Strike) -> Self {
        Self {
            foreground: None,
            background: None,
            weight:     None,
            slant:      None,
            underline:  None,
            strike:     Some(strike),
            overline:   None,
            invert:     None,
            blink:      None,
            border:     None,
        }
    }
}

/// Returns an empty [`Style`](crate::Style) with `Some(overline)`.
impl From<Overline> for Style {
    /// Returns an empty [`Style`](crate::Style) with `Some(overline)`.
    fn from(overline: Overline) -> Self {
        Self {
            foreground: None,
            background: None,
            weight:     None,
            slant:      None,
            underline:  None,
            strike:     None,
            overline:   Some(overline),
            invert:     None,
            blink:      None,
            border:     None,
        }
    }
}

/// Returns an empty [`Style`](crate::Style) with `Some(invert)`.
impl From<Invert> for Style {
    /// Returns an empty [`Style`](crate::Style) with `Some(invert)`.
    fn from(invert: Invert) -> Self {
        Self {
            foreground: None,
            background: None,
            weight:     None,
            slant:      None,
            underline:  None,
            strike:     None,
            overline:   None,
            invert:     Some(invert),
            blink:      None,
            border:     None,
        }
    }
}

/// Returns an empty [`Style`](crate::Style) with `Some(blink)`.
impl From<Blink> for Style {
    /// Returns an empty [`Style`](crate::Style) with `Some(blink)`.
    fn from(blink: Blink) -> Self {
        Self {
            foreground: None,
            background: None,
            weight:     None,
            slant:      None,
            underline:  None,
            strike:     None,
            overline:   None,
            invert:     None,
            blink:      Some(blink),
            border:     None,
        }
    }
}

/// Returns an empty [`Style`](crate::Style) with `Some(border)`.
impl From<Border> for Style {
    /// Returns an empty [`Style`](crate::Style) with `Some(border)`.
    fn from(border: Border) -> Self {
        Self {
            foreground: None,
            background: None,
            weight:     None,
            slant:      None,
            underline:  None,
            strike:     None,
            overline:   None,
            invert:     None,
            blink:      None,
            border:     Some(border),
        }
    }
}

// ========================================================================== //
// ========================================================================== //
//                               Styler* traits                               //
// ========================================================================== //
// ========================================================================== //

impl StylerIndex for Style {
    /// Gets `Option<Foreground>`.
    fn get_foreground(&self) -> Option<Foreground> {
        self.foreground
    }

    /// Gets `Option<Background>`.
    fn get_background(&self) -> Option<Background> {
        self.background
    }

    /// Gets `Option<Weight>`.
    fn get_weight(&self) -> Option<Weight> {
        self.weight
    }

    /// Gets `Option<Slant>`.
    fn get_slant(&self) -> Option<Slant> {
        self.slant
    }

    /// Gets `Option<Underline>`.
    fn get_underline(&self) -> Option<Underline> {
        self.underline
    }

    /// Gets `Option<Strike>`.
    fn get_strike(&self) -> Option<Strike> {
        self.strike
    }

    /// Gets `Option<Overline>`.
    fn get_overline(&self) -> Option<Overline> {
        self.overline
    }

    /// Gets `Option<Invert>`.
    fn get_invert(&self) -> Option<Invert> {
        self.invert
    }

    /// Gets `Option<Blink>`.
    fn get_blink(&self) -> Option<Blink> {
        self.blink
    }

    /// Gets `Option<Border>`.
    fn get_border(&self) -> Option<Border> {
        self.border
    }
}

impl StylerIndexMut for Style {
    /// Gets `&mut Option<Foreground>`.
    fn get_foreground_mut(&mut self) -> &mut Option<Foreground> {
        &mut self.foreground
    }

    /// Gets `&mut Option<Background>`.
    fn get_background_mut(&mut self) -> &mut Option<Background> {
        &mut self.background
    }

    /// Gets `&mut Option<Weight>`.
    fn get_weight_mut(&mut self) -> &mut Option<Weight> {
        &mut self.weight
    }

    /// Gets `&mut Option<Slant>`.
    fn get_slant_mut(&mut self) -> &mut Option<Slant> {
        &mut self.slant
    }

    /// Gets `&mut Option<Underline>`.
    fn get_underline_mut(&mut self) -> &mut Option<Underline> {
        &mut self.underline
    }

    /// Gets `&mut Option<Strike>`.
    fn get_strike_mut(&mut self) -> &mut Option<Strike> {
        &mut self.strike
    }

    /// Gets `&mut Option<Overline>`.
    fn get_overline_mut(&mut self) -> &mut Option<Overline> {
        &mut self.overline
    }

    /// Gets `&mut Option<Invert>`.
    fn get_invert_mut(&mut self) -> &mut Option<Invert> {
        &mut self.invert
    }

    /// Gets `&mut Option<Blink>`.
    fn get_blink_mut(&mut self) -> &mut Option<Blink> {
        &mut self.blink
    }

    /// Gets `&mut Option<Border>`.
    fn get_border_mut(&mut self) -> &mut Option<Border> {
        &mut self.border
    }
}

impl Styler for Style {
    /// Sets `Option<Foreground>`.
    fn foreground(self, foreground: impl Into<Option<Foreground>>) -> Self {
        Style {
            foreground: foreground.into(),
            ..self
        }
    }

    /// Sets `Option<Background>`.
    fn background(self, background: impl Into<Option<Background>>) -> Self {
        Style {
            background: background.into(),
            ..self
        }
    }

    /// Sets `Option<Weight>`.
    fn weight(self, weight: impl Into<Option<Weight>>) -> Self {
        Style {
            weight: weight.into(),
            ..self
        }
    }

    /// Sets `Option<Slant>`.
    fn slant(self, slant: impl Into<Option<Slant>>) -> Self {
        Style {
            slant: slant.into(),
            ..self
        }
    }

    /// Sets `Option<Underline>`.
    fn underline(self, underline: impl Into<Option<Underline>>) -> Self {
        Style {
            underline: underline.into(),
            ..self
        }
    }

    /// Sets `Option<Strike>`.
    fn strike(self, strike: impl Into<Option<Strike>>) -> Self {
        Style {
            strike: strike.into(),
            ..self
        }
    }

    /// Sets `Option<Overline>`.
    fn overline(self, overline: impl Into<Option<Overline>>) -> Self {
        Style {
            overline: overline.into(),
            ..self
        }
    }

    /// Sets `Option<Invert>`.
    fn invert(self, invert: impl Into<Option<Invert>>) -> Self {
        Style {
            invert: invert.into(),
            ..self
        }
    }

    /// Sets `Option<Blink>`.
    fn blink(self, blink: impl Into<Option<Blink>>) -> Self {
        Style {
            blink: blink.into(),
            ..self
        }
    }

    /// Sets `Option<Border>`.
    fn border(self, border: impl Into<Option<Border>>) -> Self {
        Style {
            border: border.into(),
            ..self
        }
    }
}

impl StylerMut for Style {
    /// Sets `Option<Foreground>`, mutably.
    fn foreground_mut(&mut self, foreground: impl Into<Option<Foreground>>) {
        self.foreground = foreground.into();
    }

    /// Sets `Option<Background>`, mutably.
    fn background_mut(&mut self, background: impl Into<Option<Background>>) {
        self.background = background.into();
    }

    /// Sets `Option<Weight>`, mutably.
    fn weight_mut(&mut self, weight: impl Into<Option<Weight>>) {
        self.weight = weight.into();
    }

    /// Sets `Option<Slant>`, mutably.
    fn slant_mut(&mut self, slant: impl Into<Option<Slant>>) {
        self.slant = slant.into();
    }

    /// Sets `Option<Underline>`, mutably.
    fn underline_mut(&mut self, underline: impl Into<Option<Underline>>) {
        self.underline = underline.into();
    }

    /// Sets `Option<Strike>`, mutably.
    fn strike_mut(&mut self, strike: impl Into<Option<Strike>>) {
        self.strike = strike.into();
    }

    /// Sets `Option<Overline>`, mutably.
    fn overline_mut(&mut self, overline: impl Into<Option<Overline>>) {
        self.overline = overline.into();
    }

    /// Sets `Option<Invert>`, mutably.
    fn invert_mut(&mut self, invert: impl Into<Option<Invert>>) {
        self.invert = invert.into();
    }

    /// Sets `Option<Blink>`, mutably.
    fn blink_mut(&mut self, blink: impl Into<Option<Blink>>) {
        self.blink = blink.into();
    }

    /// Sets `Option<Border>`, mutably.
    fn border_mut(&mut self, border: impl Into<Option<Border>>) {
        self.border = border.into();
    }
}
