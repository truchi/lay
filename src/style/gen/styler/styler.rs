////////////////////////////////////////////////////////////////////////////////
// 🚨🚨🚨🚨🚨🚨🚨🚨 This file is @generated by build script. 🚨🚨🚨🚨🚨🚨🚨🚨 //
// 🚧🚧🚧🚧🚧🚧🚧🚧           ⛔ DO NOT MODIFY! ⛔           🚧🚧🚧🚧🚧🚧🚧🚧 //
////////////////////////////////////////////////////////////////////////////////

use crate::*;

/// `Option`al attributes setters.
pub trait Styler: StylerIndex + Sized {
    // ====================================================================== //
    // ====================================================================== //
    //                               Foreground                               //
    // ====================================================================== //
    // ====================================================================== //

    /// Sets `Option<Foreground>`.
    fn foreground(self, foreground: impl Into<Option<Foreground>>) -> Self;

    /// `None`s `Option<Foreground>`.
    fn no_foreground(self) -> Self {
        self.foreground(None)
    }

    /// Sets `Some(Foreground(Color::White))`.
    fn white(self) -> Self {
        self.foreground(Some(Foreground(Color::White)))
    }

    /// Sets `Some(Foreground(Color::Black))`.
    fn black(self) -> Self {
        self.foreground(Some(Foreground(Color::Black)))
    }

    /// Sets `Some(Foreground(Color::Grey))`.
    fn grey(self) -> Self {
        self.foreground(Some(Foreground(Color::Grey)))
    }

    /// Sets `Some(Foreground(Color::DarkGrey))`.
    fn dark_grey(self) -> Self {
        self.foreground(Some(Foreground(Color::DarkGrey)))
    }

    /// Sets `Some(Foreground(Color::Red))`.
    fn red(self) -> Self {
        self.foreground(Some(Foreground(Color::Red)))
    }

    /// Sets `Some(Foreground(Color::DarkRed))`.
    fn dark_red(self) -> Self {
        self.foreground(Some(Foreground(Color::DarkRed)))
    }

    /// Sets `Some(Foreground(Color::Green))`.
    fn green(self) -> Self {
        self.foreground(Some(Foreground(Color::Green)))
    }

    /// Sets `Some(Foreground(Color::DarkGreen))`.
    fn dark_green(self) -> Self {
        self.foreground(Some(Foreground(Color::DarkGreen)))
    }

    /// Sets `Some(Foreground(Color::Yellow))`.
    fn yellow(self) -> Self {
        self.foreground(Some(Foreground(Color::Yellow)))
    }

    /// Sets `Some(Foreground(Color::DarkYellow))`.
    fn dark_yellow(self) -> Self {
        self.foreground(Some(Foreground(Color::DarkYellow)))
    }

    /// Sets `Some(Foreground(Color::Blue))`.
    fn blue(self) -> Self {
        self.foreground(Some(Foreground(Color::Blue)))
    }

    /// Sets `Some(Foreground(Color::DarkBlue))`.
    fn dark_blue(self) -> Self {
        self.foreground(Some(Foreground(Color::DarkBlue)))
    }

    /// Sets `Some(Foreground(Color::Magenta))`.
    fn magenta(self) -> Self {
        self.foreground(Some(Foreground(Color::Magenta)))
    }

    /// Sets `Some(Foreground(Color::DarkMagenta))`.
    fn dark_magenta(self) -> Self {
        self.foreground(Some(Foreground(Color::DarkMagenta)))
    }

    /// Sets `Some(Foreground(Color::Cyan))`.
    fn cyan(self) -> Self {
        self.foreground(Some(Foreground(Color::Cyan)))
    }

    /// Sets `Some(Foreground(Color::DarkCyan))`.
    fn dark_cyan(self) -> Self {
        self.foreground(Some(Foreground(Color::DarkCyan)))
    }

    /// Sets `Some(Foreground(Color::Rgb(r, g, b)))`.
    fn rgb(self, r: u8, g: u8, b: u8) -> Self {
        self.foreground(Some(Foreground(Color::Rgb(r, g, b))))
    }

