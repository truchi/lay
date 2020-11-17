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

impl PartialEq for Str {
    fn eq(&self, other: &Self) -> bool {
        self.string == other.string
    }
}

impl Default for Str {
    fn default() -> Self {
        Self {
            string: Default::default(),
            tree:   TokenTree::from(Group::new(Delimiter::None, Default::default())),
        }
    }
}

impl Debug for Str {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "\"{}\"", self.string)
    }
}

// -----------------------------------------------
// Ident
// -----------------------------------------------

#[derive(Clone, Default)]
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
}

impl Debug for Ident {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({:#?}, {:#?})", self.pascal, self.snake)
    }
}

// -----------------------------------------------
// StylerFn
// -----------------------------------------------

#[derive(Clone, Default)]
pub struct StylerFn {
    doc:  Doc,
    name: Str,
    sign: TokenStream,
    body: Option<TokenStream>,
}

impl StylerFn {
    pub fn new(doc: Doc, name: Str, sign: TokenStream, body: Option<TokenStream>) -> Self {
        Self {
            doc,
            name,
            sign,
            body,
        }
    }

    pub fn new_attr_get(attr: &Attr) -> (Self, Self) {
        let snake = &attr.snake;

        let (doc, doc_mut) = (
            doc!("Gets `Option<{}>`.", attr),
            doc!("Gets `&mut Option<{}>`.", attr),
        );
        let (name, name_mut) = (
            Str::new(&format!("{}{}", Self::GET, snake)),
            Str::new(&format!("{}{}{}", Self::GET, snake, Self::MUT)),
        );
        let (sign, sign_mut) = (
            quote! { fn #name(&self) -> Option<#attr> },
            quote! { fn #name_mut(&mut self) -> &mut Option<#attr> },
        );

        (
            Self::new(doc, name, sign, None),
            Self::new(doc_mut, name_mut, sign_mut, None),
        )
    }

    pub fn new_attr_set(attr: &Attr) -> (Self, Self) {
        let snake = &attr.snake;

        let (doc, doc_mut) = (
            doc!("Sets `Option<{}>`.", attr),
            doc!("Sets `Option<{}>`, mutably.", attr),
        );
        let (name, name_mut) = (
            Str::new(&format!("{}{}", Self::SET, snake)),
            Str::new(&format!("{}{}{}", Self::SET, snake, Self::MUT)),
        );
        let (sign, sign_mut) = (
            quote! { fn #name(self, #snake: impl Into<Option<#attr>>) -> Self::Output },
            quote! { fn #name_mut(&mut self, #snake: impl Into<Option<#attr>>) },
        );

        (
            Self::new(doc, name, sign, None),
            Self::new(doc_mut, name_mut, sign_mut, None),
        )
    }

    pub fn new_attr_none(attr: &Attr, set: &StylerFn, set_mut: &StylerFn) -> (Self, Self) {
        let none = Lay::NONE.to_lowercase();
        let snake = &attr.snake;

        let (doc, doc_mut) = (
            doc!("`None`s `Option<{}>`.", attr),
            doc!("`None`s `Option<{}>`, mutably.", attr),
        );
        let (name, name_mut) = (
            Str::new(&format!("{}_{}", none, snake)),
            Str::new(&format!("{}_{}{}", none, snake, Self::MUT)),
        );
        let (sign, sign_mut) = (
            quote! { fn #name(self) -> Self::Output },
            quote! { fn #name_mut(&mut self) },
        );
        let (body, body_mut) = (quote! { self.#set(None) }, quote! { self.#set_mut(None); });

        (
            Self::new(doc, name, sign, Some(body)),
            Self::new(doc_mut, name_mut, sign_mut, Some(body_mut)),
        )
    }

    pub fn new_variant_set(attr: &Attr, variant: &Variant) -> (Self, Self) {
        let (set, set_mut) = (&attr.fn_set, &attr.fn_set_mut);
        let snake = &variant.snake;
        let wrapped = &variant.wrapped;
        let args = &variant.args;
        let on = if attr.ty == AttrType::Background {
            Self::ON
        } else {
            ""
        };

        let (doc, doc_mut) = (
            doc!("Sets `Some({})`.", wrapped),
            doc!("Sets `Some({})`, mutably.", wrapped),
        );
        let (name, name_mut) = (
            Str::new(&format!("{}{}{}", on, Self::SET, snake)),
            Str::new(&format!("{}{}{}{}", on, Self::SET, snake, Self::MUT)),
        );
        let (sign, sign_mut) = (
            quote! { fn #name(self, #args) -> Self::Output },
            quote! { fn #name_mut(&mut self, #args) },
        );
        let (body, body_mut) = (
            quote! { self.#set(Some(#wrapped)) },
            quote! { self.#set_mut(Some(#wrapped)); },
        );

        (
            Self::new(doc, name, sign, Some(body)),
            Self::new(doc_mut, name_mut, sign_mut, Some(body_mut)),
        )
    }

    pub fn full(&self) -> TokenStream {
        let doc = &self.doc;
        let sign = &self.sign;
        let body = if let Some(body) = &self.body {
            quote! { { #body } }
        } else {
            quote! { ; }
        };

        quote! {
            #doc
            #sign
            #body
        }
    }
}

derefs!(self StylerFn {
    deref Str { &self.name }
    tokens { self.name.to_tokens(tokens) }
    f { <Str as Display>::fmt(&self, f) }
});

impl Debug for StylerFn {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.full().to_string())
    }
}

// -----------------------------------------------
// VariantStyler
// -----------------------------------------------

#[derive(Clone, Default, Debug)]
pub struct VariantStyler {
    pub set:     Str,
    pub set_mut: Str,
}

impl VariantStyler {
    pub fn new(ident: &Ident, on: bool) -> Self {
        Self {
            set:     Str::new(&format!(
                "{}{}{}",
                if on { AttrStyler::ON } else { "" },
                AttrStyler::SET,
                ident.snake,
            )),
            set_mut: Str::new(&format!(
                "{}{}{}{}",
                if on { AttrStyler::ON } else { "" },
                AttrStyler::SET,
                ident.snake,
                AttrStyler::MUT,
            )),
        }
    }
}

// -----------------------------------------------
// AttrStyler
// -----------------------------------------------

#[derive(Clone, Default, Debug)]
pub struct AttrStyler {
    pub get:      Str,
    pub get_mut:  Str,
    pub set:      Str,
    pub set_mut:  Str,
    pub none:     Str,
    pub none_mut: Str,
}

impl AttrStyler {
    pub fn new(ident: &Ident) -> Self {
        let VariantStyler { set, set_mut } = VariantStyler::new(ident, false);

        Self {
            get: Str::new(&format!("{}{}", Self::GET, ident.snake)),
            get_mut: Str::new(&format!("{}{}{}", Self::GET, ident.snake, Self::MUT)),
            set,
            set_mut,
            none: Str::new(&format!("{}_{}", Lay::NONE.to_lowercase(), ident.snake)),
            none_mut: Str::new(&format!(
                "{}_{}{}",
                Lay::NONE.to_lowercase(),
                ident.snake,
                Self::MUT,
            )),
        }
    }
}

// -----------------------------------------------
// Variant
// -----------------------------------------------

#[derive(Clone, Default, Debug)]
pub struct Variant {
    pub name:       Ident,
    pub args:       Str, // r: u8, g: u8, b: u8
    pub with_types: Str, // Ansi(u8)
    pub full:       Str, // Color::Ansi(ansi)
    pub wrapped:    Str, /* Color: Color::Ansi(ansi), Grounds: Foreground(Color::Ansi(ansi)),
                          * Attrs: Weight::Bold */
    pub styler:     VariantStyler,
    pub fn_set:     StylerFn, // Do not use those fields on Color's variants,
    pub fn_set_mut: StylerFn, // they are defaulted. Use foreground/background instead.
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
        let styler = VariantStyler::new(&name, on);

        Self {
            name,
            args,
            with_types,
            full,
            wrapped,
            styler,
            fn_set: Default::default(),
            fn_set_mut: Default::default(),
        }
    }

    pub fn new_attr(attr: &Attr, name: Ident, fields: &[&str]) -> Self {
        let variant = Self::new(
            name,
            match attr.ty {
                AttrType::Foreground => (Lay::COLOR, Some(&attr), false),
                AttrType::Background => (Lay::COLOR, Some(&attr), true),
                AttrType::Attribute => (&attr, None, false),
            },
            fields,
        );

        let (fn_set, fn_set_mut) = StylerFn::new_variant_set(attr, &variant);

        Self {
            fn_set,
            fn_set_mut,
            ..variant
        }
    }
}

// -----------------------------------------------
// Color
// -----------------------------------------------

#[derive(Clone, Default, Debug)]
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

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum AttrType {
    Foreground,
    Background,
    Attribute,
}

#[derive(Clone, Debug)]
pub struct Attr {
    pub ty:          AttrType,
    pub name:        Ident,
    pub short:       Str,
    pub none:        Str,
    pub styler:      AttrStyler,
    pub variants:    Vec<Variant>,
    pub reset:       Variant,
    pub fn_get:      StylerFn,
    pub fn_get_mut:  StylerFn,
    pub fn_set:      StylerFn,
    pub fn_set_mut:  StylerFn,
    pub fn_none:     StylerFn,
    pub fn_none_mut: StylerFn,
}

derefs!(self Attr {
    deref Ident { &self.name }
    tokens { self.name.to_tokens(tokens) }
    f { <Ident as Display>::fmt(&self, f) }
});

impl Attr {
    pub fn new(ty: AttrType, short: Str, name: Ident, variants: Vec<(Ident, Vec<&str>)>) -> Self {
        let new = || {
            let none = Str::new(&format!("{}{}", Lay::NONE, &name));
            let styler = AttrStyler::new(&name);

            Self {
                ty,
                short,
                name,
                none,
                styler,
                variants: Default::default(),
                reset: Default::default(),
                fn_get: Default::default(),
                fn_get_mut: Default::default(),
                fn_set: Default::default(),
                fn_set_mut: Default::default(),
                fn_none: Default::default(),
                fn_none_mut: Default::default(),
            }
        };

        let attr = new();
        let variants = variants
            .into_iter()
            .map(|(variant, fields)| Variant::new_attr(&attr, variant, &fields))
            .collect::<Vec<_>>();
        let reset = variants.last().unwrap().clone();
        let (fn_get, fn_get_mut) = StylerFn::new_attr_get(&attr);
        let (fn_set, fn_set_mut) = StylerFn::new_attr_set(&attr);
        let (fn_none, fn_none_mut) = StylerFn::new_attr_none(&attr, &fn_set, &fn_set_mut);

        Self {
            variants,
            reset,
            fn_get,
            fn_get_mut,
            fn_set,
            fn_set_mut,
            fn_none,
            fn_none_mut,
            ..attr
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
            .map(|(color, fields)| (Ident::new(color), fields.to_vec()))
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
