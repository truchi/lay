use crate::generation::*;

impl Generation {
    pub fn attr(&self, attr: &Attr) -> TokenStream {
        let variants = &attr.variants;
        let reset = &attr.reset.full;

        let decl_doc = doc!(
            "`{attr}` ({variants}).

            Prints the corresponding CSI to the terminal when `Display`ed.  
            `Default`s to `{reset}`, the unsetting CSI.",
            attr = attr,
            reset = reset,
            variants = variants
                .iter()
                .map(|variant| format!("`{}`", variant))
                .collect::<Vec<_>>()
                .join(", "),
        );

        let default_doc = doc!("Returns `{}`.", reset);

        quote! {
            pub use #attr::*;
            #LINE_BREAK

            #decl_doc
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
            pub enum #attr {
                #(#variants,)*
            }
            #LINE_BREAK

            #default_doc
            impl Default for #attr {
                #default_doc
                fn default() -> Self {
                    #reset
                }
            }
        }
    }
}