    /// Sets `Some(Foreground(Color::Ansi(ansi)))`.
    fn ansi(self, ansi: u8) -> Self {
        self.foreground(Some(Foreground(Color::Ansi(ansi))))
    }

    /// Sets `Some(Foreground(Color::ResetColor))`.
    fn reset_color(self) -> Self {
        self.foreground(Some(Foreground(Color::ResetColor)))
    }

    // ====================================================================== //
    // ====================================================================== //
    //                               Background                               //
    // ====================================================================== //
    // ====================================================================== //

    /// Sets `Option<Background>`.
    fn background(self, background: impl Into<Option<Background>>) -> Self;

    /// `None`s `Option<Background>`.
    fn no_background(self) -> Self {
        self.background(None)
    }

    /// Sets `Some(Background(Color::White))`.
    fn on_white(self) -> Self {
        self.background(Some(Background(Color::White)))
    }

    /// Sets `Some(Background(Color::Black))`.
    fn on_black(self) -> Self {
        self.background(Some(Background(Color::Black)))
    }

    /// Sets `Some(Background(Color::Grey))`.
    fn on_grey(self) -> Self {
        self.background(Some(Background(Color::Grey)))
    }

    /// Sets `Some(Background(Color::DarkGrey))`.
    fn on_dark_grey(self) -> Self {
        self.background(Some(Background(Color::DarkGrey)))
    }

    /// Sets `Some(Background(Color::Red))`.
    fn on_red(self) -> Self {
        self.background(Some(Background(Color::Red)))
    }

    /// Sets `Some(Background(Color::DarkRed))`.
    fn on_dark_red(self) -> Self {
        self.background(Some(Background(Color::DarkRed)))
    }

    /// Sets `Some(Background(Color::Green))`.
    fn on_green(self) -> Self {
        self.background(Some(Background(Color::Green)))
    }

    /// Sets `Some(Background(Color::DarkGreen))`.
    fn on_dark_green(self) -> Self {
        self.background(Some(Background(Color::DarkGreen)))
    }

    /// Sets `Some(Background(Color::Yellow))`.
    fn on_yellow(self) -> Self {
        self.background(Some(Background(Color::Yellow)))
    }

    /// Sets `Some(Background(Color::DarkYellow))`.
    fn on_dark_yellow(self) -> Self {
        self.background(Some(Background(Color::DarkYellow)))
    }

    /// Sets `Some(Background(Color::Blue))`.
    fn on_blue(self) -> Self {
        self.background(Some(Background(Color::Blue)))
    }

    /// Sets `Some(Background(Color::DarkBlue))`.
    fn on_dark_blue(self) -> Self {
        self.background(Some(Background(Color::DarkBlue)))
    }

    /// Sets `Some(Background(Color::Magenta))`.
    fn on_magenta(self) -> Self {
        self.background(Some(Background(Color::Magenta)))
    }

    /// Sets `Some(Background(Color::DarkMagenta))`.
    fn on_dark_magenta(self) -> Self {
        self.background(Some(Background(Color::DarkMagenta)))
    }

    /// Sets `Some(Background(Color::Cyan))`.
    fn on_cyan(self) -> Self {
        self.background(Some(Background(Color::Cyan)))
    }

    /// Sets `Some(Background(Color::DarkCyan))`.
    fn on_dark_cyan(self) -> Self {
        self.background(Some(Background(Color::DarkCyan)))
    }

    /// Sets `Some(Background(Color::Rgb(r, g, b)))`.
    fn on_rgb(self, r: u8, g: u8, b: u8) -> Self {
        self.background(Some(Background(Color::Rgb(r, g, b))))
    }

    /// Sets `Some(Background(Color::Ansi(ansi)))`.
    fn on_ansi(self, ansi: u8) -> Self {
        self.background(Some(Background(Color::Ansi(ansi))))
    }

    /// Sets `Some(Background(Color::ResetColor))`.
    fn on_reset_color(self) -> Self {
        self.background(Some(Background(Color::ResetColor)))
    }

    // ====================================================================== //
    // ====================================================================== //
    //                                 Weight                                 //
    // ====================================================================== //
    // ====================================================================== //

