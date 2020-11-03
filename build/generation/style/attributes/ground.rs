use crate::generation::*;

impl Lay {
    pub fn foreground(&self) -> TokenStream {
        ground(&self, &self.grounds[0])
    }

    pub fn background(&self) -> TokenStream {
        ground(&self, &self.grounds[1])
    }
}

fn ground(lay: &Lay, ground: &Ground) -> TokenStream {
    let decl_doc = doc!(
        "A `{Ground}` `{Color}`.

        Prints the corresponding CSI to the terminal when `Display`ed.
        `Default`s to `{Ground}({Color}::{ResetColor})`, user's default terminal's {ground} color.",
        Ground = ground.name,
        ground = ground.name.to_lowercase(),
        Color = lay.colors.name,
        ResetColor = lay.colors.reset,
    );

    let default_doc = doc!(
        "`Default`s to `{Ground}({Color}::{ResetColor})`.",
        Ground = ground.name,
        Color = lay.colors.name,
        ResetColor = lay.colors.reset,
    );

    let from_color_doc = doc!("Returns `{Ground}(color)`.", Ground = ground.name);

    let from_color_option_doc = doc!("Returns `Some({Ground}(color))`.", Ground = ground.name);

    let Ground = ident!(ground.name);
    let Color = ident!(lay.colors.name);
    let ResetColor = ident!(lay.colors.reset);

    quote! {
        use crate::*;
        #LINE_BREAK

        #decl_doc
        #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
        pub struct #Ground(pub #Color);
        #LINE_BREAK

        #default_doc
        impl Default for #Ground {
            #default_doc
            fn default() -> Self {
                Self(#Color::#ResetColor)
            }
        }
        #LINE_BREAK

        #from_color_doc
        impl From<#Color> for #Ground {
            #from_color_doc
            fn from(color: #Color) -> Self {
                Self(color)
            }
        }
        #LINE_BREAK

        #from_color_option_doc
        impl From<#Color> for Option<#Ground> {
            #from_color_option_doc
            fn from(color: #Color) -> Self {
                Some(#Ground(color))
            }
        }
    }
}
