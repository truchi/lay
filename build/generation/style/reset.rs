use crate::generation::*;

impl Generation {
    pub fn reset() -> TokenStream {
        let doc = doc!("`{}`s all attributes.", RESET);

        let froms = ATTRIBUTES.iter().map(|attribute| {
            let reset = attribute.reset;
            let doc = doc!("Returns `Some({})`.", reset);

            quote! {
                #doc
                impl From<#RESET> for Option<#attribute> {
                    #doc
                    fn from(_: #RESET) -> Self {
                        Some(#reset)
                    }
                }
                #LINE_BREAK
            }
        });

        quote! {
            use crate::*;
            #LINE_BREAK

            #doc
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
            pub struct #RESET;
            #LINE_BREAK

            #(#froms)*
        }
    }
}
