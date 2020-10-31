use crate::generation::{Ground, Lay, LINE_BREAK};

impl Lay {
    pub fn foreground(&self) -> String {
        ground(&self, &self.foreground)
    }

    pub fn background(&self) -> String {
        ground(&self, &self.background)
    }
}

fn ground(lay: &Lay, ground: &Ground) -> String {
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

    quote! {
        Ground = ground.name,
        Color = lay.colors.name,
        ResetColor = lay.colors.reset, {
        use crate::*;

        #LINE_BREAK

        #(#[doc = #decl_doc])*
        #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
        pub struct #Ground(pub #Color);

        #LINE_BREAK

        #(#[doc = #default_doc])*
        impl Default for #Ground {
            #(#[doc = #default_doc])*
            fn default() -> Self {
                Self(#Color::#ResetColor)
            }
        }

        #LINE_BREAK

        #(#[doc = #from_color_doc])*
        impl From<#Color> for #Ground {
            #(#[doc = #from_color_doc])*
            fn from(color: #Color) -> Self {
                Self(color)
            }
        }

        #LINE_BREAK

        #(#[doc = #from_color_option_doc])*
        impl From<#Color> for Option<#Ground> {
            #(#[doc = #from_color_option_doc])*
            fn from(color: #Color) -> Self {
                Some(#Ground(color))
            }
        }
    }}
}
