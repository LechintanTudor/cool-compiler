mod error;
mod expr;
mod op;
mod resolve;
mod stmt;

pub use self::error::*;
pub use self::expr::*;
pub use self::op::*;
pub use self::resolve::*;
pub use self::stmt::*;

use cool_resolve::ResolveContext;

#[derive(Debug)]
pub struct AstGenerator<'a> {
    context: &'a mut ResolveContext<'static>,
}

impl<'a> AstGenerator<'a> {
    #[inline]
    pub fn new(context: &'a mut ResolveContext<'static>) -> Self {
        Self { context }
    }

    #[inline]
    #[must_use]
    pub fn context(&self) -> &ResolveContext {
        self.context
    }
}
