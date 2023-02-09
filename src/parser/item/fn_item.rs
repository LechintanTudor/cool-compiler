use crate::lexer::{op, sep, Token, TokenKind};
use crate::parser::{ParseResult, Parser, Stmt, Ty, UnexpectedToken};
use crate::symbol::{kw, Symbol};
use crate::utils::Span;

#[derive(Clone, Debug)]
pub struct FnItem {
    pub span: Span,
    pub arg_list: FnArgList,
    pub return_ty: Option<Ty>,
    pub body: FnBody,
}

#[derive(Clone, Debug)]
pub struct FnArgList {
    pub span: Span,
    pub args: Vec<FnArg>,
    pub has_trailing_comma: bool,
}

#[derive(Clone, Debug)]
pub struct FnArg {
    pub span: Span,
    pub is_mutable: bool,
    pub ident: Symbol,
    pub ty: Symbol,
}

#[derive(Clone, Debug)]
pub struct FnBody {
    pub span: Span,
    pub stmts: Vec<Stmt>,
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn parse_fn_item(&mut self) -> ParseResult<FnItem> {
        let start_token = self.bump_expect(&[kw::FN])?;
        let arg_list = self.parse_fn_arg_list()?;

        let return_ty = if self.peek().kind == op::ARROW {
            self.bump();
            Some(self.parse_ty()?)
        } else {
            None
        };

        let body = self.parse_fn_body()?;

        Ok(FnItem {
            span: Span::from_start_and_end_spans(start_token.span, body.span),
            arg_list,
            return_ty,
            body,
        })
    }

    pub fn parse_fn_arg_list(&mut self) -> ParseResult<FnArgList> {
        let start_token = self.bump_expect(&[sep::OPEN_PAREN])?;

        let mut args = Vec::<FnArg>::new();

        let (end_span, has_trailing_comma) = if self.peek_kind() == sep::CLOSED_PAREN {
            (self.bump().span, false)
        } else {
            loop {
                let arg = self.parse_fn_arg()?;
                args.push(arg);

                let next_token = self.bump();

                match next_token.kind {
                    sep::CLOSED_PAREN => {
                        break (next_token.span, false);
                    }
                    sep::COMMA => {
                        if self.peek_kind() == sep::CLOSED_PAREN {
                            break (self.bump().span, true);
                        }
                    }
                    _ => {
                        return Err(UnexpectedToken {
                            found: next_token,
                            expected: &[sep::CLOSED_PAREN, sep::COMMA],
                        })?;
                    }
                }
            }
        };

        let span = Span::from_start_and_end_spans(start_token.span, end_span);

        Ok(FnArgList {
            span,
            args,
            has_trailing_comma,
        })
    }

    pub fn parse_fn_arg(&mut self) -> ParseResult<FnArg> {
        let start_token = self.bump();

        let (is_mutable, ident) = match start_token.kind {
            kw::MUT => {
                let next_token = self.bump();

                match next_token.kind {
                    TokenKind::Ident(ident) => (true, ident),
                    _ => {
                        return Err(UnexpectedToken {
                            found: next_token,
                            expected: &[],
                        })?;
                    }
                }
            }
            TokenKind::Ident(ident) => (false, ident),
            _ => {
                return Err(UnexpectedToken {
                    found: start_token,
                    expected: &[kw::MUT],
                })?;
            }
        };

        self.bump_expect(&[sep::COLON])?;

        let ty_token = self.bump();
        let ty = match ty_token.kind {
            TokenKind::Ident(ty) => ty,
            _ => {
                return Err(UnexpectedToken {
                    found: ty_token,
                    expected: &[],
                })?;
            }
        };

        let span = Span::from_start_and_end_spans(start_token.span, ty_token.span);

        Ok(FnArg {
            span,
            is_mutable,
            ident,
            ty,
        })
    }

    pub fn parse_fn_body(&mut self) -> ParseResult<FnBody> {
        let start_token = self.bump();

        if start_token.kind != sep::OPEN_BRACE {
            return Err(UnexpectedToken {
                found: start_token,
                expected: &[sep::OPEN_BRACE],
            })?;
        }

        let mut stmts = Vec::<Stmt>::new();

        let end_token = loop {
            if self.peek().kind == sep::CLOSED_BRACE {
                break self.bump();
            }

            let stmt = self.parse_stmt()?;
            stmts.push(stmt);
        };

        let span = Span::from_start_and_end_spans(start_token.span, end_token.span);

        Ok(FnBody { span, stmts })
    }
}
