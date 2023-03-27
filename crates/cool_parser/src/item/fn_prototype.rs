use crate::{Ident, ParseResult, ParseTree, Parser, Ty};
use cool_lexer::symbols::Symbol;
use cool_lexer::tokens::{tk, Literal, LiteralKind, TokenKind};
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct FnExternDecl {
    pub span: Span,
    pub abi: Option<Symbol>,
}

impl ParseTree for FnExternDecl {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone, Debug)]
pub struct FnParam {
    pub span: Span,
    pub is_mutable: bool,
    pub ident: Ident,
    pub ty: Option<Ty>,
}

impl ParseTree for FnParam {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone, Debug)]
pub struct FnParamList {
    pub span: Span,
    pub params: Vec<FnParam>,
    pub has_trailing_comma: bool,
}

impl ParseTree for FnParamList {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone, Debug)]
pub struct FnPrototype {
    pub span: Span,
    pub extern_decl: Option<FnExternDecl>,
    pub param_list: FnParamList,
    pub return_ty: Option<Ty>,
}

impl ParseTree for FnPrototype {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    fn parse_fn_extern_decl(&mut self) -> ParseResult<FnExternDecl> {
        let start_token = self.bump_expect(&tk::KW_EXTERN)?;

        let (end_span, abi) = match self.peek().kind {
            TokenKind::Literal(Literal {
                kind: LiteralKind::String,
                symbol,
            }) => (self.bump().span, Some(symbol)),
            _ => (start_token.span, None),
        };

        Ok(FnExternDecl {
            span: start_token.span.to(end_span),
            abi,
        })
    }

    fn parse_fn_param(&mut self) -> ParseResult<FnParam> {
        let start_token = self.bump();

        let (is_mutable, ident) = match start_token.kind {
            tk::KW_MUT => (true, self.parse_ident()?),
            TokenKind::Ident(symbol) => (
                false,
                Ident {
                    span: start_token.span,
                    symbol,
                },
            ),
            _ => self.error(start_token, &[tk::KW_MUT, tk::ANY_IDENT])?,
        };

        let ty = if self.bump_if_eq(tk::COLON).is_some() {
            Some(self.parse_ty()?)
        } else {
            None
        };

        let span = start_token
            .span
            .to(ty.as_ref().map(|ty| ty.span()).unwrap_or(ident.span));

        Ok(FnParam {
            span,
            is_mutable,
            ident,
            ty,
        })
    }

    fn parse_fn_param_list(&mut self) -> ParseResult<FnParamList> {
        let start_token = self.bump_expect(&tk::OPEN_PAREN)?;

        let mut params = Vec::<FnParam>::new();
        let (end_span, has_trailing_comma) = if self.peek().kind == tk::CLOSE_PAREN {
            (self.bump().span, false)
        } else {
            loop {
                params.push(self.parse_fn_param()?);

                let next_token = self.bump();

                match next_token.kind {
                    tk::CLOSE_PAREN => {
                        break (next_token.span, false);
                    }
                    tk::COMMA => {
                        if let Some(end_token) = self.bump_if_eq(tk::CLOSE_PAREN) {
                            break (end_token.span, true);
                        }
                    }
                    _ => self.error(next_token, &[tk::CLOSE_PAREN, tk::COMMA])?,
                }
            }
        };

        Ok(FnParamList {
            span: start_token.span.to(end_span),
            params,
            has_trailing_comma,
        })
    }

    pub fn parse_fn_prototype(&mut self) -> ParseResult<FnPrototype> {
        let extern_decl = if self.peek().kind == tk::KW_EXTERN {
            Some(self.parse_fn_extern_decl()?)
        } else {
            None
        };

        let fn_kw = self.bump_expect(&tk::KW_FN)?;
        let param_list = self.parse_fn_param_list()?;

        let return_ty = if self.bump_if_eq(tk::ARROW).is_some() {
            Some(self.parse_ty()?)
        } else {
            None
        };

        let span = {
            let start_span = extern_decl
                .as_ref()
                .map(|decl| decl.span)
                .unwrap_or(fn_kw.span);

            let end_span = return_ty
                .as_ref()
                .map(|ty| ty.span())
                .unwrap_or(param_list.span);

            start_span.to(end_span)
        };

        Ok(FnPrototype {
            span,
            extern_decl,
            param_list,
            return_ty,
        })
    }
}
