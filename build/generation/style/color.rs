use crate::generation::*;

impl Generation {
    pub fn color() -> TokenStream {
        let decl_doc = doc!(
            "A `{color}` for `{foreground}` & `{background}`.

            To be used with `{foreground}` and `{background}`
            (a `{color}` on its own does not `Display`).

            `Default`s to `{reset_color}`.",
            color = COLOR,
            reset_color = RESET_COLOR,
            foreground = FOREGROUND,
            background = BACKGROUND,
        );

        let default_doc = doc!("Returns `{}`.", RESET_COLOR);

        let colors = COLORS.iter().map(|color| {
            let types = color.types();
            quote! { #color#types }
        });

        quote! {
            pub use #COLOR::*;
            #LINE_BREAK

            #decl_doc
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
            pub enum #COLOR {
                #(#colors,)*
            }
            #LINE_BREAK

            #default_doc
            impl Default for #COLOR {
                #default_doc
                fn default() -> Self {
                    #RESET_COLOR
                }
            }
        }
    }
}
