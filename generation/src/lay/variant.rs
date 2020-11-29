use crate::*;

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
