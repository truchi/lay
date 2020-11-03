use crate::generation::*;

impl Lay {
    pub fn styler(&self) -> TokenStream {
        let styler = self.trait_styler();
        let styler_index = self.trait_styler_index();

        quote! {
            use crate::*; #LINE_BREAK
            #styler_index #LINE_BREAK
            #styler
        }
    }

    fn trait_styler_index(&self) -> TokenStream {
        let grounds: Vec<_> = self.grounds.iter().map(|ground| ground.name).collect();
        let attributes = self
            .attributes
            .iter()
            .map(|attribute| attribute.name)
            .collect();
        let attributes = [grounds, attributes].concat();

        let style = attributes.clone();
        let style = style.iter().map(|attribute| {
            let get = ident!("get_{}", attribute.to_lowercase());
            let attribute = ident!(attribute.to_lowercase());
            quote! { #attribute: self.#get(), }
        });

        let index = attributes.iter().map(|attribute| {
            let doc = doc!("Gets `Option<{Attribute}>`.", Attribute = attribute);
            let get = ident!("get_{}", attribute.to_lowercase());

            let Attribute = ident!(attribute);
            quote! { #doc fn #get(&self) -> Option<#Attribute>; #LINE_BREAK }
        });

        let index_mut = attributes.clone();
        let index_mut = index_mut.iter().map(|attribute| {
            let doc = doc!("Gets `&mut Option<{Attribute}>`.", Attribute = attribute);
            let get = ident!("get_{}", attribute.to_lowercase());

            let Attribute = ident!(attribute);
            quote! { #doc fn #get(&mut self) -> &mut Option<#Attribute>; #LINE_BREAK }
        });

        quote! {
            /// A trait for getting `Option`al attributes on styled types.
            pub trait StylerIndex {
                #(#index)*
                #LINE_BREAK

                /// Returns a `Style`.
                fn style(&self) -> Style {
                    Style { #(#style)* }
                }
            }
            #LINE_BREAK

            /// A trait for getting `Option`al attributes on mutable styled types.
            pub trait StylerIndexMut {
                #(#index_mut)*
            }
        }
    }

    fn trait_styler(&self) -> TokenStream {
        let ground_tuple = |ground: Ground| {
            // ident!(Ground = ground.name,);
            // (ground.name, quote! { #Ground(#Color::#ResetColor) })
        };
        let attribute_tuple = |attribute: &Attribute| {
            // ident!(Attribute = attribute.name, ResetAttibute =
            // attribute.reset,); (attribute.name, quote! {
            // #Attribute::#ResetAttibute })
        };

        let grounds: Vec<_> = self.grounds.iter().map(|ground| ground.name).collect();
        let attributes = self
            .attributes
            .iter()
            .map(|attribute| attribute.name)
            .collect();
        let attributes = [grounds, attributes].concat();

        let setters = attributes.iter().map(|attribute| {
            let doc = doc!("Sets `Option<{Attribute}>`.", Attribute = attribute);
            let no_doc = doc!("`None`s `Option<{Attribute}>`.", Attribute = attribute);
            let Attribute = ident!(attribute);
            let attribute = ident!(attribute.to_lowercase());
            let no_attribute = ident!("no_{}", attribute);
            quote! {
                #doc
                fn #attribute(self, #attribute: impl Into<Option<#Attribute>>) -> Self;
                #LINE_BREAK

                #no_doc
                fn #no_attribute(self) -> Self {
                    self.#attribute(None)
                }
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
}
