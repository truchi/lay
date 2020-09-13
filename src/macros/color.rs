macro_rules! color {
    ($(#[$inner:ident $($args:tt)*])? $Name:ident, $fmt:ident) => {
        $(#[$inner $($args)*])?
        #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
        pub struct $Name(pub ::crossterm::style::Color);

        impl ::std::convert::From<::crossterm::style::Color> for $Name {
            fn from(color: ::crossterm::style::Color) -> Self {
                Self(color)
            }
        }

        impl ::std::convert::From<$Name> for ::crossterm::style::Color {
            fn from($Name(color): $Name) -> Self {
                color
            }
        }

        impl ::std::default::Default for $Name {
            fn default() -> Self {
                Self(::crossterm::style::Color::Reset)
            }
        }

        impl ::std::fmt::Display for $Name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
                write!(f, "{}", ::crossterm::style::$fmt(self.0))
            }
        }
    };
}
