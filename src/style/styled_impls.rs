////////////////////////////////////////////////////////////////////////////////
// 🚨🚨🚨🚨🚨🚨🚨🚨 This file is @generated by build script. 🚨🚨🚨🚨🚨🚨🚨🚨 //
// 🚧🚧🚧🚧🚧🚧🚧🚧           ⛔ DO NOT MODIFY! ⛔           🚧🚧🚧🚧🚧🚧🚧🚧 //
////////////////////////////////////////////////////////////////////////////////

use crate::*;
use std::{
    fmt::Display,
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Rem},
};

impl<T: Display> StylerIndex for Styled<T> {
    fn get_foreground(&self) -> Option<Foreground> {
        StylerIndex::get_foreground(&self.style)
    }

    fn get_background(&self) -> Option<Background> {
        StylerIndex::get_background(&self.style)
    }

    fn get_weight(&self) -> Option<Weight> {
        StylerIndex::get_weight(&self.style)
    }

    fn get_slant(&self) -> Option<Slant> {
        StylerIndex::get_slant(&self.style)
    }

    fn get_underline(&self) -> Option<Underline> {
        StylerIndex::get_underline(&self.style)
    }

    fn get_strike(&self) -> Option<Strike> {
        StylerIndex::get_strike(&self.style)
    }

    fn get_overline(&self) -> Option<Overline> {
        StylerIndex::get_overline(&self.style)
    }

    fn get_invert(&self) -> Option<Invert> {
        StylerIndex::get_invert(&self.style)
    }

    fn get_blink(&self) -> Option<Blink> {
        StylerIndex::get_blink(&self.style)
    }

    fn get_border(&self) -> Option<Border> {
        StylerIndex::get_border(&self.style)
    }
}

impl<T: Display> StylerIndexMut for Styled<T> {
    fn get_foreground_mut(&mut self) -> &mut Option<Foreground> {
        StylerIndexMut::get_foreground_mut(&mut self.style)
    }

    fn get_background_mut(&mut self) -> &mut Option<Background> {
        StylerIndexMut::get_background_mut(&mut self.style)
    }

    fn get_weight_mut(&mut self) -> &mut Option<Weight> {
        StylerIndexMut::get_weight_mut(&mut self.style)
    }

    fn get_slant_mut(&mut self) -> &mut Option<Slant> {
        StylerIndexMut::get_slant_mut(&mut self.style)
    }

    fn get_underline_mut(&mut self) -> &mut Option<Underline> {
        StylerIndexMut::get_underline_mut(&mut self.style)
    }

    fn get_strike_mut(&mut self) -> &mut Option<Strike> {
        StylerIndexMut::get_strike_mut(&mut self.style)
    }

    fn get_overline_mut(&mut self) -> &mut Option<Overline> {
        StylerIndexMut::get_overline_mut(&mut self.style)
    }

    fn get_invert_mut(&mut self) -> &mut Option<Invert> {
        StylerIndexMut::get_invert_mut(&mut self.style)
    }

    fn get_blink_mut(&mut self) -> &mut Option<Blink> {
        StylerIndexMut::get_blink_mut(&mut self.style)
    }

    fn get_border_mut(&mut self) -> &mut Option<Border> {
        StylerIndexMut::get_border_mut(&mut self.style)
    }
}

impl<T: Display> Styler for Styled<T> {
    type Output = Self;

    fn foreground(self, foreground: impl Into<Option<Foreground>>) -> Self::Output {
        Styler::foreground(self.style, foreground);
        self
    }

    fn background(self, background: impl Into<Option<Background>>) -> Self::Output {
        Styler::background(self.style, background);
        self
    }

    fn weight(self, weight: impl Into<Option<Weight>>) -> Self::Output {
        Styler::weight(self.style, weight);
        self
    }

    fn slant(self, slant: impl Into<Option<Slant>>) -> Self::Output {
        Styler::slant(self.style, slant);
        self
    }

    fn underline(self, underline: impl Into<Option<Underline>>) -> Self::Output {
        Styler::underline(self.style, underline);
        self
    }

    fn strike(self, strike: impl Into<Option<Strike>>) -> Self::Output {
        Styler::strike(self.style, strike);
        self
    }

    fn overline(self, overline: impl Into<Option<Overline>>) -> Self::Output {
        Styler::overline(self.style, overline);
        self
    }

    fn invert(self, invert: impl Into<Option<Invert>>) -> Self::Output {
        Styler::invert(self.style, invert);
        self
    }

    fn blink(self, blink: impl Into<Option<Blink>>) -> Self::Output {
        Styler::blink(self.style, blink);
        self
    }

    fn border(self, border: impl Into<Option<Border>>) -> Self::Output {
        Styler::border(self.style, border);
        self
    }
}

impl<T: Display> StylerMut for Styled<T> {
    fn foreground_mut(&mut self, foreground: impl Into<Option<Foreground>>) {
        StylerMut::foreground_mut(&mut self.style, foreground)
    }

    fn background_mut(&mut self, background: impl Into<Option<Background>>) {
        StylerMut::background_mut(&mut self.style, background)
    }

    fn weight_mut(&mut self, weight: impl Into<Option<Weight>>) {
        StylerMut::weight_mut(&mut self.style, weight)
    }

    fn slant_mut(&mut self, slant: impl Into<Option<Slant>>) {
        StylerMut::slant_mut(&mut self.style, slant)
    }

