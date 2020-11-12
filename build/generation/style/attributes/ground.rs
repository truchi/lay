use crate::generation::*;

impl Generation {
    pub fn ground(ground: Attr) -> TokenStream {
        let color = COLOR.snake;
        let reset = ground.reset;

        let decl_doc = doc!(
            "A `{ground}` `{color}`.

            Prints the corresponding CSI to the terminal when `Display`ed.
            `Default`s to `{reset}`, user's default terminal's {ground_snake} color.",
            ground = ground,
            ground_snake = ground.snake,
            color = COLOR,
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
            pub struct #ground(pub #COLOR);
            #LINE_BREAK

            #default_doc
            impl Default for #ground {
                #default_doc
                fn default() -> Self {
                    #ground(#RESET_COLOR)
                }
            }
            #LINE_BREAK

            #from_color_doc
            impl From<#COLOR> for #ground {
                #from_color_doc
                fn from(#color: #COLOR) -> Self {
                    Self(#color)
                }
            }
            #LINE_BREAK

            #from_color_option_doc
            impl From<#COLOR> for Option<#ground> {
                #from_color_option_doc
                fn from(#color: #COLOR) -> Self {
                    Some(#ground(#color))
                }
            }
        }
    }
}
