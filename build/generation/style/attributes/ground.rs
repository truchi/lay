use crate::generation::*;

impl Generation<'_> {
    pub fn foreground(&self) -> TokenStream {
        ground(&self.0, &self.0.foreground)
    }

    pub fn background(&self) -> TokenStream {
        ground(&self.0, &self.0.background)
    }
}

fn ground(lay: &Lay, ground: &Ground) -> TokenStream {
    let color = &lay.color;
    let reset_color = lay.color.reset();

    let decl_doc = doc!(
        "A `{ground}` `{color}`.

        Prints the corresponding CSI to the terminal when `Display`ed.
        `Default`s to `{ground}({color}::{reset_color})`, user's default terminal's {ground_lower} color.",
        ground = ground,
        ground_lower = ground.lower(),
        color = color,
        reset_color = reset_color,
    );
    let default_doc = doc!("`Default`s to `{}({}::{})`.", ground, color, reset_color);
    let from_color_doc = doc!("Returns `{}({})`.", ground, color.lower());
    let from_color_option_doc = doc!("Returns `Some({}({}))`.", ground, color.lower());

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
                Self(#color::#reset_color)
            }
        }
        #LINE_BREAK

        #from_color_doc
        impl From<#color> for #ground {
            #from_color_doc
            fn from(color: #color) -> Self {
                Self(color)
            }
        }
        #LINE_BREAK

        #from_color_option_doc
        impl From<#color> for Option<#ground> {
            #from_color_option_doc
            fn from(color: #color) -> Self {
                Some(#ground(color))
            }
        }
    }
}
