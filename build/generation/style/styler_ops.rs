use crate::generation::*;

impl Generation {
    pub fn impl_styler_ops(&self, (ty, bounds, _): (&Str, &[TokenStream], &Str)) -> TokenStream {
        let styler = &self.styler.styler;
        let styler_index = &self.styler.styler_index;
        let styler_snake = &styler.snake;

        let ground = |op, ground: &Attr| {
            let set = &ground.fn_set;
            let rhs_snake = &ground.snake;
            let rhs = Ident {
                pascal: Str::new("Color"),
                snake:  rhs_snake.clone(),
            };
            let mut bounds = bounds.to_vec();
            bounds.push(quote! { #rhs: Into<Option<#ground>> });

            let ground_op = impl_op(
                &op,
                &set.doc,
                ty,
                &bounds,
                Some(&rhs),
                Some(quote! { Self }),
                quote! { #styler::#set(self, #rhs_snake) },
            );

            quote! { #ground_op #LINE_BREAK }
        };
        let foreground = ground(Ident::new(&["Mul"]), &self.foreground);
        let background = ground(Ident::new(&["Div"]), &self.background);

        let add = Ident::new(&["Add"]);
        let attributes = self.attributes.iter().map(|attribute| {
            let set = &attribute.fn_set;
            let rhs = &attribute.name;
            let rhs_snake = &rhs.snake;

            let add = impl_op(
                &add,
                &set.doc,
                ty,
                bounds,
                Some(&rhs),
                Some(quote! { Self }),
                quote! { #styler::#set(self, #rhs_snake) },
            );

            quote! { #add #LINE_BREAK }
        });

        let rhs = Ident {
            pascal: Str::new("&Index"),
            snake:  styler_snake.clone(),
        };
        let mut bounds_with_rhs = bounds.to_vec();
        bounds_with_rhs.push(quote! { Index: #styler_index });
        let ops = |op, set: &StylerFn| {
            impl_op(
                &op,
                &set.doc,
                ty,
                &bounds_with_rhs,
                Some(&rhs),
                Some(quote! { <<Self as #styler>::Output as #styler>::Output }),
                quote! { #styler::#set(self, #styler_snake) },
            )
        };

        let and = ops(
            Ident {
                pascal: Str::new("BitAnd"),
                snake:  Str::new("bitand"),
            },
            &self.styler.and,
        );
        let or = ops(
            Ident {
                pascal: Str::new("BitOr"),
                snake:  Str::new("bitor"),
            },
            &self.styler.or,
        );
        let xor = ops(
            Ident {
                pascal: Str::new("BitXor"),
                snake:  Str::new("bitxor"),
            },
            &self.styler.xor,
        );

        let dedup = &self.styler.dedup;
        let dedup = impl_op(
            &Ident::new(&["Rem"]),
            &dedup.doc,
            ty,
            &bounds_with_rhs,
            Some(&rhs),
            Some(quote! { Self }),
            quote! { #styler::#dedup(self, #styler_snake) },
        );

        let reset = &self.styler.reset;
        let reset = impl_op(
            &Ident::new(&["Not"]),
            &reset.doc,
            ty,
            bounds,
            None,
            Some(quote! { Self }),
            quote! { #styler::#reset(self) },
        );

        quote! {
            #foreground
            #background
            #(#attributes)*
            #and
            #or
            #xor
            #dedup
            #reset
        }
    }
}

fn impl_op(
    op: &Ident,
    doc: &Doc,
    ty: &Str,
    bounds: &[TokenStream],
    rhs: Option<&Ident>,
    output: Option<TokenStream>,
    body: TokenStream,
) -> TokenStream {
    let op_snake = &op.snake;
    let arg = rhs.as_ref().map(|rhs| {
        let rhs_snake = &rhs.snake;

        quote! { #rhs_snake: #rhs }
    });
    let ret = output.as_ref().map(|output| quote! { -> #output });
    let output = output.map(|output| quote! { type Output = #output; });

    quote! {
        #doc
        impl<#(#bounds),*> #op<#rhs> for #ty {
            #output

            #doc
            fn #op_snake(self, #arg) #ret {
                #body
            }
        }
        #LINE_BREAK
    }
}
