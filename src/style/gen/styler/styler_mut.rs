////////////////////////////////////////////////////////////////////////////////
// 🚨🚨🚨🚨🚨🚨🚨🚨 This file is @generated by build script. 🚨🚨🚨🚨🚨🚨🚨🚨 //
// 🚧🚧🚧🚧🚧🚧🚧🚧           ⛔ DO NOT MODIFY! ⛔           🚧🚧🚧🚧🚧🚧🚧🚧 //
////////////////////////////////////////////////////////////////////////////////

use crate::*;

/// `Option`al attributes setters, mutably.
pub trait StylerMut: StylerIndex {
    // ====================================================================== //
    // ====================================================================== //
    //                               Foreground                               //
    // ====================================================================== //
    // ====================================================================== //

    /// Sets `Option<Foreground>`, mutably.
    fn foreground_mut(&mut self, foreground: impl Into<Option<Foreground>>);

    /// `None`s `Option<Foreground>`, mutably.
    fn no_foreground_mut(&mut self) {
        self.foreground_mut(None)
    }

    /// Sets `Some(Foreground(Color::White))`, mutably.
    fn white_mut(&mut self) {
        self.foreground_mut(Some(Foreground(Color::White)))
    }

    /// Sets `Some(Foreground(Color::Black))`, mutably.
    fn black_mut(&mut self) {
        self.foreground_mut(Some(Foreground(Color::Black)))
    }

    /// Sets `Some(Foreground(Color::Grey))`, mutably.
    fn grey_mut(&mut self) {
        self.foreground_mut(Some(Foreground(Color::Grey)))
    }

    /// Sets `Some(Foreground(Color::DarkGrey))`, mutably.
    fn dark_grey_mut(&mut self) {
        self.foreground_mut(Some(Foreground(Color::DarkGrey)))
    }

    /// Sets `Some(Foreground(Color::Red))`, mutably.
    fn red_mut(&mut self) {
        self.foreground_mut(Some(Foreground(Color::Red)))
    }

    /// Sets `Some(Foreground(Color::DarkRed))`, mutably.
    fn dark_red_mut(&mut self) {
        self.foreground_mut(Some(Foreground(Color::DarkRed)))
    }

    /// Sets `Some(Foreground(Color::Green))`, mutably.
    fn green_mut(&mut self) {
        self.foreground_mut(Some(Foreground(Color::Green)))
    }

    /// Sets `Some(Foreground(Color::DarkGreen))`, mutably.
    fn dark_green_mut(&mut self) {
        self.foreground_mut(Some(Foreground(Color::DarkGreen)))
    }

    /// Sets `Some(Foreground(Color::Yellow))`, mutably.
    fn yellow_mut(&mut self) {
        self.foreground_mut(Some(Foreground(Color::Yellow)))
    }

    /// Sets `Some(Foreground(Color::DarkYellow))`, mutably.
    fn dark_yellow_mut(&mut self) {
        self.foreground_mut(Some(Foreground(Color::DarkYellow)))
    }

    /// Sets `Some(Foreground(Color::Blue))`, mutably.
    fn blue_mut(&mut self) {
        self.foreground_mut(Some(Foreground(Color::Blue)))
    }

    /// Sets `Some(Foreground(Color::DarkBlue))`, mutably.
    fn dark_blue_mut(&mut self) {
        self.foreground_mut(Some(Foreground(Color::DarkBlue)))
    }

    /// Sets `Some(Foreground(Color::Magenta))`, mutably.
    fn magenta_mut(&mut self) {
        self.foreground_mut(Some(Foreground(Color::Magenta)))
    }

    /// Sets `Some(Foreground(Color::DarkMagenta))`, mutably.
    fn dark_magenta_mut(&mut self) {
        self.foreground_mut(Some(Foreground(Color::DarkMagenta)))
    }

    /// Sets `Some(Foreground(Color::Cyan))`, mutably.
    fn cyan_mut(&mut self) {
        self.foreground_mut(Some(Foreground(Color::Cyan)))
    }

    /// Sets `Some(Foreground(Color::DarkCyan))`, mutably.
    fn dark_cyan_mut(&mut self) {
        self.foreground_mut(Some(Foreground(Color::DarkCyan)))
    }