    fn underline_mut(&mut self, underline: impl Into<Option<Underline>>) {
        StylerMut::underline_mut(&mut self.style, underline)
    }

    fn strike_mut(&mut self, strike: impl Into<Option<Strike>>) {
        StylerMut::strike_mut(&mut self.style, strike)
    }

    fn overline_mut(&mut self, overline: impl Into<Option<Overline>>) {
        StylerMut::overline_mut(&mut self.style, overline)
    }

    fn invert_mut(&mut self, invert: impl Into<Option<Invert>>) {
        StylerMut::invert_mut(&mut self.style, invert)
    }

    fn blink_mut(&mut self, blink: impl Into<Option<Blink>>) {
        StylerMut::blink_mut(&mut self.style, blink)
    }

    fn border_mut(&mut self, border: impl Into<Option<Border>>) {
        StylerMut::border_mut(&mut self.style, border)
    }
}

/// Sets `Option<Foreground>`.
impl<T: Display, Color: Into<Option<Foreground>>> Mul<Color> for Styled<T> {
    type Output = Self;

    /// Sets `Option<Foreground>`.
    fn mul(self, foreground: Color) -> Self {
        Styler::foreground(self, foreground)
    }
}

/// Sets `Option<Background>`.
impl<T: Display, Color: Into<Option<Background>>> Div<Color> for Styled<T> {
    type Output = Self;

    /// Sets `Option<Background>`.
    fn div(self, background: Color) -> Self {
        Styler::background(self, background)
    }
}

/// Sets `Option<Weight>`.
impl<T: Display> Add<Weight> for Styled<T> {
    type Output = Self;

    /// Sets `Option<Weight>`.
    fn add(self, weight: Weight) -> Self {
        Styler::weight(self, weight)
    }
}

/// Sets `Option<Slant>`.
impl<T: Display> Add<Slant> for Styled<T> {
    type Output = Self;

    /// Sets `Option<Slant>`.
    fn add(self, slant: Slant) -> Self {
        Styler::slant(self, slant)
    }
}

/// Sets `Option<Underline>`.
impl<T: Display> Add<Underline> for Styled<T> {
    type Output = Self;

    /// Sets `Option<Underline>`.
    fn add(self, underline: Underline) -> Self {
        Styler::underline(self, underline)
    }
}

/// Sets `Option<Strike>`.
impl<T: Display> Add<Strike> for Styled<T> {
    type Output = Self;

    /// Sets `Option<Strike>`.
    fn add(self, strike: Strike) -> Self {
        Styler::strike(self, strike)
    }
}

/// Sets `Option<Overline>`.
impl<T: Display> Add<Overline> for Styled<T> {
    type Output = Self;

    /// Sets `Option<Overline>`.
    fn add(self, overline: Overline) -> Self {
        Styler::overline(self, overline)
    }
}

/// Sets `Option<Invert>`.
impl<T: Display> Add<Invert> for Styled<T> {
    type Output = Self;

    /// Sets `Option<Invert>`.
    fn add(self, invert: Invert) -> Self {
        Styler::invert(self, invert)
    }
}

/// Sets `Option<Blink>`.
impl<T: Display> Add<Blink> for Styled<T> {
    type Output = Self;

    /// Sets `Option<Blink>`.
    fn add(self, blink: Blink) -> Self {
        Styler::blink(self, blink)
    }
}

/// Sets `Option<Border>`.
impl<T: Display> Add<Border> for Styled<T> {
    type Output = Self;

    /// Sets `Option<Border>`.
    fn add(self, border: Border) -> Self {
        Styler::border(self, border)
    }
}

/// `Option::and` fields.
impl<T: Display, Index: StylerIndex> BitAnd<&Index> for Styled<T> {
    type Output = <<Self as Styler>::Output as Styler>::Output;

    /// `Option::and` fields.
    fn bitand(self, styler: &Index) -> <<Self as Styler>::Output as Styler>::Output {
        Styler::and(self, styler)
    }
}

/// `Option::or` fields.
impl<T: Display, Index: StylerIndex> BitOr<&Index> for Styled<T> {
    type Output = <<Self as Styler>::Output as Styler>::Output;

    /// `Option::or` fields.
    fn bitor(self, styler: &Index) -> <<Self as Styler>::Output as Styler>::Output {
        Styler::or(self, styler)
    }
}

/// `Option::xor` fields.
impl<T: Display, Index: StylerIndex> BitXor<&Index> for Styled<T> {
    type Output = <<Self as Styler>::Output as Styler>::Output;

    /// `Option::xor` fields.
    fn bitxor(self, styler: &Index) -> <<Self as Styler>::Output as Styler>::Output {
        Styler::xor(self, styler)
    }
}

/// Dedups (`None`s if identicals) fields.
impl<T: Display, Index: StylerIndex> Rem<&Index> for Styled<T> {
    type Output = Self;

    /// Dedups (`None`s if identicals) fields.
    fn rem(self, styler: &Index) -> Self {
        Styler::dedup(self, styler)
    }
}

/// Resets (sets to reset value) fields which are `Some`.
impl<T: Display> Not for Styled<T> {
    type Output = Self;

    /// Resets (sets to reset value) fields which are `Some`.
    fn not(self) -> Self {
        Styler::reset(self)
    }
}
