macro_rules! derefs {
    ($self:ident $(
        $Struct:ident {
            deref $Target:ty { $deref:expr }
            $tokens:ident { $to_tokens:expr }
            $f:ident { $fmt:expr }
        }
    )*) => {$(
        impl ::std::ops::Deref for $Struct {
            type Target = $Target;
            fn deref(&$self) -> &Self::Target { $deref }
        }

        impl ::quote::ToTokens for $Struct {
            fn to_tokens(&$self, $tokens: &mut ::proc_macro2::TokenStream) { $to_tokens }
        }

        impl ::std::fmt::Display for $Struct {
            fn fmt(&$self, $f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> { $fmt }
        }
    )*};
}

#[macro_use]
mod doc;
mod ident;
mod my_str;
mod write;

pub use doc::*;
pub use ident::*;
pub use my_str::*;
pub use write::*;
