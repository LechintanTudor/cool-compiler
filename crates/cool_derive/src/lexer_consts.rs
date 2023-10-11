use proc_macro2::Ident;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Brace, Colon, Comma};
use syn::{braced, Token};

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
