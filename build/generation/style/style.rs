use crate::generation::*;

impl Generation<'_> {
    pub fn style(&self) -> TokenStream {
        let color = &self.0.color;
        let reset_color = self.0.color.reset();
        let attributes = self
            .0
            .attributes
            .iter()
            .map(|attribute| {
                let name = attribute.name();
                let name_lower = name.lower();

                match attribute {
                    Attribute::Ground(_) =>
                        (name, name_lower, quote! { #name(#color::#reset_color) }),
                    Attribute::Attr(_) => {
                        let reset_attr = name.reset();

                        (name, name_lower, quote! { #name::#reset_attr })
                    }
                }
            })
            .collect::<Vec<_>>();

        let style = attributes.clone();
        let style = style
            .iter()
            .map(|(name, name_lower, ..)| quote! { pub #name_lower: Option<#name>, });

        let none = attributes.clone();
        let none = none
            .iter()
            .map(|(_, name_lower, ..)| quote! { #name_lower: None, });

        let reset = attributes.clone();
        let reset = reset
            .iter()
            .map(|(_, name_lower, reset)| quote! { #name_lower: Some(#reset), });

        let from = attributes.iter().map(|(name, name_lower, ..)| {
            let attributes = attributes.clone();
            let attributes = attributes.iter().map(|(n, l, ..)| {
                if l == name_lower {
                    quote! { #l: Some(#l), }
                } else {
                    quote! { #l: None, }
                }
            });

            let doc = doc!("Returns an empty `Style` with `Some({})`.", name_lower);

            quote! {
                #doc
                impl From<#name> for Style {
                    #doc
                    fn from(#name_lower: #name) -> Self {
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
