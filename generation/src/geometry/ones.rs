use crate::*;

impl Generation {
    pub fn geometry_ones(&self, t: &GeometryType, ops: &[(Ident, bool, bool)]) -> TokenStream {
        let doc = &t.doc;

        let derefs_comment = centered_comment!(80, "Derefs");
        let converts_comment = centered_comment!(80, "Conversions");
        let ops_comment = centered_comment!(80, "Operations");
        let derefs = derefs(t);
        let from_to_usize = from_to_usize(t);
        let from_similars = from_similars(t, &self.lay.geometry.ones);
        let ops = ops.iter().map(|op| Self::op(t, op));

        quote! {
            use super::*; #LINE_BREAK

            #doc
            #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
            pub struct #t(pub usize); #LINE_BREAK

            #derefs_comment #LINE_BREAK
            #derefs

            #converts_comment #LINE_BREAK
            #from_to_usize
            #from_similars

            #ops_comment #LINE_BREAK
            #(#ops)*
        }
    }
}

fn derefs(t: &GeometryType) -> TokenStream {
    quote! {
        impl Deref for #t {
            type Target = usize;

            fn deref(&self) -> &usize {
                &self.0
            }
        }
        #LINE_BREAK

        impl DerefMut for #t {
            fn deref_mut(&mut self) -> &mut usize {
                &mut self.0
            }
        }
        #LINE_BREAK
    }
}

fn from_to_usize(t: &GeometryType) -> TokenStream {
    let snake = &t.snake;

    quote! {
        impl From<#t> for usize {
            fn from(#snake: #t) -> Self {
                #snake.0
            }
        }
        #LINE_BREAK

        impl From<usize> for #t {
            fn from(#snake: usize) -> Self {
                Self(#snake)
            }
        }
        #LINE_BREAK
    }
}

fn from_similars(t: &GeometryType, similars: &[GeometryType]) -> TokenStream {
    let snake = &t.snake;

    let similars = similars.iter().filter(|s| s != &t).map(|s| {
        quote! {
            impl From<#s> for #t {
                fn from(#s(#snake): #s) -> Self {
                    Self(#snake)
                }
            }
            #LINE_BREAK
        }
    });

    quote! { #(#similars)* }
}
