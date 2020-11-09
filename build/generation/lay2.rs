use crate::generation::*;
use quote::{ToTokens, TokenStreamExt};
use std::{
    fmt::{Debug, Display, Error, Formatter},
    ops::Deref,
};

macro_rules! derefs {
    ($self:ident $(
        $Struct:ident {
            deref $Target:ty { $deref:expr }
            $tokens:ident { $to_tokens:expr }
            $f:ident { $fmt:expr }
        }
    )*) => {$(
        impl Deref for $Struct {
            type Target = $Target;
            fn deref(&$self) -> &Self::Target { $deref }
        }

        impl ToTokens for $Struct {
            fn to_tokens(&$self, $tokens: &mut TokenStream) { $to_tokens }
        }

        impl Display for $Struct {
            fn fmt(&$self, $f: &mut Formatter) -> Result<(), Error> { $fmt }
        }
    )*};
}

macro_rules! struct_attr {
    ($($field:ident: $Type:ty,)*) => {
        #[derive(Copy, Clone, Eq, PartialEq, Hash)]
        pub struct Attr { $(pub $field: $Type,)* }

        impl Attribute {
            $(pub fn $field(&self) -> $Type { self.attr().$field })*
        }
    };
}

// -----------------------------------------------
// Str
// -----------------------------------------------

pub type Strs = &'static [Str];

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Str(pub &'static str);

derefs!(self Str {
    deref str { &self.0 }
    tokens { tokens.append(quote::format_ident!("{}", self.0)) }
    f { <str as Display>::fmt(self, f) }
});

impl Debug for Str {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.0)
    }
}

// -----------------------------------------------
// Ident
// -----------------------------------------------

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Ident {
    pub pascal: Str,
    pub snake:  Str,
}

derefs!(self Ident {
    deref Str { &self.pascal }
    tokens { self.pascal.to_tokens(tokens) }
    f { <Str as Display>::fmt(&self, f) }
});

impl Debug for Ident {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}: {:?}", self.snake, self.pascal)
    }
}

// -----------------------------------------------
// Color
// -----------------------------------------------

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Color {
    pub name:  Ident,
    pub reset: Str, // ResetColor
}

derefs!(self Color {
    deref Ident { &self.name }
    tokens { self.name.to_tokens(tokens) }
    f { <Ident as Display>::fmt(&self, f) }
});

impl Debug for Color {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?} ({:?})", self.name, self.reset)
    }
}

// -----------------------------------------------
// Variant
// -----------------------------------------------

pub type Variants = &'static [Variant];

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Variant {
    pub name:    Ident,
    pub set:     Str,
    pub set_mut: Str,
    pub fields:  Strs,
}

derefs!(self Variant {
    deref Ident { &self.name }
    tokens { self.name.to_tokens(tokens) }
    f { <Ident as Display>::fmt(&self, f) }
});

impl Debug for Variant {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "{:?} ({:?} {:?}) {:?}",
            self.name, self.set, self.set_mut, self.fields
        )
    }
}

// -----------------------------------------------
// Attr
// -----------------------------------------------

pub type Attrs = &'static [Attr];

struct_attr!(
    name: Ident,
    reset: Str, // Foreground(Color::ResetColor) or ResetWeight
    none: Str,  // NoForeground                  or NoWeight
    short: Str,
    get: Str,
    set: Str,
    get_mut: Str,
    set_mut: Str,
    variants: Variants,
);

derefs!(self Attr {
    deref Ident { &self.name }
    tokens { self.name.to_tokens(tokens) }
    f { <Ident as Display>::fmt(&self, f) }
});

impl Debug for Attr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_tuple("Attr")
            .field(&format!(
                "{:?} ({:?}) {:?} {:?}",
                self.name, self.short, self.reset, self.none
            ))
            .field(&format!(
                "{:?} {:?} / {:?} {:?}",
                self.get, self.get_mut, self.set, self.set_mut
            ))
            .field(&self.variants)
            .finish()
    }
}

// -----------------------------------------------
// Attribute
// -----------------------------------------------

pub type Attributes = &'static [Attribute];

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Attribute {
    Ground(Attr),
    Attribute(Attr),
}

impl Attribute {
    pub fn attr(&self) -> &Attr {
        match self {
            Self::Ground(ground) => ground,
            Self::Attribute(attribute) => attribute,
        }
    }
}

derefs!(self Attribute {
    deref Attr {
        match self {
            Self::Ground(ground) => ground,
            Self::Attribute(attribute) => attribute,
        }
    }
    tokens { self.name().to_tokens(tokens) }
    f { <Attr as Display>::fmt(self, f) }
});

// -----------------------------------------------
// Lay
// -----------------------------------------------

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Lay {
    pub index:      Str,
    pub reset:      Str,
    pub none:       Str,
    pub color:      Color,
    pub colors:     Variants,
    pub foreground: Attr,
    pub background: Attr,
    pub grounds:    Attrs,
    pub attrs:      Attrs,
    pub attributes: Attributes,
}

// -----------------------------------------------
// Macros
// -----------------------------------------------

macro_rules! Str {
    ($_0:ident $($_:ident)*) => {
        Str(concat!(stringify!($_0), $(stringify!($_),)*))
    };
    ($_0:ident $($_:ident)* $join:literal) => {
        Str(concat!(stringify!($_0), $($join, stringify!($_),)*))
    };
}

