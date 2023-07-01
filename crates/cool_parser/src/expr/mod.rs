mod access_expr;
mod align_of_expr;
mod array_expr;
mod binary_expr;
mod block_expr;
mod cast_expr;
mod cond_expr;
mod fn_call_expr;
mod fn_expr;
mod ident_expr;
mod literal_expr;
mod loop_expr;
mod match_expr;
mod offset_of_expr;
mod size_of_expr;
mod stmt_expr;
mod struct_expr;
mod subscript_expr;
mod tuple_expr;
mod unary_expr;

pub use self::access_expr::*;
pub use self::align_of_expr::*;
pub use self::array_expr::*;
pub use self::binary_expr::*;
pub use self::block_expr::*;
pub use self::cast_expr::*;
pub use self::cond_expr::*;
pub use self::fn_call_expr::*;
pub use self::fn_expr::*;
pub use self::ident_expr::*;
pub use self::literal_expr::*;
pub use self::loop_expr::*;
pub use self::match_expr::*;
pub use self::offset_of_expr::*;
pub use self::size_of_expr::*;
pub use self::stmt_expr::*;
pub use self::struct_expr::*;
pub use self::subscript_expr::*;
pub use self::tuple_expr::*;
pub use self::unary_expr::*;
use crate::{BinOp, ParseResult, Parser};
use cool_lexer::{tk, TokenKind};
use cool_span::{Section, Span};
use derive_more::From;
use paste::paste;

macro_rules! define_expr {
    { $($Variant:ident,)+ } => {
        paste! {
            #[derive(Clone, From, Debug)]
            pub enum Expr {
                $($Variant([<$Variant Expr>]),)+
            }
        }

        impl Section for Expr {
            fn span(&self) -> Span {
                match self {
                    $(Self::$Variant(e) => e.span(),)+
                }
            }
        }
    };
}

define_expr! {
    Access,
    AlignOf,
    Array,
    ArrayRepeat,
    Binary,
    Block,
    Cast,
    Cond,
    Deref,
    Fn,
    FnCall,
    Ident,
    Index,
    Literal,
    Loop,
    Match,
    OffsetOf,
    Paren,
    Range,
    SizeOf,
    Stmt,
    Struct,
    Tuple,
    Unary,
}

impl Expr {
    #[inline]
    pub fn is_promotable_to_stmt(&self) -> bool {
        matches!(self, Self::Block(_) | Self::Cond(_) | Self::Loop(_),)
    }
}

#[derive(Clone, From, Debug)]
enum ExprAtom {
    BinOp(BinOp),
    Expr(Expr),
}

impl ExprAtom {
    #[inline]
    pub fn into_expr(self) -> Expr {
        match self {
            Self::Expr(expr) => expr,
            _ => panic!("not an expression"),
        }
    }
}