    /// Sets `Option<Weight>`.
    fn weight(self, weight: impl Into<Option<Weight>>) -> Self;

    /// `None`s `Option<Weight>`.
    fn no_weight(self) -> Self {
        self.weight(None)
    }

    /// Sets `Some(Weight::Bold)`.
    fn bold(self) -> Self {
        self.weight(Some(Weight::Bold))
    }

    /// Sets `Some(Weight::Light)`.
    fn light(self) -> Self {
        self.weight(Some(Weight::Light))
    }

    /// Sets `Some(Weight::ResetWeight)`.
    fn reset_weight(self) -> Self {
        self.weight(Some(Weight::ResetWeight))
    }

    // ====================================================================== //
    // ====================================================================== //
    //                                 Slant                                  //
    // ====================================================================== //
    // ====================================================================== //

    /// Sets `Option<Slant>`.
    fn slant(self, slant: impl Into<Option<Slant>>) -> Self;

    /// `None`s `Option<Slant>`.
    fn no_slant(self) -> Self {
        self.slant(None)
    }

    /// Sets `Some(Slant::Italic)`.
    fn italic(self) -> Self {
        self.slant(Some(Slant::Italic))
    }

    /// Sets `Some(Slant::ResetSlant)`.
    fn reset_slant(self) -> Self {
        self.slant(Some(Slant::ResetSlant))
    }

    // ====================================================================== //
    // ====================================================================== //
    //                               Underline                                //
    // ====================================================================== //
    // ====================================================================== //

    /// Sets `Option<Underline>`.
    fn underline(self, underline: impl Into<Option<Underline>>) -> Self;

    /// `None`s `Option<Underline>`.
    fn no_underline(self) -> Self {
        self.underline(None)
    }

    /// Sets `Some(Underline::Underlined)`.
    fn underlined(self) -> Self {
        self.underline(Some(Underline::Underlined))
    }

    /// Sets `Some(Underline::ResetUnderline)`.
    fn reset_underline(self) -> Self {
        self.underline(Some(Underline::ResetUnderline))
    }

    // ====================================================================== //
    // ====================================================================== //
    //                                 Strike                                 //
    // ====================================================================== //
    // ====================================================================== //

    /// Sets `Option<Strike>`.
    fn strike(self, strike: impl Into<Option<Strike>>) -> Self;

    /// `None`s `Option<Strike>`.
    fn no_strike(self) -> Self {
        self.strike(None)
    }

    /// Sets `Some(Strike::Striked)`.
    fn striked(self) -> Self {
        self.strike(Some(Strike::Striked))
    }

    /// Sets `Some(Strike::ResetStrike)`.
    fn reset_strike(self) -> Self {
        self.strike(Some(Strike::ResetStrike))
    }

    // ====================================================================== //
    // ====================================================================== //
    //                                Overline                                //
    // ====================================================================== //
    // ====================================================================== //

    /// Sets `Option<Overline>`.
    fn overline(self, overline: impl Into<Option<Overline>>) -> Self;

    /// `None`s `Option<Overline>`.
    fn no_overline(self) -> Self {
        self.overline(None)
    }

    /// Sets `Some(Overline::Overlined)`.
    fn overlined(self) -> Self {
        self.overline(Some(Overline::Overlined))
    }

    /// Sets `Some(Overline::ResetOverline)`.
    fn reset_overline(self) -> Self {
        self.overline(Some(Overline::ResetOverline))
    }

    // ====================================================================== //
    // ====================================================================== //
    //                                 Invert                                 //
    // ====================================================================== //
    // ====================================================================== //

    /// Sets `Option<Invert>`.
    fn invert(self, invert: impl Into<Option<Invert>>) -> Self;

    /// `None`s `Option<Invert>`.
    fn no_invert(self) -> Self {
        self.invert(None)
    }

    /// Sets `Some(Invert::Inverted)`.
    fn inverted(self) -> Self {
        self.invert(Some(Invert::Inverted))
    }

