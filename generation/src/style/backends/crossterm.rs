use crate::*;

impl Generation {
    pub fn backend_crossterm(&self) -> TokenStream {
        let color = &self.color;
        let color_snake = &color.snake;
        let reset = &self.reset;

        let from_reset_doc = doc!(
            "Converts [`{reset}`](crate::{reset}) to `crossterm::style::Attribute::Reset`.",
            reset = reset
        );
        let from_color_doc = doc!(
            "Converts [`{color}`](crate::{color}) to `crossterm::style::Color`.",
            color = color
        );

        let from_color_for_color = self.color.variants.iter().map(|c| {
            let full = &c.full;

            // NOTE very dirty
            let val = if &*c.pascal == "Rgb" {
                quote! { Rgb { r, g, b } }
            } else if &*c.pascal == "Ansi" {
                quote! { AnsiValue(ansi) }
            } else if c.pascal == color.reset.pascal {
                quote! { Reset }
            } else {
                quote! { #c }
            };

            quote! { #full => Self::#val }
        });

        let from_attribute_for_attribute = self.attributes.iter().map(|attribute| {
            let snake = &attribute.snake;
            let doc = doc!(
                "Converts [`{attribute}`](crate::{attribute}) to `crossterm::style::Attribute`.",
                attribute = attribute,
            );

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

        let display_reset = self.display_reset();
        let display_foreground = self.display_ground(&self.foreground);
        let display_background = self.display_ground(&self.background);
        let display_attributes = self
            .attributes
            .iter()
            .map(|attribute| self.display_attribute(attribute));

        let comment_conversions = centered_comment!(80, "Conversions");
        let comment_displays = centered_comment!(80, "Displays");

        quote! {
            use crate::*;
            #LINE_BREAK

            #comment_conversions
            #LINE_BREAK

            #from_reset_doc
            impl From<#reset> for crossterm::style::Attribute {
                #from_reset_doc
                fn from(_: #reset) -> Self {
                    crossterm::style::Attribute::Reset
                }
            }
            #LINE_BREAK

            #from_color_doc
            impl From<#color> for crossterm::style::Color {
                #from_color_doc
                fn from(#color_snake: #color) -> Self {
                    match #color_snake {
                        #(#from_color_for_color,)*
                    }
                }
            }
            #LINE_BREAK

            #(#from_attribute_for_attribute)*

            #comment_displays
            #LINE_BREAK

            #display_reset
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
        let doc = doc!(
            "`Display`s [`{ground}`](crate::{ground}) with `crossterm`.",
            ground = ground
        );

        quote! {
            #doc
            impl Display for #ground {
                #doc
                fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                    Display::fmt(
                        &crossterm::style::#set_color(self.0.into()),
                        f,
                    )
                }
            }
            #LINE_BREAK
        }
    }

    fn display_attribute(&self, attribute: &Attr) -> TokenStream {
        let doc = doc!(
            "`Display`s [`{attribute}`](crate::{attribute}) with `crossterm`.",
            attribute = attribute
        );

        quote! {
            #doc
            impl Display for #attribute {
                #doc
                fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                    Display::fmt(
                        &crossterm::style::SetAttribute((*self).into()),
                        f,
                    )
                }
            }
            #LINE_BREAK
        }
    }

    fn display_reset(&self) -> TokenStream {
        let reset = &self.reset;
        let doc = doc!(
            "`Display`s [`{reset}`](crate::{reset}) with `crossterm`.",
            reset = reset
        );

        quote! {
            #doc
            impl Display for #reset {
                #doc
                fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                    Display::fmt(
                        &crossterm::style::SetAttribute((*self).into()),
                        f,
                    )
                }
            }
            #LINE_BREAK
        }
    }
}
