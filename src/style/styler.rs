use super::{
    Background,
    Blinking,
    Bordered,
    Color,
    Foreground,
    Inverted,
    Overlined,
    Slanted,
    Striked,
    Underlined,
    Weighted,
};

macro_rules! styler {
    (colors {
        $($get_color:ident $set_color:ident $set_color_mut:ident: $Color:ident {
            $set_rgb:ident $set_rgb_mut:ident $set_ansi:ident $set_ansi_mut:ident
            $($set_color_variant:ident $set_color_variant_mut:ident: $color_variant:ident)*
        })*
    }
    attributes {
        $($get_attr:ident $set_attr:ident $set_attr_mut:ident: $Attr:ty {
            $($set_attr_variant:ident $set_attr_variant_mut:ident : $attr_variant:ident)*
        })*
    }) => {
        /// A trait for styled types.
        pub trait Styler: Sized {
            $(
                doc!("Gets `" stringify!($Color) "`",
                    fn $get_color(&self) -> $Color;
                );
                doc!("Sets `" stringify!($Color) "`",
                    fn $set_color_mut(&mut self, color: $Color);
                );

                doc!("Sets `" stringify!($Color) "`",
                    fn $set_color(mut self, color: $Color) -> Self {
                        self.$set_color_mut(color);
                        self
                    }
                );

                doc!("Sets `" stringify!($Color) "(Color::Rbg { r, g, b })`",
                    fn $set_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
                        self.$set_rgb_mut(r, g, b);
                        self
                    }
                );
                doc!("Sets `" stringify!($Color) "(Color::Rbg { r, g, b })`",
                    fn $set_rgb_mut(&mut self, r: u8, g: u8, b: u8) {
                        self.$set_color_mut($Color(Color::Rgb { r, g, b }));
                    }
                );
                doc!("Sets `" stringify!($Color) "(Color::AnsiValue(value))`",
                    fn $set_ansi(mut self, value: u8) -> Self {
                        self.$set_ansi_mut(value);
                        self
                    }
                );
                doc!("Sets `" stringify!($Color) "(Color::AnsiValue(value))`",
                    fn $set_ansi_mut(&mut self, value: u8) {
                        self.$set_color_mut($Color(Color::AnsiValue(value)));
                    }
                );
                $(
                    doc!("Sets `" stringify!($Color) "(Color::" stringify!($color_variant) ")`",
                        fn $set_color_variant(mut self) -> Self {
                            self.$set_color_variant_mut();
                            self
                        }
                    );
                    doc!("Sets `" stringify!($Color) "(Color::" stringify!($color_variant) ")`",
                        fn $set_color_variant_mut(&mut self) {
                            self.$set_color_mut($Color(Color::$color_variant));
                        }
                    );
                )*
            )*
            $(
                doc!("Gets `" stringify!($Attr) "`",
                    fn $get_attr(&self) -> $Attr;
                );
                doc!("Sets `" stringify!($Attr) "`",
                    fn $set_attr_mut(&mut self, attribute: $Attr);
                );

                doc!("Sets `" stringify!($Attr) "`",
                    fn $set_attr(mut self, attribute: $Attr) -> Self {
                        self.$set_attr_mut(attribute);
                        self
                    }
                );

                $(
                    doc!("Sets `" stringify!($Attr) "::" stringify!($attr_variant) "`",
                        fn $set_attr_variant(mut self) -> Self {
                            self.$set_attr_variant_mut();
                            self
                        }
                    );
                    doc!("Sets `" stringify!($Attr) "::" stringify!($attr_variant) "`",
                        fn $set_attr_variant_mut(&mut self)  {
                            self.$set_attr_mut(<$Attr>::$attr_variant);
                        }
                    );
                )*
            )*
        }
    };
}

styler!(
    colors {
        get_foreground foreground foreground_mut: Foreground {
            rgb rgb_mut ansi ansi_mut
            white white_mut: White
            black black_mut: Black
            red red_mut: Red
            dark_red dark_red_mut: DarkRed
            green green_mut: Green
            dark_green dark_green_mut: DarkGreen
            yellow yellow_mut: Yellow
            dark_yellow dark_yellow_mut: DarkYellow
            blue blue_mut: Blue
            dark_blue dark_blue_mut: DarkBlue
            magenta magenta_mut: Magenta
            dark_magenta dark_magenta_mut: DarkMagenta
            cyan cyan_mut: Cyan
            dark_cyan dark_cyan_mut: DarkCyan
        }
        get_background background background_mut: Background {
            on_rgb on_rgb_mut on_ansi on_ansi_mut
            on_white on_white_mut: White
            on_black on_black_mut: Black
            on_red on_red_mut: Red
            on_dark_red on_dark_red_mut: DarkRed
            on_green on_green_mut: Green
            on_dark_green on_dark_green_mut: DarkGreen
            on_yellow on_yellow_mut: Yellow
            on_dark_yellow on_dark_yellow_mut: DarkYellow
            on_blue on_blue_mut: Blue
            on_dark_blue on_dark_blue_mut: DarkBlue
            on_magenta on_magenta_mut: Magenta
            on_dark_magenta on_dark_magenta_mut: DarkMagenta
            on_cyan on_cyan_mut: Cyan
            on_dark_cyan on_dark_cyan_mut: DarkCyan
        }
    }
    attributes {
        get_weighted weighted weighted_mut: Weighted {
            bold bold_mut: Bold
            light light_mut: Light
            reset_weight reset_weight_mut: ResetWeight
        }
        get_slanted slanted slanted_mut: Slanted {
            italic italic_mut: Italic
            reset_slant reset_slant_mut: ResetSlant
        }
        get_blinking blinking blinking_mut: Blinking {
            slow slow_mut: Slow
            fast fast_mut: Fast
            reset_blink reset_blink_mut: ResetBlink
        }
        get_inverted inverted inverted_mut: Inverted {
            invert invert_mut: Invert
            reset_invert reset_invert_mut: ResetInvert
        }
        get_striked striked striked_mut: Striked {
            strike strike_mut: Strike
            reset_strike reset_strike_mut: ResetStrike
        }
        get_underlined underlined underlined_mut: Underlined {
            underline underline_mut: Underline
            reset_underline reset_underline_mut: ResetUnderline
        }
        get_overlined overlined overlined_mut: Overlined {
            overline overline_mut: Overline
            reset_overline reset_overline_mut: ResetOverline
        }
        get_bordered bordered bordered_mut: Bordered {
            frame frame_mut: Frame
            circle circle_mut: Circle
            reset_border reset_border_mut: ResetBorder
        }
    }
);
