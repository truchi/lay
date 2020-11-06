use crate::generation::*;

impl Generation<'_> {
    pub fn color(&self) -> TokenStream {
        let color = &self.0.color;
        let colors = &color.colors;
        let rgb = &color.rgb;
        let ansi = &color.ansi;
        let reset_color = color.reset();
        let foreground = &self.0.foreground;
        let background = &self.0.background;

        let decl_doc = doc!(
            "A `{color}` for `{foreground}` & `{background}`.

            To be used with [`{foreground}`][{fg_link}] and [`{background}`][{bg_link}]
            (a `{color}` on its own does not `Display`).

            `Default`s to `{reset_color}`.

            [{fg_link}]: struct.{foreground}.html
            [{bg_link}]: struct.{background}.html",
            color = color,
            reset_color = reset_color,
            foreground = foreground,
            background = background,
            fg_link = foreground.lower(),
            bg_link = background.lower(),
        );

        let default_doc = doc!("Returns `{}::{}`.", color, reset_color);

        quote! {
            pub use #color::*;
            #LINE_BREAK

            #decl_doc
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
            pub enum #color {
                #(#colors,)*
                #rgb(u8, u8, u8),
                #ansi(u8),
                #reset_color,
            }
            #LINE_BREAK

            #default_doc
            impl Default for #color {
                #default_doc
                fn default() -> Self {
                    Self::#reset_color
                }
            }
        }
    }
}
