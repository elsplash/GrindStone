use std::iter::Peekable;

use crate::internals::{
    LexToken,
    Token,
};

#[derive(Clone)]
pub enum CSTNode<'linespan> {
    String{
        lquot:    LexToken<'linespan>,
        contents: Vec<LexToken<'linespan>>,
        rquot:    Option<LexToken<'linespan>>,
    },
    Number{ num: LexToken<'linespan> },
    Var{ name: LexToken<'linespan> },

    Item{
        var: Box<CSTNode<'linespan>>,
        comma: Option<LexToken<'linespan>>,
        space: Option<LexToken<'linespan>>,
    },
    Paren{
        lparen: LexToken<'linespan>,
        contents: Vec<Box<CSTNode<'linespan>>>,
        rparen: Option<LexToken<'linespan>>,
    },

    BinOp{
        lhs: Box<CSTNode<'linespan>>,
        op:  Option<LexToken<'linespan>>,
        rhs: Option<Box<CSTNode<'linespan>>>,
    },

    VarDecl{
        indent_sz: usize,
        key:   LexToken<'linespan>,
        name:  Option<LexToken<'linespan>>,
        equal: Option<LexToken<'linespan>>,
        def:   Option<Box<CSTNode<'linespan>>>,
    },
    VarDef{
        indent_sz: usize,
        name:  LexToken<'linespan>,
        equal: Option<LexToken<'linespan>>,
        def:   Option<Box<CSTNode<'linespan>>>,
    },

    FuncDecl{
        indent_sz: usize,
        key: LexToken<'linespan>,
        name: Option<LexToken<'linespan>>,
        paren: Option<Box<CSTNode<'linespan>>>,
    },
    FuncCall{
        indent_sz: usize,
        name: LexToken<'linespan>,
        paren: Option<Box<CSTNode<'linespan>>>,
    },
}

pub enum CSTReport<'linespan> {
    InternalError,

    ExpectedXGotY{
        x: Token,
        y: LexToken<'linespan>,
    },
    InvalidExpressionXAfterY{
        x: LexToken<'linespan>,
        y: Option<CSTNode<'linespan>>,
    },

    EndOfFileDuring(Option<CSTNode<'linespan>>),
    UnfinishedStatement(CSTNode<'linespan>),
    UnclosedDelimiter(CSTNode<'linespan>),
}

pub struct CSTOutput<'linespan>{
    output: Vec<CSTNode<'linespan>>,
    errs: Vec<CSTReport<'linespan>>,
}

impl<'linespan> CSTOutput<'linespan> {
    pub fn new() -> CSTOutput<'linespan> {
        CSTOutput{
            output: Vec::new(),
            errs: Vec::new(),
        }
    }

    fn parse_func_decl(
        &mut self, line_section: &'linespan Vec<LexToken<'linespan>>
    ) -> bool {
        let mut ls_iter = line_section.iter().peekable();
        let indent_sz = self.consume_indent_token(&mut ls_iter);

        let Some(key) = self.expect_consume_token_or(
            Token::Identifier, &mut ls_iter,
            CSTReport::InternalError
        ) else { return false };
        if key.span.str != "func" { return false; };

        let Some(name) = self.expect_consume_token_or(
            Token::Identifier, &mut ls_iter,
            CSTReport::EndOfFileDuring(Some(CSTNode::FuncDecl{
                indent_sz: indent_sz,
                key: key.clone(),
                name: None,
                paren: None,
            }))
        ) else { return false };

		let Some(lparen) = self.expect_consume_token_or(
            Token::LParen, &mut ls_iter,
			CSTReport::EndOfFileDuring(Some(CSTNode::FuncDecl{
                indent_sz: indent_sz,
                key: key.clone(),
                name: Some(name.clone()),
                paren: None,
            }))
        ) else { return false };

        match self.parse_paren(&mut ls_iter, lparen.clone()) {
            Some(paren) => self.output.push(CSTNode::FuncDecl{
                indent_sz: indent_sz,
                key: key.clone(),
                name: Some(name.clone()),
                paren: Some(Box::new(paren.clone())),
            }),
            None => return false,
        }

        true
    }

    fn parse_identifier(
        &mut self, line_section: &'linespan Vec<LexToken<'linespan>>
    ) -> bool {
        let mut ls_iter = line_section.iter().peekable();
        let mut is_decl: bool = false;
        let mut opt_vardecl = None::<CSTNode<'linespan>>;
        let indent_sz = self.consume_indent_token(&mut ls_iter);

        let Some(opt_key) = ls_iter.peek() else {
            self.errs.push(CSTReport::InternalError);
            return false;
        };
        let key = opt_key.clone();
        drop(opt_key);

        if key.span.str == "var" {
            return self.parse_var_decl(&mut ls_iter);
        } else if key.span.str == "func" {
            /* Use parse_func_call() */
            return true;
        }

        let Some(name) = self.expect_consume_token_or(
            Token::Identifier, &mut ls_iter,
            CSTReport::EndOfFileDuring(
                if is_decl { Some(CSTNode::VarDecl{
                    indent_sz: indent_sz,
                    key: key.clone(),
                    name: None,
                    equal: None,
                    def: None
                }) }
                else { None }
            )
        ) else { return false };

        if !is_decl {
            let Some(ltok) = ls_iter.next() else {
                self.errs.push(CSTReport::EndOfFileDuring(Some(
                    CSTNode::VarDef{
                        indent_sz: indent_sz,
                        name: name.clone(),
                        equal: None,
                        def: None
                    }
                )));
                return false;
            };
            if ltok.token == Token::LParen {
                return self.parse_func_call(&mut ls_iter, name.clone(), ltok.clone(), indent_sz);
            } if ltok.token == Token::Dot {
                /* TODO: Parse Method Function */
            } if ltok.token == Token::Newline {
                self.errs.push(CSTReport::UnfinishedStatement(
                    CSTNode::VarDef{
                        indent_sz: indent_sz,
                        name: name.clone(),
                        equal: None,
                        def: None,
                    }
                ));
                return false;
            }
            match self.parse_expr(&mut ls_iter) {
                Some(expr) => self.output.push(CSTNode::VarDef{
                    indent_sz: indent_sz,
                    name: name.clone(),
                    equal: Some(ltok.clone()),
                    def: Some(Box::new(expr.clone())),
                }),
                None => return false,
            }
            return true;
        }

        if let Some(ltok) = ls_iter.peek() {match ltok.token {
            Token::Newline => {/* Nothing, we just pass */},
            Token::Equal => {
                drop(ltok);
                let Some(ltok) = ls_iter.next() else {
                    self.errs.push(CSTReport::InternalError);
                    return false;
                };
                let _ltok = (*ltok).clone();
                match self.parse_expr(&mut ls_iter) {
                    Some(expr) => {
                        self.output.push(CSTNode::VarDecl{
                            indent_sz: indent_sz,
                            key: (*key).clone(),
                            name: Some(name.clone()),
                            equal: Some(_ltok),
                            def: Some(Box::new(expr.clone())),
                        });
                    },
                    None => return false,
                }
            },
            _ => {
                self.errs.push(CSTReport::ExpectedXGotY{
                    x: Token::Newline,
                    y: (*ltok).clone(),
                });
                return false;
            },
        }}

        true
    }

    fn parse_var_decl<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self,
        ls_iter: &mut Peekable<I>
    ) -> bool {
        true
    }

    fn parse_func_call<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self,
        ls_iter: &mut Peekable<I>,
        name: LexToken<'linespan>,
        lparen: LexToken<'linespan>,
        indent_sz: usize
    ) -> bool {
        match self.parse_paren(ls_iter, lparen) {
            Some(paren) => self.output.push(CSTNode::FuncCall{
                indent_sz: indent_sz,
                name: name,
                paren: Some(Box::new(paren)),
            }),
            None => return false,
        }

        if let Some(ltok) = ls_iter.peek() && ltok.token != Token::Newline {
            self.errs.push(CSTReport::ExpectedXGotY{
                x: Token::Newline,
                y: (*ltok).clone(),
            });
            return false;
        }

        true
    }

    fn parse_paren<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self, ls_iter: &mut Peekable<I>, lparen: LexToken<'linespan>
    ) -> Option<CSTNode<'linespan>> {
        let mut is_expect_comma = false;
        let mut paren = CSTNode::Paren{
            lparen: lparen,
            contents: Vec::new(),
            rparen: None,
        };
        let CSTNode::Paren{
            lparen: _,
            ref mut contents,
            ref mut rparen,
        } = paren else {
            self.errs.push(CSTReport::InternalError);
            return None;
        };
        loop {
            let Some(ltok) = ls_iter.peek() else {
                self.errs.push(CSTReport::UnclosedDelimiter(paren));
                return None;
            };
            match ltok.token.clone() {
                Token::RParen => {
                    *rparen = Some((*ltok).clone());
                    return Some(paren);
                },
                Token::Space => continue,

                Token::Comma => {
                    if !is_expect_comma {
                        self.errs.push(CSTReport::ExpectedXGotY{
                            x: Token::Identifier,
                            y: (*ltok).clone()
                        });
                    }
                    is_expect_comma = false;
                },

                _ => {
                    if is_expect_comma {
                        self.errs.push(CSTReport::ExpectedXGotY{
                            x: Token::Comma,
                            y: (*ltok).clone(),
                        });
                    }
                    match self.parse_expr(ls_iter) {
                        Some(expr) => {
                            contents.push(Box::new(expr));
                            is_expect_comma = true;
                        },
                        None => break None::<CSTNode<'linespan>>,
                    }
                },
            }
        }
    }

    fn parse_expr<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self, ls_iter: &mut Peekable<I>
    ) -> Option<CSTNode<'linespan>> {
        let Some(ltok) = ls_iter.next() else {
            self.errs.push(CSTReport::InternalError);
            return None;
        };

        match ltok.token.clone() {
            Token::Quote => {
                let mut quot = CSTNode::String{
                    lquot: ltok.clone(),
                    contents: Vec::new(),
                    rquot: None,
                };
                let CSTNode::String{
                    lquot: _,
                    ref mut contents,
                    ref mut rquot,
                } = quot else {
                    self.errs.push(CSTReport::InternalError);
                    return None;
                };
                loop {
                    let Some(next_tok) = ls_iter.next() else {
                        self.errs.push(CSTReport::UnclosedDelimiter(quot));
                        return None;
                    };
                    if next_tok.token == Token::Quote {
                        *rquot = Some(next_tok.clone());
                        break;
                    } if next_tok.token == Token::Newline {
                        self.errs.push(CSTReport::UnclosedDelimiter(quot));
                        return None;
                    }
                    contents.push(next_tok.clone());
                }
                return Some(quot);
            },

            Token::Number => {
                let num = CSTNode::Number{ num: ltok.clone() };
                if !self.expect_peek_token(Token::Newline, ls_iter) {
                    self.parse_bin_op(ls_iter, num);
                    return None;
                }
                return Some(num);
            },

            Token::Identifier => {
                if ltok.span.str != "import" {
                    return Some(CSTNode::Var{ name: ltok.clone() });
                }
                todo!();
            },

            Token::LParen => {
                self.parse_paren(ls_iter, ltok.clone());
                return None;
            },

            _ => {
                self.errs.push(CSTReport::InvalidExpressionXAfterY{
                    x: ltok.clone(),
                    y: None,
                });
                return None;
            },
        }
    }

    fn parse_bin_op<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self, ls_iter: &mut Peekable<I>, lhs: CSTNode<'linespan>
    ) -> bool {
        let mut bin_op = CSTNode::BinOp{
            lhs: Box::new(lhs),
            op: None,
            rhs: None,
        };
        let CSTNode::BinOp{
            lhs: _,
            ref mut op,
            ref mut rhs
        } = bin_op else { return false };
        match ls_iter.next() {
            Some(ltok) => match ltok.token {
                Token::Plus | Token::Minus | Token::Star | Token::Slash
                    => *op = Some(ltok.clone()),
                _ => {
                    self.errs.push(CSTReport::ExpectedXGotY{
                        x: Token::Plus,
                        y: ltok.clone()
                    });
                    return false;
                },
            },
            None => {
                self.errs.push(CSTReport::InternalError);
                return false;
            },
        }

        if !self.expect_peek_token(Token::Number, ls_iter) { return false }
        match self.parse_expr(ls_iter) {
            Some(expr) => *rhs = Some(Box::new(expr)),
            None => return false,
        }

        match ls_iter.peek() {
            Some(peek) => match peek.token {
                Token::Newline => {
                    ls_iter.next();
                    self.output.push(bin_op);
                    return true;
                },
                Token::Plus | Token::Minus | Token::Star | Token::Slash
                    => return self.parse_bin_op(ls_iter, bin_op),
                _ => {
                    self.errs.push(CSTReport::InvalidExpressionXAfterY{
                        x: (*peek).clone(),
                        y: Some(bin_op),
                    });
                    return true;
                }
            },
            None => {
                self.output.push(bin_op);
                return true;
            },
        }
    }

    fn expect_consume_token_or<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self,
        token: Token,
        ls_iter: &mut Peekable<I>,
        or_err: CSTReport<'linespan>,
    ) -> Option<LexToken<'linespan>> {
        let Some(ltok) = ls_iter.next() else {
            self.errs.push(or_err);
            return None;
        };

        if ltok.token != token {self.errs.push(CSTReport::ExpectedXGotY{
            x: token,
            y: ltok.clone(),
        })}

        return Some(ltok.clone());
    }

    fn expect_peek_token<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &self,
        token: Token,
        ls_iter: &mut Peekable<I>
    ) -> bool {
        if let Some(ltok) = ls_iter.peek()
            && ltok.token == token
        {
            return true;
        }

         false
    }

    fn consume_indent_token<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self, ls_iter: &mut Peekable<I>
    ) -> usize {
        let Some(ltok) = ls_iter.peek() else { return 0 };
        if ltok.token != Token::Space { return 0 }
        return ltok.span.len;
    }
}
