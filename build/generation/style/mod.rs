mod attribute;
mod color;
mod ground;
// mod markers;
// mod reset;
// mod style;
// mod styler;

use crate::generation::*;

impl Generation {
    pub fn mod_style_attributes(&self) -> TokenStream {
        let doc = self
            .0
            .all
            .iter()
            .map(|attribute| format!("`{}`", attribute))
            .collect::<Vec<_>>()
            .join(", ");
        let doc = idoc!("Attributes ({}).", doc);

        let imports = self.0.all.iter().map(|attribute| {
            let snake = &attribute.snake;
            quote! { mod #snake; pub use #snake::*; #LINE_BREAK }
        });

        quote! {
            #doc
            #LINE_BREAK
            #(#imports)*
        }
    }
}
