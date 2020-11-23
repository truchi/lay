use crate::*;

impl Generation {
    pub fn mod_style_styler(&self) -> TokenStream {
        let (styler, styler_mut, styler_index, styler_index_mut) = (
            &self.styler.styler.snake,
            &self.styler.styler_mut.snake,
            &self.styler.styler_index.snake,
            &self.styler.styler_index_mut.snake,
        );

        quote! {
            mod #styler_index;
            mod #styler_index_mut;
            mod #styler;
            mod #styler_mut;
            #LINE_BREAK
            pub use #styler_index::*;
            pub use #styler_index_mut::*;
            pub use #styler::*;
            pub use #styler_mut::*;
        }
    }

    // ============ //
    // Declarations //
    // ============ //

    pub fn styler_index(&self) -> (TokenStream, TokenStream) {
        let Styler {
            styler_index,
            styler_index_mut,
            ..
        } = &self.styler;
        let to_style = impl_to_style(self);
        let index = self
            .all
            .iter()
            .map(|attribute| &attribute.fn_get.full)
            .map(|full| quote! { #full; #LINE_BREAK });
        let index_mut = self
            .all
            .iter()
            .map(|attribute| &attribute.fn_get_mut.full)
            .map(|full| quote! { #full; #LINE_BREAK });

        (
            quote! {
                use crate::*; #LINE_BREAK

                /// `Option`al [`attributes`](crate::attributes) getters.
                pub trait #styler_index {
                    #(#index)*
                    #to_style
                }
            },
            quote! {
                use crate::*; #LINE_BREAK

                /// `Option`al [`attributes`](crate::attributes) getters, mutably.
                pub trait #styler_index_mut {
                    #(#index_mut)*
                }
            },
        )
    }

    pub fn styler(&self) -> (TokenStream, TokenStream) {
        let (styler_index, styler, styler_mut) = (
            &self.styler.styler_index,
            &self.styler.styler,
            &self.styler.styler_mut,
        );

        let (setters, setters_mut) = impl_attributes(self);
        let (style, style_mut) = impl_style(self);
        let (and, and_mut) = impl_op(self, "and");
        let (or, or_mut) = impl_op(self, "or");
        let (xor, xor_mut) = impl_op(self, "xor");
        let (dedup, dedup_mut) = impl_dedup(self);
        let (reset, reset_mut) = impl_reset(self);
        let comment = centered_comment!(76, "Additional functions");

        (
            quote! {
                use crate::*; #LINE_BREAK

                /// `Option`al [`attributes`](crate::attributes) setters.
                pub trait #styler: #styler_index + Sized {
                    /// The resulting type of the setters.
                    type Output; #LINE_BREAK
                    #setters
                    #comment #LINE_BREAK
                    #style   #LINE_BREAK
                    #and     #LINE_BREAK
                    #or      #LINE_BREAK
                    #xor     #LINE_BREAK
                    #dedup   #LINE_BREAK
                    #reset   #LINE_BREAK
                }
            },
            quote! {
                use crate::*; #LINE_BREAK

                /// `Option`al [`attributes`](crate::attributes) setters, mutably.
                pub trait #styler_mut: #styler_index {
                    #setters_mut
                    #comment   #LINE_BREAK
                    #style_mut #LINE_BREAK
                    #and_mut   #LINE_BREAK
                    #or_mut    #LINE_BREAK
                    #xor_mut   #LINE_BREAK
                    #dedup_mut #LINE_BREAK
                    #reset_mut #LINE_BREAK
                }
            },
        )
    }
}

// ================ //
// Provided methods //
// ================ //

