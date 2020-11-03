use crate::generation::*;

impl Lay {
    pub fn i(&self) -> TokenStream {
        marker(self, idoc!("Attributes `Index`ers."), |name| {
            doc!("`Index`es `{}`.", name)
        })
    }

    pub fn no(&self) -> TokenStream {
        marker(self, idoc!("Attributes `None`rs."), |name| {
            doc!("`None`s `{}`.", name)
        })
    }

    pub fn import_markers(&self) -> TokenStream {
        let grounds: Vec<_> = self
            .grounds
            .iter()
            .map(|a| (a.name, a.short, a.no))
            .collect();
        let attributes = self
            .attributes
            .iter()
            .map(|a| (a.name, a.short, a.no))
            .collect();
        let attributes = [grounds, attributes].concat();

        let attribute_as = |(attribute, r#as)| {
            let Attribute = ident!(attribute);
            let As = ident!(r#as);
            quote! { #Attribute as #As }
        };

        let i = attributes
            .iter()
            .map(|(name, short, _)| (name, short))
            .map(attribute_as);

        let no = attributes
            .iter()
            .map(|(name, _, no)| (name, no))
            .map(attribute_as);

        quote! {
            #[cfg(feature = "styler-idx")]
            pub mod i;
            #[cfg(feature = "styler-idx")]
            pub use i::{
                #(#i,)*
            };
            #LINE_BREAK

            #[cfg(feature = "styler-ops")]
            pub mod no;
            #[cfg(feature = "styler-ops")]
            pub use no::{
                #(#no,)*
            };
        }
    }
}

fn marker(lay: &Lay, mod_doc: Doc, doc_fn: fn(&str) -> Doc) -> TokenStream {
    let grounds: Vec<_> = lay.grounds.iter().map(|a| a.name).collect();
    let attributes: Vec<_> = lay.attributes.iter().map(|a| a.name).collect();
    let markers = [grounds, attributes].concat();
    let markers = markers.iter().map(|name| {
        let doc = doc_fn(name);
        let Name = ident!(name);

        quote! {
            #doc
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
            pub struct #Name;
            #LINE_BREAK
        }
    });

    quote! {
        #mod_doc
        #LINE_BREAK

        #(#markers)*
    }
}
