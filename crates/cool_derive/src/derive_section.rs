use quote::{quote, quote_spanned};
use syn::{parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Fields};

pub fn derive_section(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = parse_macro_input!(input as DeriveInput);

    let fn_impl_tokens = match data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => {
            let has_span_field = fields
                .named
                .iter()
                .flat_map(|field| field.ident.as_ref())
                .any(|ident| ident == "span");

            if !has_span_field {
                quote_spanned! {
                   ident.span() => compile_error!("Struct has no 'span' field")
                }
            } else {
                quote! {
                    self.span
                }
            }
        }
        Data::Enum(DataEnum { variants, .. }) => {
            if variants.is_empty() {
                quote! {
                    ::cool_span::Span::empty()
                }
            } else {
                let variants = variants.iter().map(|variant| &variant.ident);

                quote! {
                    match self {
                        #(Self::#variants(variant) => variant.span(),)*
                    }
                }
            }
        }
        _ => {
            quote_spanned! {
                ident.span() => compile_error!("Unsupported data type")
            }
        }
    };

    let impl_tokens = quote! {
        impl #generics ::cool_span::Section for #ident #generics {
            #[inline]
            fn span(&self) -> ::cool_span::Span {
                #fn_impl_tokens
            }
        }
    };

    impl_tokens.into()
}