    /// Sets `Some(Invert::ResetInvert)`.
    fn reset_invert(self) -> Self {
        self.invert(Some(Invert::ResetInvert))
    }

    // ====================================================================== //
    // ====================================================================== //
    //                                 Blink                                  //
    // ====================================================================== //
    // ====================================================================== //

    /// Sets `Option<Blink>`.
    fn blink(self, blink: impl Into<Option<Blink>>) -> Self;

    /// `None`s `Option<Blink>`.
    fn no_blink(self) -> Self {
        self.blink(None)
    }

    /// Sets `Some(Blink::Slow)`.
    fn slow(self) -> Self {
        self.blink(Some(Blink::Slow))
    }

    /// Sets `Some(Blink::Fast)`.
    fn fast(self) -> Self {
        self.blink(Some(Blink::Fast))
    }

    /// Sets `Some(Blink::ResetBlink)`.
    fn reset_blink(self) -> Self {
        self.blink(Some(Blink::ResetBlink))
    }

    // ====================================================================== //
    // ====================================================================== //
    //                                 Border                                 //
    // ====================================================================== //
    // ====================================================================== //

    /// Sets `Option<Border>`.
    fn border(self, border: impl Into<Option<Border>>) -> Self;

    /// `None`s `Option<Border>`.
    fn no_border(self) -> Self {
        self.border(None)
    }

    /// Sets `Some(Border::Circle)`.
    fn circle(self) -> Self {
        self.border(Some(Border::Circle))
    }

    /// Sets `Some(Border::Frame)`.
    fn frame(self) -> Self {
        self.border(Some(Border::Frame))
    }

    /// Sets `Some(Border::ResetBorder)`.
    fn reset_border(self) -> Self {
        self.border(Some(Border::ResetBorder))
    }

    // ====================================================================== //
    // ====================================================================== //
    //                          Additional functions                          //
    // ====================================================================== //
    // ====================================================================== //

    /// Applies `styler`'s styles.
    fn style(self, styler: &impl StylerIndex) -> Self {
        self.foreground(styler.get_foreground())
            .background(styler.get_background())
            .weight(styler.get_weight())
            .slant(styler.get_slant())
            .underline(styler.get_underline())
            .strike(styler.get_strike())
            .overline(styler.get_overline())
            .invert(styler.get_invert())
            .blink(styler.get_blink())
            .border(styler.get_border())
    }

    /// `Option::and` fields.
    fn and(self, other: &impl StylerIndex) -> Self {
        let output = self;

        let foreground = output.get_foreground().and(other.get_foreground());
        let output = output.foreground(foreground);

        let background = output.get_background().and(other.get_background());
        let output = output.background(background);

        let weight = output.get_weight().and(other.get_weight());
        let output = output.weight(weight);

        let slant = output.get_slant().and(other.get_slant());
        let output = output.slant(slant);

        let underline = output.get_underline().and(other.get_underline());
        let output = output.underline(underline);

        let strike = output.get_strike().and(other.get_strike());
        let output = output.strike(strike);

        let overline = output.get_overline().and(other.get_overline());
        let output = output.overline(overline);

        let invert = output.get_invert().and(other.get_invert());
        let output = output.invert(invert);

        let blink = output.get_blink().and(other.get_blink());
        let output = output.blink(blink);

        let border = output.get_border().and(other.get_border());
        let output = output.border(border);

        output
    }

    /// `Option::or` fields.
    fn or(self, other: &impl StylerIndex) -> Self {
        let output = self;

        let foreground = output.get_foreground().or(other.get_foreground());
        let output = output.foreground(foreground);

        let background = output.get_background().or(other.get_background());
        let output = output.background(background);

        let weight = output.get_weight().or(other.get_weight());
        let output = output.weight(weight);

        let slant = output.get_slant().or(other.get_slant());
        let output = output.slant(slant);

        let underline = output.get_underline().or(other.get_underline());
        let output = output.underline(underline);

        let strike = output.get_strike().or(other.get_strike());
        let output = output.strike(strike);

        let overline = output.get_overline().or(other.get_overline());
        let output = output.overline(overline);

        let invert = output.get_invert().or(other.get_invert());
        let output = output.invert(invert);

        let blink = output.get_blink().or(other.get_blink());
        let output = output.blink(blink);

        let border = output.get_border().or(other.get_border());
        let output = output.border(border);

        output
    }

