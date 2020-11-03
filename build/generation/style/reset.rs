use crate::generation::*;

impl Lay {
    pub fn reset(&self) -> TokenStream {
        let reset = self.reset;
        let color = self.colors.name;
        let reset_color = self.colors.reset;
        let grounds = self.grounds;
        let attributes = self.attributes;

        let Reset = ident!(reset);
        let Color = ident!(color);
        let ResetColor = ident!(reset_color);

        let grounds = grounds.iter().map(|ground| {
            let doc = doc!(
                "Returns `Some({Ground}({Color}::{ResetColor}))`.",
                Ground = ground.name,
                Color = color,
                ResetColor = reset_color
            );

            let Ground = ident!(ground.name);
            quote! {
                #doc
                impl From<#Reset> for Option<#Ground> {
                    #doc
                    fn from(_: #Reset) -> Self {
                        Some(#Ground(#Color::#ResetColor))
                    }
                }
                #LINE_BREAK
            }
        });
        let attributes: Vec<_> = attributes
            .iter()
            .map(|attr| {
                let attribute = attr.name;
                let reset_attribute = attr.reset;
                let doc = doc!(
                    "Returns `Some({Attribute}::{ResetAttribute})`.",
                    Attribute = attribute,
                    ResetAttribute = reset_attribute
                );

                let Attribute = ident!(attribute);
                let ResetAttribute = ident!(reset_attribute);

                quote! {
                    #doc
                    impl From<#Reset> for Option<#Attribute> {
                        #doc
                        fn from(_: #Reset) -> Self {
                            Some(#Attribute::#ResetAttribute)
                        }
                    }
                    #LINE_BREAK
                }
            })
            .collect();

        quote! {
            use crate::*;
            #LINE_BREAK

            /// `Reset`s all terminal attributes.
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
            pub struct Reset;
            #LINE_BREAK

            #(#grounds)*
            #LINE_BREAK

            #(#attributes)*
        }
    }
}
