use crate::generation::*;

impl Generation {
    pub fn backend_crossterm(&self) -> TokenStream {
        let from_color_for_color = self.color.variants.iter().map(|color| {
            let full = &color.full;

            // NOTE very dirty
            let val = if &*color.pascal == "Rgb" {
                quote! { Rgb { r, g, b } }
            } else if &*color.pascal == "Ansi" {
                quote! { AnsiValue(ansi) }
            } else if &*color.pascal == "ResetColor" {
                quote! { Reset }
            } else {
                quote! { #color }
            };

            quote! { #full => Self::#val }
        });

        let from_attribute_for_attribute = self.attributes.iter().map(|attribute| {
            let snake = &attribute.snake;
            let doc = doc!("Converts `{}` to `crossterm::style::Attribute`.", attribute,);

            // NOTE very dirty
            let variants = attribute.variants.iter().map(|variant| {
                let val = match &*variant.pascal {
                    "ResetWeight" => quote! { NormalIntensity },
                    "ResetSlant" => quote! { NoItalic },
                    "ResetUnderline" => quote! { NoUnderline },
                    "ResetStrike" => quote! { NotCrossedOut },
                    "ResetOverline" => quote! { NotOverLined },
                    "ResetInvert" => quote! { NoReverse },
                    "ResetBlink" => quote! { NoBlink },
                    "ResetBorder" => quote! { NotFramedOrEncircled },
                    "Light" => quote! { Dim },
                    "Striked" => quote! { CrossedOut },
                    "Overlined" => quote! { OverLined },
                    "Inverted" => quote! { Reverse },
                    "Slow" => quote! { SlowBlink },
                    "Fast" => quote! { RapidBlink },
                    "Circle" => quote! { Encircled },
                    "Frame" => quote! { Framed },
                    _ => quote! { #variant },
                };

                quote! { #attribute::#variant => Self::#val }
            });

            quote! {
                #doc
                impl From<#attribute> for crossterm::style::Attribute {
                    #doc
                    fn from(#snake: #attribute) -> Self {
                        match #snake {
                            #(#variants,)*
                        }
                    }
                }
                #LINE_BREAK
            }
        });

        let display_foreground = self.display_ground(&self.foreground);
        let display_background = self.display_ground(&self.background);
        let display_attributes = self
            .attributes
            .iter()
            .map(|attribute| self.display_attribute(attribute));

        let comment_conversions = "Conversions";
        let comment_conversions = comment!(
            "{sep} //
            {sep} //
            {comment_conversions}{spaces} //
            {sep} //
            {sep} //",
            sep = "=".repeat(74),
            comment_conversions = comment_conversions,
            spaces = " ".repeat(74 - comment_conversions.len())
        );
        let comment_displays = "Displays";
        let comment_displays = comment!(
            "{sep} //
            {sep} //
            {comment_displays}{spaces} //
            {sep} //
            {sep} //",
            sep = "=".repeat(74),
            comment_displays = comment_displays,
            spaces = " ".repeat(74 - comment_displays.len())
        );

        quote! {
            use crate::*;
            use std::fmt::{Display, Error, Formatter};
            #LINE_BREAK

            #comment_conversions
            #LINE_BREAK

            /// Converts to `crossterm::style::Color`.
            impl From<Color> for crossterm::style::Color {
                /// Converts to `crossterm::style::Color`.
                fn from(color: Color) -> Self {
                    match color {
                        #(#from_color_for_color,)*
                    }
                }
            }
            #LINE_BREAK

            #(#from_attribute_for_attribute)*

            #comment_displays
            #LINE_BREAK

            #display_foreground
            #display_background
            #(#display_attributes)*
        }
    }

    // ======= //
    // Helpers //
    // ======= //

    fn display_ground(&self, ground: &Attr) -> TokenStream {
        let set_color = quote::format_ident!("Set{}Color", &*ground.pascal);
        let doc = doc!("`Display`s `{}` with `crossterm`.", ground);

        quote! {
            #doc
            impl Display for #ground {
                #doc
                fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                    <crossterm::style::#set_color as Display>::fmt(
                        &crossterm::style::#set_color(self.0.into()),
                        f,
                    )
                }
            }
            #LINE_BREAK
        }
    }

    fn display_attribute(&self, attribute: &Attr) -> TokenStream {
        let doc = doc!("`Display`s `{}` with `crossterm`.", attribute);

        quote! {
            #doc
            impl Display for #attribute {
                #doc
                fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                    <crossterm::style::SetAttribute as Display>::fmt(
                        &crossterm::style::SetAttribute((*self).into()),
                        f,
                    )
                }
            }
            #LINE_BREAK
        }
    }
}