    /// `Option::xor` fields.
    fn xor(self, other: &impl StylerIndex) -> Self {
        let output = self;

        let foreground = output.get_foreground().xor(other.get_foreground());
        let output = output.foreground(foreground);

        let background = output.get_background().xor(other.get_background());
        let output = output.background(background);

        let weight = output.get_weight().xor(other.get_weight());
        let output = output.weight(weight);

        let slant = output.get_slant().xor(other.get_slant());
        let output = output.slant(slant);

        let underline = output.get_underline().xor(other.get_underline());
        let output = output.underline(underline);

        let strike = output.get_strike().xor(other.get_strike());
        let output = output.strike(strike);

        let overline = output.get_overline().xor(other.get_overline());
        let output = output.overline(overline);

        let invert = output.get_invert().xor(other.get_invert());
        let output = output.invert(invert);

        let blink = output.get_blink().xor(other.get_blink());
        let output = output.blink(blink);

        let border = output.get_border().xor(other.get_border());
        let output = output.border(border);

        output
    }

    /// Dedups (`None`s if identicals) fields.
    fn dedup(mut self, before: &impl StylerIndex) -> Self {
        if self.get_foreground() == before.get_foreground() {
            self = self.foreground(None);
        }

        if self.get_background() == before.get_background() {
            self = self.background(None);
        }

        if self.get_weight() == before.get_weight() {
            self = self.weight(None);
        }

        if self.get_slant() == before.get_slant() {
            self = self.slant(None);
        }

        if self.get_underline() == before.get_underline() {
            self = self.underline(None);
        }

        if self.get_strike() == before.get_strike() {
            self = self.strike(None);
        }

        if self.get_overline() == before.get_overline() {
            self = self.overline(None);
        }

        if self.get_invert() == before.get_invert() {
            self = self.invert(None);
        }

        if self.get_blink() == before.get_blink() {
            self = self.blink(None);
        }

        if self.get_border() == before.get_border() {
            self = self.border(None);
        }

        self
    }

    /// Resets (sets to reset value) fields which are `Some`.
    fn reset(mut self) -> Self {
        match self.get_foreground() {
            Some(Foreground(Color::ResetColor)) => {}
            Some(_) => self = self.foreground(Some(Foreground(Color::ResetColor))),
            None => {}
        }

        match self.get_background() {
            Some(Background(Color::ResetColor)) => {}
            Some(_) => self = self.background(Some(Background(Color::ResetColor))),
            None => {}
        }

        match self.get_weight() {
            Some(Weight::ResetWeight) => {}
            Some(_) => self = self.weight(Some(Weight::ResetWeight)),
            None => {}
        }

        match self.get_slant() {
            Some(Slant::ResetSlant) => {}
            Some(_) => self = self.slant(Some(Slant::ResetSlant)),
            None => {}
        }

        match self.get_underline() {
            Some(Underline::ResetUnderline) => {}
            Some(_) => self = self.underline(Some(Underline::ResetUnderline)),
            None => {}
        }

        match self.get_strike() {
            Some(Strike::ResetStrike) => {}
            Some(_) => self = self.strike(Some(Strike::ResetStrike)),
            None => {}
        }

        match self.get_overline() {
            Some(Overline::ResetOverline) => {}
            Some(_) => self = self.overline(Some(Overline::ResetOverline)),
            None => {}
        }

        match self.get_invert() {
            Some(Invert::ResetInvert) => {}
            Some(_) => self = self.invert(Some(Invert::ResetInvert)),
            None => {}
        }

        match self.get_blink() {
            Some(Blink::ResetBlink) => {}
            Some(_) => self = self.blink(Some(Blink::ResetBlink)),
            None => {}
        }

        match self.get_border() {
            Some(Border::ResetBorder) => {}
            Some(_) => self = self.border(Some(Border::ResetBorder)),
            None => {}
        }

        self
    }
}
