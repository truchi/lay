////////////////////////////////////////////////////////////////////////////////
// 🚨🚨🚨🚨🚨🚨🚨🚨 This file is @generated by build script. 🚨🚨🚨🚨🚨🚨🚨🚨 //
// 🚧🚧🚧🚧🚧🚧🚧🚧           ⛔ DO NOT MODIFY! ⛔           🚧🚧🚧🚧🚧🚧🚧🚧 //
////////////////////////////////////////////////////////////////////////////////

use crate::*;
use std::{
    fmt::{Display, Error, Formatter},
    ops::Not,
};

/// `Style`s.
///
/// A straightforward implementation of `Styler`.
///
/// `Display`s the corresponding CSIs to the terminal.
///
/// `Default`s as an empty `Style` (all fields set to `None`).
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
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
    /// A `Style` with fields set to `None`.
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
    /// A `Style` with fields set to their reset value.
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

impl Display for Style {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
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

// ========================================================================== //
// ========================================================================== //
//                                Conversions                                 //
// ========================================================================== //
// ========================================================================== //

/// Returns an empty `Style` with `Some(foreground)`.
impl From<Foreground> for Style {
    /// Returns an empty `Style` with `Some(foreground)`.
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

/// Returns an empty `Style` with `Some(background)`.
impl From<Background> for Style {
    /// Returns an empty `Style` with `Some(background)`.
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

/// Returns an empty `Style` with `Some(weight)`.
impl From<Weight> for Style {
    /// Returns an empty `Style` with `Some(weight)`.
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

/// Returns an empty `Style` with `Some(slant)`.
impl From<Slant> for Style {
    /// Returns an empty `Style` with `Some(slant)`.
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

/// Returns an empty `Style` with `Some(underline)`.
impl From<Underline> for Style {
    /// Returns an empty `Style` with `Some(underline)`.
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

/// Returns an empty `Style` with `Some(strike)`.
impl From<Strike> for Style {
    /// Returns an empty `Style` with `Some(strike)`.
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

/// Returns an empty `Style` with `Some(overline)`.
impl From<Overline> for Style {
    /// Returns an empty `Style` with `Some(overline)`.
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

/// Returns an empty `Style` with `Some(invert)`.
impl From<Invert> for Style {
    /// Returns an empty `Style` with `Some(invert)`.
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

/// Returns an empty `Style` with `Some(blink)`.
impl From<Blink> for Style {
    /// Returns an empty `Style` with `Some(blink)`.
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

/// Returns an empty `Style` with `Some(border)`.
impl From<Border> for Style {
    /// Returns an empty `Style` with `Some(border)`.
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
    fn get_foreground(&self) -> Option<Foreground> {
        self.foreground
    }

    fn get_background(&self) -> Option<Background> {
        self.background
    }

    fn get_weight(&self) -> Option<Weight> {
        self.weight
    }

    fn get_slant(&self) -> Option<Slant> {
        self.slant
    }

    fn get_underline(&self) -> Option<Underline> {
        self.underline
    }

    fn get_strike(&self) -> Option<Strike> {
        self.strike
    }

    fn get_overline(&self) -> Option<Overline> {
        self.overline
    }

    fn get_invert(&self) -> Option<Invert> {
        self.invert
    }

    fn get_blink(&self) -> Option<Blink> {
        self.blink
    }

    fn get_border(&self) -> Option<Border> {
        self.border
    }
}

impl StylerIndexMut for Style {
    fn get_foreground_mut(&mut self) -> &mut Option<Foreground> {
        &mut self.foreground
    }

    fn get_background_mut(&mut self) -> &mut Option<Background> {
        &mut self.background
    }

    fn get_weight_mut(&mut self) -> &mut Option<Weight> {
        &mut self.weight
    }

    fn get_slant_mut(&mut self) -> &mut Option<Slant> {
        &mut self.slant
    }

    fn get_underline_mut(&mut self) -> &mut Option<Underline> {
        &mut self.underline
    }

    fn get_strike_mut(&mut self) -> &mut Option<Strike> {
        &mut self.strike
    }

    fn get_overline_mut(&mut self) -> &mut Option<Overline> {
        &mut self.overline
    }

