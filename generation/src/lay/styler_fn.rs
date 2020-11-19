use crate::*;
use std::fmt::{Debug, Display, Error, Formatter};

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
