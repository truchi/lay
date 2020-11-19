use crate::*;
use std::fmt::Display;

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
