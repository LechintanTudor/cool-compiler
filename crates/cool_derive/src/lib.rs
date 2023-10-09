use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(Section)]
pub fn derive_section(input: TokenStream) -> TokenStream {
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
                .any(|ident| ident == &Ident::new("span", ident.span()));

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
        _ => {
            quote_spanned! {
                ident.span() => compile_error!("Unsupported data type")
            }
        }
    };

    let impl_tokens = quote! {
        impl #generics ::cool_span::Section for #ident #generics {
            #[inline]
            #[must_use]
            fn span(&self) -> ::cool_span::Span {
                #fn_impl_tokens
            }
        }
    };

    impl_tokens.into()
}
