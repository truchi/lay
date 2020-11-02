use crate::generation::*;

impl Lay {
    pub fn style(&self) -> TokenStream {
        let Color = ident!(self.colors.name);
        let ResetColor = ident!(self.colors.reset);
        let ground_tuple = |ground: Ground| {
            let Ground = ident!(ground.name);
            (ground.name, quote! { #Ground(#Color::#ResetColor) })
        };
        let attribute_tuple = |attribute: &Attribute| {
            let Attribute = ident!(attribute.name);
            let ResetAttibute = ident!(attribute.reset);
            (attribute.name, quote! { #Attribute::#ResetAttibute })
        };

        let grounds = vec![ground_tuple(self.foreground), ground_tuple(self.background)];
        let attributes = self.attributes.iter().map(attribute_tuple).collect();
        let attributes = [grounds, attributes].concat();

        let style = attributes.clone();
        let style = style.iter().map(|(name, ..)| {
            let Attribute = ident!(name);
            let attribute = ident!(name.to_lowercase());
            quote! { pub #attribute: Option<#Attribute>, }
        });

        let none = attributes.clone();
        let none = none.iter().map(|(name, ..)| {
            let attribute = ident!(name.to_lowercase());
            quote! { #attribute: None, }
        });

        let reset = attributes.clone();
        let reset = reset.iter().map(|(name, reset)| {
            let attribute = ident!(name.to_lowercase());
            quote! { #attribute: Some(#reset), }
        });

        let from = attributes.iter().map(|(name, ..)| {
            let attributes = attributes.clone();
            let attributes = attributes.iter().map(|(n, ..)| {
                let attribute = ident!(n.to_lowercase());
                if n == name {
                    quote! { #attribute: Some(#attribute), }
                } else {
                    quote! { #attribute: None, }
                }
            });

            let doc = doc!(
                "Returns an empty `Style` with `Some({attribute})`.",
                attribute = name.to_lowercase()
            );
            let Attribute = ident!(name);
            let attribute = ident!(name.to_lowercase());
            quote! {
                #doc
                impl From<#Attribute> for Style {
                    #doc
                    fn from(#attribute: #Attribute) -> Self {
                        Self {
                            #(#attributes)*
                        }
                    }
                }
                #LINE_BREAK
            }
        });

        quote! {
            use crate::*;
            #LINE_BREAK

            /// `Style`s.
            ///
            /// A straightforward implementation of `Styler`.
            ///
            /// `Display`s the corresponding CSIs to the terminal.
            ///
            /// `Default`s as an empty `Style` (all fields set to `None`).
            #[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
            pub struct Style {
                #(#style)*
            }
            #LINE_BREAK

            impl Style {
                /// A `Style` with fields set to `None`.
                pub const NONE: Self = Self {
                    #(#none)*
                };

                /// A `Style` with fields set to their reset value.
                pub const RESET: Self = Self {
                    #(#reset)*
                };
            }
            #LINE_BREAK

            #(#from)*
        }
    }
}
