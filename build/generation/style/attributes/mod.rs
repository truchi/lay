mod attributes;
mod ground;

pub use attributes::*;
pub use ground::*;

use crate::generation::*;

impl Lay {
    pub fn mod_style_attributes(&self) -> TokenStream {
        let grounds: Vec<_> = self.grounds.iter().map(|a| a.name).collect();
        let attributes = self.attributes.iter().map(|a| a.name).collect();
        let attributes = [grounds, attributes].concat();

        let doc = attributes
            .iter()
            .map(|m| format!("`{}`", m))
            .collect::<Vec<_>>()
            .join(", ");
        let doc = idoc!("Attributes ({}).", doc);

        let r#mod = attributes.iter().map(|m| ident!("{}", m.to_lowercase()));

        quote! {
            #doc
            #LINE_BREAK

            #(mod #r#mod; pub use #r#mod::*;)*
        }
    }
}