    /// Sets `Some(Foreground(Color::Rgb(r, g, b)))`, mutably.
    fn rgb_mut(&mut self, r: u8, g: u8, b: u8) {
        self.foreground_mut(Some(Foreground(Color::Rgb(r, g, b))))
    }

    /// Sets `Some(Foreground(Color::Ansi(ansi)))`, mutably.
    fn ansi_mut(&mut self, ansi: u8) {
        self.foreground_mut(Some(Foreground(Color::Ansi(ansi))))
    }

    /// Sets `Some(Foreground(Color::ResetColor))`, mutably.
    fn reset_color_mut(&mut self) {
        self.foreground_mut(Some(Foreground(Color::ResetColor)))
    }

    // ====================================================================== //
    // ====================================================================== //
    //                               Background                               //
    // ====================================================================== //
    // ====================================================================== //

    /// Sets `Option<Background>`, mutably.
    fn background_mut(&mut self, background: impl Into<Option<Background>>);

    /// `None`s `Option<Background>`, mutably.
    fn no_background_mut(&mut self) {
        self.background_mut(None)
    }

    /// Sets `Some(Background(Color::White))`, mutably.
    fn on_white_mut(&mut self) {
        self.background_mut(Some(Background(Color::White)))
    }

    /// Sets `Some(Background(Color::Black))`, mutably.
    fn on_black_mut(&mut self) {
        self.background_mut(Some(Background(Color::Black)))
    }

    /// Sets `Some(Background(Color::Grey))`, mutably.
    fn on_grey_mut(&mut self) {
        self.background_mut(Some(Background(Color::Grey)))
    }

    /// Sets `Some(Background(Color::DarkGrey))`, mutably.
    fn on_dark_grey_mut(&mut self) {
        self.background_mut(Some(Background(Color::DarkGrey)))
    }

    /// Sets `Some(Background(Color::Red))`, mutably.
    fn on_red_mut(&mut self) {
        self.background_mut(Some(Background(Color::Red)))
    }

    /// Sets `Some(Background(Color::DarkRed))`, mutably.
    fn on_dark_red_mut(&mut self) {
        self.background_mut(Some(Background(Color::DarkRed)))
    }

    /// Sets `Some(Background(Color::Green))`, mutably.
    fn on_green_mut(&mut self) {
        self.background_mut(Some(Background(Color::Green)))
    }

    /// Sets `Some(Background(Color::DarkGreen))`, mutably.
    fn on_dark_green_mut(&mut self) {
        self.background_mut(Some(Background(Color::DarkGreen)))
    }

    /// Sets `Some(Background(Color::Yellow))`, mutably.
    fn on_yellow_mut(&mut self) {
        self.background_mut(Some(Background(Color::Yellow)))
    }

    /// Sets `Some(Background(Color::DarkYellow))`, mutably.
    fn on_dark_yellow_mut(&mut self) {
        self.background_mut(Some(Background(Color::DarkYellow)))
    }

    /// Sets `Some(Background(Color::Blue))`, mutably.
    fn on_blue_mut(&mut self) {
        self.background_mut(Some(Background(Color::Blue)))
    }

    /// Sets `Some(Background(Color::DarkBlue))`, mutably.
    fn on_dark_blue_mut(&mut self) {
        self.background_mut(Some(Background(Color::DarkBlue)))
    }

    /// Sets `Some(Background(Color::Magenta))`, mutably.
    fn on_magenta_mut(&mut self) {
        self.background_mut(Some(Background(Color::Magenta)))
    }

    /// Sets `Some(Background(Color::DarkMagenta))`, mutably.
    fn on_dark_magenta_mut(&mut self) {
        self.background_mut(Some(Background(Color::DarkMagenta)))
    }

    /// Sets `Some(Background(Color::Cyan))`, mutably.
    fn on_cyan_mut(&mut self) {
        self.background_mut(Some(Background(Color::Cyan)))
    }

    /// Sets `Some(Background(Color::DarkCyan))`, mutably.
    fn on_dark_cyan_mut(&mut self) {
        self.background_mut(Some(Background(Color::DarkCyan)))
    }

    /// Sets `Some(Background(Color::Rgb(r, g, b)))`, mutably.
    fn on_rgb_mut(&mut self, r: u8, g: u8, b: u8) {
        self.background_mut(Some(Background(Color::Rgb(r, g, b))))
    }

