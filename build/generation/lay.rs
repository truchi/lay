use crate::generation::*;
use proc_macro2::{Delimiter, Group, TokenTree};
use quote::{ToTokens, TokenStreamExt};
use std::{
    fmt::{Debug, Display, Error, Formatter},
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

#[derive(Clone)]
pub struct Str {
    string: String,
    tree:   TokenTree,
}

derefs!(self Str {
    deref str { &self.string }
    tokens { tokens.append(self.tree.clone()) }
    f { <str as Display>::fmt(self, f) }
});

impl Str {
    pub fn new(string: &str) -> Self {
        let string = string.to_string();
        let tokens = TokenStream::from_str(&string).expect("Cannot convert to TokenStream");
        let tree = TokenTree::from(Group::new(Delimiter::None, tokens));
        Self { string, tree }
    }
}

impl Debug for Str {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.string)
    }
}

// -----------------------------------------------
// Ident
// -----------------------------------------------

#[derive(Clone)]
pub struct Ident {
    pub pascal: Str,
    pub snake:  Str,
}

derefs!(self Ident {
    deref Str { &self.pascal }
    tokens { self.pascal.to_tokens(tokens) }
    f { <Str as Display>::fmt(&self, f) }
});

impl Ident {
    pub fn new(parts: &[&str]) -> Self {
        Self {
            pascal: Self::pascal(parts),
            snake:  Self::snake(parts),
        }
    }

    pub fn pascal(parts: &[&str]) -> Str {
        Str::new(&parts.join(""))
    }

    pub fn snake(parts: &[&str]) -> Str {
        Str::new(
            &parts
                .iter()
                .map(|part| part.to_lowercase())
                .collect::<Vec<_>>()
                .join("_"),
        )
    }

    pub fn none(&self) -> Str {
        Str::new(&format!("{}_{}", Lay::NONE.to_lowercase(), self.snake))
    }

    pub fn get(&self) -> Str {
        Str::new(&format!("{}{}", Self::GET, self.snake))
    }

    pub fn set(&self, on: bool) -> Str {
        Str::new(&format!(
            "{}{}{}",
            if on { Self::ON } else { "" },
            Self::SET,
            self.snake,
        ))
    }

    pub fn get_mut(&self) -> Str {
        Str::new(&format!("{}{}{}", Self::GET, self.snake, Self::MUT))
    }

    pub fn set_mut(&self, on: bool) -> Str {
        Str::new(&format!(
            "{}{}{}{}",
            if on { Self::ON } else { "" },
            Self::SET,
            self.snake,
            Self::MUT,
        ))
    }

    pub fn none_mut(&self) -> Str {
        Str::new(&format!(
            "{}_{}{}",
            Lay::NONE.to_lowercase(),
            self.snake,
            Self::MUT,
        ))
    }
}

impl Debug for Ident {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({}, {})", self.pascal, self.snake)
    }
}

// -----------------------------------------------
// Variant
// -----------------------------------------------

#[derive(Clone, Debug)]
pub struct Variant {
    pub name:       Ident,
    pub args:       Str, // r: u8, g: u8, b: u8
    pub with_types: Str, // Ansi(u8)
    pub full:       Str, // Color::Ansi(ansi)
    pub wrapped:    Str, /* Color: Color::Ansi(ansi), Grounds: Foreground(Color::Ansi(ansi)),
                          * Attrs: Weight::Bold */
    pub set:        Str,
    pub set_mut:    Str,
}

derefs!(self Variant {
    deref Ident { &self.name }
    tokens { self.name.to_tokens(tokens) }
    f { <Ident as Display>::fmt(&self, f) }
});

impl Variant {
    pub fn new(
        name: Ident,
        (parent, wrapper, on): (&str, Option<&str>, bool),
        fields: &[&str],
    ) -> Self {
        let concat = |wrap, f: fn(&&str) -> String| {
            let r = fields.iter().map(f).collect::<Vec<_>>().join(", ");
            if wrap && r.len() > 0 {
                format!("({})", r)
            } else {
                r
            }
        };
        let names = concat(true, |field| field.to_string());
        let types = concat(true, |_| String::from(Self::TYPE));
        let args = Str::new(&concat(false, |field| format!("{}: {}", field, Self::TYPE)));

        let with_types = Str::new(&format!("{}{}", name, types));
        let full = Str::new(&format!("{}::{}{}", parent, name, names));
        let wrapped = wrapper.map_or_else(
            || full.clone(),
            |wrapper| Str::new(&format!("{}({})", wrapper, full)),
        );
        let set = name.set(on);
        let set_mut = name.set_mut(on);

        Self {
            name,
            args,
            with_types,
            full,
            wrapped,
            set,
            set_mut,
        }
    }
}

// -----------------------------------------------
// Color
// -----------------------------------------------

#[derive(Clone, Debug)]
pub struct Color {
    pub name:     Ident,
    pub reset:    Variant,
    pub variants: Vec<Variant>,
}

derefs!(self Color {
    deref Ident { &self.name }
    tokens { self.name.to_tokens(tokens) }
    f { <Ident as Display>::fmt(&self, f) }
});

