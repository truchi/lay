use crate::generation::*;

impl Generation {
    pub fn styler() -> TokenStream {
        let styler = trait_styler();
        let styler_index = trait_styler_index();

        quote! {
            use crate::*;
            #LINE_BREAK
            #styler_index
            #LINE_BREAK
            #styler
        }
    }
}

fn trait_styler_index() -> TokenStream {
    let attributes = ATTRIBUTES
        .iter()
        .map(|attribute| (attribute, attribute.snake, attribute.get))
        .collect::<Vec<_>>();

    let style = attributes
        .iter()
        .map(|(_, attribute_lower, get)| quote! { #attribute_lower: self.#get(), });

    let index = attributes.iter().map(|(attribute, _, get)| {
        let doc = doc!("Gets `Option<{}>`.", attribute);

        quote! {
            #doc
            fn #get(&self) -> Option<#attribute>;
            #LINE_BREAK
        }
    });

    let index_mut = attributes.iter().map(|(attribute, _, get)| {
        let doc = doc!("Gets `&mut Option<{}>`.", attribute);

        quote! {
            #doc
            fn #get(&mut self) -> &mut Option<#attribute>;
            #LINE_BREAK
        }
    });

    quote! {
        /// A trait for getting `Option`al attributes on styled types.
        pub trait StylerIndex {
            #(#index)*
            #LINE_BREAK

            /// Returns a `Style`.
            fn style(&self) -> Style {
                Style {
                    #(#style)*
                }
            }
        }
        #LINE_BREAK

        /// A trait for getting `Option`al attributes on mutable styled types.
        pub trait StylerIndexMut {
            #(#index_mut)*
        }
    }
}

fn trait_styler() -> TokenStream {
    // let ground_variants = |ground: &Ground| {
    // color
    // .colors
    // .iter()
    // .map(|variant| {
    // (
    // format!("{}({}::{})", ground, color, variant),
    // if ground == &lay.foreground {
    // variant.set().snake()
    // } else {
    // variant.on_set().snake()
    // },
    // quote! { #ground(#color::#variant) },
    // )
    // })
    // .collect::<Vec<_>>()
    // };
    //
    // let attr_variants = |attr: &Attr| {
    // attr.variants
    // .iter()
    // .map(|variant| {
    // (
    // format!("{}::{}", attr, variant),
    // variant.set().snake(),
    // quote! { #attr::#variant },
    // )
    // })
    // .collect::<Vec<_>>()
    // };
    //
    // let specials = |ground: &Ground, set| {
    // let rgb = &color.rgb;
    // let ansi = &color.ansi;
    //
    // let rgb_doc = doc!("Sets `Some({}({}::{}(r, g, b)))`.", ground, color, rgb);
    // let ansi_doc = doc!("Sets `Some({}({}::{}(ansi)))`.", ground, color, ansi);
    //
    // let (set_rgb, set_ansi) = if ground == &lay.foreground {
    // (rgb.set().snake(), ansi.set().snake())
    // } else {
    // (rgb.on_set().snake(), ansi.on_set().snake())
    // };
    //
    // quote! {
    // #rgb_doc
    // fn #set_rgb(self, r: u8, g: u8, b: u8) -> Self {
    // self.#set(Some(#ground(#color::#rgb(r, g, b))))
    // }
    // #LINE_BREAK
    //
    // #ansi_doc
    // fn #set_ansi(self, ansi: u8) -> Self {
    // self.#set(Some(#ground(#color::#ansi(ansi))))
    // }
    // #LINE_BREAK
    // }
    // };
    //
    // let ground_reset = |ground: &Ground| {
    // (
    // format!("{}({}::{})", ground, color, reset_color),
    // if ground == &lay.foreground {
    // reset_color.snake()
    // } else {
    // on_reset_color.snake()
    // },
    // quote! { #ground(#color::#reset_color) },
    // )
    // };
    //
    // let attr_reset = |attr: &Attr| {
    // let reset_attr = attr.reset();
    //
    // (
    // format!("{}::{}", attr, reset_attr),
    // attr.reset().snake(),
    // quote! { #attr::#reset_attr },
    // )
    // };
    //
    // let attributes = lay
    // .attributes
    // .iter()
    // .map(|attribute| {
    // let name = attribute.name();
    // let lower = attribute.lower();
    // let get = attribute.get().snake();
    // let set = attribute.set().snake();
    // let no = attribute.no().snake();
    //
    // let (variants, specials, reset) = match attribute {
    // Attribute::Ground(ground) => (
    // ground_variants(ground),
    // Some(specials(ground, set.clone())),
    // ground_reset(ground),
    // ),
    // Attribute::Attr(attr) => (attr_variants(attr), None, attr_reset(attr)),
    // };
    //
    // (name, lower, get, set, variants, specials, reset, no)
    // })
    // .collect::<Vec<_>>();

    // ========================================
    // ========================================
    // ========================================
    // ========================================

    // let setters = attributes
    // .iter()
    // .map(|(name, lower, _, set, variants, specials, reset, no)| {
    // let doc = doc!("Sets `Option<{}>`.", name);
    // let no_doc = doc!("`None`s `Option<{}>`.", name);
    //
    // let variants = variants.iter().map(|(doc, set_variant, expr)| {
    // let doc = doc!("Sets `Some({})`.", doc);
    //
    // quote! {
    // #doc fn #set_variant(self) -> Self { self.#set(Some(#expr)) }
    // #LINE_BREAK
    // }
    // });
    //
    // let (reset_doc, reset, reset_expr) = reset;
    // let reset_doc = doc!("Sets `Some({})`.", reset_doc);
    //
    // quote! {
    // #doc
    // fn #set(self, #lower: impl Into<Option<#name>>) -> Self;
    // #LINE_BREAK
    //
    // #(#variants)*
    // #specials
    //
    // #reset_doc
    // fn #reset(self) -> Self { self.#set(Some(#reset_expr)) }
    // #LINE_BREAK
    //
    // #no_doc
    // fn #no(self) -> Self { self.#set(None) }
    // #LINE_BREAK
    // }
    // });

    let setters = ATTRIBUTES.iter().map(|attribute| {
        let snake = attribute.snake;
        let reset = attribute.reset;
        let none = attribute.none.snake;
        let set = attribute.set;

        let set_doc = doc!("Sets `Option<{}>`.", attribute);
        let none_doc = doc!("`None`s `Option<{}>`.", attribute);

        let variants = attribute.variants.iter().map(|variant| {
            let set_variant = variant.set;
            // let full = variant.full();
            let params = variant.params();
            let val = attribute.wrap_variant(variant);
            // FIXME Wrong!
            let doc = doc!("Sets `Some({})`.", val.to_string().replace(' ', ""));

            quote! {
                #doc
                fn #set_variant(self, #params) -> Self {
                    self.#set(#val)
                }
                #LINE_BREAK
            }
        });

        quote! {
            #set_doc
            fn #set(self, #snake: impl Into<Option<#attribute>>) -> Self;
            #LINE_BREAK

            #(#variants)*

            #none_doc
            fn #none(self) -> Self { self.#set(None) }
            #LINE_BREAK
        }
    });

    quote! {
        /// A trait for setting `Option`al attributes on styled types.
        pub trait Styler: StylerIndex + Sized {
            /// The resulting type of the setters.
            type Output;
            #LINE_BREAK

            #(#setters)*
        }
    }
}