    /// Sets `Some(Background(Color::Ansi(ansi)))`, mutably.
    fn on_ansi_mut(&mut self, ansi: u8) {
        self.background_mut(Some(Background(Color::Ansi(ansi))))
    }

    /// Sets `Some(Background(Color::ResetColor))`, mutably.
    fn on_reset_color_mut(&mut self) {
        self.background_mut(Some(Background(Color::ResetColor)))
    }

    // ====================================================================== //
    // ====================================================================== //
    //                                 Weight                                 //
    // ====================================================================== //
    // ====================================================================== //

    /// Sets `Option<Weight>`, mutably.
    fn weight_mut(&mut self, weight: impl Into<Option<Weight>>);

    /// `None`s `Option<Weight>`, mutably.
    fn no_weight_mut(&mut self) {
        self.weight_mut(None)
    }

    /// Sets `Some(Weight::Bold)`, mutably.
    fn bold_mut(&mut self) {
        self.weight_mut(Some(Weight::Bold))
    }

    /// Sets `Some(Weight::Light)`, mutably.
    fn light_mut(&mut self) {
        self.weight_mut(Some(Weight::Light))
    }

    /// Sets `Some(Weight::ResetWeight)`, mutably.
    fn reset_weight_mut(&mut self) {
        self.weight_mut(Some(Weight::ResetWeight))
    }

    // ====================================================================== //
    // ====================================================================== //
    //                                 Slant                                  //
    // ====================================================================== //
    // ====================================================================== //

    /// Sets `Option<Slant>`, mutably.
    fn slant_mut(&mut self, slant: impl Into<Option<Slant>>);

    /// `None`s `Option<Slant>`, mutably.
    fn no_slant_mut(&mut self) {
        self.slant_mut(None)
    }

    /// Sets `Some(Slant::Italic)`, mutably.
    fn italic_mut(&mut self) {
        self.slant_mut(Some(Slant::Italic))
    }

    /// Sets `Some(Slant::ResetSlant)`, mutably.
    fn reset_slant_mut(&mut self) {
        self.slant_mut(Some(Slant::ResetSlant))
    }

    // ====================================================================== //
    // ====================================================================== //
    //                               Underline                                //
    // ====================================================================== //
    // ====================================================================== //

    /// Sets `Option<Underline>`, mutably.
    fn underline_mut(&mut self, underline: impl Into<Option<Underline>>);

    /// `None`s `Option<Underline>`, mutably.
    fn no_underline_mut(&mut self) {
        self.underline_mut(None)
    }

    /// Sets `Some(Underline::Underlined)`, mutably.
    fn underlined_mut(&mut self) {
        self.underline_mut(Some(Underline::Underlined))
    }

    /// Sets `Some(Underline::ResetUnderline)`, mutably.
    fn reset_underline_mut(&mut self) {
        self.underline_mut(Some(Underline::ResetUnderline))
    }

    // ====================================================================== //
    // ====================================================================== //
    //                                 Strike                                 //
    // ====================================================================== //
    // ====================================================================== //

    /// Sets `Option<Strike>`, mutably.
    fn strike_mut(&mut self, strike: impl Into<Option<Strike>>);

    /// `None`s `Option<Strike>`, mutably.
    fn no_strike_mut(&mut self) {
        self.strike_mut(None)
    }

    /// Sets `Some(Strike::Striked)`, mutably.
    fn striked_mut(&mut self) {
        self.strike_mut(Some(Strike::Striked))
    }

    /// Sets `Some(Strike::ResetStrike)`, mutably.
    fn reset_strike_mut(&mut self) {
        self.strike_mut(Some(Strike::ResetStrike))
    }

    // ====================================================================== //
    // ====================================================================== //
    //                                Overline                                //
    // ====================================================================== //
    // ====================================================================== //

    /// Sets `Option<Overline>`, mutably.
    fn overline_mut(&mut self, overline: impl Into<Option<Overline>>);

    /// `None`s `Option<Overline>`, mutably.
    fn no_overline_mut(&mut self) {
        self.overline_mut(None)
    }

    /// Sets `Some(Overline::Overlined)`, mutably.
    fn overlined_mut(&mut self) {
        self.overline_mut(Some(Overline::Overlined))
    }

    /// Sets `Some(Overline::ResetOverline)`, mutably.
    fn reset_overline_mut(&mut self) {
        self.overline_mut(Some(Overline::ResetOverline))
    }

