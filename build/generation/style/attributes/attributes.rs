use crate::generation::*;

impl Generation<'_> {
    pub fn attributes(&self) -> Vec<(String, TokenStream)> {
        self.0
            .attrs
            .iter()
            .map(|attr| (attr.lower().to_string(), attribute(attr)))
            .collect()
    }
}

fn attribute(attr: &Attr) -> TokenStream {
    let variants = &attr.variants;
    let reset_attr = attr.reset();

    let decl_doc = doc!(
        "`{attr}` ({variants}).

        Prints the corresponding CSI to the terminal when `Display`ed.
        `Default`s to `{reset_attr}`, the unsetting CSI.",
        attr = attr,
        reset_attr = reset_attr,
        variants = variants
            .iter()
            .chain([reset_attr.clone()].iter())
            .map(|variant| format!("`{}`", variant))
            .collect::<Vec<_>>()
            .join(", "),
    );

    let default_doc = doc!("Returns `{}::{}`.", attr, reset_attr);

    quote! {
        pub use #attr::*;
        #LINE_BREAK

        #decl_doc
        #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
        pub enum #attr {
            #(#variants,)*
            #reset_attr
        }
        #LINE_BREAK

        #default_doc
        impl Default for #attr {
            #default_doc
            fn default() -> Self {
                Self::#reset_attr
            }
        }
    }
}
