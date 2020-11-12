use crate::generation::*;

impl Generation {
    pub fn style() -> TokenStream {
        let attributes = ATTRIBUTES
            .iter()
            .map(|attribute| (attribute, attribute.snake, attribute.reset))
            .collect::<Vec<_>>();

        let style = attributes
            .iter()
            .map(|(attribute, snake, _)| quote! { pub #snake: Option<#attribute> });

        let none = attributes
            .iter()
            .map(|(_, snake, _)| quote! { #snake: None });

        let reset = attributes
            .iter()
            .map(|(_, snake, reset)| quote! { #snake: Some(#reset) });

        let from = attributes.iter().map(|(attribute, snake, reset)| {
            let attributes = attributes.iter().map(|(a, s, r)| {
                if s == snake {
                    quote! { #s: Some(#s) }
                } else {
                    quote! { #s: None }
                }
            });

            let doc = doc!("Returns an empty `Style` with `Some({})`.", snake);

            quote! {
                #doc
                impl From<#attribute> for Style {
                    #doc
                    fn from(#snake: #attribute) -> Self {
                        Self {
                            #(#attributes,)*
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
                #(#style,)*
            }
            #LINE_BREAK

            impl Style {
                /// A `Style` with fields set to `None`.
                pub const NONE: Self = Self {
                    #(#none,)*
                };

                /// A `Style` with fields set to their reset value.
                pub const RESET: Self = Self {
                    #(#reset,)*
                };
            }
            #LINE_BREAK

            #(#from)*
        }
    }
}
