use crate::generation::*;

impl Generation {
    pub fn i() -> TokenStream {
        marker(idoc!("Attributes `Index`ers."), "`Index`es")
    }

    pub fn no() -> TokenStream {
        marker(idoc!("Attributes `None`rs."), "`None`s")
    }

    pub fn import_markers() -> TokenStream {
        let attributes = ATTRIBUTES
            .iter()
            .map(|attribute| {
                (
                    attribute.name.pascal,
                    attribute.short,
                    attribute.none.pascal,
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

        let mut indexes = attributes.iter().map(|(name, short, _)| (name, short));
        let mut nones = attributes.iter().map(|(name, _, none)| (name, none));

        let indexes = make("styler-idx", INDEX, &mut indexes);
        let nones = make("styler-ops", NONE, &mut nones);

        quote! {
            #indexes
            #LINE_BREAK
            #nones
        }
    }
}

fn marker(mod_doc: Doc, fn_doc: &str) -> TokenStream {
    let markers = ATTRIBUTES.iter().map(|attribute| {
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
