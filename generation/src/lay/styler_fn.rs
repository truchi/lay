use crate::*;
use std::fmt::{Debug, Display, Error, Formatter};

#[derive(Clone, Default)]
pub struct StylerFn {
    pub doc:  Doc,
    pub name: Str,
    pub sign: TokenStream,
}

impl StylerFn {
    pub fn new(doc: Doc, name: Str, sign: TokenStream) -> Self {
        Self { doc, name, sign }
    }

    pub fn new_tuple(
        doc: Doc,
        doc_mut: Doc,
        name: String,
        sign: impl Fn(&Str) -> TokenStream,
        sign_mut: impl Fn(&Str) -> TokenStream,
    ) -> (Self, Self) {
        let name_mut = format!("{}{}", name, Self::MUT);
        let name = Str::new(&name);
        let name_mut = Str::new(&name_mut);
        let sign = sign(&name);
        let sign_mut = sign_mut(&name_mut);

        (
            Self::new(doc, name, sign),
            Self::new(doc_mut, name_mut, sign_mut),
        )
    }

    pub fn new_attr_get(attr: &Attr) -> (Self, Self) {
        Self::new_tuple(
            doc!("Gets `Option<{}>`.", attr),
            doc!("Gets `&mut Option<{}>`.", attr),
            format!("{}{}", Self::GET, attr.snake),
            |name| quote! { fn #name(&self) -> Option<#attr> },
            |name| quote! { fn #name(&mut self) -> &mut Option<#attr> },
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
        )
    }

    pub fn new_attr_none(attr: &Attr) -> (Self, Self) {
        let none = Lay::NONE.to_lowercase();

        Self::new_tuple(
            doc!("`None`s `Option<{}>`.", attr),
            doc!("`None`s `Option<{}>`, mutably.", attr),
            format!("{}_{}", none, attr.snake),
            |name| quote! { fn #name(self) -> Self::Output },
            |name| quote! { fn #name(&mut self) },
        )
    }

    pub fn new_variant_set(attr: &Attr, variant: &Variant) -> (Self, Self) {
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
        )
    }
}

derefs!(self StylerFn {
    deref Str { &self.name }
    tokens { self.name.to_tokens(tokens) }
    f { <Str as Display>::fmt(&self, f) }
});

impl Debug for StylerFn {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.sign.to_string())
    }
}