impl Color {
    pub fn new(colors: Vec<(Ident, Vec<&str>)>) -> Self {
        let name = Ident::new(&[Lay::COLOR]);
        let variants = colors
            .clone()
            .into_iter()
            .map(|(name, fields)| Variant::new(name, (Lay::COLOR, None, false), &fields))
            .collect::<Vec<_>>();
        let reset = variants.last().unwrap().clone();

        Self {
            name,
            variants,
            reset,
        }
    }
}

// -----------------------------------------------
// Attr
// -----------------------------------------------

#[derive(Clone, Debug)]
pub enum AttrType {
    Foreground,
    Background,
    Attribute,
}

#[derive(Clone, Debug)]
pub struct Attr {
    pub ty:       AttrType,
    pub name:     Ident,
    pub short:    Str,
    pub get:      Str,
    pub set:      Str,
    pub none:     Str,
    pub get_mut:  Str,
    pub set_mut:  Str,
    pub none_mut: Str,
    pub variants: Vec<Variant>,
    pub reset:    Variant,
}

derefs!(self Attr {
    deref Ident { &self.name }
    tokens { self.name.to_tokens(tokens) }
    f { <Ident as Display>::fmt(&self, f) }
});

impl Attr {
    pub fn new(ty: AttrType, short: Str, name: Ident, variants: Vec<(Ident, Vec<&str>)>) -> Self {
        let get = name.get();
        let set = name.set(false);
        let none = name.none();
        let get_mut = name.get_mut();
        let set_mut = name.set_mut(false);
        let none_mut = name.none_mut();

        let variants = variants
            .into_iter()
            .map(|(variant, fields)| {
                Variant::new(
                    variant,
                    match ty {
                        AttrType::Foreground => (Lay::COLOR, Some(&name), false),
                        AttrType::Background => (Lay::COLOR, Some(&name), true),
                        AttrType::Attribute => (&name, None, false),
                    },
                    &fields,
                )
            })
            .collect::<Vec<_>>();
        let reset = variants.last().unwrap().clone();

        Self {
            ty,
            short,
            name,
            get,
            set,
            none,
            get_mut,
            set_mut,
            none_mut,
            variants,
            reset,
        }
    }

    pub fn grounds(colors: Vec<(Ident, Vec<&str>)>) -> (Self, Self) {
        let fg = Str::new(Lay::FOREGROUND.0);
        let foreground = Ident::new(&[Lay::FOREGROUND.1]);
        let foreground = Attr::new(AttrType::Foreground, fg, foreground, colors.clone());

        let bg = Str::new(Lay::BACKGROUND.0);
        let background = Ident::new(&[Lay::BACKGROUND.1]);
        let background = Attr::new(AttrType::Background, bg, background, colors);

        (foreground, background)
    }

    pub fn attributes(attributes: Vec<(Str, Ident, Vec<(Ident, Vec<&str>)>)>) -> Vec<Self> {
        attributes
            .into_iter()
            .map(|(short, name, fields)| Attr::new(AttrType::Attribute, short, name, fields))
            .collect()
    }
}

// -----------------------------------------------
// Lay
// -----------------------------------------------

#[derive(Clone, Debug)]
pub struct Lay {
    pub index:      Ident,
    pub reset:      Ident,
    pub none:       Ident,
    pub color:      Color,
    pub foreground: Attr,
    pub background: Attr,
    pub attributes: Vec<Attr>,
    pub grounds:    Vec<Attr>,
    pub all:        Vec<Attr>,
}

impl Lay {
    pub fn new() -> Self {
        let colors: Vec<(Ident, Vec<&str>)> = Self::COLORS
            .iter()
            .map(|(prefix, colors)| {
                colors
                    .iter()
                    .map(|(color, fields)| {
                        let fields = fields.to_vec();

                        if let Some(prefix) = prefix {
                            (Ident::new(&[prefix, color]), fields)
                        } else {
                            (Ident::new(&[color]), fields)
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .chain(vec![(Ident::new(&[Self::RESET, Self::COLOR]), vec![])])
            .collect::<Vec<_>>();

        let attributes: Vec<(Str, Ident, Vec<(Ident, Vec<&str>)>)> = Self::ATTRIBUTES
            .iter()
            .map(|(short, name, variants)| {
                let short = Str::new(short);
                let name = Ident::new(&[name]);

                let variants = variants
                    .iter()
                    .map(|variant| (Ident::new(&[variant]), vec![]))
                    .chain(vec![(Ident::new(&[Self::RESET, &name]), vec![])])
                    .collect();

                (short, name, variants)
            })
            .collect::<Vec<_>>();

        let (foreground, background) = Attr::grounds(colors.clone());
        let grounds = vec![foreground.clone(), background.clone()];
        let attributes = Attr::attributes(attributes);
        let all = [grounds.clone(), attributes.clone()].concat();

        Self {
            index: Ident::new(&[Self::INDEX]),
            reset: Ident::new(&[Self::RESET]),
            none: Ident::new(&[Self::NONE]),
            color: Color::new(colors),
            foreground,
            background,
            attributes,
            grounds,
            all,
        }
    }
}