impl Parser<'_> {
    #[inline]
    pub fn parse_expr(&mut self) -> ParseResult<Expr> {
        self.parse_expr_full(true)
    }

    #[inline]
    pub fn parse_non_struct_expr(&mut self) -> ParseResult<Expr> {
        self.parse_expr_full(false)
    }

    pub fn parse_expr_full(&mut self, allow_struct_expr: bool) -> ParseResult<Expr> {
        let expr = self.parse_primary_expr(allow_struct_expr)?;

        let (first_bin_op, second_expr) = match BinOp::from_token_kind(self.peek().kind) {
            Some(bin_op) => {
                self.bump();
                (bin_op, self.parse_primary_expr(allow_struct_expr)?)
            }
            None => return Ok(expr),
        };

        let mut atoms = vec![ExprAtom::Expr(expr), ExprAtom::Expr(second_expr)];
        let mut bin_ops = vec![first_bin_op];

        while let Some(bin_op) = BinOp::from_token_kind(self.peek().kind) {
            self.bump();

            while let Some(&last_bin_op) = bin_ops.last() {
                if last_bin_op.precedence() < bin_op.precedence() {
                    break;
                }

                atoms.push(last_bin_op.into());
                bin_ops.pop();
            }

            bin_ops.push(bin_op);

            let next_expr = self.parse_primary_expr(allow_struct_expr)?;
            atoms.push(next_expr.into());
        }

        while let Some(bin_op) = bin_ops.pop() {
            atoms.push(bin_op.into());
        }

        let mut atom_stack = Vec::<ExprAtom>::new();

        for atom in atoms.drain(..) {
            match atom {
                ExprAtom::Expr(_) => {
                    atom_stack.push(atom);
                }
                ExprAtom::BinOp(bin_op) => {
                    let rhs = atom_stack.pop().unwrap().into_expr();
                    let lhs = atom_stack.pop().unwrap().into_expr();

                    atom_stack.push(ExprAtom::Expr(
                        BinaryExpr {
                            bin_op,
                            lhs: lhs.into(),
                            rhs: rhs.into(),
                        }
                        .into(),
                    ));
                }
            }
        }

        debug_assert!(!atom_stack.is_empty());
        Ok(atom_stack.pop().unwrap().into_expr())
    }

    fn parse_primary_expr(&mut self, allow_struct_expr: bool) -> ParseResult<Expr> {
        let mut expr: Expr = match self.peek().kind {
            TokenKind::Ident(_) => self.parse_ident_expr()?.into(),
            TokenKind::Prefix(_) | TokenKind::Literal(_) => self.parse_literal_expr()?.into(),
            tk::KW_ALIGN_OF => self.parse_align_of_expr()?.into(),
            tk::KW_IF => self.parse_cond_expr()?.into(),
            tk::KW_LOOP => self.parse_loop_expr()?.into(),
            tk::KW_MATCH => self.parse_match_expr()?.into(),
            tk::KW_OFFSET_OF => self.parse_offset_of_expr()?.into(),
            tk::KW_SIZE_OF => self.parse_size_of_expr()?.into(),
            tk::MINUS | tk::NOT | tk::AND => self.parse_unary_expr()?.into(),
            tk::OPEN_BRACE => self.parse_block_expr()?.into(),
            tk::OPEN_BRACKET => self.parse_array_expr()?,
            tk::OPEN_PAREN => self.parse_tuple_expr()?,
            _ => {
                return self.peek_error(&[
                    tk::DIAG_IDENT,
                    tk::DIAG_LITERAL,
                    tk::KW_ALIGN_OF,
                    tk::KW_OFFSET_OF,
                    tk::KW_RETURN,
                    tk::KW_SIZE_OF,
                    tk::OPEN_BRACE,
                    tk::OPEN_BRACKET,
                    tk::OPEN_PAREN,
                ]);
            }
        };

        loop {
            expr = match &expr {
                Expr::Access(_) | Expr::Ident(_) => {
                    match self.peek().kind {
                        tk::DOT => self.continue_parse_access_expr(Box::new(expr))?,
                        tk::KW_AS => self.continue_parse_cast_expr(Box::new(expr))?.into(),
                        tk::OPEN_BRACE if allow_struct_expr => {
                            self.continue_parse_struct_expr(Box::new(expr))?.into()
                        }
                        tk::OPEN_PAREN => self.continue_parse_fn_call_expr(Box::new(expr))?.into(),
                        tk::OPEN_BRACKET => self.continue_parse_subscript_expr(Box::new(expr))?,
                        _ => break,
                    }
                }
                Expr::Cast(_) => {
                    match self.peek().kind {
                        tk::KW_AS => self.continue_parse_cast_expr(Box::new(expr))?.into(),
                        _ => break,
                    }
                }
                Expr::Array(_) | Expr::ArrayRepeat(_) | Expr::Block(_) | Expr::Cond(_) => {
                    match self.peek().kind {
                        tk::DOT => self.continue_parse_access_expr(Box::new(expr))?,
                        tk::KW_AS => self.continue_parse_cast_expr(Box::new(expr))?.into(),
                        tk::OPEN_BRACKET => self.continue_parse_subscript_expr(Box::new(expr))?,
                        _ => break,
                    }
                }
                Expr::Deref(_)
                | Expr::FnCall(_)
                | Expr::Index(_)
                | Expr::Paren(_)
                | Expr::Range(_) => {
                    match self.peek().kind {
                        tk::DOT => self.continue_parse_access_expr(Box::new(expr))?,
                        tk::KW_AS => self.continue_parse_cast_expr(Box::new(expr))?.into(),
                        tk::OPEN_PAREN => self.continue_parse_fn_call_expr(Box::new(expr))?.into(),
                        tk::OPEN_BRACKET => self.continue_parse_subscript_expr(Box::new(expr))?,
                        _ => break,
                    }
                }
                _ => break,
            }
        }

        Ok(expr)
    }
}
