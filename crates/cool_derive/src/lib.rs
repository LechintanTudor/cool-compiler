mod define_symbols;
mod define_tys;
mod derive_section;

#[proc_macro_derive(Section)]
pub fn derive_section(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_section::derive_section(input)
}

#[proc_macro]
pub fn define_symbols(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    define_symbols::define_symbols(input)
}

#[proc_macro]
pub fn define_tys(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    define_tys::define_tys(input)
}
