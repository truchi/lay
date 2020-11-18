use crate::generation::*;

impl Generation {
    pub fn impl_styler(
        &self,
        uses: TokenStream,
        ty: &str,
        bounds: TokenStream,
        field: &str,
        impl_styler_index: bool,
        impl_styler_index_mut: bool,
        impl_styler: bool,
        impl_styler_mut: bool,
    ) -> TokenStream {
        let ty = Str::new(ty);
        let field = Str::new(field);

        let impl_styler_index = if impl_styler_index == true {
            styler_index(self, &ty, &bounds, &field)
        } else {
            quote! {}
        };
        let impl_styler_index_mut = if impl_styler_index_mut == true {
            styler_index_mut(self, &ty, &bounds, &field)
        } else {
            quote! {}
        };
        let impl_styler = if impl_styler == true {
            styler(self, &ty, &bounds, &field)
        } else {
            quote! {}
        };
        let impl_styler_mut = if impl_styler_mut == true {
            styler_mut(self, &ty, &bounds, &field)
        } else {
            quote! {}
        };

        quote! {
            use crate::*;
            #uses
            #LINE_BREAK

            #impl_styler_index     #LINE_BREAK
            #impl_styler_index_mut #LINE_BREAK
            #impl_styler           #LINE_BREAK
            #impl_styler_mut       #LINE_BREAK
        }
    }
}

pub fn styler_index(lay: &Lay, ty: &Str, bounds: &TokenStream, field: &Str) -> TokenStream {
    let styler_index = &lay.styler.styler_index;

    let getters = lay.all.iter().map(|attribute| {
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

pub fn styler_index_mut(lay: &Lay, ty: &Str, bounds: &TokenStream, field: &Str) -> TokenStream {
    let styler_index_mut = &lay.styler.styler_index_mut;

    let getters = lay.all.iter().map(|attribute| {
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

pub fn styler(lay: &Lay, ty: &Str, bounds: &TokenStream, field: &Str) -> TokenStream {
    let styler = &lay.styler.styler;

    let setters = lay.all.iter().map(|attribute| {
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

pub fn styler_mut(lay: &Lay, ty: &Str, bounds: &TokenStream, field: &Str) -> TokenStream {
    let styler_mut = &lay.styler.styler_mut;

    let setters = lay.all.iter().map(|attribute| {
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
