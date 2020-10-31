use crate::generation::{Lay, LINE_BREAK};

impl Lay {
    #[allow(non_snake_case)]
    pub fn color(&self) -> String {
        let Colors = [self.colors.colors, self.colors.lights, self.colors.darks].concat();
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
            Foreground = self.foreground.name,
            Background = self.background.name,
            foreground = self.foreground.name.to_lowercase(),
            background = self.background.name.to_lowercase(),
        );

        let default_doc = doc!(
            "Returns `{Color}::{ResetColor}`.",
            Color = self.colors.name,
            ResetColor = self.colors.reset
        );

        quote! {
            Color = self.colors.name,
            Rgb = self.colors.rgb,
            Ansi = self.colors.ansi,
            ResetColor = self.colors.reset, {
            pub use #Color::*;

            #LINE_BREAK

            #(#[doc = #decl_doc])*
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
            pub enum #Color {
                #(#Colors,)*
                #Rgb(u8, u8, u8),
                #Ansi(u8),
                #ResetColor,
            }

            #LINE_BREAK

            #(#[doc = #default_doc])*
            impl Default for Color {
                #(#[doc = #default_doc])*
                fn default() -> Self {
                    Self::ResetColor
                }
            }
        }}
    }
}
