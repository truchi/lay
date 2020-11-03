use crate::generation::*;

impl Lay {
    pub fn color(&self) -> TokenStream {
        let Colors = self.colors.colors;
        let Colors = Colors.iter().map(|color| ident!("{}", color));

        let decl_doc = doc!(
            "A `{Color}` for `{Foreground}` & `{Background}`.

            To be used with [`{Foreground}`][{foreground}] and [`{Background}`][{background}] (a
            `{Color}` on its own does not `impl Display`).

            Defaults to `{ResetColor}`.

            [{foreground}]: struct.{Foreground}.html
            [{background}]: struct.{Background}.html",
            Color = self.colors.name,
            ResetColor = self.colors.reset,
            Foreground = self.grounds[0].name,
            Background = self.grounds[1].name,
            foreground = self.grounds[0].name.to_lowercase(),
            background = self.grounds[1].name.to_lowercase(),
        );

        let default_doc = doc!(
            "Returns `{Color}::{ResetColor}`.",
            Color = self.colors.name,
            ResetColor = self.colors.reset
        );

        let Color = ident!(self.colors.name);
        let Rgb = ident!(self.colors.rgb);
        let Ansi = ident!(self.colors.ansi);
        let ResetColor = ident!(self.colors.reset);

        quote! {
            pub use #Color::*;
            #LINE_BREAK

            #decl_doc
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
            pub enum #Color {
                #(#Colors,)*
                #Rgb(u8, u8, u8),
                #Ansi(u8),
                #ResetColor,
            }
            #LINE_BREAK

            #default_doc
            impl Default for Color {
                #default_doc
                fn default() -> Self {
                    Self::ResetColor
                }
            }
        }
    }
}
