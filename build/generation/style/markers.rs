use crate::generation::*;

impl Generation<'_> {
    pub fn i(&self) -> TokenStream {
        marker(self.0, idoc!("Attributes `Index`ers."), "`Index`es")
    }

    pub fn no(&self) -> TokenStream {
        marker(self.0, idoc!("Attributes `None`rs."), "`None`s")
    }

    pub fn import_markers(&self) -> TokenStream {
        let attributes = self
            .0
            .attributes
            .iter()
            .map(|attribute| {
                (
                    attribute.name().clone(),
                    attribute.short().clone(),
                    attribute.no(),
                )
            })
            .collect::<Vec<_>>();

        let make = |feature, mod_name, exports: &mut dyn Iterator<Item = _>| {
            let exports = exports.map(|(attribute, export)| quote! { #attribute as #export });

            quote! {
                #[cfg(feature = #feature)]
                pub mod #mod_name;
                #[cfg(feature = #feature)]
                pub use #mod_name::{ #(#exports,)* };
            }
        };

        let mut i = attributes.iter().map(|(name, short, _)| (name, short));
        let mut no = attributes.iter().map(|(name, _, no)| (name, no));

        let i = make("styler-idx", self.0.i.lower(), &mut i);
        let no = make("styler-ops", self.0.no.lower(), &mut no);

        quote! {
            #i
            #LINE_BREAK
            #no
        }
    }
}

fn marker(lay: &Lay, mod_doc: Doc, fn_doc: &str) -> TokenStream {
    let markers = lay.attributes.iter().map(|attribute| {
        let doc = doc!("{} `{}`.", fn_doc, attribute);

        quote! {
            #doc
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
            pub struct #attribute;
            #LINE_BREAK
        }
    });

    quote! {
        #mod_doc
        #LINE_BREAK

        #(#markers)*
    }
}