fn impl_to_style(lay: &Lay) -> TokenStream {
    let to_style = &lay.styler.to_style;
    let full = &to_style.full;
    let style = lay
        .all
        .iter()
        .map(|attribute| (&attribute.snake, &attribute.fn_get))
        .map(|(snake, get)| quote! { #snake: self.#get() });

    quote! { #full { Style { #(#style),* } }}
}

fn impl_attributes(lay: &Lay) -> (TokenStream, TokenStream) {
    let mapper = |set: fn(&Attr) -> &StylerFn,
                  none: fn(&Attr) -> &StylerFn,
                  vset: fn(&Variant) -> &StylerFn| {
        move |attribute: &Attr| {
            let comment = centered_comment!(76, "{}", attribute);
            let set = set(attribute);
            let none = none(attribute);
            let set_full = &set.full;
            let none_full = &none.full;

            let variants = attribute.variants.iter().map(|variant| {
                let variant_set = vset(variant);
                let variant_full = &variant_set.full;
                let wrapped = &variant.wrapped;

                quote! { #variant_full { self.#set(Some(#wrapped)) } #LINE_BREAK }
            });

            quote! {
                #comment                       #LINE_BREAK
                #set_full;                     #LINE_BREAK
                #none_full { self.#set(None) } #LINE_BREAK
                #(#variants)*
            }
        }
    };

    let setters = lay.all.iter().map(mapper(
        |attribute| &attribute.fn_set,
        |attribute| &attribute.fn_none,
        |variant| &variant.fn_set,
    ));
    let setters_mut = lay.all.iter().map(mapper(
        |attribute| &attribute.fn_set_mut,
        |attribute| &attribute.fn_none_mut,
        |variant| &variant.fn_set_mut,
    ));

    (quote! { #(#setters)* }, quote! { #(#setters_mut)* })
}

fn impl_style(lay: &Lay) -> (TokenStream, TokenStream) {
    let (style, style_mut) = (&lay.styler.style, &lay.styler.style_mut);
    let full = &style.full;
    let full_mut = &style_mut.full;

    let body = lay
        .all
        .iter()
        .map(|attribute| (&attribute.fn_get, &attribute.fn_set))
        .map(|(get, set)| quote! { .#set(styler.#get()) });
    let body_mut = lay
        .all
        .iter()
        .map(|attribute| (&attribute.fn_get, &attribute.fn_set_mut))
        .map(|(get, set_mut)| quote! { self.#set_mut(styler.#get()); });

    (
        quote! { #full { self#(#body)* }},
        quote! { #full_mut { #(#body_mut)* } },
    )
}

fn impl_op(lay: &Lay, op: &str) -> (TokenStream, TokenStream) {
    let (op, op_mut) = lay.styler.op(op);
    let full = &op.full;
    let full_mut = &op_mut.full;

    let body = lay
        .all
        .iter()
        .map(|attribute| (&attribute.snake, &attribute.fn_get, &attribute.fn_set))
        .map(|(snake, get, set)| {
            quote! {
                let #snake = output.#get().#op(other.#get());
                let output = output.#set(#snake); #LINE_BREAK
            }
        });
    let body_mut = lay
        .all
        .iter()
        .map(|attribute| (&attribute.fn_get, &attribute.fn_set_mut))
        .map(|(get, set_mut)| quote! { self.#set_mut(self.#get().#op(other.#get())); });

    (
        quote! { #full { let output = self; #LINE_BREAK #(#body)* output }},
        quote! { #full_mut { #(#body_mut)* } },
    )
}

fn impl_dedup(lay: &Lay) -> (TokenStream, TokenStream) {
    let (dedup, dedup_mut) = (&lay.styler.dedup, &lay.styler.dedup_mut);
    let full = &dedup.full;
    let full_mut = &dedup_mut.full;

    let body = lay
        .all
        .iter()
        .map(|attribute| (&attribute.fn_get, &attribute.fn_set))
        .map(|(get, set)| {
            quote! {
                if self.#get() == before.#get() { self = self.#set(None); }
                #LINE_BREAK
            }
        });
    let body_mut = lay
        .all
        .iter()
        .map(|attribute| (&attribute.fn_get, &attribute.fn_set_mut))
        .map(|(get, set_mut)| {
            quote! {
                if self.#get() == before.#get() { self.#set_mut(None); }
                #LINE_BREAK
            }
        });

    (
        quote! { #full { #(#body)* self } },
        quote! { #full_mut { #(#body_mut)* } },
    )
}

fn impl_reset(lay: &Lay) -> (TokenStream, TokenStream) {
    let (reset, reset_mut) = (&lay.styler.reset, &lay.styler.reset_mut);
    let full = &reset.full;
    let full_mut = &reset_mut.full;

    let body = lay
        .all
        .iter()
        .map(|attr| (&attr.fn_get, &attr.fn_set, &attr.reset.wrapped))
        .map(|(get, set, reset)| {
            quote! {
                if let Some(_) = self.#get() { self = self.#set(Some(#reset)); }
                #LINE_BREAK
            }
        });
    let body_mut = lay
        .all
        .iter()
        .map(|attr| (&attr.fn_get, &attr.fn_set_mut, &attr.reset.wrapped))
        .map(|(get, set_mut, reset)| {
            quote! {
                if let Some(_) = self.#get() { self.#set_mut(Some(#reset)); }
                #LINE_BREAK
            }
        });

    (
        quote! { #full { #(#body)* self } },
        quote! { #full_mut { #(#body_mut)* } },
    )
}
