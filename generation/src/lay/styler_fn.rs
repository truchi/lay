use crate::*;

#[derive(Clone, Default)]
pub struct StylerFn {
    pub doc:  Doc,
    pub name: Str,
    pub sign: TokenStream,
    pub full: TokenStream,
}

impl StylerFn {
    pub fn new(doc: Doc, name: &str, sign: TokenStream) -> Self {
        let name = Str::new(name);
        let sign = quote! { fn #name #sign };
        let full = quote! { #doc #sign };

        Self {
            doc,
            name,
            sign,
            full,
        }
    }

    pub fn new_tuple(
        doc: Doc,
        doc_mut: Doc,
        name: &str,
        sign: TokenStream,
        sign_mut: TokenStream,
    ) -> (Self, Self) {
        let name_mut = &format!("{}{}", name, Self::MUT);

        (
            Self::new(doc, name, sign),
            Self::new(doc_mut, name_mut, sign_mut),
        )
    }

    pub fn new_attr_get(attr: &Attr) -> (Self, Self) {
        Self::new_tuple(
            doc!("Gets `Option<{}>`.", attr),
            doc!("Gets `&mut Option<{}>`.", attr),
            &format!("{}{}", Self::GET, attr.snake),
            quote! { (&self) -> Option<#attr> },
            quote! { (&mut self) -> &mut Option<#attr> },
        )
    }

    pub fn new_attr_set(attr: &Attr) -> (Self, Self) {
        let snake = &attr.snake;

        Self::new_tuple(
            doc!("Sets `Option<{}>`.", attr),
            doc!("Sets `Option<{}>`, mutably.", attr),
            &format!("{}{}", Self::SET, snake),
            quote! { (self, #snake: impl Into<Option<#attr>>) -> Self },
            quote! { (&mut self, #snake: impl Into<Option<#attr>>) },
        )
    }

    pub fn new_attr_none(attr: &Attr) -> (Self, Self) {
        let none = Lay::NONE.to_lowercase();

        Self::new_tuple(
            doc!("`None`s `Option<{}>`.", attr),
            doc!("`None`s `Option<{}>`, mutably.", attr),
            &format!("{}_{}", none, attr.snake),
            quote! { (self) -> Self },
            quote! { (&mut self) },
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
            &format!("{}{}{}", on, Self::SET, snake),
            quote! { (self, #args) -> Self },
            quote! { (&mut self, #args) },
        )
    }
}

derefs!(self StylerFn {
    deref Str { &self.name }
    tokens { self.name.to_tokens(tokens) }
    f { <Str as Display>::fmt(&self, f) }
});

impl Debug for StylerFn {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.sign.to_string())
    }
}
