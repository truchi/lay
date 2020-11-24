////////////////////////////////////////////////////////////////////////////////
// 🚨🚨🚨🚨🚨🚨🚨🚨 This file is @generated by build script. 🚨🚨🚨🚨🚨🚨🚨🚨 //
// 🚧🚧🚧🚧🚧🚧🚧🚧           ⛔ DO NOT MODIFY! ⛔           🚧🚧🚧🚧🚧🚧🚧🚧 //
////////////////////////////////////////////////////////////////////////////////

use crate::*;
use std::fmt::Display;

impl<T: Display> StylerIndex for Styled<T> {
    /// Gets `Option<Foreground>`.
    fn get_foreground(&self) -> Option<Foreground> {
        StylerIndex::get_foreground(&self.style)
    }

    /// Gets `Option<Background>`.
    fn get_background(&self) -> Option<Background> {
        StylerIndex::get_background(&self.style)
    }

    /// Gets `Option<Weight>`.
    fn get_weight(&self) -> Option<Weight> {
        StylerIndex::get_weight(&self.style)
    }

    /// Gets `Option<Slant>`.
    fn get_slant(&self) -> Option<Slant> {
        StylerIndex::get_slant(&self.style)
    }

    /// Gets `Option<Underline>`.
    fn get_underline(&self) -> Option<Underline> {
        StylerIndex::get_underline(&self.style)
    }

    /// Gets `Option<Strike>`.
    fn get_strike(&self) -> Option<Strike> {
        StylerIndex::get_strike(&self.style)
    }

    /// Gets `Option<Overline>`.
    fn get_overline(&self) -> Option<Overline> {
        StylerIndex::get_overline(&self.style)
    }

    /// Gets `Option<Invert>`.
    fn get_invert(&self) -> Option<Invert> {
        StylerIndex::get_invert(&self.style)
    }

    /// Gets `Option<Blink>`.
    fn get_blink(&self) -> Option<Blink> {
        StylerIndex::get_blink(&self.style)
    }

    /// Gets `Option<Border>`.
    fn get_border(&self) -> Option<Border> {
        StylerIndex::get_border(&self.style)
    }
}

impl<T: Display> StylerIndexMut for Styled<T> {
    /// Gets `&mut Option<Foreground>`.
    fn get_foreground_mut(&mut self) -> &mut Option<Foreground> {
        StylerIndexMut::get_foreground_mut(&mut self.style)
    }

    /// Gets `&mut Option<Background>`.
    fn get_background_mut(&mut self) -> &mut Option<Background> {
        StylerIndexMut::get_background_mut(&mut self.style)
    }

    /// Gets `&mut Option<Weight>`.
    fn get_weight_mut(&mut self) -> &mut Option<Weight> {
        StylerIndexMut::get_weight_mut(&mut self.style)
    }

    /// Gets `&mut Option<Slant>`.
    fn get_slant_mut(&mut self) -> &mut Option<Slant> {
        StylerIndexMut::get_slant_mut(&mut self.style)
    }

    /// Gets `&mut Option<Underline>`.
    fn get_underline_mut(&mut self) -> &mut Option<Underline> {
        StylerIndexMut::get_underline_mut(&mut self.style)
    }

    /// Gets `&mut Option<Strike>`.
    fn get_strike_mut(&mut self) -> &mut Option<Strike> {
        StylerIndexMut::get_strike_mut(&mut self.style)
    }

    /// Gets `&mut Option<Overline>`.
    fn get_overline_mut(&mut self) -> &mut Option<Overline> {
        StylerIndexMut::get_overline_mut(&mut self.style)
    }

    /// Gets `&mut Option<Invert>`.
    fn get_invert_mut(&mut self) -> &mut Option<Invert> {
        StylerIndexMut::get_invert_mut(&mut self.style)
    }

    /// Gets `&mut Option<Blink>`.
    fn get_blink_mut(&mut self) -> &mut Option<Blink> {
        StylerIndexMut::get_blink_mut(&mut self.style)
    }

    /// Gets `&mut Option<Border>`.
    fn get_border_mut(&mut self) -> &mut Option<Border> {
        StylerIndexMut::get_border_mut(&mut self.style)
    }
}

