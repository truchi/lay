use crate::*;
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

#[derive(Clone, PartialEq, Default)]
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
    pub doc:  Doc,
    pub name: Str,
    pub sign: TokenStream,
    pub body: Option<TokenStream>,
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

    pub fn new_tuple(
        doc: Doc,
        doc_mut: Doc,
        name: String,
        sign: impl Fn(&Str) -> TokenStream,
        sign_mut: impl Fn(&Str) -> TokenStream,
        body: Option<TokenStream>,
        body_mut: Option<TokenStream>,
    ) -> (Self, Self) {
        let name_mut = format!("{}{}", name, Self::MUT);
        let name = Str::new(&name);
        let name_mut = Str::new(&name_mut);
        let sign = sign(&name);
        let sign_mut = sign_mut(&name_mut);

        (
            Self::new(doc, name, sign, body),
            Self::new(doc_mut, name_mut, sign_mut, body_mut),
        )
    }

    pub fn new_attr_get(attr: &Attr) -> (Self, Self) {
        Self::new_tuple(
            doc!("Gets `Option<{}>`.", attr),
            doc!("Gets `&mut Option<{}>`.", attr),
            format!("{}{}", Self::GET, attr.snake),
            |name| quote! { fn #name(&self) -> Option<#attr> },
            |name| quote! { fn #name(&mut self) -> &mut Option<#attr> },
            None,
            None,
        )
    }

    pub fn new_attr_set(attr: &Attr) -> (Self, Self) {
        let snake = &attr.snake;

        Self::new_tuple(
            doc!("Sets `Option<{}>`.", attr),
            doc!("Sets `Option<{}>`, mutably.", attr),
            format!("{}{}", Self::SET, snake),
            |name| quote! { fn #name(self, #snake: impl Into<Option<#attr>>) -> Self::Output },
            |name| quote! { fn #name(&mut self, #snake: impl Into<Option<#attr>>) },
            None,
            None,
        )
    }

    pub fn new_attr_none(attr: &Attr) -> (Self, Self) {
        let none = Lay::NONE.to_lowercase();
        let (set, set_mut) = (&attr.fn_set, &attr.fn_set_mut);

        Self::new_tuple(
            doc!("`None`s `Option<{}>`.", attr),
            doc!("`None`s `Option<{}>`, mutably.", attr),
            format!("{}_{}", none, attr.snake),
            |name| quote! { fn #name(self) -> Self::Output },
            |name| quote! { fn #name(&mut self) },
            Some(quote! { self.#set(None) }),
            Some(quote! { self.#set_mut(None); }),
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

        Self::new_tuple(
            doc!("Sets `Some({})`.", wrapped),
            doc!("Sets `Some({})`, mutably.", wrapped),
            format!("{}{}{}", on, Self::SET, snake),
            |name| quote! { fn #name(self, #args) -> Self::Output },
            |name| quote! { fn #name(&mut self, #args) },
            Some(quote! { self.#set(Some(#wrapped)) }),
            Some(quote! { self.#set_mut(Some(#wrapped)); }),
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
// Styler
// -----------------------------------------------

#[derive(Clone, Default, Debug)]
pub struct Styler {
    pub styler:           Ident,
    pub styler_mut:       Ident,
    pub styler_index:     Ident,
    pub styler_index_mut: Ident,
    pub to_style:         StylerFn,
    pub style:            StylerFn,
    pub style_mut:        StylerFn,
    pub and:              StylerFn,
    pub and_mut:          StylerFn,
    pub or:               StylerFn,
    pub or_mut:           StylerFn,
    pub xor:              StylerFn,
    pub xor_mut:          StylerFn,
    pub dedup:            StylerFn,
    pub dedup_mut:        StylerFn,
    pub reset:            StylerFn,
    pub reset_mut:        StylerFn,
}

type A<'a> = (&'a Str, &'a StylerFn, &'a StylerFn, &'a StylerFn, &'a Str);
impl Styler {
    pub fn new_tuple(
        attributes: &[A],
        doc: Doc,
        doc_mut: Doc,
        name: &str,
        sign: impl Fn(&Str) -> TokenStream,
        sign_mut: impl Fn(&Str) -> TokenStream,
        body_inner: impl Fn(&A) -> TokenStream,
        body_inner_mut: impl Fn(&A) -> TokenStream,
        body: impl Fn(&mut dyn Iterator<Item = TokenStream>) -> TokenStream,
        body_mut: impl Fn(&mut dyn Iterator<Item = TokenStream>) -> TokenStream,
    ) -> (StylerFn, StylerFn) {
        let name = name.to_string();
        let body = body(&mut attributes.iter().map(body_inner));
        let body_mut = body_mut(&mut attributes.iter().map(body_inner_mut));

        StylerFn::new_tuple(
            doc,
            doc_mut,
            name,
            sign,
            sign_mut,
            Some(body),
            Some(body_mut),
        )
    }

    pub fn new(attributes: &[Attr]) -> Self {
        let attributes = attributes
            .iter()
            .map(|attribute| {
                (
                    &attribute.snake,
                    &attribute.fn_get,
                    &attribute.fn_set,
                    &attribute.fn_set_mut,
                    &attribute.reset.wrapped,
                )
            })
            .collect::<Vec<_>>();

        let (styler, styler_mut) = (Ident::new(Self::STYLER), Ident::new(Self::STYLER_MUT));
        let (styler_index, styler_index_mut) = (
            Ident::new(Self::STYLER_INDEX),
            Ident::new(Self::STYLER_INDEX_MUT),
        );

        let option = |op| {
            let op = Str::new(op);

            Self::new_tuple(
                &attributes,
                doc!("`Option::{}` fields.", op),
                doc!("`Option::{}` fields, mutably.", op),
                &op,
                |name| {
                    quote! {
                        fn #name(self, other: &impl #styler_index) -> <Self::Output as #styler>::Output
                        where
                            Self::Output: #styler<Output = Self::Output>
                    }
                },
                |name| quote! { fn #name(&mut self, other: &impl #styler_index) },
                |(attribute, get, set, ..)| {
                    quote! {
                        let #attribute = output.#get().#op(other.#get());
                        let output = output.#set(#attribute); #LINE_BREAK
                    }
                },
                |(_, get, _, set_mut, ..)| {
                    quote! {
                        self.#set_mut(self.#get().#op(other.#get()));
                    }
                },
                |body| quote! { let output = self; #LINE_BREAK #(#body)* output },
                |body| quote! { #(#body)* },
            )
        };

        let to_style = attributes
            .iter()
            .map(|(snake, get, ..)| quote! { #snake: self.#get() });
        let to_style = StylerFn::new(
            doc!("Returns a `Style`."),
            Str::new("to_style"),
            quote! { fn to_style(&self) -> Style },
            Some(quote! { Style { #(#to_style,)* } }),
        );

        let (style, style_mut) = Self::new_tuple(
            &attributes,
            doc!("Applies `styler`'s styles."),
            doc!("Applies `styler`'s styles, mutably."),
            "style",
            |name| {
                quote! {
                    fn #name(self, styler: &impl #styler_index) -> <Self::Output as #styler>::Output
                    where
                        Self::Output: #styler<Output = Self::Output>
                }
            },
            |name| quote! { fn #name(&mut self, styler: &impl #styler_index) },
            |(_, get, set, ..)| quote! { .#set(styler.#get()) },
            |(_, get, _, set_mut, _)| quote! { self.#set_mut(styler.#get()); },
            |body| quote! { self#(#body)* },
            |body| quote! { #(#body)* },
        );

        let (dedup, dedup_mut) = Self::new_tuple(
            &attributes,
            doc!("Dedups (`None`s if identicals) fields."),
            doc!("Dedups (`None`s if identicals) fields, mutably."),
            "dedup",
            |name| {
                quote! {
                    fn #name(mut self, before: &impl #styler_index) -> Self
                    where
                        Self: #styler<Output = Self>
                }
            },
            |name| quote! { fn #name(&mut self, before: &impl #styler_index) },
            |(_, get, set, ..)| {
                quote! {
                    if self.#get() == before.#get() {
                        self = self.#set(None);
                    } #LINE_BREAK
                }
            },
            |(_, get, _, set_mut, _)| {
                quote! {
                    if self.#get() == before.#get() {
                        self.#set_mut(None);
                    } #LINE_BREAK
                }
            },
            |body| quote! { #(#body)* self },
            |body| quote! { #(#body)* },
        );

        let (reset, reset_mut) = Self::new_tuple(
            &attributes,
            doc!("Resets (sets to reset value) fields which are `Some`."),
            doc!("Resets (sets to reset value) fields which are `Some`, mutably."),
            "reset",
            |name| {
                quote! {
                    fn #name(mut self) -> Self
                    where
                        Self: #styler<Output = Self>
                }
            },
            |name| quote! { fn #name(&mut self) },
            |(_, get, set, _, reset)| {
                quote! {
                    if let Some(_) = self.#get() {
                        self = self.#set(Some(#reset));
                    } #LINE_BREAK
                }
            },
            |(_, get, _, set_mut, reset)| {
                quote! {
                    if let Some(_) = self.#get() {
                        self.#set_mut(Some(#reset));
                    } #LINE_BREAK
                }
            },
            |body| quote! { #(#body)* self },
            |body| quote! { #(#body)* },
        );

        let (and, and_mut) = option("and");
        let (or, or_mut) = option("or");
        let (xor, xor_mut) = option("xor");

        Self {
            styler,
            styler_mut,
            styler_index,
            styler_index_mut,
            to_style,
            style,
            style_mut,
            and,
            and_mut,
            or,
            or_mut,
            xor,
            xor_mut,
            dedup,
            dedup_mut,
            reset,
            reset_mut,
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
    pub fn_set:     StylerFn, // Do not use those fields on Color's variants,
    pub fn_set_mut: StylerFn, // they are defaulted. Use foreground/background instead.
}

derefs!(self Variant {
    deref Ident { &self.name }
    tokens { self.name.to_tokens(tokens) }
    f { <Ident as Display>::fmt(&self, f) }
});

impl Variant {
    pub fn new(name: Ident, (parent, wrapper): (&str, Option<&str>), fields: &[&str]) -> Self {
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

        Self {
            name,
            args,
            with_types,
            full,
            wrapped,
            fn_set: Default::default(),
            fn_set_mut: Default::default(),
        }
    }

    pub fn new_attr(attr: &Attr, name: Ident, fields: &[&str]) -> Self {
        let variant = Self::new(
            name,
            match attr.ty {
                AttrType::Foreground | AttrType::Background => (Lay::COLOR, Some(&attr)),
                AttrType::Attribute => (&attr, None),
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
            .map(|(name, fields)| Variant::new(name, (Lay::COLOR, None), &fields))
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
    pub none:        Str,
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
    pub fn new(ty: AttrType, name: Ident, variants: Vec<(Ident, Vec<&str>)>) -> Self {
        let new = || {
            let none = Str::new(&format!("{}{}", Lay::NONE, &name));

            Self {
                ty,
                name,
                none,
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
        let (fn_get, fn_get_mut) = StylerFn::new_attr_get(&attr);
        let (fn_set, fn_set_mut) = StylerFn::new_attr_set(&attr);
        let attr = Self {
            fn_get,
            fn_get_mut,
            fn_set,
            fn_set_mut,
            ..attr
        };
        let (fn_none, fn_none_mut) = StylerFn::new_attr_none(&attr);
        let variants = variants
            .into_iter()
            .map(|(variant, fields)| Variant::new_attr(&attr, variant, &fields))
            .collect::<Vec<_>>();
        let reset = variants.last().unwrap().clone();

        Self {
            variants,
            reset,
            fn_none,
            fn_none_mut,
            ..attr
        }
    }

    pub fn grounds(colors: Vec<(Ident, Vec<&str>)>) -> (Self, Self) {
        let foreground = Ident::new(&[Lay::FOREGROUND]);
        let foreground = Attr::new(AttrType::Foreground, foreground, colors.clone());

        let background = Ident::new(&[Lay::BACKGROUND]);
        let background = Attr::new(AttrType::Background, background, colors);

        (foreground, background)
    }

    pub fn attributes(attributes: Vec<(Ident, Vec<(Ident, Vec<&str>)>)>) -> Vec<Self> {
        attributes
            .into_iter()
            .map(|(name, fields)| Attr::new(AttrType::Attribute, name, fields))
            .collect()
    }
}

impl PartialEq for Attr {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
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
    pub styler:     Styler,
}

impl Lay {
    pub fn new() -> Self {
        let colors: Vec<(Ident, Vec<&str>)> = Self::COLORS
            .iter()
            .map(|(color, fields)| (Ident::new(color), fields.to_vec()))
            .chain(vec![(Ident::new(&[Self::RESET, Self::COLOR]), vec![])])
            .collect::<Vec<_>>();

        let attributes: Vec<(Ident, Vec<(Ident, Vec<&str>)>)> = Self::ATTRIBUTES
            .iter()
            .map(|(name, variants)| {
                let name = Ident::new(&[name]);

                let variants = variants
                    .iter()
                    .map(|variant| (Ident::new(&[variant]), vec![]))
                    .chain(vec![(Ident::new(&[Self::RESET, &name]), vec![])])
                    .collect();

                (name, variants)
            })
            .collect::<Vec<_>>();

        let (foreground, background) = Attr::grounds(colors.clone());
        let grounds = vec![foreground.clone(), background.clone()];
        let attributes = Attr::attributes(attributes);
        let all = [grounds.clone(), attributes.clone()].concat();

        let styler = Styler::new(&all);

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
            styler,
        }
    }
}
