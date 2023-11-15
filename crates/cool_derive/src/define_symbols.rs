use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Brace, Colon, Comma};
use syn::{braced, parse_macro_input, Token};

pub struct SymbolGroupSet {
    pub keywords: KeywordSymbolGroup,
    pub other: OtherSymbolGroup,
}

impl Parse for SymbolGroupSet {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let keywords = input.parse()?;
        input.parse::<Comma>()?;

        let other = input.parse()?;
        if input.peek(Token![,]) {
            input.parse::<Comma>()?;
        }

        Ok(Self { keywords, other })
    }
}

pub mod kw {
    syn::custom_keyword!(keywords);
    syn::custom_keyword!(other);
}

pub struct KeywordSymbolGroup {
    pub keywords: kw::keywords,
    pub colon: Colon,
    pub brace: Brace,
    pub symbols: Punctuated<Ident, Token![,]>,
}

impl Parse for KeywordSymbolGroup {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let symbols;

        Ok(Self {
            keywords: input.parse()?,
            colon: input.parse()?,
            brace: braced!(symbols in input),
            symbols: symbols.parse_terminated(Ident::parse_any, Token![,])?,
        })
    }
}

pub struct OtherSymbolGroup {
    pub other: kw::other,
    pub colon: Colon,
    pub brace: Brace,
    pub symbols: Punctuated<Ident, Token![,]>,
}

impl Parse for OtherSymbolGroup {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let symbols;

        Ok(Self {
            other: input.parse()?,
            colon: input.parse()?,
            brace: braced!(symbols in input),
            symbols: symbols.parse_terminated(Ident::parse_any, Token![,])?,
        })
    }
}

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
