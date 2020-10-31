use crate::generation::{Lay, LINE_BREAK};

impl Lay {
    pub fn i(&self) -> String {
        marker(self, "Attributes `Index`ers.", |name| {
            doc!("`Index`es `{}`.", name)
        })
    }

    pub fn no(&self) -> String {
        marker(self, "Attributes `None`rs.", |name| {
            doc!("`None`s `{}`.", name)
        })
    }

    pub fn import_markers(&self) -> String {
        let grounds: Vec<_> = vec![
            (
                self.foreground.name,
                self.foreground.short,
                self.foreground.no,
            ),
            (
                self.background.name,
                self.background.short,
                self.background.no,
            ),
        ];
        let attributes: Vec<_> = self
            .attributes
            .iter()
            .map(|a| (a.name, a.short, a.no))
            .collect();
        let attributes = [grounds, attributes].concat();

        let i = attributes.iter().map(|(name, short, _)| {
            ident!(Attribute = name, Short = short,);
            quote::quote! { #Attribute as #Short }
        });

        let no = attributes.iter().map(|(name, _, no)| {
            ident!(Attribute = name, No = no,);
            quote::quote! { #Attribute as #No }
        });

        quote! {{
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
        }}
    }
}

fn marker(lay: &Lay, mod_doc: &str, doc_fn: fn(&str) -> Vec<String>) -> String {
    let grounds: Vec<_> = vec![lay.foreground.name, lay.background.name];
    let attributes: Vec<_> = lay.attributes.iter().map(|a| a.name).collect();
    let markers = [grounds, attributes].concat();
    let markers = markers.iter().map(|name| {
        let doc = doc_fn(name);
        ident!(Name = name,);

        quote::quote! {
            #(#[doc = #doc])*
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
            pub struct #Name;

            #LINE_BREAK
        }
    });

    quote! {{
        #![doc = #mod_doc]

        #LINE_BREAK

        #(#markers)*
    }}
}
