use cool_resolve::ResolveContext;

pub struct AstGenerator<'a> {
    pub resolve: &'a mut ResolveContext,
}

impl<'a> AstGenerator<'a> {
    #[inline]
    pub fn new(resolve: &'a mut ResolveContext) -> Self {
        Self { resolve }
    }
}
