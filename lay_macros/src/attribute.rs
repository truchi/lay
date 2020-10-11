use proc_macro2::*;
use quote::*;

pub struct Attr {
    pub name:     &'static str,
    pub variants: &'static [&'static str],
    pub reset:    &'static str,
}

impl Attr {
    pub fn def(&self) -> TokenStream {
        let Self {
            name,
            variants,
            reset,
        } = self;

        let doc = self.doc();
        let name = Ident::new(name, Span::call_site());
        let variants = variants.iter().map(|v| Ident::new(v, Span::call_site()));
        let reset = Ident::new(reset, Span::call_site());

        quote! {
            #[doc = #doc]
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
            pub enum #name {
                #(#variants,)*
                #reset
            }
        }
    }

    fn doc(&self) -> String {
        let Attr {
            name,
            variants,
            reset,
        } = self;

        let mut doc = Vec::with_capacity(5 + 2 * variants.len());
        doc.push("`");
        doc.push(name);
        doc.push("` (`");
        for variant in *variants {
            doc.push(variant);
            doc.push("`, `");
        }
        doc.push(reset);
        doc.push("`).");

        doc.concat()
    }
}

macro_rules! attr {
    ($name:ident: $($variant:ident)*) => {
        Attr {
            name:     stringify!($name),
            variants: &[$(stringify!($variant),)*],
            reset:    concat!("Reset", stringify!($name)),
        }
    }
}
