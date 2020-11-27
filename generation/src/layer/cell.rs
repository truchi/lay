use crate::*;

impl Generation {
    pub fn cell_impls(&self) -> TokenStream {
        let Styler {
            styler_index,
            styler,
            styler_mut,
            ..
        } = &self.styler;

        let impl_styler_index = self.all.iter().map(|attribute| {
            let get = &attribute.fn_get;
            let full = &get.full;
            quote! { #full { self.as_ref().map(#styler_index::#get).flatten() } }
        });
        let impl_styler = self.all.iter().map(|attribute| {
            let snake = &attribute.snake;
            let set = &attribute.fn_set;
            let full = &set.full;
            quote! { #full { self.map(|styled| #styler::#set(styled, #snake)).into() } }
        });
        let impl_styler_mut = self.all.iter().map(|attribute| {
            let snake = &attribute.snake;
            let set_mut = &attribute.fn_set_mut;
            let full = &set_mut.full;
            quote! { #full { self.as_mut().map(|styled| #styler_mut::#set_mut(styled, #snake)); } }
        });

        quote! {
            use crate::*;
            #LINE_BREAK

            impl #styler_index for Cell { #(#impl_styler_index)* }
            #LINE_BREAK

            impl #styler for Cell { #(#impl_styler)* }
            #LINE_BREAK

            impl #styler_mut for Cell { #(#impl_styler_mut)* }
            #LINE_BREAK
        }
    }
}
