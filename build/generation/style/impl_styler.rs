use crate::generation::*;

impl Generation {
    pub fn impl_styler_index(
        &self,
        (ty, bounds, field): (&Str, &TokenStream, &Str),
    ) -> TokenStream {
        let styler_index = &self.styler.styler_index;

        let getters = self.all.iter().map(|attribute| {
            let get = &attribute.fn_get;
            let get_sign = &get.sign;
            quote! { #get_sign { #styler_index::#get(&self.#field) } }
        });

        quote! {
            impl<#bounds> #styler_index for #ty {
                #(#getters)*
            }
        }
    }

    pub fn impl_styler_index_mut(
        &self,
        (ty, bounds, field): (&Str, &TokenStream, &Str),
    ) -> TokenStream {
        let styler_index_mut = &self.styler.styler_index_mut;

        let getters = self.all.iter().map(|attribute| {
            let get = &attribute.fn_get_mut;
            let get_sign = &get.sign;
            quote! { #get_sign { #styler_index_mut::#get(&mut self.#field) } }
        });

        quote! {
            impl<#bounds> #styler_index_mut for #ty {
                #(#getters)*
            }
        }
    }

    pub fn impl_styler(&self, (ty, bounds, field): (&Str, &TokenStream, &Str)) -> TokenStream {
        let styler = &self.styler.styler;

        let setters = self.all.iter().map(|attribute| {
            let set = &attribute.fn_set;
            let set_sign = &set.sign;
            let snake = &attribute.snake;
            quote! { #set_sign { #styler::#set(self.#field, #snake); self } }
        });

        quote! {
            impl<#bounds> #styler for #ty {
                type Output = Self;

                #(#setters)*
            }
        }
    }

    pub fn impl_styler_mut(&self, (ty, bounds, field): (&Str, &TokenStream, &Str)) -> TokenStream {
        let styler_mut = &self.styler.styler_mut;

        let setters = self.all.iter().map(|attribute| {
            let set = &attribute.fn_set_mut;
            let set_sign = &set.sign;
            let snake = &attribute.snake;
            quote! { #set_sign { #styler_mut::#set(&mut self.#field, #snake) } }
        });

        quote! {
            impl<#bounds> #styler_mut for #ty {
                #(#setters)*
            }
        }
    }
}