    // ====================================================================== //
    // ====================================================================== //
    //                                 Invert                                 //
    // ====================================================================== //
    // ====================================================================== //

    /// Sets `Option<Invert>`, mutably.
    fn invert_mut(&mut self, invert: impl Into<Option<Invert>>);

    /// `None`s `Option<Invert>`, mutably.
    fn no_invert_mut(&mut self) {
        self.invert_mut(None)
    }

    /// Sets `Some(Invert::Inverted)`, mutably.
    fn inverted_mut(&mut self) {
        self.invert_mut(Some(Invert::Inverted))
    }

    /// Sets `Some(Invert::ResetInvert)`, mutably.
    fn reset_invert_mut(&mut self) {
        self.invert_mut(Some(Invert::ResetInvert))
    }

    // ====================================================================== //
    // ====================================================================== //
    //                                 Blink                                  //
    // ====================================================================== //
    // ====================================================================== //

    /// Sets `Option<Blink>`, mutably.
    fn blink_mut(&mut self, blink: impl Into<Option<Blink>>);

    /// `None`s `Option<Blink>`, mutably.
    fn no_blink_mut(&mut self) {
        self.blink_mut(None)
    }

    /// Sets `Some(Blink::Slow)`, mutably.
    fn slow_mut(&mut self) {
        self.blink_mut(Some(Blink::Slow))
    }

    /// Sets `Some(Blink::Fast)`, mutably.
    fn fast_mut(&mut self) {
        self.blink_mut(Some(Blink::Fast))
    }

    /// Sets `Some(Blink::ResetBlink)`, mutably.
    fn reset_blink_mut(&mut self) {
        self.blink_mut(Some(Blink::ResetBlink))
    }

    // ====================================================================== //
    // ====================================================================== //
    //                                 Border                                 //
    // ====================================================================== //
    // ====================================================================== //

    /// Sets `Option<Border>`, mutably.
    fn border_mut(&mut self, border: impl Into<Option<Border>>);

    /// `None`s `Option<Border>`, mutably.
    fn no_border_mut(&mut self) {
        self.border_mut(None)
    }

    /// Sets `Some(Border::Circle)`, mutably.
    fn circle_mut(&mut self) {
        self.border_mut(Some(Border::Circle))
    }

    /// Sets `Some(Border::Frame)`, mutably.
    fn frame_mut(&mut self) {
        self.border_mut(Some(Border::Frame))
    }

    /// Sets `Some(Border::ResetBorder)`, mutably.
    fn reset_border_mut(&mut self) {
        self.border_mut(Some(Border::ResetBorder))
    }

    // ====================================================================== //
    // ====================================================================== //
    //                          Additional functions                          //
    // ====================================================================== //
    // ====================================================================== //

    /// Applies `styler`'s styles, mutably.
    fn style_mut(&mut self, styler: &impl StylerIndex) {
        self.foreground_mut(styler.get_foreground());
        self.background_mut(styler.get_background());
        self.weight_mut(styler.get_weight());
        self.slant_mut(styler.get_slant());
        self.underline_mut(styler.get_underline());
        self.strike_mut(styler.get_strike());
        self.overline_mut(styler.get_overline());
        self.invert_mut(styler.get_invert());
        self.blink_mut(styler.get_blink());
        self.border_mut(styler.get_border());
    }

    /// `Option::and` fields, mutably.
    fn and_mut(&mut self, other: &impl StylerIndex) {
        self.foreground_mut(self.get_foreground().and(other.get_foreground()));
        self.background_mut(self.get_background().and(other.get_background()));
        self.weight_mut(self.get_weight().and(other.get_weight()));
        self.slant_mut(self.get_slant().and(other.get_slant()));
        self.underline_mut(self.get_underline().and(other.get_underline()));
        self.strike_mut(self.get_strike().and(other.get_strike()));
        self.overline_mut(self.get_overline().and(other.get_overline()));
        self.invert_mut(self.get_invert().and(other.get_invert()));
        self.blink_mut(self.get_blink().and(other.get_blink()));
        self.border_mut(self.get_border().and(other.get_border()));
    }

