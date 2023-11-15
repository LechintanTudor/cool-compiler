use proc_macro2::Ident;
use quote::quote;
use syn::ext::IdentExt;
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Token};

pub fn define_tys(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let idents = parse_macro_input!(input with parse_idents);

    let indexes = 1..(idents.len() as u32 + 1);
    let ident_iter = idents.iter();

    let tokens = quote! {
        #[allow(non_upper_case_globals)]
        pub mod tys {
            use crate::TyId;

            #(
                pub const #ident_iter: TyId = unsafe { TyId::new_unchecked(#indexes) };
            )*
        }
    };

    tokens.into()
}

fn parse_idents(input: ParseStream) -> syn::Result<Punctuated<Ident, Token![,]>> {
    input.parse_terminated(Ident::parse_any, Token![,])
}
