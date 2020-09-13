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

macro_rules! getters {
    ([$get:ident $get_mut:ident] -> $ReturnType:ty) => {
        doc!("Returns a `" stringify!($ReturnType) "`",
            fn $get(&self) -> $ReturnType;
        );
        doc!("Returns a `&mut " stringify!($ReturnType) "`",
            fn $get_mut(&mut self) -> &mut $ReturnType;
        );
    }
}

macro_rules! setter {
    ($($doc:expr)*, [$set:ident $set_mut:ident] ($($arg:ident: $ArgType:ty)*)) => {
        doc!($($doc)*,
            fn $set(mut self $(, $arg: $ArgType)*) -> Self {
                self.$set_mut($($arg,)*);
                self
            }
        );

    }
}

macro_rules! setter_mut {
    ($($doc:expr)*, [$get_mut:ident $set_mut:ident] ($($arg:ident: $ArgType:ty)*) $body:tt) => {
        doc!($($doc)*,
            fn $set_mut(&mut self $(, $arg: $ArgType)*) {
                *self.$get_mut() = { $body };
            }
        );

    }
}

macro_rules! with {
    ($($doc:expr)*, $Trait:ident {
        get { $get:ident $get_mut:ident -> $ReturnType:ty }
        set {
            $($set_doc:expr)*,
            [$set:ident $set_mut:ident]
            ($set_arg:ident: $SetArgType:ty)
            $set_body:tt
        }
        $(unset { [$unset:ident $unset_mut:ident] })?
        $(specials { $(
            $($special_doc:expr)*,
            [$special:ident $special_mut:ident]
            ($($special_arg:ident: $SpecialArgType:ty)*)
            $special_body:tt
        )* })?
        variants { $(
            $($variant_doc:expr)*,
            [$set_variant:ident $set_variant_mut:ident]
            $variant_body:tt
        )* }
    }) => {
        doc!($($doc)*,
            pub trait $Trait: Sized {
                getters!([$get $get_mut] -> $ReturnType);

                setter!($($set_doc)*,
                    [$set $set_mut]
                    ($set_arg: $SetArgType));
                setter_mut!($($set_doc)*,
                    [$get_mut $set_mut]
                    ($set_arg: $SetArgType)
                    $set_body);

                $(
                    setter!("Sets `None`",
                        [$unset $unset_mut] ());
                    setter_mut!("Sets `None`",
                        [$get_mut $unset_mut] ()
                        { None });
                )?

                $(
                    setter!($($variant_doc)*,
                        [$set_variant $set_variant_mut] ());
                    setter_mut!($($variant_doc)*,
                        [$get_mut $set_variant_mut] ()
                        $variant_body);
                )*

                $($(
                    setter!($($special_doc)*,
                        [$special $special_mut]
                        ($($special_arg: $SpecialArgType)*));
                    setter_mut!($($special_doc)*,
                        [$get_mut $special_mut]
                        ($($special_arg: $SpecialArgType)*)
                        $special_body);
                )*)?
            }
        );
    };
}