    fn get_invert_mut(&mut self) -> &mut Option<Invert> {
        &mut self.invert
    }

    fn get_blink_mut(&mut self) -> &mut Option<Blink> {
        &mut self.blink
    }

    fn get_border_mut(&mut self) -> &mut Option<Border> {
        &mut self.border
    }
}

impl Styler for Style {
    type Output = Self;

    fn foreground(self, foreground: impl Into<Option<Foreground>>) -> Self::Output {
        Style {
            foreground: foreground.into(),
            ..self
        }
    }

    fn background(self, background: impl Into<Option<Background>>) -> Self::Output {
        Style {
            background: background.into(),
            ..self
        }
    }

    fn weight(self, weight: impl Into<Option<Weight>>) -> Self::Output {
        Style {
            weight: weight.into(),
            ..self
        }
    }

    fn slant(self, slant: impl Into<Option<Slant>>) -> Self::Output {
        Style {
            slant: slant.into(),
            ..self
        }
    }

    fn underline(self, underline: impl Into<Option<Underline>>) -> Self::Output {
        Style {
            underline: underline.into(),
            ..self
        }
    }

    fn strike(self, strike: impl Into<Option<Strike>>) -> Self::Output {
        Style {
            strike: strike.into(),
            ..self
        }
    }

    fn overline(self, overline: impl Into<Option<Overline>>) -> Self::Output {
        Style {
            overline: overline.into(),
            ..self
        }
    }

    fn invert(self, invert: impl Into<Option<Invert>>) -> Self::Output {
        Style {
            invert: invert.into(),
            ..self
        }
    }

    fn blink(self, blink: impl Into<Option<Blink>>) -> Self::Output {
        Style {
            blink: blink.into(),
            ..self
        }
    }

    fn border(self, border: impl Into<Option<Border>>) -> Self::Output {
        Style {
            border: border.into(),
            ..self
        }
    }
}

impl StylerMut for Style {
    fn foreground_mut(&mut self, foreground: impl Into<Option<Foreground>>) {
        self.foreground = foreground.into();
    }

    fn background_mut(&mut self, background: impl Into<Option<Background>>) {
        self.background = background.into();
    }

    fn weight_mut(&mut self, weight: impl Into<Option<Weight>>) {
        self.weight = weight.into();
    }

    fn slant_mut(&mut self, slant: impl Into<Option<Slant>>) {
        self.slant = slant.into();
    }

    fn underline_mut(&mut self, underline: impl Into<Option<Underline>>) {
        self.underline = underline.into();
    }

    fn strike_mut(&mut self, strike: impl Into<Option<Strike>>) {
        self.strike = strike.into();
    }

    fn overline_mut(&mut self, overline: impl Into<Option<Overline>>) {
        self.overline = overline.into();
    }

    fn invert_mut(&mut self, invert: impl Into<Option<Invert>>) {
        self.invert = invert.into();
    }

    fn blink_mut(&mut self, blink: impl Into<Option<Blink>>) {
        self.blink = blink.into();
    }

    fn border_mut(&mut self, border: impl Into<Option<Border>>) {
        self.border = border.into();
    }
}

#[cfg(feature = "styler-ops")]
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Rem};

#[cfg(feature = "styler-ops")]
/// Sets `Option<Foreground>`.
impl<Color: Into<Option<Foreground>>> Mul<Color> for Style {
    type Output = Self;

