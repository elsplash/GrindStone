use std::iter::Peekable;

use crate::internals::{
    LineSpan,
    LexToken,
    Token,
};

#[derive(Clone)]
pub enum CSTNode<'linespan> {
    String{
        lquot:    LexToken<'linespan>,
        contents: Option<LexToken<'linespan>>,
        rquot:    Option<LexToken<'linespan>>,
    },
    Number{ num: LexToken<'linespan> },
    Label{ name: LexToken<'linespan> },

    Import{
        indent_sz: usize,
        key:       LexToken<'linespan>,
        path:      Option<Box<CSTNode<'linespan>>>,
    },
    New{
        indent_sz: usize,
        key:       LexToken<'linespan>,
        path:      Option<Box<CSTNode<'linespan>>>,
    },

    Item{
        var:   Box<CSTNode<'linespan>>,
        comma: Option<LexToken<'linespan>>,
        space: Option<LexToken<'linespan>>,
    },
    Paren{
        lparen:   LexToken<'linespan>,
        contents: Vec<Box<CSTNode<'linespan>>>,
        rparen:   Option<LexToken<'linespan>>,
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
        name:      Box<CSTNode<'linespan>>,
        equal:     Option<LexToken<'linespan>>,
        def:       Option<Box<CSTNode<'linespan>>>,
    },
    VarElement{
        indent_sz: usize,
        var:       Box<CSTNode<'linespan>>,
        dot:       LexToken<'linespan>,
        element:   Option<Box<CSTNode<'linespan>>>,
    },
    VarMethod{
        indent_sz: usize,
        var:       Box<CSTNode<'linespan>>,
        dot:       LexToken<'linespan>,
        method:    Option<Box<CSTNode<'linespan>>>,
    },

    FuncDecl{
        indent_sz: usize,
        key:       LexToken<'linespan>,
        name:      Option<LexToken<'linespan>>,
        paren:     Option<Box<CSTNode<'linespan>>>,
    },
    FuncCall{
        indent_sz: usize,
        name:      LexToken<'linespan>,
        paren:     Option<Box<CSTNode<'linespan>>>,
    },

    If{
        indent_sz: usize,
        lhs:       Box<CSTNode<'linespan>>,
        op:        Option<LexToken<'linespan>>,
        rhs:       Option<Box<CSTNode<'linespan>>>,
    },
    Else{
        indent_sz:    usize,
        colon:        LexToken<'linespan>,
        if_statement: Option<Box<CSTNode<'linespan>>>,
    },

    Ascii{
        key_start: LexToken<'linespan>,
        lines:     Vec<LineSpan>,
        key_end:   Option<LexToken<'linespan>>,
    },
}

pub enum IntErrID {
    FetchCSTNode,
    ExpectedElement,
    ExpectedGuarantee,
}

pub enum CSTReport<'linespan> {
    InternalError(IntErrID),

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

/*
 * This is a TODO list, because I can't keep track of everything:
 *
 * [TODO] 1. I should also implement the parse method function.
 * [TODO] 2. I should do the bracket parser.
 * [TODO] 3. I should implement the if statements by now.
 * [TODO] 4. I should now do the main parser.
 */
impl<'linespan> CSTOutput<'linespan> {
    pub fn new() -> CSTOutput<'linespan> {
        CSTOutput{
            output: Vec::new(),
            errs: Vec::new(),
        }
    }

    fn parse_identifier(
        &mut self, line_section: &'linespan Vec<LexToken<'linespan>>
    ) -> bool {
        let mut ls_iter = line_section.iter().peekable();
        let indent_sz = self.consume_indent_token(&mut ls_iter);

        let Some(opt_key) = ls_iter.peek() else {
            self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee));
            return false;
        };
        let key = opt_key.clone();
        drop(opt_key);

        if key.span.str == "var" {
            ls_iter.next();
            return self.parse_var_decl(&mut ls_iter, key, indent_sz);
        } else if key.span.str == "func" {
            ls_iter.next();
            return self.parse_func_decl(&mut ls_iter, key.clone(), indent_sz);
        }

        let Some(name) = self.expect_consume_token_or(
            Token::Identifier, &mut ls_iter,
            CSTReport::EndOfFileDuring(Some(CSTNode::VarDecl{
                indent_sz,
                key: key.clone(),
                name: None,
                equal: None,
                def: None
            }))
        ) else { return false };

        let Some(ltok) = ls_iter.next() else {
            self.errs.push(CSTReport::EndOfFileDuring(Some(
                CSTNode::VarDef{
                    indent_sz,
                    name: Box::new(CSTNode::Label{ name: name.clone() }),
                    equal: None,
                    def: None
                }
            )));
            return false;
        };
        match ltok.token {
            Token::LParen => return self.parse_func_call(&mut ls_iter, name.clone(), ltok.clone(), indent_sz),

            Token::Dot => {
                let Some(method) = self.parse_element(
                    &mut ls_iter,
                    CSTNode::Label{ name: name.clone() },
                    ltok.clone(),
                    0,
                ) else { return false };
                match method {
                    CSTNode::VarElement{
                        indent_sz, 
                        var,
                        dot,
                        element,
                    } => {
                        match ls_iter.next() {
                            Some(ltok) => match ltok.token {
                                Token::Newline => {
                                    self.output.push(method);
                                    return true;
                                },
                                Token::Space => {},
                                _ => {
                                    self.errs.push(CSTReport::ExpectedXGotY{
                                        x: Token::Newline,
                                        y: ltok.clone(),
                                    });
                                }
                            }
                            None => {
                                self.output.push(method);
                                return true;
                            },
                        }
                        if self.expect_peek_token(Token::Equal, &mut ls_iter) {
                        }
                    },

                    CSTNode::VarMethod{
                        indent_sz,
                        var,
                        dot,
                        method,
                    } => {},

                    _ => {}
                }

                return true;
            },

            Token::Newline => {
                self.errs.push(CSTReport::UnfinishedStatement(
                    CSTNode::VarDef{
                        indent_sz,
                        name: Box::new(CSTNode::Label{ name: name.clone() }),
                        equal: None,
                        def: None,
                    }
                ));
                return false;
            },

            _ => {
                self.consume_space(&mut ls_iter);
                let Some(equal) = self.expect_consume_token_or(
                    Token::Equal, &mut ls_iter,
                    CSTReport::UnfinishedStatement(CSTNode::VarDef{
                        indent_sz,
                        name: Box::new(CSTNode::Label{ name: name.clone() }),
                        equal: None,
                        def: None,
                    })
                ) else { return false; };
                self.consume_space(&mut ls_iter);
                let Some(expr) = self.parse_expr(&mut ls_iter, 0) else { return false };
                self.output.push(CSTNode::VarDef{
                    indent_sz,
                    name: Box::new(CSTNode::Label{ name: name.clone() }),
                    equal: Some(equal),
                    def: Some(Box::new(expr)),
                });
                return true;
            },
        }
    }

    fn parse_func_decl<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self,
        ls_iter: &mut Peekable<I>,
        func_key: LexToken<'linespan>,
        indent_sz: usize,
    ) -> bool {
        if self.expect_consume_token_or(
            Token::Space, ls_iter,
            CSTReport::EndOfFileDuring(Some(CSTNode::FuncDecl{
                indent_sz,
                key: func_key.clone(),
                name: None,
                paren: None,
            }))
        ).is_some() { return false }
        let Some(name) = self.expect_consume_token_or(
            Token::Identifier, ls_iter,
            CSTReport::EndOfFileDuring(Some(CSTNode::FuncDecl{
                indent_sz,
                key: func_key.clone(),
                name: None,
                paren: None,
            }))
        ) else { return false };

        let Some(lparen) = self.expect_consume_token_or(
            Token::LParen, ls_iter,
            CSTReport::EndOfFileDuring(Some(CSTNode::FuncDecl{
                indent_sz,
                key: func_key.clone(),
                name: Some(name.clone()),
                paren: None,
            }))
        ) else { return false };

        match self.parse_paren(ls_iter, lparen.clone()) {
            Some(paren) => self.output.push(CSTNode::FuncDecl{
                indent_sz,
                key: func_key.clone(),
                name: Some(name.clone()),
                paren: Some(Box::new(paren.clone())),
            }),
            None => return false,
        }

        true
    }

    fn parse_var_decl<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self,
        ls_iter: &mut Peekable<I>,
        var: &LexToken<'linespan>,
        indent_sz: usize
    ) -> bool {
        if self.expect_consume_token_or(
            Token::Space, ls_iter,
            CSTReport::EndOfFileDuring(Some(CSTNode::VarDecl{
                indent_sz,
                key: var.clone(),
                name: None,
                equal: None,
                def: None,
            }))
        ).is_some() { return false }
        let Some(name) = self.expect_consume_token_or(
            Token::Identifier, ls_iter,
            CSTReport::InternalError(IntErrID::ExpectedGuarantee)
        ) else { return false };

        self.consume_space(ls_iter);

        let Some(equal) = self.expect_consume_token_or(
            Token::Equal, ls_iter,
            CSTReport::EndOfFileDuring(Some(CSTNode::VarDecl{
                indent_sz,
                key: var.clone(),
                name: Some(name.clone()),
                equal: None,
                def: None,
            }))
        ) else { return false; };

        self.consume_space(ls_iter);

        let Some(expr) = self.parse_expr(ls_iter, 0) else { return false };

        self.output.push(CSTNode::VarDecl{
            indent_sz,
            key: var.clone(),
            name: Some(name),
            equal: Some(equal),
            def: Some(Box::new(expr)),
        });

        if let Some(ltok) = ls_iter.peek() && ltok.token != Token::Newline {
            self.errs.push(CSTReport::ExpectedXGotY{
                x: Token::Newline,
                y: (*ltok).clone(),
            });
            return true;
        }
        ls_iter.next();

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
                indent_sz,
                name,
                paren: Some(Box::new(paren)),
            }),
            None => return false,
        }

        true
    }

    fn parse_paren<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self, ls_iter: &mut Peekable<I>, lparen: LexToken<'linespan>
    ) -> Option<CSTNode<'linespan>> {
        let mut is_expect_comma = false;
        let mut paren = CSTNode::Paren{
            lparen,
            contents: Vec::new(),
            rparen: None,
        };
        let CSTNode::Paren{
            lparen: _,
            ref mut contents,
            ref mut rparen,
        } = paren else {
            self.errs.push(CSTReport::InternalError(IntErrID::FetchCSTNode));
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
                Token::Continue | Token::Space => continue,

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
                    match self.parse_expr(ls_iter, 0) {
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
        &mut self, ls_iter: &mut Peekable<I>, indent_sz: usize,
    ) -> Option<CSTNode<'linespan>> {
        let Some(ltok) = ls_iter.next() else {
            self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee));
            return None;
        };

        match ltok.token.clone() {
            Token::Quote => {
                let mut quot = CSTNode::String{
                    lquot: ltok.clone(),
                    contents: None,
                    rquot: None,
                };
                let CSTNode::String{
                    lquot: _,
                    ref mut contents,
                    ref mut rquot,
                } = quot else {
                    self.errs.push(CSTReport::InternalError(IntErrID::FetchCSTNode));
                    return None;
                };
                loop {
                    let Some(next_tok) = ls_iter.peek() else {
                        self.errs.push(CSTReport::UnclosedDelimiter(quot));
                        return None;
                    };
                    if next_tok.token == Token::Continue { continue }
                    if next_tok.token == Token::Quote {
                        *rquot = Some((*next_tok).clone());
                        ls_iter.next();
                        return Some(quot);
                    } else if next_tok.token == Token::Newline {
                        ls_iter.next();
                        self.errs.push(CSTReport::UnclosedDelimiter(quot));
                        return None;
                    }
                    let Some(stok) = contents else {
                        *contents = Some((*next_tok).clone());
                        continue;
                    };
                    stok.concat((*next_tok).clone());
                    ls_iter.next();
                }
            },

            Token::Number => {
                let mut num_node = CSTNode::Number{ num: ltok.clone() };
                let CSTNode::Number{ ref mut num } = num_node else {
                    self.errs.push(CSTReport::InternalError(IntErrID::FetchCSTNode));
                    return None;
                };
                while self.expect_peek_token(Token::Continue, ls_iter) {
                    ls_iter.next();
                    let Some(ltok) = ls_iter.peek() else {
                        self.errs.push(CSTReport::EndOfFileDuring(Some(num_node.clone())));
                        return Some(num_node);
                    };
                    if ltok.token != Token::Number {
                        self.errs.push(CSTReport::ExpectedXGotY{
                            x: Token::Number,
                            y: (*ltok).clone(),
                        });
                    }
                    num.concat((*ltok).clone());
                }
                if self.consume_space(ls_iter) {
                    let Some(op) = ls_iter.peek() else {
                        self.errs.push(CSTReport::ExpectedXGotY{
                            x: Token::Newline,
                            y: ltok.clone(),
                        });
                        return Some(num_node);
                    };
                    match op.token {
                        Token::Plus | Token::Minus | Token::Star | Token::Slash
                            => {
                                ls_iter.next();
                                self.parse_bin_op(ls_iter, num_node.clone());
                            },
                        _ => {
                        	self.errs.push(CSTReport::ExpectedXGotY{
                        	    x: Token::Newline,
                        	    y: ltok.clone(),
                        	});
                            return Some(num_node);
                        }
                    }
                }
                return Some(num_node);
            },

            Token::Identifier => {
                match ltok.span.str.as_str() {
                    "import" => {
                        let Some(filepath) = self.parse_file_path(ls_iter) else { return None };
                        return Some(CSTNode::Import{
                            indent_sz,
                            key: ltok.clone(),
                            path: Some(Box::new(filepath)),
                        });
                    },

                    "new" => {
                        let Some(filepath) = self.parse_file_path(ls_iter) else { return None };
                        return Some(CSTNode::New{
                            indent_sz,
                            key: ltok.clone(),
                            path: Some(Box::new(filepath)),
                        });
                    },

                    "ascii" => return self.parse_ascii(ls_iter, ltok.clone()),

                    _ => {},
                }
                if !self.expect_peek_token(Token::Dot, ls_iter) { return Some(CSTNode::Label{ name: ltok.clone() }) }
                let Some(dot) = ls_iter.next() else {
                    return Some(CSTNode::Label{ name: ltok.clone(), })
                };
                if dot.token != Token::Dot {
                    self.errs.push(CSTReport::ExpectedXGotY{
                        x: Token::Newline,
                        y: dot.clone(),
                    });
                    return Some(CSTNode::Label{ name: ltok.clone() });
                }
                return self.parse_element(ls_iter, CSTNode::Label{ name: ltok.clone() }, dot.clone(), 0);
            },

            Token::LParen => {
                return self.parse_paren(ls_iter, ltok.clone());
            },

            Token::Continue | Token::Space => self.parse_expr(ls_iter, 0),

            _ => {
                self.errs.push(CSTReport::InvalidExpressionXAfterY{
                    x: ltok.clone(),
                    y: None,
                });
                return None;
            },
        }
    }

    fn parse_element<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self,
        ls_iter: &mut Peekable<I>,
        var: CSTNode<'linespan>,
        dot: LexToken<'linespan>,
        indent_sz: usize,
    ) -> Option<CSTNode<'linespan>> {
        let Some(ltok) = self.expect_consume_token_or(
            Token::Identifier, ls_iter,
            CSTReport::EndOfFileDuring(Some(CSTNode::VarElement{
                indent_sz,
                var: Box::new(var),
                dot,
                element: None
            }))
        ) else { return None; };
        if self.expect_peek_token(Token::LParen, ls_iter) {
            let Some(lparen) = ls_iter.next() else {
                self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee));
                return None;
            };
        } if self.expect_consume_token_or().is_some() {}

        None
    }

    fn parse_file_path<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self,
        ls_iter: &mut Peekable<I>,
    ) -> Option<CSTNode<'linespan>> {
        let mut path: Option<CSTNode<'linespan>> = None;
        let mut is_expect_slash = false;
        loop {
            let Some(ltok) = ls_iter.next() else {
                break;
            };
            let Some(ref mut _path) = path else {
                path = Some(CSTNode::Label{
                    name: ltok.clone(),
                });
                continue;
            };
            let CSTNode::Label{name} = _path else {
                self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee));
                return None;
            };
            match ltok.token {
                Token::Identifier => {
                    if is_expect_slash {
                        self.errs.push(CSTReport::ExpectedXGotY{
                            x: Token::Slash,
                            y: ltok.clone(),
                        });
                    }
                    is_expect_slash = true;
                    if !name.concat(ltok.clone()) { break }
                },
                Token::Slash => {
                    if !is_expect_slash {
                        self.errs.push(CSTReport::ExpectedXGotY{
                            x: Token::Identifier,
                            y: ltok.clone(),
                        });
                    }
                    is_expect_slash = false;
                    if name.concat(ltok.clone()) { break }
                },
                Token::Newline => break,
                Token::Space => {
                    self.errs.push(CSTReport::ExpectedXGotY{
                        x: Token::Newline,
                        y: ltok.clone(),
                    });
                    break;
                },
                _ => if is_expect_slash {
                    self.errs.push(CSTReport::ExpectedXGotY{
                        x: Token::Slash,
                        y: ltok.clone()
                    });
                } else {
                    self.errs.push(CSTReport::ExpectedXGotY{
                        x: Token::Identifier,
                        y: ltok.clone()
                    });
                },
            }
        }

        path
    }

    fn parse_bin_op<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self, ls_iter: &mut Peekable<I>, lhs: CSTNode<'linespan>, 
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
                self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee));
                return false;
            },
        }

        if !self.expect_peek_token(Token::Number, ls_iter) { return false }
        match self.parse_expr(ls_iter, 0) {
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

    fn parse_ascii<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self,
        ls_iter: &mut Peekable<I>,
        key: LexToken<'linespan>,
    ) -> Option<CSTNode<'linespan>> {
        let mut ascii = CSTNode::Ascii{
            key_start: key,
            lines: Vec::new(),
            key_end: None
        };
        let CSTNode::Ascii{
            key_start: _,
            ref mut lines,
            ref mut key_end
        } = ascii else {
            self.errs.push(CSTReport::InternalError(IntErrID::FetchCSTNode));
            return None;
        };
        let mut is_asciiend_check = true;

        loop {
            let Some(ltok) = ls_iter.next() else {
                self.errs.push(CSTReport::UnclosedDelimiter(ascii.clone()));
                return Some(ascii);
            };
            if !is_asciiend_check {
                if ltok.token == Token::Newline { is_asciiend_check = true; }
                continue;
            }
            if ltok.span.str == "asciiend" {
                *key_end = Some(ltok.clone());
                break;
            }
            is_asciiend_check = false;
            lines.push(ltok.span.line.clone());
        }

        Some(ascii)
    }

    fn consume_space<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &self, ls_iter: &mut Peekable<I>
    ) -> bool {
        if self.expect_peek_token(Token::Space, ls_iter) {
            ls_iter.next();
            return true;
        }

        false
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
        return ltok.span.str.len();
    }
}
