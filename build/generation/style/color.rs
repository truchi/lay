use crate::generation::*;

impl Generation {
    pub fn color(&self) -> TokenStream {
        let color = &self.color;
        let colors = &color.variants;
        let foreground = &self.foreground;
        let background = &self.background;
        let reset_color = &color.reset.full;

        let decl_doc = doc!(
            "A `{color}` for `{foreground}` and `{background}`.

            To be used with `{foreground}` and `{background}`
            (a `{color}` on its own does not `Display`).  
            `Default`s to `{reset_color}`.",
            color = color,
            reset_color = reset_color,
            foreground = foreground,
            background = background,
        );

        let default_doc = doc!("Returns `{}`.", reset_color);

        let colors = colors.iter().map(|color| &color.with_types);

        quote! {
            pub use #color::*;
            #LINE_BREAK

            #decl_doc
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
            pub enum #color {
                #(#colors,)*
            }
            #LINE_BREAK

            #default_doc
            impl Default for #color {
                #default_doc
                fn default() -> Self {
                    #reset_color
                }
            }
        }
    }
}
