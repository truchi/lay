mod attr;
mod ground;

use crate::generation::*;

impl Generation {
    pub fn mod_style_attributes() -> TokenStream {
        let doc = ATTRIBUTES
            .iter()
            .map(|attribute| format!("`{}`", attribute))
            .collect::<Vec<_>>()
            .join(", ");
        let doc = idoc!("Attributes ({}).", doc);

        let imports = ATTRIBUTES.iter().map(|attribute| {
            let snake = attribute.snake;
            quote! { mod #snake; pub use #snake::*; #LINE_BREAK }
        });

        quote! {
            #doc
            #LINE_BREAK
            #(#imports)*
        }
    }
}
