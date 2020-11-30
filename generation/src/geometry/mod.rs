mod one_d;
mod two_d;

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

        for one in &self.lay.geometry.ones {
            write(
                dir,
                &format!("{}.rs", one.snake),
                self.geometry_1d(one, &ops),
            )
        }

        for two in &self.lay.geometry.twos {
            write(
                dir,
                &format!("{}.rs", two.snake),
                self.geometry_2d(two, &ops),
            )
        }

        let imports = ops.iter().map(|(op, ..)| quote! { use std::ops::#op; });
        write(
            dir,
            "mod.rs",
            concat(&[
                quote! {
                    use std::ops::{Deref, DerefMut};
                    #(#imports)*
                },
                self.mod_geometry(),
            ]),
        );
    }

    fn mod_geometry(&self) -> TokenStream {
        let geometry = &self.lay.geometry;

        let types = [geometry.ones.clone(), geometry.twos.clone()].concat();
        let types = types.iter().map(|t| &t.snake).map(|snake| {
            quote! {
                mod #snake;
                pub use #snake::*;
                #LINE_BREAK
            }
        });

        quote! {
            #(#types)*
            // #rect
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