macro_rules! with_color {
    (
        $Trait:ident
        $OptionalTrait:ident
        $Type:ident {
            get { [$get:ident $get_mut:ident] }
            set { [$set:ident $set_mut:ident] }
            unset { [$unset:ident $unset_mut:ident] }
            rgb { [$set_rgb:ident $set_rgb_mut:ident] }
            ansi { [$set_ansi:ident $set_ansi_mut:ident] }
            variants {
                $($variant:ident => [$set_variant:ident $set_variant_mut:ident])*
            }
        }
    ) => {
        with!("A trait for styles with a `" stringify!($Type) "` field.",
            $Trait {
                get { $get $get_mut -> $Type }
                set {
                    "Sets `" stringify!($Type) "(color)`",
                    [$set $set_mut]
                    (color: Color)
                    { color.into() }
                }
                specials {
                    "Sets `" stringify!($Type) "(Color::Rgb { r, g, b })`",
                    [$set_rgb $set_rgb_mut]
                    (r: u8 g: u8 b: u8)
                    { Color::Rgb { r, g, b }.into() }
                    "Sets `" stringify!($Type) "(Color::AnsiValue(value))`",
                    [$set_ansi $set_ansi_mut]
                    (value: u8)
                    { Color::AnsiValue(value).into() }
                }
                variants { $(
                    "Sets `" stringify!($Type) "(Color::" stringify!($variant) ")`",
                    [$set_variant $set_variant_mut]
                    { Color::$variant.into() }
                )* }
            }
        );
        with!("A trait for styles with an `Option<" stringify!($Type) ">` field.",
            $OptionalTrait {
                get { $get $get_mut -> Option<$Type> }
                set {
                    "Sets `Some(" stringify!($Type) "(color))`",
                    [$set $set_mut]
                    (color: Color)
                    { Some(color.into()) }
                }
                unset { [$unset $unset_mut] }
                specials {
                    "Sets `Some(" stringify!($Type) "(Color::Rgb { r, g, b }))`",
                    [$set_rgb $set_rgb_mut]
                    (r: u8 g: u8 b: u8)
                    { Some(Color::Rgb { r, g, b }.into()) }
                    "Sets `Some(" stringify!($Type) "(Color::AnsiValue(value)))`",
                    [$set_ansi $set_ansi_mut]
                    (value: u8)
                    { Some(Color::AnsiValue(value).into()) }
                }
                variants { $(
                    "Sets `Some(" stringify!($Type) "(Color::" stringify!($variant) "))`",
                    [$set_variant $set_variant_mut]
                    { Some(Color::$variant.into()) }
                )* }
            }
        );
    };
}

macro_rules! with_attribute {
    (
        $Trait:ident
        $OptionalTrait:ident
        $Type:ident $type:ident {
            get { [$get:ident $get_mut:ident] }
            set { [$set:ident $set_mut:ident] }
            unset { [$unset:ident $unset_mut:ident] }
            variants {
                $($variant:ident => [$set_variant:ident $set_variant_mut:ident])*
            }
        }
    ) => {
        with!("A trait for styles with a `" stringify!($Type) "` field.",
            $Trait {
                get { $get $get_mut -> $Type }
                set {
                    "Sets `" stringify!($type) "`",
                    [$set $set_mut]
                    ($type: $Type)
                    { $type }
                }
                variants { $(
                    "Sets `" stringify!($Type) "::" stringify!($variant) "`",
                    [$set_variant $set_variant_mut]
                    { <$Type>::$variant }
                )* }
            }
        );
        with!("A trait for styles with an `Option<" stringify!($Type) ">` field.",
            $OptionalTrait {
                get { $get $get_mut -> Option<$Type> }
                set {
                    "Sets `Some(" stringify!($type) ")`",
                    [$set $set_mut]
                    ($type: $Type)
                    { Some($type) }
                }
                unset { [$unset $unset_mut] }
                variants { $(
                    "Sets `Some(" stringify!($Type) "::" stringify!($variant) ")`",
                    [$set_variant $set_variant_mut]
                    { Some(<$Type>::$variant) }
                )* }
            }
        );
    };
}

