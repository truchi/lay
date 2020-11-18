use crate::generation::*;

impl Generation {
    pub fn impl_styler_mut_ops(
        &self,
        (ty, bounds, _): (&Str, &[TokenStream], &Str),
        mutable: bool,
    ) -> TokenStream {
        let (styler, styler_index) = (&self.styler.styler, &self.styler.styler_index);
        let styler_snake = &styler.snake;

        let (add, mul, div, bitand, bitor, bitxor, rem) = ops_names(mutable);
        let (the_trait, output_self, output_output, and, or, xor, dedup) = if mutable {
            (
                &self.styler.styler_mut,
                None,
                None,
                &self.styler.and_mut,
                &self.styler.or_mut,
                &self.styler.xor_mut,
                &self.styler.dedup_mut,
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

        let attributes = self.attributes.iter().map(|attribute| {
            let set = if mutable {
                &attribute.fn_set_mut
            } else {
                &attribute.fn_set
            };
            let rhs = &attribute.name;
            let rhs_snake = &rhs.snake;

            impl_op(
                mutable,
                &add,
                &set.doc,
                ty,
                bounds,
                Some(&rhs),
                &output_self,
                quote! { #the_trait::#set(self, #rhs_snake) },
            )
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

        // TODO reset / reset_mut

        quote! {
            #[cfg(feature = "styler-ops")]
            use std::ops::{#add, #div, #mul, #bitand, #bitor, #bitxor, #rem};
            #LINE_BREAK

            #foreground #background
            #(#attributes)*
            #and #or #xor
            #dedup
        }
    }
}

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

fn ops_names(mutable: bool) -> (Ident, Ident, Ident, Ident, Ident, Ident, Ident) {
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
        )
    }
}
