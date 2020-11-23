use crate::*;

impl Generation {
    pub fn mod_style_attributes(&self) -> TokenStream {
        let doc = self
            .all
            .iter()
            .map(|attribute| format!("[`{a}`](crate::{a})", a = attribute))
            .collect::<Vec<_>>()
            .join(", ");
        let doc = idoc!("Attributes ({}).", doc);

        let imports = self.all.iter().map(|attribute| {
            let snake = &attribute.snake;
            quote! { mod #snake; pub use #snake::*; #LINE_BREAK }
        });

        quote! {
            #doc
            #LINE_BREAK
            #(#imports)*
        }
    }

    pub fn ground(&self, ground: &Attr) -> TokenStream {
        let color = &self.color;
        let color_snake = &color.snake;
        let reset = &ground.reset.wrapped;

        let decl_doc = doc!(
            "A [`{ground}`](crate::{ground}) [`{color}`](crate::{color}).

            Prints the corresponding CSI to the terminal when `Display`ed.

            `Default`s to `{reset}`, user's default terminal's {ground_snake} color.",
            ground = ground,
            ground_snake = ground.snake,
            color = color,
            reset = reset,
        );
        let default_doc = doc!("`Default`s to `{}`.", reset);
        let from_color_doc = doc!("Returns `{}({})`.", ground, color);
        let from_color_option_doc = doc!("Returns `Some({}({}))`.", ground, color);

        let styler_index = impl_styler_index(self, ground);
        let styler = impl_styler(self, ground);

        quote! {
            use crate::*;
            #LINE_BREAK

            #decl_doc
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
            pub struct #ground(pub #color);
            #LINE_BREAK

            #default_doc
            impl Default for #ground {
                #default_doc
                fn default() -> Self {
                    #reset
                }
            }
            #LINE_BREAK

            #from_color_doc
            impl From<#color> for #ground {
                #from_color_doc
                fn from(#color_snake: #color) -> Self {
                    #ground(#color_snake)
                }
            }
            #LINE_BREAK

            #from_color_option_doc
            impl From<#color> for Option<#ground> {
                #from_color_option_doc
                fn from(#color_snake: #color) -> Self {
                    Some(#ground(#color_snake))
                }
            }
            #LINE_BREAK

            #styler_index #LINE_BREAK
            #styler
        }
    }

    pub fn attr(&self, attr: &Attr) -> TokenStream {
        let variants = &attr.variants;
        let reset = &attr.reset.full;

        let decl_doc = doc!(
            "[`{attr}`](crate::{attr}) ({variants}).

            Prints the corresponding CSI to the terminal when `Display`ed.

            `Default`s to `{reset}`, the unsetting CSI.",
            attr = attr,
            reset = reset,
            variants = variants
                .iter()
                .map(|variant| format!("`{}`", variant))
                .collect::<Vec<_>>()
                .join(", "),
        );

        let default_doc = doc!("Returns `{}`.", reset);

        let styler_index = impl_styler_index(self, attr);
        let styler = impl_styler(self, attr);

        quote! {
            use crate::*;
            pub use #attr::*;
            #LINE_BREAK

            #decl_doc
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
            pub enum #attr {
                #(#variants,)*
            }
            #LINE_BREAK

            #default_doc
            impl Default for #attr {
                #default_doc
                fn default() -> Self {
                    #reset
                }
            }
            #LINE_BREAK

            #styler_index #LINE_BREAK
            #styler
        }
    }
}

// ======= //
// Helpers //
// ======= //

fn impl_styler_index(lay: &Lay, attribute: &Attr) -> TokenStream {
    let styler_index = &lay.styler.styler_index;

    let getters = lay.all.iter().map(|getter| {
        let get = &getter.fn_get.sign;
        let (doc, val) = if attribute == getter {
            (doc!("Returns `Some(self)`."), quote! { Some(*self) })
        } else {
            (doc!("Returns `None`."), quote! { None })
        };

        quote! { #doc #get { #val } }
    });

    quote! {
        impl #styler_index for #attribute {
            #(#getters)*
        }
    }
}

fn impl_styler(lay: &Lay, attribute: &Attr) -> TokenStream {
    let attribute_snake = &attribute.snake;
    let styler = &lay.styler.styler;

    let setters = lay.all.iter().map(|setter| {
        let snake = &setter.snake;
        let set = &setter.fn_set.sign;

        let (doc, fields) = if attribute == setter {
            (
                doc!("Returns a [`Style`](crate::Style) with `{}`.", snake),
                quote! { #snake: #snake.into(), },
            )
        } else {
            (
                doc!(
                    "Returns a [`Style`](crate::Style) with `{}` (self) and `{}`.",
                    attribute_snake,
                    snake
                ),
                quote! {
                    #attribute_snake: Some(self),
                    #snake: #snake.into(),
                },
            )
        };

        quote! { #doc #set { Style { #fields ..Style::NONE } } }
    });

    quote! {
        impl #styler for #attribute {
            type Output = Style;
            #(#setters)*
        }
    }
}
