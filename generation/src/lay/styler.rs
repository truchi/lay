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
