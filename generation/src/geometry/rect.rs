use crate::*;

impl Generation {
    pub fn geometry_rect(&self, rect: &GeometryType, ops: &[(Ident, bool, bool)]) -> TokenStream {
        let doc = &rect.doc;
        let rect_fields = self.lay.geometry.rect_fields();
        let fields = rect
            .fields
            .iter()
            .map(|field| (&field.snake, &field.pascal))
            .map(|(snake, pascal)| quote! { pub #snake: #pascal, });

        let converts_comment = centered_comment!(80, "Conversions");
        let ops_comment = centered_comment!(80, "Operations");
        let from_to_usize = from_to_usize(rect, &rect_fields);
        let _from_fields = from_fields(rect, &rect_fields);
        let ops = ops.iter().map(|op| Self::op(rect, op));

        quote! {
            use super::*; #LINE_BREAK

            #doc
            #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
            pub struct #rect { #(#fields)* } #LINE_BREAK

            #converts_comment #LINE_BREAK
            #from_to_usize
            // #from_fields

            #ops_comment #LINE_BREAK
            #(#ops)*
        }
    }
}

fn from_to_usize(rect: &GeometryType, fields: &[(&Ident, Vec<Ident>)]) -> TokenStream {
    let rect_snake = &rect.snake;

    let from_rect = fields.iter().map(|(two, ones)| {
        let two_snake = &two.snake;
        let fields = ones
            .iter()
            .map(|one| &one.snake)
            .map(|one_snake| quote! { #rect_snake.#two_snake.#one_snake.0, });

        quote! { (#(#fields)*), }
    });

    let from_usize = fields.iter().map(|(two, ones)| {
        let two_snake = &two.snake;
        let fields = ones
            .iter()
            .map(|one| (one, &one.snake))
            .map(|(one, one_snake)| quote! { #one_snake: #one(usize), });

        quote! { #two_snake: #two { #(#fields)* }, }
    });

    let from_tuple_args = rect
        .fields
        .iter()
        .map(|field| &field.snake)
        .map(|snake| quote! { #snake, });
    let from_tuple = fields.iter().map(|(two, ones)| {
        let two_snake = &two.snake;
        let fields = ones
            .iter()
            .map(|one| (one, &one.snake))
            .map(|(one, one_snake)| quote! { #one_snake: #one(#two_snake), });

        quote! { #two_snake: #two { #(#fields)* }, }
    });

    let from_double_args = fields.iter().map(|(_, ones)| {
        let args = ones
            .iter()
            .map(|one| &one.snake)
            .map(|one_snake| quote! { #one_snake, });

        quote! { (#(#args)*), }
    });
    let from_double = fields.iter().map(|(two, ones)| {
        let two_snake = &two.snake;
        let fields = ones
            .iter()
            .map(|one| (one, &one.snake))
            .map(|(one, one_snake)| quote! { #one_snake: #one(#one_snake), });

        quote! { #two_snake: #two { #(#fields)* }, }
    });

    quote! {
         impl From<#rect> for ((usize, usize), (usize, usize)) {
             fn from(#rect_snake: #rect) -> Self {
                 (#(#from_rect)*)
             }
         }
         #LINE_BREAK

        impl From<usize> for #rect {
            fn from(usize: usize) -> Self {
                Self { #(#from_usize)* }
            }
        }
        #LINE_BREAK

        impl From<(usize, usize)> for #rect {
            fn from((#(#from_tuple_args)*): (usize, usize)) -> Self {
                Self { #(#from_tuple)* }
            }
        }
        #LINE_BREAK

        impl From<((usize, usize), (usize, usize))> for #rect {
            fn from((#(#from_double_args)*): ((usize, usize), (usize, usize))) -> Self {
                Self { #(#from_double)* }
            }
        }
        #LINE_BREAK
    }
}

fn from_fields(rect: &GeometryType, fields: &[(&Ident, Vec<Ident>)]) -> TokenStream {
    let rect_snake = &rect.snake;

    let a = fields.iter().map(|(two, ones)| {
        let two_snake = &two.snake;

        let from_two = fields.iter().map(|(t, _)| {
            let t_snake = &t.snake;

            if t_snake == two_snake {
                quote! { #t_snake, }
            } else {
                quote! { #t_snake: Lol, }
            }
        });
        let from_two = {
            quote! {
                impl From<#two> for #rect {
                    fn from(#two_snake: #two) -> Self {
                        Self { #(#from_two)* }
                    }
                }
                #LINE_BREAK
            }
        };

        from_two
    });

    quote! { #(#a)* }
}
