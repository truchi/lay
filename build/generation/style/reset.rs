use crate::generation::{Lay, LINE_BREAK};

impl Lay {
    pub fn reset(&self) -> String {
        let reset = self.reset;
        let color = self.colors.name;
        let reset_color = self.colors.reset;
        let grounds = vec![self.foreground.name, self.background.name];
        let attributes = self.attributes;

        ident!(Reset = reset, Color = color, ResetColor = reset_color,);

        let grounds = grounds.iter().map(|ground| {
            let doc = doc!(
                "Returns `Some({Ground}({Color}::{ResetColor}))`.",
                Ground = ground,
                Color = color,
                ResetColor = reset_color
            );

            ident!(Ground = ground,);
            quote::quote! {
                #(#[doc = #doc])*
                impl From<#Reset> for Option<#Ground> {
                    #(#[doc = #doc])*
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

                ident!(Attribute = attribute, ResetAttribute = reset_attribute,);

                quote::quote! {
                    #(#[doc = #doc])*
                    impl From<#Reset> for Option<#Attribute> {
                        #(#[doc = #doc])*
                        fn from(_: #Reset) -> Self {
                            Some(#Attribute::#ResetAttribute)
                        }
                    }

                    #LINE_BREAK
                }
            })
            .collect();

        quote! {{
            use crate::*;

            #LINE_BREAK

            /// `Reset`s all terminal attributes.
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
            pub struct Reset;

            #LINE_BREAK

            #(#grounds)*

            #LINE_BREAK

            #(#attributes)*
        }}
    }
}
