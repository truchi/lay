mod attributes;
mod ground;

use crate::generation::*;

impl Generation<'_> {
    pub fn mod_style_attributes(&self) -> TokenStream {
        let attributes = self
            .0
            .attributes
            .iter()
            .map(Attribute::name)
            .collect::<Vec<_>>();

        let doc = attributes
            .iter()
            .map(|m| format!("`{}`", m))
            .collect::<Vec<_>>()
            .join(", ");
        let doc = idoc!("Attributes ({}).", doc);

        let mod_mod = attributes.iter().map(|m| ident!("{}", m.lower()));
        let use_mod = mod_mod.clone();

        quote! {
            #doc
            #LINE_BREAK
            #(mod #mod_mod;)*
            #LINE_BREAK
            #(pub use #use_mod::*;)*
        }
    }
}
