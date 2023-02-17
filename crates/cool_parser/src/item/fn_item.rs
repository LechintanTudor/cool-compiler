use crate::error::{ParseResult, UnexpectedToken};
use crate::parser::Parser;
use crate::stmt::Stmt;
use crate::ty::Ty;
use crate::ParseTree;
use cool_lexer::symbols::Symbol;
use cool_lexer::tokens::{tk, Token, TokenKind};
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct FnItem {
    pub span: Span,
    pub arg_list: FnArgList,
    pub return_ty: Option<Ty>,
    pub body: FnBody,
}

impl ParseTree for FnItem {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone, Debug)]
pub struct FnArgList {
    pub span: Span,
    pub args: Vec<FnArg>,
    pub has_trailing_comma: bool,
}

impl ParseTree for FnArgList {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone, Debug)]
pub struct FnArg {
    pub span: Span,
    pub is_mutable: bool,
    pub ident: Symbol,
    pub ty: Ty,
}

impl ParseTree for FnArg {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone, Debug)]
pub struct FnBody {
    pub span: Span,
    pub stmts: Vec<Stmt>,
}

impl ParseTree for FnBody {
    fn span(&self) -> Span {
        self.span
    }
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn parse_fn_item(&mut self) -> ParseResult<FnItem> {
        let start_token = self.bump_expect(&[tk::KW_FN])?;
        let arg_list = self.parse_fn_arg_list()?;

        let return_ty = if self.peek().kind == tk::ARROW {
            self.bump();
            Some(self.parse_ty()?)
        } else {
            None
        };

        let body = self.parse_fn_body()?;

        Ok(FnItem {
            span: start_token.span.to(body.span),
            arg_list,
            return_ty,
            body,
        })
    }

    pub fn parse_fn_arg_list(&mut self) -> ParseResult<FnArgList> {
        let start_token = self.bump_expect(&[tk::OPEN_PAREN])?;

        let mut args = Vec::<FnArg>::new();

        let (end_span, has_trailing_comma) = if self.peek().kind == tk::CLOSE_PAREN {
            (self.bump().span, false)
        } else {
            loop {
                let arg = self.parse_fn_arg()?;
                args.push(arg);

                let next_token = self.bump();

                match next_token.kind {
                    tk::CLOSE_PAREN => {
                        break (next_token.span, false);
                    }
                    tk::COMMA => {
                        if self.peek().kind == tk::CLOSE_PAREN {
                            break (self.bump().span, true);
                        }
                    }
                    _ => {
                        return Err(UnexpectedToken {
                            found: next_token,
                            expected: &[tk::CLOSE_PAREN, tk::COMMA],
                        })?;
                    }
                }
            }
        };

        Ok(FnArgList {
            span: start_token.span.to(end_span),
            args,
            has_trailing_comma,
        })
    }

    pub fn parse_fn_arg(&mut self) -> ParseResult<FnArg> {
        let start_token = self.bump();

        let (is_mutable, ident) = match start_token.kind {
            tk::KW_MUT => {
                let next_token = self.bump();

                match next_token.kind {
                    TokenKind::Ident(ident) => (true, ident),
                    _ => {
                        return Err(UnexpectedToken {
                            found: next_token,
                            expected: &[tk::ANY_IDENT],
                        })?;
                    }
                }
            }
            TokenKind::Ident(ident) => (false, ident),
            _ => {
                return Err(UnexpectedToken {
                    found: start_token,
                    expected: &[tk::KW_MUT],
                })?;
            }
        };

        self.bump_expect(&[tk::COLON])?;
        let ty = self.parse_ty()?;

        Ok(FnArg {
            span: start_token.span.to(ty.span()),
            is_mutable,
            ident,
            ty,
        })
    }

    pub fn parse_fn_body(&mut self) -> ParseResult<FnBody> {
        let start_token = self.bump();

        if start_token.kind != tk::OPEN_BRACE {
            return Err(UnexpectedToken {
                found: start_token,
                expected: &[tk::OPEN_BRACE],
            })?;
        }

        let mut stmts = Vec::<Stmt>::new();

        let end_token = loop {
            if self.peek().kind == tk::CLOSE_BRACE {
                break self.bump();
            }

            let stmt = self.parse_stmt()?;
            stmts.push(stmt);
        };

        Ok(FnBody {
            span: start_token.span.to(end_token.span),
            stmts,
        })
    }
}
