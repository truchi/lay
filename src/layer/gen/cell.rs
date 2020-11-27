////////////////////////////////////////////////////////////////////////////////
// 🚨🚨🚨🚨🚨🚨🚨🚨 This file is @generated by build script. 🚨🚨🚨🚨🚨🚨🚨🚨 //
// 🚧🚧🚧🚧🚧🚧🚧🚧           ⛔ DO NOT MODIFY! ⛔           🚧🚧🚧🚧🚧🚧🚧🚧 //
////////////////////////////////////////////////////////////////////////////////

use crate::*;

impl StylerIndex for Cell {
    /// Gets `Option<Foreground>`.
    fn get_foreground(&self) -> Option<Foreground> {
        self.as_ref()
            .as_ref()
            .map(StylerIndex::get_foreground)
            .flatten()
    }

    /// Gets `Option<Background>`.
    fn get_background(&self) -> Option<Background> {
        self.as_ref()
            .as_ref()
            .map(StylerIndex::get_background)
            .flatten()
    }

    /// Gets `Option<Weight>`.
    fn get_weight(&self) -> Option<Weight> {
        self.as_ref()
            .as_ref()
            .map(StylerIndex::get_weight)
            .flatten()
    }

    /// Gets `Option<Slant>`.
    fn get_slant(&self) -> Option<Slant> {
        self.as_ref().as_ref().map(StylerIndex::get_slant).flatten()
    }

    /// Gets `Option<Underline>`.
    fn get_underline(&self) -> Option<Underline> {
        self.as_ref()
            .as_ref()
            .map(StylerIndex::get_underline)
            .flatten()
    }

    /// Gets `Option<Strike>`.
    fn get_strike(&self) -> Option<Strike> {
        self.as_ref()
            .as_ref()
            .map(StylerIndex::get_strike)
            .flatten()
    }

    /// Gets `Option<Overline>`.
    fn get_overline(&self) -> Option<Overline> {
        self.as_ref()
            .as_ref()
            .map(StylerIndex::get_overline)
            .flatten()
    }

    /// Gets `Option<Invert>`.
    fn get_invert(&self) -> Option<Invert> {
        self.as_ref()
            .as_ref()
            .map(StylerIndex::get_invert)
            .flatten()
    }

    /// Gets `Option<Blink>`.
    fn get_blink(&self) -> Option<Blink> {
        self.as_ref().as_ref().map(StylerIndex::get_blink).flatten()
    }

    /// Gets `Option<Border>`.
    fn get_border(&self) -> Option<Border> {
        self.as_ref()
            .as_ref()
            .map(StylerIndex::get_border)
            .flatten()
    }
}

impl Styler for Cell {
    /// Sets `Option<Foreground>`.
    fn foreground(self, foreground: impl Into<Option<Foreground>>) -> Self {
        self.as_ref()
            .map(|styled| Styler::foreground(styled, foreground))
            .into()
    }

    /// Sets `Option<Background>`.
    fn background(self, background: impl Into<Option<Background>>) -> Self {
        self.as_ref()
            .map(|styled| Styler::background(styled, background))
            .into()
    }

    /// Sets `Option<Weight>`.
    fn weight(self, weight: impl Into<Option<Weight>>) -> Self {
        self.as_ref()
            .map(|styled| Styler::weight(styled, weight))
            .into()
    }

    /// Sets `Option<Slant>`.
    fn slant(self, slant: impl Into<Option<Slant>>) -> Self {
        self.as_ref()
            .map(|styled| Styler::slant(styled, slant))
            .into()
    }

    /// Sets `Option<Underline>`.
    fn underline(self, underline: impl Into<Option<Underline>>) -> Self {
        self.as_ref()
            .map(|styled| Styler::underline(styled, underline))
            .into()
    }

    /// Sets `Option<Strike>`.
    fn strike(self, strike: impl Into<Option<Strike>>) -> Self {
        self.as_ref()
            .map(|styled| Styler::strike(styled, strike))
            .into()
    }

    /// Sets `Option<Overline>`.
    fn overline(self, overline: impl Into<Option<Overline>>) -> Self {
        self.as_ref()
            .map(|styled| Styler::overline(styled, overline))
            .into()
    }

    /// Sets `Option<Invert>`.
    fn invert(self, invert: impl Into<Option<Invert>>) -> Self {
        self.as_ref()
            .map(|styled| Styler::invert(styled, invert))
            .into()
    }

    /// Sets `Option<Blink>`.
    fn blink(self, blink: impl Into<Option<Blink>>) -> Self {
        self.as_ref()
            .map(|styled| Styler::blink(styled, blink))
            .into()
    }

    /// Sets `Option<Border>`.
    fn border(self, border: impl Into<Option<Border>>) -> Self {
        self.as_ref()
            .map(|styled| Styler::border(styled, border))
            .into()
    }
}

impl StylerMut for Cell {
    /// Sets `Option<Foreground>`, mutably.
    fn foreground_mut(&mut self, foreground: impl Into<Option<Foreground>>) {
        self.as_mut()
            .map(|mut styled| StylerMut::foreground_mut(&mut styled, foreground));
    }

    /// Sets `Option<Background>`, mutably.
    fn background_mut(&mut self, background: impl Into<Option<Background>>) {
        self.as_mut()
            .map(|mut styled| StylerMut::background_mut(&mut styled, background));
    }

    /// Sets `Option<Weight>`, mutably.
    fn weight_mut(&mut self, weight: impl Into<Option<Weight>>) {
        self.as_mut()
            .map(|mut styled| StylerMut::weight_mut(&mut styled, weight));
    }

    /// Sets `Option<Slant>`, mutably.
    fn slant_mut(&mut self, slant: impl Into<Option<Slant>>) {
        self.as_mut()
            .map(|mut styled| StylerMut::slant_mut(&mut styled, slant));
    }

    /// Sets `Option<Underline>`, mutably.
    fn underline_mut(&mut self, underline: impl Into<Option<Underline>>) {
        self.as_mut()
            .map(|mut styled| StylerMut::underline_mut(&mut styled, underline));
    }

    /// Sets `Option<Strike>`, mutably.
    fn strike_mut(&mut self, strike: impl Into<Option<Strike>>) {
        self.as_mut()
            .map(|mut styled| StylerMut::strike_mut(&mut styled, strike));
    }

    /// Sets `Option<Overline>`, mutably.
    fn overline_mut(&mut self, overline: impl Into<Option<Overline>>) {
        self.as_mut()
            .map(|mut styled| StylerMut::overline_mut(&mut styled, overline));
    }

    /// Sets `Option<Invert>`, mutably.
    fn invert_mut(&mut self, invert: impl Into<Option<Invert>>) {
        self.as_mut()
            .map(|mut styled| StylerMut::invert_mut(&mut styled, invert));
    }

    /// Sets `Option<Blink>`, mutably.
    fn blink_mut(&mut self, blink: impl Into<Option<Blink>>) {
        self.as_mut()
            .map(|mut styled| StylerMut::blink_mut(&mut styled, blink));
    }

    /// Sets `Option<Border>`, mutably.
    fn border_mut(&mut self, border: impl Into<Option<Border>>) {
        self.as_mut()
            .map(|mut styled| StylerMut::border_mut(&mut styled, border));
    }
}
