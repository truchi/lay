macro_rules! attribute {
    (
        $(#[$inner:ident $($args:tt)*])?
        $Name:ident:
            $($variant:ident($xvariant:ident))* + $reset:ident($xreset:ident)
    ) => {
        $(#[$inner $($args)*])?
        #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
        pub enum $Name {
            $($variant,)*
            $reset
        }
        pub use $Name::*;

        impl ::std::default::Default for $Name {
            fn default() -> Self {
                Self::$reset
            }
        }

        impl ::std::fmt::Display for $Name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
                match self {
                    $($Name::$variant => write!(f, "{}", ::crossterm::style::Attribute::$xvariant),)*
                    $Name::$reset => write!(f, "{}", ::crossterm::style::Attribute::$xreset)
                }
            }
        }
    };
}