macro_rules! Ident {
    ($cap0:ident $($cap:ident)*, $low0:ident $($low:ident)*) => {
        Ident {
            pascal: Str!($cap0 $($cap)*),
            snake:  Str!($low0 $($low)* "_"),
        }
    };
}

macro_rules! Styler {
    ($cap0:ident $($cap:ident)*, $low0:ident $($low:ident)* $(, $prefix:ident)?) => {
        Styler {
            get:     Str!(get         $low0 $($low)*     "_"),
            set:     Str!($($prefix)? $low0 $($low)*     "_"),
            get_mut: Str!(get         $low0 $($low)* mut "_"),
            set_mut: Str!($($prefix)? $low0 $($low)* mut "_"),
        }
    };
}

macro_rules! Attr {
    (
        $Attr:ident $attr:ident ($Short:ident)
        $None:ident
        $(const $ResetConst:ident)?
        $(ident $ResetIdent:ident)?
        [$(
            $(+ $prefix:ident)?
            $cap0:ident $($cap:ident)*,
            $low0:ident $($low:ident)*
            $(( $($field:ident)* ))?,
        )*]
    ) => {
        Attr {
            name:     Ident!($Attr, $attr),
            reset:    $($ResetConst)? $(Str!($ResetIdent $Attr))?,
            none:     Str!($None $Attr),
            short:    Str!($Short),
            get:      Str!(get $attr     "_"),
            set:      Str!(    $attr     "_"),
            get_mut:  Str!(get $attr mut "_"),
            set_mut:  Str!(    $attr mut "_"),
            variants: &[$(
                Variant {
                    name:    Ident!($cap0 $($cap)*, $low0 $($low)*),
                    set:     Str!($($prefix)? $low0 $($low)*     "_"),
                    set_mut: Str!($($prefix)? $low0 $($low)* mut "_"),
                    fields:  &[$( $(Str!($field),)* )?]
                },
            )*],
        }
    };
}

macro_rules! Lay {
    (
        $Reset:ident $reset:ident
        $None:ident  $none:ident
        $Color:ident $color:ident
        [$($C:ident)* ($Dark:ident) $($Light:ident)*] $Rgb:ident $Ansi:ident
        [$($c:ident)* ($dark:ident) $($light:ident)*] $rgb:ident $ansi:ident
        $Foreground:ident $foreground:ident ($Fg:ident)
        $Background:ident $background:ident ($Bg:ident)
        [$(
            $i:literal $Attr:ident($A:ident) [$($Variant:ident)*]
                       $attr:ident           [$($variant:ident)*]
        )*]
    ) => {
        pub const INDEX: Str = Str!(i);
        pub const RESET: Str = Str!($Reset);
        pub const NONE: Str = Str!($none);

        pub const COLOR: Color = Color {
            name:  Ident!($Color, $color),
            reset: Str!($Reset $Color),
        };

        pub const COLORS: Variants = FOREGROUND.variants;

        pub const FOREGROUND: Attr = Attr!(
            $Foreground $foreground ($Fg)
            $None
            const _RESET_FOREGROUND
            [
                $($C, $c,)*
                $($Light, $light, $Dark $Light, $dark $light,)*
                $Rgb, $rgb (r g b),
                $Ansi, $ansi (ansi),
                $Reset $Color, $reset $color,
            ]
        );

        pub const BACKGROUND: Attr = Attr!(
            $Background $background ($Bg)
            $None
            const _RESET_BACKGROUND
            [
                $(+ on $C, $c,)*
                $(+ on $Light, $light, $Dark $Light, $dark $light,)*
                + on $Rgb, $rgb (r g b),
                + on $Ansi, $ansi (ansi),
                + on $Reset $Color, $reset $color,
            ]
        );

        pub const GROUNDS: Attrs = &[FOREGROUND, BACKGROUND];

        pub const ATTRS: Attrs = &[$(
            Attr!(
                $Attr $attr ($A)
                $None
                ident $Reset
                [
                    $($Variant, $variant,)*
                    $Reset $Attr, $reset $attr,
                ]
            ),
        )*];

        pub const ATTRIBUTES: Attributes = &[
            Attribute::Ground(FOREGROUND),
            Attribute::Ground(BACKGROUND),
            $(Attribute::Attribute(ATTRS[$i]),)*
        ];

        pub const LAY: Lay = Lay {
            index:      INDEX,
            reset:      RESET,
            none:       NONE,
            color:      COLOR,
            colors:     COLORS,
            foreground: FOREGROUND,
            background: BACKGROUND,
            grounds:    GROUNDS,
            attrs:      ATTRS,
            attributes: ATTRIBUTES,
        };

        // -----

        pub const _RESET_FOREGROUND: Str= Str(concat!(
            stringify!($Foreground), "(",
                stringify!($Color), "::", stringify!($Reset), stringify!($Color),
            ")",
        ));

        pub const _RESET_BACKGROUND: Str= Str(concat!(
            stringify!($Background), "(",
                stringify!($Color), "::", stringify!($Reset), stringify!($Color),
            ")",
        ));
    };
}