impl<T: Display> Styler for Styled<T> {
    type Output = Self;

    /// Sets `Option<Foreground>`.
    fn foreground(mut self, foreground: impl Into<Option<Foreground>>) -> Self::Output {
        StylerMut::foreground_mut(&mut self.style, foreground);
        self
    }

    /// Sets `Option<Background>`.
    fn background(mut self, background: impl Into<Option<Background>>) -> Self::Output {
        StylerMut::background_mut(&mut self.style, background);
        self
    }

    /// Sets `Option<Weight>`.
    fn weight(mut self, weight: impl Into<Option<Weight>>) -> Self::Output {
        StylerMut::weight_mut(&mut self.style, weight);
        self
    }

    /// Sets `Option<Slant>`.
    fn slant(mut self, slant: impl Into<Option<Slant>>) -> Self::Output {
        StylerMut::slant_mut(&mut self.style, slant);
        self
    }

    /// Sets `Option<Underline>`.
    fn underline(mut self, underline: impl Into<Option<Underline>>) -> Self::Output {
        StylerMut::underline_mut(&mut self.style, underline);
        self
    }

    /// Sets `Option<Strike>`.
    fn strike(mut self, strike: impl Into<Option<Strike>>) -> Self::Output {
        StylerMut::strike_mut(&mut self.style, strike);
        self
    }

    /// Sets `Option<Overline>`.
    fn overline(mut self, overline: impl Into<Option<Overline>>) -> Self::Output {
        StylerMut::overline_mut(&mut self.style, overline);
        self
    }

    /// Sets `Option<Invert>`.
    fn invert(mut self, invert: impl Into<Option<Invert>>) -> Self::Output {
        StylerMut::invert_mut(&mut self.style, invert);
        self
    }

    /// Sets `Option<Blink>`.
    fn blink(mut self, blink: impl Into<Option<Blink>>) -> Self::Output {
        StylerMut::blink_mut(&mut self.style, blink);
        self
    }

    /// Sets `Option<Border>`.
    fn border(mut self, border: impl Into<Option<Border>>) -> Self::Output {
        StylerMut::border_mut(&mut self.style, border);
        self
    }
}

impl<T: Display> StylerMut for Styled<T> {
    /// Sets `Option<Foreground>`, mutably.
    fn foreground_mut(&mut self, foreground: impl Into<Option<Foreground>>) {
        StylerMut::foreground_mut(&mut self.style, foreground)
    }

    /// Sets `Option<Background>`, mutably.
    fn background_mut(&mut self, background: impl Into<Option<Background>>) {
        StylerMut::background_mut(&mut self.style, background)
    }

    /// Sets `Option<Weight>`, mutably.
    fn weight_mut(&mut self, weight: impl Into<Option<Weight>>) {
        StylerMut::weight_mut(&mut self.style, weight)
    }

    /// Sets `Option<Slant>`, mutably.
    fn slant_mut(&mut self, slant: impl Into<Option<Slant>>) {
        StylerMut::slant_mut(&mut self.style, slant)
    }

    /// Sets `Option<Underline>`, mutably.
    fn underline_mut(&mut self, underline: impl Into<Option<Underline>>) {
        StylerMut::underline_mut(&mut self.style, underline)
    }

    /// Sets `Option<Strike>`, mutably.
    fn strike_mut(&mut self, strike: impl Into<Option<Strike>>) {
        StylerMut::strike_mut(&mut self.style, strike)
    }

    /// Sets `Option<Overline>`, mutably.
    fn overline_mut(&mut self, overline: impl Into<Option<Overline>>) {
        StylerMut::overline_mut(&mut self.style, overline)
    }

    /// Sets `Option<Invert>`, mutably.
    fn invert_mut(&mut self, invert: impl Into<Option<Invert>>) {
        StylerMut::invert_mut(&mut self.style, invert)
    }

    /// Sets `Option<Blink>`, mutably.
    fn blink_mut(&mut self, blink: impl Into<Option<Blink>>) {
        StylerMut::blink_mut(&mut self.style, blink)
    }

    /// Sets `Option<Border>`, mutably.
    fn border_mut(&mut self, border: impl Into<Option<Border>>) {
        StylerMut::border_mut(&mut self.style, border)
    }
}