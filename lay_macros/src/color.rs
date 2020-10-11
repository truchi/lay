use proc_macro2::*;
use quote::*;

pub struct Color {
    pub variants: &'static [&'static str],
    pub rgb:      &'static str,
    pub ansi:     &'static str,
    pub reset:    &'static str,
}

impl Color {
    pub fn def(&self) -> TokenStream {
        let Self {
            variants,
            rgb,
            ansi,
            reset,
        } = self;

        let doc = "A `Color`.";
        let variants = variants.iter().map(|v| Ident::new(v, Span::call_site()));
        let rgb = Ident::new(rgb, Span::call_site());
        let ansi = Ident::new(ansi, Span::call_site());
        let reset = Ident::new(reset, Span::call_site());

        quote! {
            #[doc = #doc]
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
            pub enum Color {
                #(#variants,)*
                #rgb(u8, u8, u8),
                #ansi(u8),
                #reset
            }
        }
    }
}

macro_rules! color {
    ($($color:ident)*) => {
        Color {
            variants: &[
                "White",
                "Black",
                $(
                    stringify!($color),
                    concat!("Dark", stringify!($color)),
                )*
            ],
            rgb:  "Rgb",
            ansi: "Ansi",
            reset: "ResetColor",
        }
    }
}
