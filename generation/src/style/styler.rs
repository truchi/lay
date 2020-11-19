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

        let mapper = |set: fn(&Attr) -> &StylerFn,
                      none: fn(&Attr) -> &StylerFn,
                      vset: fn(&Variant) -> &StylerFn| {
            move |attribute: &Attr| {
                let comment = centered_comment!(76, "{}", attribute);
                let set = set(attribute).full();
                let variants = attribute.variants.iter().map(|variant| {
                    let set = vset(variant).full();
                    quote! { #set #LINE_BREAK }
                });
                let none = none(attribute).full();

                quote! {
                    #comment #LINE_BREAK
                    #set     #LINE_BREAK
                    #none    #LINE_BREAK
                    #(#variants)*
                }
            }
        };

        let setters = self.all.iter().map(mapper(
            |attribute| &attribute.fn_set,
            |attribute| &attribute.fn_none,
            |variant| &variant.fn_set,
        ));
        let setters_mut = self.all.iter().map(mapper(
            |attribute| &attribute.fn_set_mut,
            |attribute| &attribute.fn_none_mut,
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

    // ========================= //
    // Operation implementations //
    // ========================= //

    pub fn impl_styler_ops(
        &self,
        (ty, bounds, _): (&Str, &[TokenStream], &Str),
        mutable: bool,
    ) -> TokenStream {
        let (styler, styler_index) = (&self.styler.styler, &self.styler.styler_index);
        let styler_snake = &styler.snake;

        let (add, mul, div, bitand, bitor, bitxor, rem, not) = ops_names(mutable);
        let (the_trait, output_self, output_output, and, or, xor, dedup, reset) = if mutable {
            (
                &self.styler.styler_mut,
                None,
                None,
                &self.styler.and_mut,
                &self.styler.or_mut,
                &self.styler.xor_mut,
                &self.styler.dedup_mut,
                &self.styler.reset_mut,
            )
        } else {
            (
                &self.styler.styler,
                Some(quote! { Self }),
                Some(quote! { <<Self as #styler>::Output as #styler>::Output }),
                &self.styler.and,
                &self.styler.or,
                &self.styler.xor,
                &self.styler.dedup,
                &self.styler.reset,
            )
        };

        let rhs = Ident {
            pascal: Str::new("&Index"),
            snake:  styler_snake.clone(),
        };
        let mut bounds_with_rhs = bounds.to_vec();
        bounds_with_rhs.push(quote! { Index: #styler_index });

        let ground = |op, ground: &Attr| {
            let set = if mutable {
                &ground.fn_set_mut
            } else {
                &ground.fn_set
            };
            let rhs_snake = &ground.snake;
            let rhs = Ident {
                pascal: Str::new("Color"),
                snake:  rhs_snake.clone(),
            };
            let mut bounds = bounds.to_vec();
            bounds.push(quote! { #rhs: Into<Option<#ground>> });

            impl_op(
                mutable,
                op,
                &set.doc,
                ty,
                &bounds,
                Some(&rhs),
                &output_self,
                quote! { #the_trait::#set(self, #rhs_snake) },
            )
        };

        let attributes = self.all.iter().map(|attribute| {
            let (set, none) = if mutable {
                (&attribute.fn_set_mut, &attribute.fn_none_mut)
            } else {
                (&attribute.fn_set, &attribute.fn_none)
            };

            let rhs = &attribute.name;
            let rhs_snake = &rhs.snake;
            let yes = impl_op(
                mutable,
                &add,
                &set.doc,
                ty,
                bounds,
                Some(&rhs),
                &output_self,
                quote! { #the_trait::#set(self, #rhs_snake) },
            );

            let rhs = &Ident {
                pascal: attribute.none.clone(),
                snake:  Str::new("_"),
            };
            let no = impl_op(
                mutable,
                &add,
                &none.doc,
                ty,
                bounds,
                Some(&rhs),
                &output_self,
                quote! { #the_trait::#none(self) },
            );

            quote! { #yes #no }
        });

        let ops = |op, set: &StylerFn| {
            impl_op(
                mutable,
                op,
                &set.doc,
                ty,
                &bounds_with_rhs,
                Some(&rhs),
                &output_output,
                quote! { #the_trait::#set(self, #styler_snake) },
            )
        };

        let foreground = ground(&mul, &self.foreground);
        let background = ground(&div, &self.background);
        let and = ops(&bitand, &and);
        let or = ops(&bitor, &or);
        let xor = ops(&bitxor, &xor);

        let dedup = impl_op(
            mutable,
            &rem,
            &dedup.doc,
            ty,
            &bounds_with_rhs,
            Some(&rhs),
            &output_self,
            quote! { #the_trait::#dedup(self, #styler_snake) },
        );

        let reset = if mutable {
            impl_op(
                false,
                &not,
                &reset.doc,
                &Str::new(&format!("&mut {}", ty)),
                &bounds,
                None,
                &Some(quote! { Self }),
                quote! { #the_trait::#reset(self); self },
            )
        } else {
            impl_op(
                false,
                &not,
                &reset.doc,
                ty,
                &bounds,
                None,
                &Some(quote! { Self }),
                quote! { #the_trait::#reset(self) },
            )
        };

        quote! {
            #[cfg(feature = "styler-ops")]
            use std::ops::{#add, #div, #mul, #bitand, #bitor, #bitxor, #rem};
            #LINE_BREAK

            #foreground #background
            #(#attributes)*
            #and #or #xor
            #dedup #reset
        }
    }
}

// ======= //
// Helpers //
// ======= //

fn impl_op(
    mutable: bool,
    op: &Ident,
    doc: &Doc,
    ty: &Str,
    bounds: &[TokenStream],
    rhs: Option<&Ident>,
    output: &Option<TokenStream>,
    body: TokenStream,
) -> TokenStream {
    let self_arg = if mutable {
        quote! { &mut self }
    } else {
        quote! { self }
    };
    let op_snake = &op.snake;
    let arg = rhs.as_ref().map(|rhs| {
        let rhs_snake = &rhs.snake;

        quote! { #rhs_snake: #rhs }
    });
    let output = output.as_ref();
    let ret = output.map(|output| quote! { -> #output });
    let output = output.map(|output| quote! { type Output = #output; });

    quote! {
        #[cfg(feature = "styler-ops")]
        #doc
        impl<#(#bounds),*> #op<#rhs> for #ty {
            #output

            #doc
            fn #op_snake(#self_arg, #arg) #ret {
                #body
            }
        }
        #LINE_BREAK
    }
}

fn ops_names(mutable: bool) -> (Ident, Ident, Ident, Ident, Ident, Ident, Ident, Ident) {
    let not = Ident::new(&["Not"]);

    if mutable {
        (
            Ident::new(&["Add", "Assign"]),
            Ident::new(&["Mul", "Assign"]),
            Ident::new(&["Div", "Assign"]),
            Ident {
                pascal: Str::new("BitAndAssign"),
                snake:  Str::new("bitand_assign"),
            },
            Ident {
                pascal: Str::new("BitOrAssign"),
                snake:  Str::new("bitor_assign"),
            },
            Ident {
                pascal: Str::new("BitXorAssign"),
                snake:  Str::new("bitxor_assign"),
            },
            Ident::new(&["Rem", "Assign"]),
            not,
        )
    } else {
        (
            Ident::new(&["Add"]),
            Ident::new(&["Mul"]),
            Ident::new(&["Div"]),
            Ident {
                pascal: Str::new("BitAnd"),
                snake:  Str::new("bitand"),
            },
            Ident {
                pascal: Str::new("BitOr"),
                snake:  Str::new("bitor"),
            },
            Ident {
                pascal: Str::new("BitXor"),
                snake:  Str::new("bitxor"),
            },
            Ident::new(&["Rem"]),
            not,
        )
    }
}
