use crate::*;

// TODO Styler* provided method could be overrided with better impl

impl Generation {
    pub fn style(&self) -> TokenStream {
        let Styler {
            styler_index,
            styler_index_mut,
            styler,
            styler_mut,
            ..
        } = &self.styler;

        let attributes = self
            .all
            .iter()
            .map(|attribute| (attribute, &attribute.snake, &attribute.reset.wrapped))
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

        let display = self.all.iter().map(|attribute| {
            let snake = &attribute.snake;
            let get = &attribute.fn_get;

            quote! {
                if let Some(#snake) = self.#get() { Display::fmt(&#snake, f)?; }
                #LINE_BREAK
            }
        });

        let debug = self.all.iter().map(|attribute| {
            let snake = &attribute.snake;
            let get = &attribute.fn_get;

            quote! {
                if let Some(#snake) = self.#get() { tuple.field(&#snake); }
                #LINE_BREAK
            }
        });

        let from = attributes.iter().map(|(attribute, snake, _)| {
            let attributes = attributes.iter().map(|(_, s, _)| {
                if s == snake {
                    quote! { #s: Some(#s) }
                } else {
                    quote! { #s: None }
                }
            });

            let doc = doc!(
                "Returns an empty [`Style`](crate::Style) with `Some({})`.",
                snake
            );

            quote! {
                #doc
                impl From<#attribute> for Style {
                    #doc
                    fn from(#snake: #attribute) -> Self {
                        Self { #(#attributes,)* }
                    }
                }
                #LINE_BREAK
            }
        });

        let impl_styler_index = self.all.iter().map(|attribute| {
            let snake = &attribute.snake;
            let get = &attribute.fn_get.full;
            quote! { #get { self.#snake } }
        });
        let impl_styler_index_mut = self.all.iter().map(|attribute| {
            let snake = &attribute.snake;
            let get_mut = &attribute.fn_get_mut.full;
            quote! { #get_mut { &mut self.#snake } }
        });
        let impl_styler = self.all.iter().map(|attribute| {
            let snake = &attribute.snake;
            let set = &attribute.fn_set.full;
            quote! { #set { Style { #snake: #snake.into(), ..self } } }
        });
        let impl_styler_mut = self.all.iter().map(|attribute| {
            let snake = &attribute.snake;
            let set_mut = &attribute.fn_set_mut.full;
            quote! { #set_mut { self.#snake = #snake.into(); } }
        });

        let comment_conversions = centered_comment!(80, "Conversions");
        let comment_styler = centered_comment!(80, "Styler* traits");

        quote! {
            use crate::*;
            use std::fmt::{Debug, Display, Error, Formatter};
            #LINE_BREAK

            /// [`Style`](crate::Style)s.
            ///
            /// A straightforward implementation of [`Styler`](crate::Styler).
            ///
            /// `Display`s the corresponding CSIs to the terminal.
            ///
            /// `Default`s as an empty [`Style`](crate::Style) (all fields set to `None`).
            #[derive(Copy, Clone, Eq, PartialEq, Default)]
            pub struct Style { #(#style,)* }
            #LINE_BREAK

            impl Style {
                /// A [`Style`](crate::Style) with fields set to `None`.
                pub const NONE: Self = Self { #(#none,)* };

                /// A [`Style`](crate::Style) with fields set to their reset value.
                pub const RESET: Self = Self { #(#reset,)* };
            }
            #LINE_BREAK

            /// `Display`s the corresponding CSIs to the terminal.
            impl Display for Style {
                /// `Display`s the corresponding CSIs to the terminal.
                fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                    #(#display)*
                    Ok(())
                }
            }
            #LINE_BREAK

            impl Debug for Style {
                fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                    let mut tuple = f.debug_tuple("Style");
                    #(#debug)*
                    tuple.finish()
                }
            }
            #LINE_BREAK

            #comment_conversions
            #LINE_BREAK

            #(#from)*
            #LINE_BREAK

            #comment_styler
            #LINE_BREAK

            impl #styler_index for Style { #(#impl_styler_index)* }
            #LINE_BREAK

            impl #styler_index_mut for Style { #(#impl_styler_index_mut)* }
            #LINE_BREAK

            impl #styler for Style { #(#impl_styler)* }
            #LINE_BREAK

            impl #styler_mut for Style { #(#impl_styler_mut)* }
            #LINE_BREAK
        }
    }
}
