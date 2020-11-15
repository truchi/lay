use crate::generation::*;

impl Generation {
    pub fn mod_style_attributes(&self) -> TokenStream {
        let doc = self
            .0
            .all
            .iter()
            .map(|attribute| format!("`{}`", attribute))
            .collect::<Vec<_>>()
            .join(", ");
        let doc = idoc!("Attributes ({}).", doc);

        let imports = self.0.all.iter().map(|attribute| {
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
            "A `{ground}` `{color}`.

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
        }
    }

    pub fn attr(&self, attr: &Attr) -> TokenStream {
        let variants = &attr.variants;
        let reset = &attr.reset.full;

        let decl_doc = doc!(
            "`{attr}` ({variants}).

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

        quote! {
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
        }
    }
}
