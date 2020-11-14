use crate::generation::*;
use proc_macro2::{Delimiter, Group, TokenTree};
use quote::{ToTokens, TokenStreamExt};
use std::{
    fmt::{Display, Error, Formatter},
    ops::Deref,
    str::FromStr,
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

// -----------------------------------------------
// Str
// -----------------------------------------------

pub type Strs = &'static [Str];

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Str(pub &'static str);

derefs!(self Str {
    deref str { &self.0 }
    tokens {
        tokens.append(
            TokenTree::from(
                Group::new(
                    Delimiter::None,
                    TokenStream::from_str(self.0).expect("Cannot convert to TokenStream")
                )
            )
        )
    }
    f { <str as Display>::fmt(self, f) }
});

// -----------------------------------------------
// Ident
// -----------------------------------------------

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Ident {
    pub pascal: Str,
    pub snake:  Str,
}

derefs!(self Ident {
    deref Str { &self.pascal }
    tokens { self.pascal.to_tokens(tokens) }
    f { <Str as Display>::fmt(&self, f) }
});

// -----------------------------------------------
// Color
// -----------------------------------------------

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Color {
    pub name:  Ident,
    pub reset: Str, // Color::ResetColor
}

derefs!(self Color {
    deref Ident { &self.name }
    tokens { self.name.to_tokens(tokens) }
    f { <Ident as Display>::fmt(&self, f) }
});

// -----------------------------------------------
// Variant
// -----------------------------------------------

pub type Variants = &'static [Variant];

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Variant {
    pub name:    Ident,
    // TODO Yes I have to do this
    // Color::Ansi(ansi)
    // Foreground(Color::Ansi(ansi))
    pub set:     Str,
    pub set_mut: Str,
    pub fields:  Strs,
}

impl Variant {
    pub fn decl(&self) -> TokenStream {
        let types = self.types();
        quote! { #self#types }
    }

    pub fn types(&self) -> TokenStream {
        let fields = self.fields;

        if fields.len() > 0 {
            let types = fields.iter().map(|field| quote! { u8 });

            quote! { (#(#types),*) }
        } else {
            quote! {}
        }
    }

    pub fn val(&self) -> TokenStream {
        let fields = self.fields();
        quote! { #self#fields }
    }

    pub fn fields(&self) -> TokenStream {
        let fields = self.fields;

        if fields.len() > 0 {
            quote! { (#(#fields),*) }
        } else {
            quote! {}
        }
    }

    pub fn params(&self) -> TokenStream {
        let fields = self.fields.iter().map(|field| quote! { #field: u8 });

        quote! { #(#fields),* }
    }
}

derefs!(self Variant {
    deref Ident { &self.name }
    tokens { self.name.to_tokens(tokens) }
    f { <Ident as Display>::fmt(&self, f) }
});

// -----------------------------------------------
// Attr
// -----------------------------------------------

pub type Attrs = &'static [Attr];

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Type {
    Ground,
    Attribute,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Attr {
    pub r#type:   Type,
    pub name:     Ident,
    pub reset:    Str, // Foreground(Color::ResetColor) or Weight::ResetWeight
    pub none:     Ident,
    pub short:    Str,
    pub get:      Str,
    pub set:      Str,
    pub get_mut:  Str,
    pub set_mut:  Str,
    pub variants: Variants,
}

impl Attr {
    pub fn full_variant(&self, variant: &Variant) -> TokenStream {
        let val = variant.val();
        match self.r#type {
            Type::Ground => quote! { #COLOR::#val },
            Type::Attribute => quote! { #self::#val },
        }
    }

    pub fn wrap_variant(&self, variant: &Variant) -> TokenStream {
        let full = self.full_variant(variant);
        match self.r#type {
            Type::Ground => quote! { #self(#full) },
            Type::Attribute => quote! { #full },
        }
    }
}

derefs!(self Attr {
    deref Ident { &self.name }
    tokens { self.name.to_tokens(tokens) }
    f { <Ident as Display>::fmt(&self, f) }
});

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
    tokens { self.name.to_tokens(tokens) }
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
        $Type:ident $Attr:ident $attr:ident ($Short:ident)
        $None:ident $none:ident
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
            r#type:   Type::$Type,
            name:     Ident!($Attr, $attr),
            $(reset:  $ResetConst,)?
            $(reset:  Str(concat!(stringify!($Attr), "::", stringify!($ResetIdent), stringify!($Attr))),)?
            none:     Ident!($None $Attr, $none $attr),
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
            reset: RESET_COLOR,
        };

        pub const COLORS: Variants = FOREGROUND.variants;

        pub const FOREGROUND: Attr = Attr!(
            Ground $Foreground $foreground ($Fg)
            $None $none
            const RESET_FOREGROUND
            [
                $($C, $c,)*
                $($Light, $light, $Dark $Light, $dark $light,)*
                $Rgb, $rgb (r g b),
                $Ansi, $ansi (ansi),
                $Reset $Color, $reset $color,
            ]
        );

        pub const BACKGROUND: Attr = Attr!(
            Ground $Background $background ($Bg)
            $None $none
            const RESET_BACKGROUND
            [
                $(+ on $C, $c,)*
                $(+ on $Light, $light, + on $Dark $Light, $dark $light,)*
                + on $Rgb, $rgb (r g b),
                + on $Ansi, $ansi (ansi),
                + on $Reset $Color, $reset $color,
            ]
        );

        pub const GROUNDS: Attrs = &[FOREGROUND, BACKGROUND];

        pub const ATTRS: Attrs = &[$(
            Attr!(
                Attribute $Attr $attr ($A)
                $None $none
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

        pub const RESET_COLOR: Str = Str(concat!(
            stringify!($Color), "::", stringify!($Reset), stringify!($Color),
        ));

        pub const RESET_FOREGROUND: Str = Str(concat!(
            stringify!($Foreground), "(",
                stringify!($Color), "::", stringify!($Reset), stringify!($Color),
            ")",
        ));

        pub const RESET_BACKGROUND: Str = Str(concat!(
            stringify!($Background), "(",
                stringify!($Color), "::", stringify!($Reset), stringify!($Color),
            ")",
        ));
    };
}
