#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Colors {
    pub name:   &'static str,
    pub colors: &'static [&'static str],
    pub lights: &'static [&'static str],
    pub darks:  &'static [&'static str],
    pub rgb:    &'static str,
    pub ansi:   &'static str,
    pub reset:  &'static str,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Ground {
    pub name:  &'static str,
    pub short: &'static str,
    pub reset: &'static str,
    pub no:    &'static str,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Attribute {
    pub name:     &'static str,
    pub short:    &'static str,
    pub variants: &'static [&'static str],
    pub reset:    &'static str,
    pub no:       &'static str,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Lay {
    pub reset:      &'static str,
    pub colors:     Colors,
    pub foreground: Ground,
    pub background: Ground,
    pub attributes: &'static [Attribute],
}

macro_rules! lay {
    (
        ($reset:literal $no:literal)
        $Color:literal: [$($color:literal)+] ($rgb:literal $ansi:literal)
        $dark:literal: [$($light:literal)+]
        [$foreground_short:literal/$foreground:literal $background_short:literal/$background:literal]
        $($short:literal/$name:literal: [$($variant:literal)+])+
    ) => {
        $crate::generation::lay::Lay {
            reset:  $reset,
            colors: $crate::generation::lay::Colors {
                name:   $Color,
                colors: &[$($color,)+],
                lights: &[$($light,)+],
                darks:  &[$(concat!($dark, $light),)+],
                rgb:    $rgb,
                ansi:   $ansi,
                reset:  concat!($reset, $Color),
            },
            foreground: $crate::generation::lay::Ground {
                name:  $foreground,
                short: $foreground_short,
                reset: concat!($reset, $foreground),
                no:    concat!($no, $foreground),
            },
            background: $crate::generation::lay::Ground {
                name:  $background,
                short: $background_short,
                reset: concat!($reset, $background),
                no:    concat!($no, $background),
            },
            attributes: &[$(
                $crate::generation::lay::Attribute {
                    name:     $name,
                    short:    $short,
                    variants: &[$($variant,)+],
                    reset:    concat!($reset, $name),
                    no:       concat!($no, $name),
                },
            )+],
        }
    };
}
