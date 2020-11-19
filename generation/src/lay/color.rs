use crate::*;
use std::fmt::Display;

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
