use crate::generation::{Lay, LINE_BREAK};

impl Lay {
    pub fn styler_index(&self) -> String {
        let grounds = vec![self.foreground.name, self.background.name];
        let attributes = self.attributes.iter().map(|attribute| attribute.name);

        let get_fn = |name: &str| {
            ident!(get = ("get_{}", name.to_lowercase()),);
            get
        };

        let style = [grounds.clone(), attributes.clone().collect()].concat();
        let style = style.iter().map(|attribute| {
            let get = get_fn(attribute);
            ident!(attribute = attribute,);
            quote::quote! { #attribute: self.#get(), }
        });

        let grounds = grounds.iter().map(|ground| {
            let doc = doc!("Gets `Option<{Ground}>`.", Ground = ground);
            let get = get_fn(ground);

            ident!(Ground = ground,);
            quote::quote! {
                #(#[doc = #doc])*
                fn #get(&self) -> Option<#Ground>;
                #LINE_BREAK
            }
        });

        let attributes = attributes.map(|attribute| {
            let doc = doc!("Gets `Option<{Attribute}>`.", Attribute = attribute);
            let get = get_fn(attribute);

            ident!(Attribute = attribute,);
            quote::quote! {
                #(#[doc = #doc])*
                fn #get(&self) -> Option<#Attribute>;
                #LINE_BREAK
            }
        });

        quote! {{
            use crate::*;

            #LINE_BREAK

            /// A trait for getting `Option`al attributes on styled types.
            pub trait StylerIndex {
                #(#grounds)*
                #LINE_BREAK
                #(#attributes)*
                #LINE_BREAK
                fn style(&self) -> Style {
                    Self { #(#style)* }
                }
            }
        }}
    }
}
