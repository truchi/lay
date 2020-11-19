use crate::*;

impl Generation {
    pub fn reset(&self) -> TokenStream {
        let reset = &self.reset;
        let doc = doc!("`{}`s all attributes.", reset);

        let froms = self.all.iter().map(|attribute| {
            let reset_attribute = &attribute.reset.wrapped;
            let doc = doc!("Returns `Some({})`.", reset_attribute);

            quote! {
                #doc
                impl From<#reset> for Option<#attribute> {
                    #doc
                    fn from(_: #reset) -> Self {
                        Some(#reset_attribute)
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
            pub struct #reset;
            #LINE_BREAK

            #(#froms)*
        }
    }
}
