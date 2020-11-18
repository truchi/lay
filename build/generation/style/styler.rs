use crate::generation::*;

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

    pub fn styler_index(&self) -> (TokenStream, TokenStream) {
        let Styler {
            styler_index,
            styler_index_mut,
            to_style,
            ..
        } = &self.styler;
        let to_style = to_style.full();
        let index = self
            .all
            .iter()
            .map(|attribute| attribute.fn_get.full())
            .map(|t| quote! { #t #LINE_BREAK });
        let index_mut = self
            .all
            .iter()
            .map(|attribute| attribute.fn_get_mut.full())
            .map(|t| quote! { #t #LINE_BREAK });

        (
            quote! {
                use crate::*; #LINE_BREAK

                /// A trait for getting `Option`al attributes on styled types.
                pub trait #styler_index {
                    #(#index)* #LINE_BREAK
                    #to_style
                }
            },
            quote! {
                use crate::*; #LINE_BREAK

                /// A trait for getting `Option`al attributes on mutable styled types.
                pub trait #styler_index_mut {
                    #(#index_mut)*
                }
            },
        )
    }

    pub fn styler(&self) -> (TokenStream, TokenStream) {
        let (
            styler,
            styler_mut,
            styler_index,
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
        ) = (
            &self.styler.styler,
            &self.styler.styler_mut,
            &self.styler.styler_index,
            &self.styler.style.full(),
            &self.styler.style_mut.full(),
            &self.styler.and.full(),
            &self.styler.and_mut.full(),
            &self.styler.or.full(),
            &self.styler.or_mut.full(),
            &self.styler.xor.full(),
            &self.styler.xor_mut.full(),
            &self.styler.dedup.full(),
            &self.styler.dedup_mut.full(),
            &self.styler.reset.full(),
            &self.styler.reset_mut.full(),
        );

        let mapper = |f1: fn(&Attr) -> &StylerFn, f2: fn(&Variant) -> &StylerFn| {
            move |attribute: &Attr| {
                let comment = centered_comment!(76, "{}", attribute);
                let set = f1(attribute).full();
                let variants = attribute.variants.iter().map(|variant| {
                    let set = f2(variant).full();
                    quote! { #set #LINE_BREAK }
                });

                quote! { #comment #LINE_BREAK #set #LINE_BREAK #(#variants)* }
            }
        };

        let setters = self.all.iter().map(mapper(
            |attribute| &attribute.fn_set,
            |variant| &variant.fn_set,
        ));
        let setters_mut = self.all.iter().map(mapper(
            |attribute| &attribute.fn_set_mut,
            |variant| &variant.fn_set_mut,
        ));
        let comment = centered_comment!(76, "Additional functions");

        (
            quote! {
                use crate::*; #LINE_BREAK

                /// A trait for setting `Option`al attributes on styled types.
                pub trait #styler: #styler_index + Sized {
                    /// The resulting type of the setters.
                    type Output; #LINE_BREAK
                    #(#setters)*
                    #comment #LINE_BREAK
                    #style #LINE_BREAK
                    #and   #LINE_BREAK
                    #or    #LINE_BREAK
                    #xor   #LINE_BREAK
                    #dedup #LINE_BREAK
                    #reset #LINE_BREAK
                }
            },
            quote! {
                use crate::*; #LINE_BREAK

                /// A trait for setting `Option`al attributes on mutable styled types.
                pub trait #styler_mut: #styler_index {
                    #(#setters_mut)*
                    #comment #LINE_BREAK
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

    // =============== //
    // Implementations //
    // =============== //

    pub fn impl_styler_index(
        &self,
        (ty, bounds, field): (&Str, &[TokenStream], &Str),
    ) -> TokenStream {
        let styler_index = &self.styler.styler_index;

        let getters = self.all.iter().map(|attribute| {
            let get = &attribute.fn_get;
            let get_sign = &get.sign;
            quote! { #get_sign { #styler_index::#get(&self.#field) } }
        });

        quote! {
            impl<#(#bounds)*> #styler_index for #ty {
                #(#getters)*
            }
        }
    }

    pub fn impl_styler_index_mut(
        &self,
        (ty, bounds, field): (&Str, &[TokenStream], &Str),
    ) -> TokenStream {
        let styler_index_mut = &self.styler.styler_index_mut;

        let getters = self.all.iter().map(|attribute| {
            let get = &attribute.fn_get_mut;
            let get_sign = &get.sign;
            quote! { #get_sign { #styler_index_mut::#get(&mut self.#field) } }
        });

        quote! {
            impl<#(#bounds)*> #styler_index_mut for #ty {
                #(#getters)*
            }
        }
    }

    pub fn impl_styler(&self, (ty, bounds, field): (&Str, &[TokenStream], &Str)) -> TokenStream {
        let styler = &self.styler.styler;

        let setters = self.all.iter().map(|attribute| {
            let set = &attribute.fn_set;
            let set_sign = &set.sign;
            let snake = &attribute.snake;
            quote! { #set_sign { #styler::#set(self.#field, #snake); self } }
        });

        quote! {
            impl<#(#bounds)*> #styler for #ty {
                type Output = Self;

                #(#setters)*
            }
        }
    }

    pub fn impl_styler_mut(
        &self,
        (ty, bounds, field): (&Str, &[TokenStream], &Str),
    ) -> TokenStream {
        let styler_mut = &self.styler.styler_mut;

        let setters = self.all.iter().map(|attribute| {
            let set = &attribute.fn_set_mut;
            let set_sign = &set.sign;
            let snake = &attribute.snake;
            quote! { #set_sign { #styler_mut::#set(&mut self.#field, #snake) } }
        });

        quote! {
            impl<#(#bounds)*> #styler_mut for #ty {
                #(#setters)*
            }
        }
    }
}
