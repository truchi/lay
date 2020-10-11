#[macro_use]
mod attribute;
#[macro_use]
mod color;
#[macro_use]
mod ground;

use attribute::*;
use color::*;
use ground::*;

use quote::*;
use syn::{parse::*, *};

const COLOR: Color = color!(Red Green Yellow Blue Magenta Cyan Grey);
const FOREGROUND: Ground = Ground("Foreground");
const BACKGROUND: Ground = Ground("Background");
const ATTRS: [Attr; 7] = [
    attr!(Weight: Bold Light),
    attr!(Slant: Italic),
    attr!(Invert: Inverted),
    attr!(Strike: Striked),
    attr!(Underline: Underlined),
    attr!(Overline: Overlined),
    attr!(Border: Framed Circled),
];

#[derive(Debug)]
struct Doc(Vec<Attribute>);
impl Parse for Doc {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self(input.call(Attribute::parse_outer)?))
    }
}

#[proc_macro]
pub fn test(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as Doc);
    dbg!(item);
    // dbg!(item.to_string());
    proc_macro::TokenStream::new()
}

#[proc_macro_attribute]
pub fn color(
    _attr: proc_macro::TokenStream,
    _item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // dbg!(attr);
    // dbg!(item);
    // dbg!(item.to_string());
    proc_macro::TokenStream::new()
}

#[proc_macro]
pub fn make_color(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    COLOR.def().into()
}

#[proc_macro]
pub fn make_foreground(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    FOREGROUND.def().into()
}

#[proc_macro]
pub fn make_background(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    BACKGROUND.def().into()
}

#[proc_macro]
pub fn make_style(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let foreground = quote! { foreground: Option<Foreground> };

    (quote! {
        pub struct Style {
            #foreground
        }
    })
    .into()
}

#[proc_macro]
pub fn make_attrs(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let attrs = ATTRS.iter().map(Attr::def);

    (quote! {
        #(#attrs)*
    })
    .into()
}
