use crate::*;

impl Generation {
    pub fn import_no(&self) -> TokenStream {
        let exports = self
            .all
            .iter()
            .map(|attribute| (&attribute.name.pascal, &attribute.none))
            .map(|(attribute, export)| quote! { #attribute as #export });

        quote! {
            #[cfg(feature = "styler-ops")]
            pub mod no;
            #[cfg(feature = "styler-ops")]
            pub use no::{ #(#exports,)* };
        }
    }

    pub fn no(&self) -> TokenStream {
        let markers = self.all.iter().map(|attribute| {
            let doc = doc!("`None`s `{}`.", attribute);

            quote! {
                #doc
                #[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
                pub struct #attribute;
                #LINE_BREAK
            }
        });

        quote! {
            //! Attributes `None`rs.
            #LINE_BREAK

            #(#markers)*
        }
    }
}
