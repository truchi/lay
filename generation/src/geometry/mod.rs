use crate::*;

impl Generation {
    pub fn generate_geometry(&self) {
        let dir = &self.geometry;

        let ops = vec![
            "Add", "Sub", "Mul", "Div", "Rem", "Shl", "Shr", "BitAnd", "BitOr", "BitXor",
        ];
        let len = ops.len();
        let unary = (Ident::new(&["Not"]), false, false);
        let mut binary = ops.iter().map(|op| (Ident::new(&[op]), false, true));
        let mut binary_mut = ops
            .iter()
            .map(|op| (Ident::new(&[op, "Assign"]), true, true));

        let mut ops = Vec::with_capacity(1 + len * 2);
        ops.push(unary);

        for _ in 0..len {
            let op = binary.next().unwrap();
            let op_mut = binary_mut.next().unwrap();
            ops.push(op);
            ops.push(op_mut);
        }

        for t in &self.lay.geometry.all {
            write(dir, &format!("{}.rs", t.snake), Self::ops(t, &ops))
        }

        let imports = ops.iter().map(|(op, ..)| quote! { use std::ops::#op; });
        write(
            dir,
            "mod.rs",
            concat(&[quote! { #(#imports)* }, self.mod_geometry()]),
        );
    }

    fn mod_geometry(&self) -> TokenStream {
        let imports = &self
            .lay
            .geometry
            .all
            .iter()
            .map(|t| &t.snake)
            .map(|snake| {
                quote! {
                    mod #snake;
                    pub use #snake::*;
                    #LINE_BREAK
                }
            })
            .collect::<Vec<_>>();

        quote! { use crate::*; #(#imports)* }
    }

    fn ops(t: &GeometryType, ops: &[(Ident, bool, bool)]) -> TokenStream {
        let ops = ops.iter().map(|op| Self::op(t, op));

        quote! {
            use super::*;
            #LINE_BREAK

            #(#ops)*
        }
    }

    fn op(t: &GeometryType, (op, mutable, rhs): &(Ident, bool, bool)) -> TokenStream {
        let op_snake = &op.snake;

        let fields = t.fields.iter().map(|field| {
            let field_snake = &field.snake;
            let rhs = if *rhs {
                Some(quote! { rhs.#field_snake })
            } else {
                None
            };

            let field = quote! { self.#field_snake.#op_snake(#rhs) };
            if *mutable {
                field
            } else {
                quote! { #field_snake: #field }
            }
        });

        let output = if *mutable {
            None
        } else {
            Some(quote! { type Output = Self; })
        };
        let ret = if *mutable {
            None
        } else {
            Some(quote! { -> Self })
        };
        let mutability = if *mutable {
            Some(quote! { &mut })
        } else {
            None
        };
        let rhs = if *rhs {
            Some(quote! { , rhs: Self })
        } else {
            None
        };
        let expr = if *mutable {
            quote! { #(#fields;)* }
        } else {
            quote! { Self { #(#fields,)* } }
        };

        quote! {
            impl #op for #t {
                #output

                fn #op_snake(#mutability self #rhs) #ret {
                    #expr
                }
            }
            #LINE_BREAK
        }
    }
}
