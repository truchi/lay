use crate::*;

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

impl Styler {
    pub fn new_tuple(
        doc: Doc,
        doc_mut: Doc,
        name: &str,
        sign: TokenStream,
        sign_mut: TokenStream,
    ) -> (StylerFn, StylerFn) {
        StylerFn::new_tuple(doc, doc_mut, name, sign, sign_mut)
    }

    pub fn new() -> Self {
        let (styler, styler_mut) = (Ident::new(Self::STYLER), Ident::new(Self::STYLER_MUT));
        let (styler_index, styler_index_mut) = (
            Ident::new(Self::STYLER_INDEX),
            Ident::new(Self::STYLER_INDEX_MUT),
        );

        let option = |op| {
            let op = Str::new(op);

            Self::new_tuple(
                doc!("`Option::{}` fields.", op),
                doc!("`Option::{}` fields, mutably.", op),
                &op,
                quote! {
                    (self, other: &impl #styler_index)
                    -> <Self::Output as #styler>::Output
                    where
                        Self::Output: #styler<Output = Self::Output>
                },
                quote! { (&mut self, other: &impl #styler_index) },
            )
        };

        let to_style = StylerFn::new(
            doc!("Returns a `Style`."),
            "to_style",
            quote! { (&self) -> Style },
        );

        let (style, style_mut) = Self::new_tuple(
            doc!("Applies `styler`'s styles."),
            doc!("Applies `styler`'s styles, mutably."),
            "style",
            quote! {
                (self, styler: &impl #styler_index)
                -> <Self::Output as #styler>::Output
                where
                    Self::Output: #styler<Output = Self::Output>
            },
            quote! { (&mut self, styler: &impl #styler_index) },
        );

        let (dedup, dedup_mut) = Self::new_tuple(
            doc!("Dedups (`None`s if identicals) fields."),
            doc!("Dedups (`None`s if identicals) fields, mutably."),
            "dedup",
            quote! {
                (mut self, before: &impl #styler_index) -> Self
                where
                    Self: #styler<Output = Self>
            },
            quote! { (&mut self, before: &impl #styler_index) },
        );

        let (reset, reset_mut) = Self::new_tuple(
            doc!("Resets (sets to reset value) fields which are `Some`."),
            doc!("Resets (sets to reset value) fields which are `Some`, mutably."),
            "reset",
            quote! {
                (mut self) -> Self
                where
                    Self: #styler<Output = Self>
            },
            quote! { (&mut self) },
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

    pub fn op(&self, op: &str) -> (StylerFn, StylerFn) {
        let (styler_index, styler) = (&self.styler_index, &self.styler);

        StylerFn::new_tuple(
            doc!("`Option::{}` fields.", op),
            doc!("`Option::{}` fields, mutably.", op),
            op,
            quote! {
                (self, other: &impl #styler_index)
                -> <Self::Output as #styler>::Output
                where
                    Self::Output: #styler<Output = Self::Output>
            },
            quote! { (&mut self, other: &impl #styler_index) },
        )
    }
}
