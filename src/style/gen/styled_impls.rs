////////////////////////////////////////////////////////////////////////////////
// 🚨🚨🚨🚨🚨🚨🚨🚨 This file is @generated by build script. 🚨🚨🚨🚨🚨🚨🚨🚨 //
// 🚧🚧🚧🚧🚧🚧🚧🚧           ⛔ DO NOT MODIFY! ⛔           🚧🚧🚧🚧🚧🚧🚧🚧 //
////////////////////////////////////////////////////////////////////////////////

use crate::*;
use std::{fmt::Display, ops::Not};

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

#[cfg(feature = "styler-ops")]
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Rem};

#[cfg(feature = "styler-ops")]
/// Sets `Option<Foreground>`.
impl<T: Display, Color: Into<Option<Foreground>>> Mul<Color> for Styled<T> {
    type Output = Self;

    /// Sets `Option<Foreground>`.
    fn mul(self, foreground: Color) -> Self {
        Styler::foreground(self, foreground)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Background>`.
impl<T: Display, Color: Into<Option<Background>>> Div<Color> for Styled<T> {
    type Output = Self;

    /// Sets `Option<Background>`.
    fn div(self, background: Color) -> Self {
        Styler::background(self, background)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Foreground>`.
impl<T: Display> Add<Foreground> for Styled<T> {
    type Output = Self;

    /// Sets `Option<Foreground>`.
    fn add(self, foreground: Foreground) -> Self {
        Styler::foreground(self, foreground)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Foreground>`.
impl<T: Display> Add<NoForeground> for Styled<T> {
    type Output = Self;

    /// `None`s `Option<Foreground>`.
    fn add(self, _: NoForeground) -> Self {
        Styler::no_foreground(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Background>`.
impl<T: Display> Add<Background> for Styled<T> {
    type Output = Self;

    /// Sets `Option<Background>`.
    fn add(self, background: Background) -> Self {
        Styler::background(self, background)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Background>`.
impl<T: Display> Add<NoBackground> for Styled<T> {
    type Output = Self;

    /// `None`s `Option<Background>`.
    fn add(self, _: NoBackground) -> Self {
        Styler::no_background(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Weight>`.
impl<T: Display> Add<Weight> for Styled<T> {
    type Output = Self;

    /// Sets `Option<Weight>`.
    fn add(self, weight: Weight) -> Self {
        Styler::weight(self, weight)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Weight>`.
impl<T: Display> Add<NoWeight> for Styled<T> {
    type Output = Self;

    /// `None`s `Option<Weight>`.
    fn add(self, _: NoWeight) -> Self {
        Styler::no_weight(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Slant>`.
impl<T: Display> Add<Slant> for Styled<T> {
    type Output = Self;

    /// Sets `Option<Slant>`.
    fn add(self, slant: Slant) -> Self {
        Styler::slant(self, slant)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Slant>`.
impl<T: Display> Add<NoSlant> for Styled<T> {
    type Output = Self;

    /// `None`s `Option<Slant>`.
    fn add(self, _: NoSlant) -> Self {
        Styler::no_slant(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Underline>`.
impl<T: Display> Add<Underline> for Styled<T> {
    type Output = Self;

    /// Sets `Option<Underline>`.
    fn add(self, underline: Underline) -> Self {
        Styler::underline(self, underline)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Underline>`.
impl<T: Display> Add<NoUnderline> for Styled<T> {
    type Output = Self;

    /// `None`s `Option<Underline>`.
    fn add(self, _: NoUnderline) -> Self {
        Styler::no_underline(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Strike>`.
impl<T: Display> Add<Strike> for Styled<T> {
    type Output = Self;

    /// Sets `Option<Strike>`.
    fn add(self, strike: Strike) -> Self {
        Styler::strike(self, strike)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Strike>`.
impl<T: Display> Add<NoStrike> for Styled<T> {
    type Output = Self;

    /// `None`s `Option<Strike>`.
    fn add(self, _: NoStrike) -> Self {
        Styler::no_strike(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Overline>`.
impl<T: Display> Add<Overline> for Styled<T> {
    type Output = Self;

    /// Sets `Option<Overline>`.
    fn add(self, overline: Overline) -> Self {
        Styler::overline(self, overline)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Overline>`.
impl<T: Display> Add<NoOverline> for Styled<T> {
    type Output = Self;

    /// `None`s `Option<Overline>`.
    fn add(self, _: NoOverline) -> Self {
        Styler::no_overline(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Invert>`.
impl<T: Display> Add<Invert> for Styled<T> {
    type Output = Self;

    /// Sets `Option<Invert>`.
    fn add(self, invert: Invert) -> Self {
        Styler::invert(self, invert)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Invert>`.
impl<T: Display> Add<NoInvert> for Styled<T> {
    type Output = Self;

    /// `None`s `Option<Invert>`.
    fn add(self, _: NoInvert) -> Self {
        Styler::no_invert(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Blink>`.
impl<T: Display> Add<Blink> for Styled<T> {
    type Output = Self;

    /// Sets `Option<Blink>`.
    fn add(self, blink: Blink) -> Self {
        Styler::blink(self, blink)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Blink>`.
impl<T: Display> Add<NoBlink> for Styled<T> {
    type Output = Self;

    /// `None`s `Option<Blink>`.
    fn add(self, _: NoBlink) -> Self {
        Styler::no_blink(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Border>`.
impl<T: Display> Add<Border> for Styled<T> {
    type Output = Self;

    /// Sets `Option<Border>`.
    fn add(self, border: Border) -> Self {
        Styler::border(self, border)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Border>`.
impl<T: Display> Add<NoBorder> for Styled<T> {
    type Output = Self;

    /// `None`s `Option<Border>`.
    fn add(self, _: NoBorder) -> Self {
        Styler::no_border(self)
    }
}

#[cfg(feature = "styler-ops")]
/// `Option::and` fields.
impl<T: Display, Index: StylerIndex> BitAnd<&Index> for Styled<T> {
    type Output = <<Self as Styler>::Output as Styler>::Output;

    /// `Option::and` fields.
    fn bitand(self, styler: &Index) -> <<Self as Styler>::Output as Styler>::Output {
        Styler::and(self, styler)
    }
}

#[cfg(feature = "styler-ops")]
/// `Option::or` fields.
impl<T: Display, Index: StylerIndex> BitOr<&Index> for Styled<T> {
    type Output = <<Self as Styler>::Output as Styler>::Output;

    /// `Option::or` fields.
    fn bitor(self, styler: &Index) -> <<Self as Styler>::Output as Styler>::Output {
        Styler::or(self, styler)
    }
}

#[cfg(feature = "styler-ops")]
/// `Option::xor` fields.
impl<T: Display, Index: StylerIndex> BitXor<&Index> for Styled<T> {
    type Output = <<Self as Styler>::Output as Styler>::Output;

    /// `Option::xor` fields.
    fn bitxor(self, styler: &Index) -> <<Self as Styler>::Output as Styler>::Output {
        Styler::xor(self, styler)
    }
}

#[cfg(feature = "styler-ops")]
/// Dedups (`None`s if identicals) fields.
impl<T: Display, Index: StylerIndex> Rem<&Index> for Styled<T> {
    type Output = Self;

    /// Dedups (`None`s if identicals) fields.
    fn rem(self, styler: &Index) -> Self {
        Styler::dedup(self, styler)
    }
}

#[cfg(feature = "styler-ops")]
/// Resets (sets to reset value) fields which are `Some`.
impl<T: Display> Not for Styled<T> {
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
impl<T: Display, Color: Into<Option<Foreground>>> MulAssign<Color> for Styled<T> {
    /// Sets `Option<Foreground>`, mutably.
    fn mul_assign(&mut self, foreground: Color) {
        StylerMut::foreground_mut(self, foreground)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Background>`, mutably.
impl<T: Display, Color: Into<Option<Background>>> DivAssign<Color> for Styled<T> {
    /// Sets `Option<Background>`, mutably.
    fn div_assign(&mut self, background: Color) {
        StylerMut::background_mut(self, background)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Foreground>`, mutably.
impl<T: Display> AddAssign<Foreground> for Styled<T> {
    /// Sets `Option<Foreground>`, mutably.
    fn add_assign(&mut self, foreground: Foreground) {
        StylerMut::foreground_mut(self, foreground)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Foreground>`, mutably.
impl<T: Display> AddAssign<NoForeground> for Styled<T> {
    /// `None`s `Option<Foreground>`, mutably.
    fn add_assign(&mut self, _: NoForeground) {
        StylerMut::no_foreground_mut(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Background>`, mutably.
impl<T: Display> AddAssign<Background> for Styled<T> {
    /// Sets `Option<Background>`, mutably.
    fn add_assign(&mut self, background: Background) {
        StylerMut::background_mut(self, background)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Background>`, mutably.
impl<T: Display> AddAssign<NoBackground> for Styled<T> {
    /// `None`s `Option<Background>`, mutably.
    fn add_assign(&mut self, _: NoBackground) {
        StylerMut::no_background_mut(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Weight>`, mutably.
impl<T: Display> AddAssign<Weight> for Styled<T> {
    /// Sets `Option<Weight>`, mutably.
    fn add_assign(&mut self, weight: Weight) {
        StylerMut::weight_mut(self, weight)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Weight>`, mutably.
impl<T: Display> AddAssign<NoWeight> for Styled<T> {
    /// `None`s `Option<Weight>`, mutably.
    fn add_assign(&mut self, _: NoWeight) {
        StylerMut::no_weight_mut(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Slant>`, mutably.
impl<T: Display> AddAssign<Slant> for Styled<T> {
    /// Sets `Option<Slant>`, mutably.
    fn add_assign(&mut self, slant: Slant) {
        StylerMut::slant_mut(self, slant)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Slant>`, mutably.
impl<T: Display> AddAssign<NoSlant> for Styled<T> {
    /// `None`s `Option<Slant>`, mutably.
    fn add_assign(&mut self, _: NoSlant) {
        StylerMut::no_slant_mut(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Underline>`, mutably.
impl<T: Display> AddAssign<Underline> for Styled<T> {
    /// Sets `Option<Underline>`, mutably.
    fn add_assign(&mut self, underline: Underline) {
        StylerMut::underline_mut(self, underline)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Underline>`, mutably.
impl<T: Display> AddAssign<NoUnderline> for Styled<T> {
    /// `None`s `Option<Underline>`, mutably.
    fn add_assign(&mut self, _: NoUnderline) {
        StylerMut::no_underline_mut(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Strike>`, mutably.
impl<T: Display> AddAssign<Strike> for Styled<T> {
    /// Sets `Option<Strike>`, mutably.
    fn add_assign(&mut self, strike: Strike) {
        StylerMut::strike_mut(self, strike)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Strike>`, mutably.
impl<T: Display> AddAssign<NoStrike> for Styled<T> {
    /// `None`s `Option<Strike>`, mutably.
    fn add_assign(&mut self, _: NoStrike) {
        StylerMut::no_strike_mut(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Overline>`, mutably.
impl<T: Display> AddAssign<Overline> for Styled<T> {
    /// Sets `Option<Overline>`, mutably.
    fn add_assign(&mut self, overline: Overline) {
        StylerMut::overline_mut(self, overline)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Overline>`, mutably.
impl<T: Display> AddAssign<NoOverline> for Styled<T> {
    /// `None`s `Option<Overline>`, mutably.
    fn add_assign(&mut self, _: NoOverline) {
        StylerMut::no_overline_mut(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Invert>`, mutably.
impl<T: Display> AddAssign<Invert> for Styled<T> {
    /// Sets `Option<Invert>`, mutably.
    fn add_assign(&mut self, invert: Invert) {
        StylerMut::invert_mut(self, invert)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Invert>`, mutably.
impl<T: Display> AddAssign<NoInvert> for Styled<T> {
    /// `None`s `Option<Invert>`, mutably.
    fn add_assign(&mut self, _: NoInvert) {
        StylerMut::no_invert_mut(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Blink>`, mutably.
impl<T: Display> AddAssign<Blink> for Styled<T> {
    /// Sets `Option<Blink>`, mutably.
    fn add_assign(&mut self, blink: Blink) {
        StylerMut::blink_mut(self, blink)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Blink>`, mutably.
impl<T: Display> AddAssign<NoBlink> for Styled<T> {
    /// `None`s `Option<Blink>`, mutably.
    fn add_assign(&mut self, _: NoBlink) {
        StylerMut::no_blink_mut(self)
    }
}

#[cfg(feature = "styler-ops")]
/// Sets `Option<Border>`, mutably.
impl<T: Display> AddAssign<Border> for Styled<T> {
    /// Sets `Option<Border>`, mutably.
    fn add_assign(&mut self, border: Border) {
        StylerMut::border_mut(self, border)
    }
}

#[cfg(feature = "styler-ops")]
/// `None`s `Option<Border>`, mutably.
impl<T: Display> AddAssign<NoBorder> for Styled<T> {
    /// `None`s `Option<Border>`, mutably.
    fn add_assign(&mut self, _: NoBorder) {
        StylerMut::no_border_mut(self)
    }
}

#[cfg(feature = "styler-ops")]
/// `Option::and` fields, mutably.
impl<T: Display, Index: StylerIndex> BitAndAssign<&Index> for Styled<T> {
    /// `Option::and` fields, mutably.
    fn bitand_assign(&mut self, styler: &Index) {
        StylerMut::and_mut(self, styler)
    }
}

#[cfg(feature = "styler-ops")]
/// `Option::or` fields, mutably.
impl<T: Display, Index: StylerIndex> BitOrAssign<&Index> for Styled<T> {
    /// `Option::or` fields, mutably.
    fn bitor_assign(&mut self, styler: &Index) {
        StylerMut::or_mut(self, styler)
    }
}

#[cfg(feature = "styler-ops")]
/// `Option::xor` fields, mutably.
impl<T: Display, Index: StylerIndex> BitXorAssign<&Index> for Styled<T> {
    /// `Option::xor` fields, mutably.
    fn bitxor_assign(&mut self, styler: &Index) {
        StylerMut::xor_mut(self, styler)
    }
}

#[cfg(feature = "styler-ops")]
/// Dedups (`None`s if identicals) fields, mutably.
impl<T: Display, Index: StylerIndex> RemAssign<&Index> for Styled<T> {
    /// Dedups (`None`s if identicals) fields, mutably.
    fn rem_assign(&mut self, styler: &Index) {
        StylerMut::dedup_mut(self, styler)
    }
}

#[cfg(feature = "styler-ops")]
/// Resets (sets to reset value) fields which are `Some`, mutably.
impl<T: Display> Not for &mut Styled<T> {
    type Output = Self;

    /// Resets (sets to reset value) fields which are `Some`, mutably.
    fn not(self) -> Self {
        StylerMut::reset_mut(self);
        self
    }
}