    /// Sets `Option<Foreground>`.
    fn mul(self, foreground: Color) -> Self {
        Styler::foreground(self, foreground)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Background>`.
impl<Color: Into<Option<Background>>> Div<Color> for Style {
    type Output = Self;

    /// Sets `Option<Background>`.
    fn div(self, background: Color) -> Self {
        Styler::background(self, background)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Foreground>`.
impl Add<Foreground> for Style {
    type Output = Self;

    /// Sets `Option<Foreground>`.
    fn add(self, foreground: Foreground) -> Self {
        Styler::foreground(self, foreground)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Foreground>`.
impl Add<NoForeground> for Style {
    type Output = Self;

    /// `None`s `Option<Foreground>`.
    fn add(self, _: NoForeground) -> Self {
        Styler::no_foreground(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Background>`.
impl Add<Background> for Style {
    type Output = Self;

    /// Sets `Option<Background>`.
    fn add(self, background: Background) -> Self {
        Styler::background(self, background)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Background>`.
impl Add<NoBackground> for Style {
    type Output = Self;

    /// `None`s `Option<Background>`.
    fn add(self, _: NoBackground) -> Self {
        Styler::no_background(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Weight>`.
impl Add<Weight> for Style {
    type Output = Self;

    /// Sets `Option<Weight>`.
    fn add(self, weight: Weight) -> Self {
        Styler::weight(self, weight)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Weight>`.
impl Add<NoWeight> for Style {
    type Output = Self;

    /// `None`s `Option<Weight>`.
    fn add(self, _: NoWeight) -> Self {
        Styler::no_weight(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Slant>`.
impl Add<Slant> for Style {
    type Output = Self;

    /// Sets `Option<Slant>`.
    fn add(self, slant: Slant) -> Self {
        Styler::slant(self, slant)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Slant>`.
impl Add<NoSlant> for Style {
    type Output = Self;

    /// `None`s `Option<Slant>`.
    fn add(self, _: NoSlant) -> Self {
        Styler::no_slant(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Underline>`.
impl Add<Underline> for Style {
    type Output = Self;

    /// Sets `Option<Underline>`.
    fn add(self, underline: Underline) -> Self {
        Styler::underline(self, underline)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Underline>`.
impl Add<NoUnderline> for Style {
    type Output = Self;

    /// `None`s `Option<Underline>`.
    fn add(self, _: NoUnderline) -> Self {
        Styler::no_underline(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Strike>`.
impl Add<Strike> for Style {
    type Output = Self;

    /// Sets `Option<Strike>`.
    fn add(self, strike: Strike) -> Self {
        Styler::strike(self, strike)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Strike>`.
impl Add<NoStrike> for Style {
    type Output = Self;

    /// `None`s `Option<Strike>`.
    fn add(self, _: NoStrike) -> Self {
        Styler::no_strike(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Overline>`.
impl Add<Overline> for Style {
    type Output = Self;

    /// Sets `Option<Overline>`.
    fn add(self, overline: Overline) -> Self {
        Styler::overline(self, overline)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Overline>`.
impl Add<NoOverline> for Style {
    type Output = Self;

    /// `None`s `Option<Overline>`.
    fn add(self, _: NoOverline) -> Self {
        Styler::no_overline(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Invert>`.
impl Add<Invert> for Style {
    type Output = Self;

    /// Sets `Option<Invert>`.
    fn add(self, invert: Invert) -> Self {
        Styler::invert(self, invert)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Invert>`.
impl Add<NoInvert> for Style {
    type Output = Self;

    /// `None`s `Option<Invert>`.
    fn add(self, _: NoInvert) -> Self {
        Styler::no_invert(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Blink>`.
impl Add<Blink> for Style {
    type Output = Self;

    /// Sets `Option<Blink>`.
    fn add(self, blink: Blink) -> Self {
        Styler::blink(self, blink)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Blink>`.
impl Add<NoBlink> for Style {
    type Output = Self;

    /// `None`s `Option<Blink>`.
    fn add(self, _: NoBlink) -> Self {
        Styler::no_blink(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Border>`.
impl Add<Border> for Style {
    type Output = Self;

    /// Sets `Option<Border>`.
    fn add(self, border: Border) -> Self {
        Styler::border(self, border)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Border>`.
impl Add<NoBorder> for Style {
    type Output = Self;

    /// `None`s `Option<Border>`.
    fn add(self, _: NoBorder) -> Self {
        Styler::no_border(self)
    }
}

#[cfg(feature = "styler-ops")]
/// `Option::and` fields.
impl<Index: StylerIndex> BitAnd<&Index> for Style {
    type Output = <<Self as Styler>::Output as Styler>::Output;

    /// `Option::and` fields.
    fn bitand(self, styler: &Index) -> <<Self as Styler>::Output as Styler>::Output {
        Styler::and(self, styler)
    }
}

#[cfg(feature = "styler-ops")]
/// `Option::or` fields.
impl<Index: StylerIndex> BitOr<&Index> for Style {
    type Output = <<Self as Styler>::Output as Styler>::Output;

    /// `Option::or` fields.
    fn bitor(self, styler: &Index) -> <<Self as Styler>::Output as Styler>::Output {
        Styler::or(self, styler)
    }
}

#[cfg(feature = "styler-ops")]
/// `Option::xor` fields.
impl<Index: StylerIndex> BitXor<&Index> for Style {
    type Output = <<Self as Styler>::Output as Styler>::Output;

    /// `Option::xor` fields.
    fn bitxor(self, styler: &Index) -> <<Self as Styler>::Output as Styler>::Output {
        Styler::xor(self, styler)
    }
}

#[cfg(feature = "styler-ops")]
/// Dedups (`None`s if identicals) fields.
impl<Index: StylerIndex> Rem<&Index> for Style {
    type Output = Self;

    /// Dedups (`None`s if identicals) fields.
    fn rem(self, styler: &Index) -> Self {
        Styler::dedup(self, styler)
    }
}

#[cfg(feature = "styler-ops")]
/// Resets (sets to reset value) fields which are `Some`.
impl Not for Style {
    type Output = Self;

    /// Resets (sets to reset value) fields which are `Some`.
    fn not(self) -> Self {
        Styler::reset(self)
    }
}

#[cfg(feature = "styler-ops")]
use std::ops::{
    AddAssign,
    BitAndAssign,
    BitOrAssign,
    BitXorAssign,
    DivAssign,
    MulAssign,
    RemAssign,
};

#[cfg(feature = "styler-ops")]
/// Sets `Option<Foreground>`, mutably.
impl<Color: Into<Option<Foreground>>> MulAssign<Color> for Style {
    /// Sets `Option<Foreground>`, mutably.
    fn mul_assign(&mut self, foreground: Color) {
        StylerMut::foreground_mut(self, foreground)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Background>`, mutably.
impl<Color: Into<Option<Background>>> DivAssign<Color> for Style {
    /// Sets `Option<Background>`, mutably.
    fn div_assign(&mut self, background: Color) {
        StylerMut::background_mut(self, background)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Foreground>`, mutably.
impl AddAssign<Foreground> for Style {
    /// Sets `Option<Foreground>`, mutably.
    fn add_assign(&mut self, foreground: Foreground) {
        StylerMut::foreground_mut(self, foreground)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Foreground>`, mutably.
impl AddAssign<NoForeground> for Style {
    /// `None`s `Option<Foreground>`, mutably.
    fn add_assign(&mut self, _: NoForeground) {
        StylerMut::no_foreground_mut(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Background>`, mutably.
impl AddAssign<Background> for Style {
    /// Sets `Option<Background>`, mutably.
    fn add_assign(&mut self, background: Background) {
        StylerMut::background_mut(self, background)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Background>`, mutably.
impl AddAssign<NoBackground> for Style {
    /// `None`s `Option<Background>`, mutably.
    fn add_assign(&mut self, _: NoBackground) {
        StylerMut::no_background_mut(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Weight>`, mutably.
impl AddAssign<Weight> for Style {
    /// Sets `Option<Weight>`, mutably.
    fn add_assign(&mut self, weight: Weight) {
        StylerMut::weight_mut(self, weight)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Weight>`, mutably.
impl AddAssign<NoWeight> for Style {
    /// `None`s `Option<Weight>`, mutably.
    fn add_assign(&mut self, _: NoWeight) {
        StylerMut::no_weight_mut(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Slant>`, mutably.
impl AddAssign<Slant> for Style {
    /// Sets `Option<Slant>`, mutably.
    fn add_assign(&mut self, slant: Slant) {
        StylerMut::slant_mut(self, slant)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Slant>`, mutably.
impl AddAssign<NoSlant> for Style {
    /// `None`s `Option<Slant>`, mutably.
    fn add_assign(&mut self, _: NoSlant) {
        StylerMut::no_slant_mut(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Underline>`, mutably.
impl AddAssign<Underline> for Style {
    /// Sets `Option<Underline>`, mutably.
    fn add_assign(&mut self, underline: Underline) {
        StylerMut::underline_mut(self, underline)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Underline>`, mutably.
impl AddAssign<NoUnderline> for Style {
    /// `None`s `Option<Underline>`, mutably.
    fn add_assign(&mut self, _: NoUnderline) {
        StylerMut::no_underline_mut(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Strike>`, mutably.
impl AddAssign<Strike> for Style {
    /// Sets `Option<Strike>`, mutably.
    fn add_assign(&mut self, strike: Strike) {
        StylerMut::strike_mut(self, strike)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Strike>`, mutably.
impl AddAssign<NoStrike> for Style {
    /// `None`s `Option<Strike>`, mutably.
    fn add_assign(&mut self, _: NoStrike) {
        StylerMut::no_strike_mut(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Overline>`, mutably.
impl AddAssign<Overline> for Style {
    /// Sets `Option<Overline>`, mutably.
    fn add_assign(&mut self, overline: Overline) {
        StylerMut::overline_mut(self, overline)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Overline>`, mutably.
impl AddAssign<NoOverline> for Style {
    /// `None`s `Option<Overline>`, mutably.
    fn add_assign(&mut self, _: NoOverline) {
        StylerMut::no_overline_mut(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Invert>`, mutably.
impl AddAssign<Invert> for Style {
    /// Sets `Option<Invert>`, mutably.
    fn add_assign(&mut self, invert: Invert) {
        StylerMut::invert_mut(self, invert)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Invert>`, mutably.
impl AddAssign<NoInvert> for Style {
    /// `None`s `Option<Invert>`, mutably.
    fn add_assign(&mut self, _: NoInvert) {
        StylerMut::no_invert_mut(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Blink>`, mutably.
impl AddAssign<Blink> for Style {
    /// Sets `Option<Blink>`, mutably.
    fn add_assign(&mut self, blink: Blink) {
        StylerMut::blink_mut(self, blink)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Blink>`, mutably.
impl AddAssign<NoBlink> for Style {
    /// `None`s `Option<Blink>`, mutably.
    fn add_assign(&mut self, _: NoBlink) {
        StylerMut::no_blink_mut(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Border>`, mutably.
impl AddAssign<Border> for Style {
    /// Sets `Option<Border>`, mutably.
    fn add_assign(&mut self, border: Border) {
        StylerMut::border_mut(self, border)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Border>`, mutably.
impl AddAssign<NoBorder> for Style {
    /// `None`s `Option<Border>`, mutably.
    fn add_assign(&mut self, _: NoBorder) {
        StylerMut::no_border_mut(self)
    }
}

#[cfg(feature = "styler-ops")]
/// `Option::and` fields, mutably.
impl<Index: StylerIndex> BitAndAssign<&Index> for Style {
    /// `Option::and` fields, mutably.
    fn bitand_assign(&mut self, styler: &Index) {
        StylerMut::and_mut(self, styler)
    }
}

#[cfg(feature = "styler-ops")]
/// `Option::or` fields, mutably.
impl<Index: StylerIndex> BitOrAssign<&Index> for Style {
    /// `Option::or` fields, mutably.
    fn bitor_assign(&mut self, styler: &Index) {
        StylerMut::or_mut(self, styler)
    }
}

#[cfg(feature = "styler-ops")]
/// `Option::xor` fields, mutably.
impl<Index: StylerIndex> BitXorAssign<&Index> for Style {
    /// `Option::xor` fields, mutably.
    fn bitxor_assign(&mut self, styler: &Index) {
        StylerMut::xor_mut(self, styler)
    }
}

#[cfg(feature = "styler-ops")]
/// Dedups (`None`s if identicals) fields, mutably.
impl<Index: StylerIndex> RemAssign<&Index> for Style {
    /// Dedups (`None`s if identicals) fields, mutably.
    fn rem_assign(&mut self, styler: &Index) {
        StylerMut::dedup_mut(self, styler)
    }
}

#[cfg(feature = "styler-ops")]
/// Resets (sets to reset value) fields which are `Some`, mutably.
impl Not for &mut Style {
    type Output = Self;

    /// Resets (sets to reset value) fields which are `Some`, mutably.
    fn not(self) -> Self {
        StylerMut::reset_mut(self);
        self
    }
}
