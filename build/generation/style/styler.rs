use crate::generation::*;

impl Generation {
    pub fn mod_style_styler(&self) -> TokenStream {
        quote! {
            mod styler_index;
            mod styler_index_mut;
            mod styler;
            mod styler_mut;
            #LINE_BREAK

            pub use styler_index::*;
            pub use styler_index_mut::*;
            pub use styler::*;
            pub use styler_mut::*;
        }
    }

    pub fn styler_index(&self) -> (TokenStream, TokenStream) {
        let attributes = self
            .all
            .iter()
            .map(|attribute| {
                (
                    attribute,
                    &attribute.snake,
                    &attribute.styler.get,
                    &attribute.styler.get_mut,
                )
            })
            .collect::<Vec<_>>();

        let style = attributes
            .iter()
            .map(|(_, snake, get, _)| quote! { #snake: self.#get(), });

        let index = attributes.iter().map(|(attribute, _, get, _)| {
            let doc = doc!("Gets `Option<{}>`.", attribute);

            quote! {
                #doc
                fn #get(&self) -> Option<#attribute>;
                #LINE_BREAK
            }
        });

        let index_mut = attributes.iter().map(|(attribute, _, _, get_mut)| {
            let doc = doc!("Gets `&mut Option<{}>`.", attribute);

            quote! {
                #doc
                fn #get_mut(&mut self) -> &mut Option<#attribute>;
                #LINE_BREAK
            }
        });

        (
            quote! {
                use crate::*;
                #LINE_BREAK

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
            },
            quote! {
                use crate::*;
                #LINE_BREAK

                /// A trait for getting `Option`al attributes on mutable styled types.
                pub trait StylerIndexMut {
                    #(#index_mut)*
                }
            },
        )
    }

    pub fn styler(&self) -> (TokenStream, TokenStream) {
        let setters = self.styler_setters();
        let (setters, setters_mut) = (
            setters.iter().map(|(x, _)| x),
            setters.iter().map(|(_, x)| x),
        );

        let (others, others_mut) = self.styler_others();

        (
            quote! {
                use crate::*;
                #LINE_BREAK

                /// A trait for setting `Option`al attributes on styled types.
                pub trait Styler: StylerIndex + Sized {
                    /// The resulting type of the setters.
                    type Output;
                    #LINE_BREAK

                    #(#setters)*

                    #others
                }
                #LINE_BREAK
            },
            quote! {
                use crate::*;
                #LINE_BREAK

                /// A trait for setting `Option`al attributes on mutable styled types.
                pub trait StylerMut: StylerIndex {
                    #(#setters_mut)*

                    #others_mut
                }
            },
        )
    }

    // ======= //
    // Helpers //
    // ======= //

    fn styler_setters(&self) -> Vec<(TokenStream, TokenStream)> {
        self.all
            .iter()
            .map(|attribute| {
                let snake = &attribute.snake;
                let set = &attribute.styler.set;
                let set_mut = &attribute.styler.set_mut;
                let none = &attribute.styler.none;
                let none_mut = &attribute.styler.none_mut;

                let set_doc = doc!("Sets `Option<{}>`.", attribute);
                let set_mut_doc = doc!("Sets `Option<{}>`, mutably.", attribute);
                let none_doc = doc!("`None`s `Option<{}>`.", attribute);
                let none_mut_doc = doc!("`None`s `Option<{}>`, mutably.", attribute);

                let variants = self.styler_variant_setters(attribute, set, set_mut);
                let (variants, variants_mut) = (
                    variants.iter().map(|(x, _)| x),
                    variants.iter().map(|(_, x)| x),
                );

                (
                    quote! {
                        #set_doc
                        fn #set(self, #snake: impl Into<Option<#attribute>>) -> Self::Output;
                        #LINE_BREAK

                        #none_doc
                        fn #none(self) -> Self::Output {
                            self.#set(None)
                        }
                        #LINE_BREAK

                        #(#variants)*
                    },
                    quote! {
                        #set_mut_doc
                        fn #set_mut(&mut self, #snake: impl Into<Option<#attribute>>);
                        #LINE_BREAK

                        #none_mut_doc
                        fn #none_mut(&mut self) {
                            self.#set_mut(None);
                        }
                        #LINE_BREAK

                        #(#variants_mut)*
                    },
                )
            })
            .collect::<Vec<_>>()
    }

