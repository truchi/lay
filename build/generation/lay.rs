use super::*;
use quote::ToTokens;
use std::{
    fmt::{Display, Error, Formatter},
    ops::Deref,
};

macro_rules! derefs {
    ($($Struct:ident)*) => {
        $(
            impl Deref for $Struct {
                type Target = Ident;
                fn deref(&self) -> &Ident { &self.name }
            }

            impl ToTokens for $Struct {
                fn to_tokens(&self, tokens: &mut TokenStream) {
                    self.name.to_tokens(tokens)
                }
            }

            impl Display for $Struct {
                fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                    self.name.fmt(f)
                }
            }
        )*
    };
}

derefs!(Color Ground Attr);

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Color {
    pub name:   Ident,
    pub colors: Vec<Ident>,
    pub rgb:    Ident,
    pub ansi:   Ident,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Ground {
    pub name:  Ident,
    pub short: Ident,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Attr {
    pub name:     Ident,
    pub short:    Ident,
    pub variants: Vec<Ident>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Attribute {
    Ground(Ground),
    Attr(Attr),
}

impl Attribute {
    pub fn name(&self) -> &Ident {
        match self {
            Self::Ground(ground) => &ground.name,
            Self::Attr(attr) => &attr.name,
        }
    }

    pub fn short(&self) -> &Ident {
        match self {
            Self::Ground(ground) => &ground.short,
            Self::Attr(attr) => &attr.short,
        }
    }

    pub fn variants(&self) -> Option<&Vec<Ident>> {
        match self {
            Self::Ground(_) => None,
            Self::Attr(attr) => Some(&attr.variants),
        }
    }

    pub fn reset_expr(&self, lay: &Lay) -> TokenStream {
        match self {
            Self::Ground(ground) => {
                let color = &lay.color;
                let reset_color = color.reset();
                quote! { #ground(#color::#reset_color) }
            }
            Self::Attr(attr) => {
                let reset_attr = attr.reset();
                quote! { #attr::#reset_attr }
            }
        }
    }

    pub fn reset_str(&self, lay: &Lay) -> String {
        match self {
            Self::Ground(ground) => {
                let color = &lay.color;
                let reset_color = color.reset();
                format!("{}({}::{})", ground, color, reset_color)
            }
            Self::Attr(attr) => {
                let reset_attr = attr.reset();
                format!("{}::{}", attr, reset_attr)
            }
        }
    }
}

impl Deref for Attribute {
    type Target = Ident;

    fn deref(&self) -> &Ident {
        self.name()
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.name().to_tokens(tokens)
    }
}

impl Display for Attribute {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.name().fmt(f)
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Lay {
    pub i:          Ident,
    pub no:         Ident,
    pub reset:      Ident,
    pub color:      Color,
    pub foreground: Ground,
    pub background: Ground,
    pub grounds:    Vec<Ground>,
    pub attrs:      Vec<Attr>,
    pub attributes: Vec<Attribute>,
}

impl Lay {
    pub fn new() -> Self {
        Self {
            i:          Ident::new(&[Lay::I]),
            no:         Ident::new(&[Lay::NO]),
            reset:      Ident::new(&[Lay::RESET]),
            color:      Color {
                name:   Ident::new(&[Lay::COLOR]),
                colors: Lay::COLORS.iter().map(|parts| Ident::new(parts)).collect(),
                rgb:    Ident::new(&[Lay::RGB]),
                ansi:   Ident::new(&[Lay::ANSI]),
            },
            foreground: Ground {
                name:  Ident::new(&[Lay::GROUNDS[0][0]]),
                short: Ident::new(&[Lay::GROUNDS[0][1]]),
            },
            background: Ground {
                name:  Ident::new(&[Lay::GROUNDS[1][0]]),
                short: Ident::new(&[Lay::GROUNDS[1][1]]),
            },
            grounds:    Lay::GROUNDS
                .iter()
                .map(|[name, short]| Ground {
                    name:  Ident::new(&[name]),
                    short: Ident::new(&[short]),
                })
                .collect(),
            attrs:      Lay::ATTRIBUTES
                .iter()
                .map(|attribute| Attr {
                    name:     Ident::new(&[attribute[0]]),
                    short:    Ident::new(&[attribute[1]]),
                    variants: attribute[2..]
                        .iter()
                        .map(|variant| Ident::new(&[variant]))
                        .collect(),
                })
                .collect(),
            attributes: Lay::GROUNDS
                .iter()
                .map(|[name, short]| {
                    Attribute::Ground(Ground {
                        name:  Ident::new(&[name]),
                        short: Ident::new(&[short]),
                    })
                })
                .chain(Lay::ATTRIBUTES.iter().map(|attribute| {
                    Attribute::Attr(Attr {
                        name:     Ident::new(&[attribute[0]]),
                        short:    Ident::new(&[attribute[1]]),
                        variants: attribute[2..]
                            .iter()
                            .map(|variant| Ident::new(&[variant]))
                            .collect(),
                    })
                }))
                .collect(),
        }
    }
}

macro_rules! consts {
    (
        $I:ident $No:ident $Reset:ident $Get:ident $On:ident $Mut:ident

        $Color:ident
        [$($color:ident)* ($dark:ident) $($light:ident)*]
        $Rgb:ident $Ansi:ident

        [$($Ground:ident($grnd:ident))*]
        [$($Attribute:ident($attr:ident) [$($variant:ident)*])*]
    ) => {
        impl Lay {
            const I: &'static str = stringify!($I);
            const NO: &'static str = stringify!($No);
            const RESET: &'static str = stringify!($Reset);
            const GET: &'static str = stringify!($Get);
            const ON: &'static str = stringify!($On);
            const MUT: &'static str = stringify!($Mut);

            const COLOR: &'static str = stringify!($Color);
            const COLORS: &'static [&'static [&'static str]] = &[
                $(&[stringify!($color)],)*
                $(
                    &[stringify!($light)],
                    &[stringify!($dark), stringify!($light)],
                )*
            ];
            const RGB: &'static str = stringify!($Rgb);
            const ANSI: &'static str = stringify!($Ansi);

            const GROUNDS: &'static [[&'static str; 2]] = &[$(
                [stringify!($Ground), stringify!($grnd)],
            )*];
            const ATTRIBUTES: &'static [&'static [&'static str]] = &[$(
                &[stringify!($Attribute), stringify!($attr), $(stringify!($variant),)*],
            )*];
        }
    };
}
