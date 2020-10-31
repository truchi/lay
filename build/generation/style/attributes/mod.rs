mod attributes;
mod ground;

pub use attributes::*;
pub use ground::*;

use crate::generation::{Lay, LINE_BREAK};

impl Lay {
    pub fn mod_style_attributes(&self) -> String {
        let grounds: Vec<_> = vec![self.foreground.name, self.background.name];
        let attributes: Vec<_> = self.attributes.iter().map(|a| a.name).collect();
        let attributes = [grounds, attributes].concat();

        let doc = attributes
            .iter()
            .map(|m| format!("`{}`", m))
            .collect::<Vec<_>>()
            .join(", ");
        let doc = doc!("Attributes ({}).", doc);

        let r#mod = attributes.iter().map(|m| ident!("{}", m.to_lowercase()));

        quote! {{
            #(#![doc = #doc])*

            #LINE_BREAK

            #(
                mod #r#mod;
                pub use #r#mod::*;
            )*
        }}
    }
}