    fn styler_variant_setters(
        &self,
        attribute: &Attr,
        set: &Str,
        set_mut: &Str,
    ) -> Vec<(TokenStream, TokenStream)> {
        attribute
            .variants
            .iter()
            .map(|variant| {
                let set_variant = &variant.styler.set;
                let set_mut_variant = &variant.styler.set_mut;
                let wrapped = &variant.wrapped;
                let args = &variant.args;
                let doc = doc!("Sets `Some({})`.", wrapped);
                let doc_mut = doc!("Sets `Some({})`, mutably.", wrapped);

                (
                    quote! {
                        #doc
                        fn #set_variant(self, #args) -> Self::Output {
                            self.#set(Some(#wrapped))
                        }
                        #LINE_BREAK
                    },
                    quote! {
                        #doc_mut
                        fn #set_mut_variant(&mut self, #args) {
                            self.#set_mut(Some(#wrapped));
                        }
                        #LINE_BREAK
                    },
                )
            })
            .collect::<Vec<_>>()
    }

    fn styler_others(&self) -> (TokenStream, TokenStream) {
        let styler = self
            .all
            .iter()
            .map(|attribute| {
                (
                    &attribute.snake,
                    &attribute.styler.get,
                    &attribute.styler.set,
                    &attribute.styler.set_mut,
                    &attribute.reset.wrapped,
                )
            })
            .collect::<Vec<_>>();

        let (style, style_mut) = make(
            &styler,
            |(_, get, set, ..)| quote! { .#set(styler.#get()) },
            |(_, get, _, set_mut, _)| quote! { self.#set_mut(styler.#get()); },
            |style, style_mut| {
                (
                    quote! {
                        /// Applies `styler`'s styles.
                        fn style(self, styler: &impl StylerIndex) -> <Self::Output as Styler>::Output
                        where
                            Self::Output: Styler<Output = Self::Output>
                        {
                            self#(#style)*
                        }
                    },
                    quote! {
                        /// Applies `styler`'s styles, mutably.
                        fn style_mut(&mut self, styler: &impl StylerIndex) {
                            #(#style_mut)*
                        }
                    },
                )
            },
        );

        macro_rules! options {
            ($op:ident $op_mut:ident) => {
                make(
                    &styler,
                    |(attribute, get, set, ..)| {
                        quote! {
                            let #attribute = output.#get().$op(other.#get());
                            let output = output.#set(#attribute);
                        }
                    },
                    |(_, get, _, set_mut, ..)| {
                        quote! {
                            self.#set_mut(self.#get().$op(other.#get()));
                        }
                    },
                    |op, op_mut| {
                        let doc = doc!("`Option::{}` fields.", stringify!($op));
                        let doc_mut = doc!("`Option::{}` fields, mutably.", stringify!($op));

                        (
                            quote! {
                                #doc
                                fn $op(self, other: &impl StylerIndex) -> <Self::Output as Styler>::Output
                                where
                                    Self::Output: Styler<Output = Self::Output>
                                {
                                    let output = self;
                                    #(#op)*
                                    output
                                }
                            },
                            quote! {
                                #doc_mut
                                fn $op_mut(&mut self, other: &impl StylerIndex) {
                                    #(#op_mut)*
                                }
                            },
                        )
                    },
                );
            };
        }

        let (and, and_mut) = options!(and and_mut);
        let (or, or_mut) = options!(or or_mut);
        let (xor, xor_mut) = options!(xor xor_mut);

        let (dedup, dedup_mut) = make(
            &styler,
            |(_, get, set, ..)| {
                quote! {
                    if self.#get() == before.#get() {
                        self = self.#set(None);
                    }
                }
            },
            |(_, get, _, set_mut, ..)| {
                quote! {
                    if self.#get() == before.#get() {
                        self.#set_mut(None);
                    }
                }
            },
            |dedup, dedup_mut| {
                (
                    quote! {
                        /// Dedups (`None`s if identicals) fields.
                        fn dedup(mut self, before: &impl StylerIndex) -> Self
                        where
                            Self: Styler<Output = Self>
                        {
                            #(#dedup)*
                            self
                        }
                    },
                    quote! {
                        /// Dedups (`None`s if identicals) fields, mutably.
                        fn dedup_mut(&mut self, before: &impl StylerIndex) {
                            #(#dedup_mut)*
                        }
                    },
                )
            },
        );

        let (reset, reset_mut) = make(
            &styler,
            |(_, get, set, _, reset)| {
                quote! {
                    if let Some(_) = self.#get() {
                        self = self.#set(Some(#reset));
                    }
                }
            },
            |(_, get, _, set_mut, reset)| {
                quote! {
                    if let Some(_) = self.#get() {
                        self.#set_mut(Some(#reset));
                    }
                }
            },
            |reset, reset_mut| {
                (
                    quote! {
                        /// Resets (sets to reset value) fields which are `Some`.
                        fn reset(mut self) -> Self
                        where
                            Self: Styler<Output = Self>
                        {
                            #(#reset)*
                            self
                        }
                    },
                    quote! {
                        /// Resets (sets to reset value) fields which are `Some`, mutably.
                        fn reset_mut(&mut self) {
                            #(#reset_mut)*
                        }
                    },
                )
            },
        );

        (
            quote! {
                #style #LINE_BREAK
                #and   #LINE_BREAK
                #or    #LINE_BREAK
                #xor   #LINE_BREAK
                #dedup #LINE_BREAK
                #reset #LINE_BREAK
            },
            quote! {
                #style_mut #LINE_BREAK
                #and_mut   #LINE_BREAK
                #or_mut    #LINE_BREAK
                #xor_mut   #LINE_BREAK
                #dedup_mut #LINE_BREAK
                #reset_mut #LINE_BREAK
            },
        )
    }
}

fn make<T>(
    arr: &[T],
    f1: impl Fn(&T) -> TokenStream,
    f2: impl Fn(&T) -> TokenStream,
    f3: impl Fn(
        &mut dyn Iterator<Item = TokenStream>,
        &mut dyn Iterator<Item = TokenStream>,
    ) -> (TokenStream, TokenStream),
) -> (TokenStream, TokenStream) {
    f3(&mut arr.iter().map(f1), &mut arr.iter().map(f2))
}
