use proc_macro2::*;
use quote::*;

pub struct Ground(pub &'static str);

impl Ground {
    pub fn def(&self) -> TokenStream {
        let doc = format!("A `{}` `Color`.", self.0);
        let name = Ident::new(self.0, Span::call_site());

        quote! {
            #[doc = #doc]
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
            pub struct #name(pub Color);
        }
    }
}