    /// `Option::or` fields, mutably.
    fn or_mut(&mut self, other: &impl StylerIndex) {
        self.foreground_mut(self.get_foreground().or(other.get_foreground()));
        self.background_mut(self.get_background().or(other.get_background()));
        self.weight_mut(self.get_weight().or(other.get_weight()));
        self.slant_mut(self.get_slant().or(other.get_slant()));
        self.underline_mut(self.get_underline().or(other.get_underline()));
        self.strike_mut(self.get_strike().or(other.get_strike()));
        self.overline_mut(self.get_overline().or(other.get_overline()));
        self.invert_mut(self.get_invert().or(other.get_invert()));
        self.blink_mut(self.get_blink().or(other.get_blink()));
        self.border_mut(self.get_border().or(other.get_border()));
    }

    /// `Option::xor` fields, mutably.
    fn xor_mut(&mut self, other: &impl StylerIndex) {
        self.foreground_mut(self.get_foreground().xor(other.get_foreground()));
        self.background_mut(self.get_background().xor(other.get_background()));
        self.weight_mut(self.get_weight().xor(other.get_weight()));
        self.slant_mut(self.get_slant().xor(other.get_slant()));
        self.underline_mut(self.get_underline().xor(other.get_underline()));
        self.strike_mut(self.get_strike().xor(other.get_strike()));
        self.overline_mut(self.get_overline().xor(other.get_overline()));
        self.invert_mut(self.get_invert().xor(other.get_invert()));
        self.blink_mut(self.get_blink().xor(other.get_blink()));
        self.border_mut(self.get_border().xor(other.get_border()));
    }

    /// Dedups (`None`s if identicals) fields, mutably.
    fn dedup_mut(&mut self, before: &impl StylerIndex) {
        if self.get_foreground() == before.get_foreground() {
            self.foreground_mut(None);
        }

        if self.get_background() == before.get_background() {
            self.background_mut(None);
        }

        if self.get_weight() == before.get_weight() {
            self.weight_mut(None);
        }

        if self.get_slant() == before.get_slant() {
            self.slant_mut(None);
        }

        if self.get_underline() == before.get_underline() {
            self.underline_mut(None);
        }

        if self.get_strike() == before.get_strike() {
            self.strike_mut(None);
        }

        if self.get_overline() == before.get_overline() {
            self.overline_mut(None);
        }

        if self.get_invert() == before.get_invert() {
            self.invert_mut(None);
        }

        if self.get_blink() == before.get_blink() {
            self.blink_mut(None);
        }

        if self.get_border() == before.get_border() {
            self.border_mut(None);
        }
    }

    /// Resets (sets to reset value) fields which are `Some`, mutably.
    fn reset_mut(&mut self) {
        match self.get_foreground() {
            Some(Foreground(Color::ResetColor)) => {}
            Some(_) => self.foreground_mut(Some(Foreground(Color::ResetColor))),
            None => {}
        }

        match self.get_background() {
            Some(Background(Color::ResetColor)) => {}
            Some(_) => self.background_mut(Some(Background(Color::ResetColor))),
            None => {}
        }

        match self.get_weight() {
            Some(Weight::ResetWeight) => {}
            Some(_) => self.weight_mut(Some(Weight::ResetWeight)),
            None => {}
        }

        match self.get_slant() {
            Some(Slant::ResetSlant) => {}
            Some(_) => self.slant_mut(Some(Slant::ResetSlant)),
            None => {}
        }

        match self.get_underline() {
            Some(Underline::ResetUnderline) => {}
            Some(_) => self.underline_mut(Some(Underline::ResetUnderline)),
            None => {}
        }

        match self.get_strike() {
            Some(Strike::ResetStrike) => {}
            Some(_) => self.strike_mut(Some(Strike::ResetStrike)),
            None => {}
        }

        match self.get_overline() {
            Some(Overline::ResetOverline) => {}
            Some(_) => self.overline_mut(Some(Overline::ResetOverline)),
            None => {}
        }

        match self.get_invert() {
            Some(Invert::ResetInvert) => {}
            Some(_) => self.invert_mut(Some(Invert::ResetInvert)),
            None => {}
        }

        match self.get_blink() {
            Some(Blink::ResetBlink) => {}
            Some(_) => self.blink_mut(Some(Blink::ResetBlink)),
            None => {}
        }

        match self.get_border() {
            Some(Border::ResetBorder) => {}
            Some(_) => self.border_mut(Some(Border::ResetBorder)),
            None => {}
        }
    }
}
