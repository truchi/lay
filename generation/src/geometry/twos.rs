use crate::*;

impl Generation {
    pub fn geometry_twos(&self, t: &GeometryType, ops: &[(Ident, bool, bool)]) -> TokenStream {
        let doc = &t.doc;
        let fields = t
            .fields
            .iter()
            .map(|field| (&field.snake, &field.pascal))
            .map(|(snake, pascal)| quote! { pub #snake: #pascal, });

        let converts_comment = centered_comment!(80, "Conversions");
        let ops_comment = centered_comment!(80, "Operations");
        let from_to_usize = from_to_usize(t);
        let from_fields = from_fields(t);
        let from_similars = from_similars(t, &self.lay.geometry.twos);
        let ops = ops.iter().map(|op| Self::op(t, op));

        quote! {
            use super::*; #LINE_BREAK

            #doc
            #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
            pub struct #t { #(#fields)* } #LINE_BREAK

            #converts_comment #LINE_BREAK
            #from_to_usize
            #from_fields
            #from_similars

            #ops_comment #LINE_BREAK
            #(#ops)*
        }
    }
}

fn from_to_usize(t: &GeometryType) -> TokenStream {
    let snake = &t.snake;

    let from_usize = t
        .fields
        .iter()
        .map(|field| (&field.snake, &field.pascal))
        .map(|(snake, pascal)| quote! { #snake: #pascal(usize), });

    let from_tuple_args = t
        .fields
        .iter()
        .map(|field| &field.snake)
        .map(|snake| quote! { #snake, });
    let from_tuple = t
        .fields
        .iter()
        .map(|field| (&field.snake, &field.pascal))
        .map(|(snake, pascal)| quote! { #snake: #pascal(#snake), });

    let from_t = t
        .fields
        .iter()
        .map(|field| &field.snake)
        .map(|fsnake| quote! { #snake.#fsnake.0, });

    quote! {
        impl From<#t> for (usize, usize) {
            fn from(#snake: #t) -> Self {
                (#(#from_t)*)
            }
        }
        #LINE_BREAK

        impl From<usize> for #t {
            fn from(usize: usize) -> Self {
                Self { #(#from_usize)* }
            }
        }
        #LINE_BREAK

        impl From<(usize, usize)> for #t {
            fn from((#(#from_tuple_args)*): (usize, usize)) -> Self {
                Self { #(#from_tuple)* }
            }
        }
        #LINE_BREAK
    }
}

fn from_fields(t: &GeometryType) -> TokenStream {
    let fields = t.fields.iter().map(|field| {
        let field_snake = &field.snake;

        let fields = t.fields.iter().map(|f| {
            let f_snake = &f.snake;

            if f_snake == field_snake {
                quote! { #f_snake, }
            } else {
                quote! { #f_snake: #f(0), }
            }
        });

        quote! {
            impl From<#field> for #t {
                fn from(#field_snake: #field) -> Self {
                    Self { #(#fields)* }
                }
            }
            #LINE_BREAK
        }
    });

    let tuple = t
        .fields
        .iter()
        .map(|field| quote! { #field, })
        .collect::<Vec<_>>();
    let tuple_snake = t
        .fields
        .iter()
        .map(|field| &field.snake)
        .map(|snake| quote! { #snake, })
        .collect::<Vec<_>>();

    quote! {
        #(#fields)*

        impl From<(#(#tuple)*)> for #t {
            fn from((#(#tuple_snake)*): (#(#tuple)*)) -> Self {
                Self { #(#tuple_snake)* }
            }
        }
        #LINE_BREAK
    }
}

fn from_similars(t: &GeometryType, similars: &[GeometryType]) -> TokenStream {
    let similars = similars.iter().filter(|s| s != &t).map(|s| {
        let s_snake = &s.snake;

        let fields = t
            .fields
            .iter()
            .zip(s.fields.iter())
            .map(|(t_field, s_field)| {
                let t_field_snake = &t_field.snake;
                let s_field_snake = &s_field.snake;

                quote! { #t_field_snake: #t_field(#s_snake.#s_field_snake.0), }
            });

        quote! {
            impl From<#s> for #t {
                fn from(#s_snake: #s) -> Self {
                    Self { #(#fields)* }
                }
            }
            #LINE_BREAK
        }
    });

    quote! { #(#similars)* }
}
