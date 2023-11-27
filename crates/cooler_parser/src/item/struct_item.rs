use cool_collections::define_index_newtype;
use cool_derive::Section;
use cool_span::Span;

define_index_newtype!(StructId);

#[derive(Clone, Section, Debug)]
pub struct Struct {
    pub span: Span,
}