with_color!(WithForeground WithOptionalForeground Foreground {
    get { [get_foreground get_foreground_mut] }
    set { [foreground foreground_mut] }
    unset { [no_foreground no_foreground_mut] }
    rgb { [rgb rgb_mut] }
    ansi { [ansi ansi_mut] }
    variants {
        Reset => [reset reset_mut]
        White => [white white_mut]
        Black => [black black_mut]
        Red => [red red_mut]
        DarkRed => [dark_red dark_red_mut]
        Green => [green green_mut]
        DarkGreen => [dark_green dark_green_mut]
        Yellow => [yellow yellow_mut]
        DarkYellow => [dark_yellow dark_yellow_mut]
        Blue => [blue blue_mut]
        DarkBlue => [dark_blue dark_blue_mut]
        Magenta => [magenta magenta_mut]
        DarkMagenta => [dark_magenta dark_magenta_mut]
        Cyan => [cyan cyan_mut]
        DarkCyan => [dark_cyan dark_cyan_mut]
    }
});
with_color!(WithBackground WithOptionalBackground Background {
    get { [get_background get_background_mut] }
    set { [background background_mut] }
    unset { [no_background no_background_mut] }
    rgb { [on_rgb on_rgb_mut] }
    ansi { [on_ansi on_ansi_mut] }
    variants {
        Reset => [on_reset on_reset_mut]
        White => [on_white on_white_mut]
        Black => [on_black on_black_mut]
        Red => [on_red on_red_mut]
        DarkRed => [on_dark_red on_dark_red_mut]
        Green => [on_green on_green_mut]
        DarkGreen => [on_dark_green on_dark_green_mut]
        Yellow => [on_yellow on_yellow_mut]
        DarkYellow => [on_dark_yellow on_dark_yellow_mut]
        Blue => [on_blue on_blue_mut]
        DarkBlue => [on_dark_blue on_dark_blue_mut]
        Magenta => [on_magenta on_magenta_mut]
        DarkMagenta => [on_dark_magenta on_dark_magenta_mut]
        Cyan => [on_cyan on_cyan_mut]
        DarkCyan => [on_dark_cyan on_dark_cyan_mut]
    }
});
with_attribute!(WithWeight WithOptionalWeight Weighted weight {
    get { [get_weight get_weight_mut] }
    set { [weighted weighted_mut] }
    unset { [no_weight no_weight_mut] }
    variants {
        ResetWeight => [reset_weight reset_weight_mut]
        Bold => [bold bold_mut]
        Light => [light light_mut]
    }
});
with_attribute!(WithSlant WithOptional Slanted slant {
    get { [get_slant get_slant_mut] }
    set { [slanted slanted_mut] }
    unset { [no_slant no_slant_mut] }
    variants {
        ResetSlant => [reset_slant reset_slant_mut]
        Italic => [italic italic_mut]
    }
});
with_attribute!(WithBlink WithOptionalBlink Blinking blink {
    get { [get_blink get_blink_mut] }
    set { [blinking blinking_mut] }
    unset { [no_blink no_blink_mut] }
    variants {
        ResetBlink => [reset_blink reset_blink_mut]
        Slow => [slow slow_mut]
        Fast => [fast fast_mut]
    }
});
with_attribute!(WithInvert WithOptionalInvert Inverted invert {
    get { [get_invert get_invert_mut] }
    set { [inverted inverted_mut] }
    unset { [no_invert no_invert_mut] }
    variants {
        ResetInvert => [reset_invert reset_invert_mut]
        Invert => [invert invert_mut]
    }
});
with_attribute!(WithStrike WithOptionalStrike Striked strike {
    get { [get_strike get_strike_mut] }
    set { [striked striked_mut] }
    unset { [no_strike no_strike_mut] }
    variants {
        ResetStrike => [reset_strike reset_strike_mut]
        Strike => [strike strike_mut]
    }
});
with_attribute!(WithUnderline WithOptionalUnderline Underlined underline {
    get { [get_underline get_underline_mut] }
    set { [underlined underlined_mut] }
    unset { [no_underline no_underline_mut] }
    variants {
        ResetUnderline => [reset_underline reset_underline_mut]
        Underline => [underline underline_mut]
    }
});
with_attribute!(WithOverline WithOptionalOverline Overlined overline {
    get { [get_overline get_overline_mut] }
    set { [overlined overlined_mut] }
    unset { [no_overline no_overline_mut] }
    variants {
        ResetOverline => [reset_overline reset_overline_mut]
        Overline => [overline overline_mut]
    }
});
with_attribute!(WithBorder WithOptionalBorder Bordered border {
    get { [get_border get_border_mut] }
    set { [bordered bordered_mut] }
    unset { [no_border no_border_mut] }
    variants {
        ResetBorder => [reset_border reset_border_mut]
        Frame => [frame frame_mut]
        Circle => [circle circle_mut]
    }
});
