mod lexer_consts;
mod ty_consts;

use self::lexer_consts::*;
use proc_macro2::Ident;
use quote::{format_ident, quote, quote_spanned};
use syn::{parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(Section)]
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
        Data::Enum(DataEnum { variants, .. }) => {
            if variants.is_empty() {
                quote! {
                    ::cool_span::Span::default()
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

#[proc_macro]
pub fn define_symbols(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let symbol_groups = parse_macro_input!(input as SymbolGroupSet);

    // Indexes
    let mut indexes = (1..=(symbol_groups.keywords.symbols.len()
        + symbol_groups.other.symbols.len()))
        .map(|index| index as u32);

    let indexes = indexes.by_ref();

    // Keywords
    let keyword_ident_1 = symbol_groups
        .keywords
        .symbols
        .iter()
        .map(|ident| format_ident!("kw_{}", ident));

    let keyword_ident_2 = keyword_ident_1.clone();

    // Other
    let other_ident_1 = symbol_groups.other.symbols.iter();
    let other_ident_2 = other_ident_1.clone();

    // Insert
    let insert_keywords = symbol_groups.keywords.symbols.iter();
    let insert_other = symbol_groups.other.symbols.iter();

    let output = quote! {
        #[allow(non_upper_case_globals)]
        pub mod sym {
            #(
                pub const #keyword_ident_1: crate::Symbol
                    = unsafe { crate::Symbol::new_unchecked(#indexes) };
            )*

            #(
                pub const #other_ident_1: crate::Symbol
                    = unsafe { crate::Symbol::new_unchecked(#indexes) };
            )*

            pub fn insert_symbols(symbols: &mut crate::SymbolTable) {
                #(
                    symbols.insert_str(stringify!(#insert_keywords));
                )*

                #(
                    symbols.insert_str(stringify!(#insert_other));
                )*
            }
        }

        #[allow(non_upper_case_globals)]
        pub(crate) mod sym_tk {
            #(
                pub const #keyword_ident_2: crate::TokenKind
                    = crate::TokenKind::Keyword(super::sym::#keyword_ident_2);
            )*

            #(
                pub const #other_ident_2: crate::TokenKind
                    = crate::TokenKind::Ident(super::sym::#other_ident_2);
            )*
        }
    };

    output.into()
}

#[proc_macro]
pub fn define_tys(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    ty_consts::define_tys(input)
}
