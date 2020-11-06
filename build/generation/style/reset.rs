use crate::generation::*;

impl Generation<'_> {
    pub fn reset(&self) -> TokenStream {
        let reset = &self.0.reset;
        let color = &self.0.color;
        let reset_color = self.0.color.reset();
        let doc = doc!("`{}`s all attributes.", reset);

        let froms = self
            .0
            .attributes
            .iter()
            .map(|attribute| {
                let name = attribute.name();

                match attribute {
                    Attribute::Ground(_) => (
                        name,
                        format!("{}({}::{})", name, color, reset_color),
                        quote! { #name(#color::#reset_color) },
                    ),
                    Attribute::Attr(_) => {
                        let reset_attr = name.reset();

                        (
                            name,
                            format!("{}::{}", name, reset_attr),
                            quote! { #name::#reset_attr },
                        )
                    }
                }
            })
            .map(|(attribute, doc, reset_attribute)| {
                let doc = doc!("Returns `Some({})`.", doc);

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
