use crate::generation::*;
use quote::{ToTokens, TokenStreamExt};
use std::fmt::{Display, Error, Formatter};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Ident(Vec<&'static str>);

impl Ident {
    pub fn new(parts: &[&'static str]) -> Self {
        Self(parts.to_vec())
    }

    pub fn lower(&self) -> proc_macro2::Ident {
        let i = self.0.iter().map(low).collect::<Vec<_>>().join("");
        ident!(i)
    }

    pub fn pascal(&self) -> proc_macro2::Ident {
        let i = self.0.iter().map(cap).collect::<Vec<_>>().join("");
        ident!(i)
    }

    pub fn snake(&self) -> proc_macro2::Ident {
        let i = self.0.iter().map(low).collect::<Vec<_>>().join("_");
        ident!(i)
    }

    pub fn fix(&self, prefix: &[&'static str], suffix: &[&'static str]) -> Self {
        let mut parts = Vec::with_capacity(self.0.len() + prefix.len() + suffix.len());

        for part in prefix.iter().chain(&self.0).chain(suffix) {
            parts.push(*part);
        }

        Self(parts)
    }

    pub fn no(&self) -> Self {
        self.fix(&[Lay::NO], &[])
    }

    pub fn reset(&self) -> Self {
        self.fix(&[Lay::RESET], &[])
    }

    pub fn get(&self) -> Self {
        self.fix(&[Lay::GET], &[])
    }

    pub fn get_mut(&self) -> Self {
        self.fix(&[Lay::GET], &[Lay::MUT])
    }

    pub fn set(&self) -> Self {
        self.fix(&[], &[])
    }

    pub fn set_mut(&self) -> Self {
        self.fix(&[], &[Lay::MUT])
    }

    pub fn on_set(&self) -> Self {
        self.fix(&[Lay::ON], &[])
    }

    pub fn on_set_mut(&self) -> Self {
        self.fix(&[Lay::ON], &[Lay::MUT])
    }

    pub fn on_reset(&self) -> Self {
        self.fix(&[Lay::ON, Lay::RESET], &[])
    }

    pub fn on_reset_mut(&self) -> Self {
        self.fix(&[Lay::ON, Lay::RESET], &[Lay::MUT])
    }
}

impl ToTokens for Ident {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(self.pascal());
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.pascal())
    }
}

fn low(s: &&str) -> String {
    s.to_lowercase()
}

fn cap(s: &&str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
