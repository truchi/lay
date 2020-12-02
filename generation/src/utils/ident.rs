use crate::*;

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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({:#?}, {:#?})", self.pascal, self.snake)
    }
}
