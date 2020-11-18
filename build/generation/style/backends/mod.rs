mod crossterm;

use crate::generation::*;

impl Generation {
    pub fn mod_style_backends(&self) -> TokenStream {
        quote! {
            // #[cfg(feature = "backend-crossterm")]
            mod crossterm;
        }
    }
}
