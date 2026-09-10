/*
 * NOTE: standardcombo, why the FUCK did you make this language as complex as it is?
 *       Please, I am curious as to why the fuck you ever made me come through this hellhole.
 *		 I ask of you, I sure as hell ask of you, to please translate this Parser code to C#,
 *		 and never speak of the old one. I BEG, dear standardcombo, your unrelenting implementations.
 *		 Why oh why have you made me carry such burden? You fucking bitch.
 * - ESplash (09.09.2026)
 */

use std::iter::Peekable;

use crate::internals::{
    LexerOutput,
    LineSpan,
    LexToken,
    Token,
};

pub enum PrintType {
    Normal,
    BigHead,
    AdvNormal,
    AdvCenter,
    AdvFoe,
    AdvHead,
    AdvHat,
}

#[derive(Clone)]
pub enum OpType {
    Increment,
    Decrement,
    AddAssign,
    MinusAssign,
    TimesAssign,
    DivideAssign,
}

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
        var:   Option<Box<CSTNode<'linespan>>>,
        comma: Option<LexToken<'linespan>>,
    },
    Paren{
        lparen:   LexToken<'linespan>,
        contents: Vec<CSTNode<'linespan>>,
        rparen:   Option<LexToken<'linespan>>,
    },
    Brack{
        lbrack:   LexToken<'linespan>,
        contents: Vec<CSTNode<'linespan>>,
        rbrack:   Option<LexToken<'linespan>>,
    },

    BinOp{
        lhs: Box<CSTNode<'linespan>>,
        op:  LexToken<'linespan>,
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

    VarFetch{
        at1: LexToken<'linespan>,
        var: Option<Box<CSTNode<'linespan>>>,
        at2: Option<LexToken<'linespan>>,
    },

    VarMutator{
        indent_sz: usize,
        name: LexToken<'linespan>,
        operator: Option<OpType>,
        amount: Option<Box<CSTNode<'linespan>>>,
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
        key:       LexToken<'linespan>,
        bin_op:    Option<Box<CSTNode<'linespan>>>,
    },
    Else{
        indent_sz: usize,
        colon:     LexToken<'linespan>,
        if_stmnt:  Option<Box<CSTNode<'linespan>>>,
    },

    Ascii{
        key_start: LexToken<'linespan>,
        lines:     Vec<LineSpan>,
        key_end:   Option<LexToken<'linespan>>,
    },

    Equip{
        indent_sz: usize,
        key: LexToken<'linespan>,
        tool_name: Option<Box<CSTNode<'linespan>>>,
        minus: Option<Box<CSTNode<'linespan>>>,
        star: Option<Box<CSTNode<'linespan>>>,
        plus: Option<Box<CSTNode<'linespan>>>,
    },

    Enable{
        indent_sz: usize,
        key: LexToken<'linespan>,
        opt: Option<Box<CSTNode<'linespan>>>,
        opt_opts: Option<Box<CSTNode<'linespan>>>,
    },
    Disable{
        indent_sz: usize,
        key: LexToken<'linespan>,
        opt: Option<Box<CSTNode<'linespan>>>,
        opt_opts: Option<Box<CSTNode<'linespan>>>,
    },

    Brew{
        indent_sz: usize,
        key: LexToken<'linespan>,
        ingr1: Option<Box<CSTNode<'linespan>>>,
        plus: Option<LexToken<'linespan>>,
        ingr2: Option<Box<CSTNode<'linespan>>>,
    },

    Loadout{
        indent_sz: usize,
        key: LexToken<'linespan>,
        num: Option<Box<CSTNode<'linespan>>>,
    },

    Activate{
        indent_sz: usize,
        key: LexToken<'linespan>,
        opt: Option<Box<CSTNode<'linespan>>>,
    },

    Play{
        indent_sz: usize,
        key: LexToken<'linespan>,
        sound: Option<Box<CSTNode<'linespan>>>,
        pitch: Option<Box<CSTNode<'linespan>>>,
    },

    Print{
        indent_sz: usize,
        key: LexToken<'linespan>,
        spcl_key: Option<LexToken<'linespan>>,
        spcl_x_pos: Option<Box<CSTNode<'linespan>>>,
        spcl_y_pos: Option<Box<CSTNode<'linespan>>>,
        spcl_clr_key: Option<LexToken<'linespan>>,
        spcl_clr_val: Option<LexToken<'linespan>>,
        contents: Vec<CSTNode<'linespan>>
    }
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

    InvalidExpressionX{ x: Option<CSTNode<'linespan>> },

    ImplicitPrintCastInLine{ x: usize },

    EndOfFileDuring(Option<CSTNode<'linespan>>),
    UnfinishedStatement(CSTNode<'linespan>),
    UnclosedDelimiter(CSTNode<'linespan>),
}

pub struct CSTOutput<'linespan>{
    output: Vec<CSTNode<'linespan>>,
    errs: Vec<CSTReport<'linespan>>,
}

const fn is_hexadecimal(c: char) -> bool {
    (c >= '0' && c <= '9') || (c >= 'A' && c <= 'F') || (c >= 'a' && c <= 'f')
}

impl<'linespan> CSTOutput<'linespan> {
    pub fn new() -> CSTOutput<'linespan> {
        CSTOutput{
            output: Vec::new(),
            errs: Vec::new(),
        }
    }

    pub fn parse_all(&mut self, lex_output: &'linespan mut LexerOutput) -> bool {
        let mut ls_iter = lex_output.0.iter().peekable();
        let mut is_expect_newline = false;
        loop {
            let Some(peek) = ls_iter.peek() else { break };
            match peek.token {
                Token::Space | Token::Identifier
                    => if !self.parse_identifier(&mut ls_iter) { break },

                Token::Lesser => {
                    if is_expect_newline {
                        self.errs.push(CSTReport::ExpectedXGotY{
                            x: Token::Newline,
                            y: (*peek).clone(),
                        });
                    }
                    is_expect_newline = true;
                    let _peek = (*peek).clone();
                    if !self.parse_print(&mut ls_iter, _peek, 0) { break }
                    ls_iter.next();
                },

                Token::Colon | Token::QuestMark => {
                    if is_expect_newline {
                        self.errs.push(CSTReport::ExpectedXGotY{
                            x: Token::Newline,
                            y: (*peek).clone(),
                        });
                    }
                    is_expect_newline = true;
                    let _peek = (*peek).clone();
                    if !self.parse_if_stmnt(&mut ls_iter, _peek, 0) { break }
                    ls_iter.next();
                },

                Token::Newline => {
                    is_expect_newline = false;
                    continue;
                },

                Token::Comment | Token::MultiComment => continue,

                _ => {
                    self.errs.push(CSTReport::ExpectedXGotY{
                        x: if is_expect_newline { Token::Newline } else { Token::Identifier },
                        y: (*peek).clone(),
                    });
                },
            }
        }

        true
    }

    fn parse_identifier<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self, ls_iter: &mut Peekable<I>
    ) -> bool {
        let indent_sz = self.consume_indent_sz(ls_iter);

        let Some(opt_key) = ls_iter.peek() else {
            self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee));
            return false;
        };
        let key  = (*opt_key).clone();

        if key.span.str == "var" {
            ls_iter.next();
            return self.parse_var_decl(ls_iter, key, indent_sz);
        } else if key.span.str == "func" {
            ls_iter.next();
            return self.parse_func_decl(ls_iter, key.clone(), indent_sz);
        }

        let Some(name) = self.expect_consume_token_or(
            Token::Identifier, ls_iter,
            CSTReport::EndOfFileDuring(Some(CSTNode::VarDecl{
                indent_sz,
                key: key.clone(),
                name: None,
                equal: None,
                def: None
            }))
        ) else { return false };

        if name.token == Token::Identifier {
            match name.span.str.as_str() {
                "enable" => {
                    match ls_iter.next() {
                        Some(space) => match space.token {
                            Token::Space => {},
                            _ => {
                                let enable_node = CSTNode::Enable{
                                    indent_sz, key: name.clone(), opt: None, opt_opts: None
                                };
                                self.output.push(enable_node);
                                self.errs.push(CSTReport::ExpectedXGotY{
                                    x: Token::Space,
                                    y: space.clone(),
                                });
                                return false;
                            }
                        },
                        None => {
                            let enable_node = CSTNode::Enable{
                                indent_sz, key: name.clone(), opt: None, opt_opts: None
                            };
                            self.output.push(enable_node.clone());
                            self.errs.push(CSTReport::EndOfFileDuring(Some(enable_node)));
                            return false;
                        },
                    }

                    let Some(ident) = self.parse_expr(ls_iter, 0) else { return false };
                    match ident {
                        CSTNode::Label{ name: _ } | CSTNode::VarFetch{
                            at1: _,
                            var: _,
                            at2: _,
                        } => {},
                        _ => self.errs.push(CSTReport::InvalidExpressionX{ x: Some(ident.clone()) }),
                    }

                    if self.consume_space(ls_iter) {
                        let Some(opt) = self.parse_expr(ls_iter, 0) else { return false };
                        match opt {
                            CSTNode::Label{ name: _ } | CSTNode::VarFetch{
                                at1: _,
                                var: _,
                                at2: _,
                            } => {},
                            _ => self.errs.push(CSTReport::InvalidExpressionX{ x: Some(ident.clone()) }),
                        }
                        self.output.push(CSTNode::Enable{
                            indent_sz, key: name.clone(), opt: Some(Box::new(ident.clone())), opt_opts: Some(Box::new(opt.clone())),
                        });
                    }

                    self.output.push(CSTNode::Enable{
                        indent_sz, key: name.clone(), opt: Some(Box::new(ident.clone())), opt_opts: None
                    });
                    return false;
                },

                "disable" => {
                    match ls_iter.next() {
                        Some(space) => match space.token {
                            Token::Space => {},
                            _ => {
                                let enable_node = CSTNode::Disable{
                                    indent_sz, key: name.clone(), opt: None, opt_opts: None
                                };
                                self.output.push(enable_node);
                                self.errs.push(CSTReport::ExpectedXGotY{
                                    x: Token::Space,
                                    y: space.clone(),
                                });
                                return false;
                            }
                        },
                        None => {
                            let disable_node = CSTNode::Disable{
                                indent_sz, key: name.clone(), opt: None, opt_opts: None
                            };
                            self.output.push(disable_node.clone());
                            self.errs.push(CSTReport::EndOfFileDuring(Some(disable_node)));
                            return false;
                        },
                    }

                    let Some(ident) = self.parse_expr(ls_iter, 0) else { return false };
                    match ident {
                        CSTNode::Label{ name: _ } | CSTNode::VarFetch{
                            at1: _,
                            var: _,
                            at2: _,
                        } => {},
                        _ => self.errs.push(CSTReport::InvalidExpressionX{ x: Some(ident.clone()) }),
                    }

                    if self.consume_space(ls_iter) {
                        let Some(opt) = self.parse_expr(ls_iter, 0) else { return false };
                        match opt {
                            CSTNode::Label{ name: _ } | CSTNode::VarFetch{
                                at1: _,
                                var: _,
                                at2: _,
                            } => {},
                            _ => self.errs.push(CSTReport::InvalidExpressionX{ x: Some(ident.clone()) }),
                        }
                        self.output.push(CSTNode::Disable{
                            indent_sz, key: name.clone(), opt: Some(Box::new(ident.clone())), opt_opts: Some(Box::new(opt.clone())),
                        });
                    }

                    self.output.push(CSTNode::Disable{
                        indent_sz, key: name.clone(), opt: Some(Box::new(ident.clone())), opt_opts: None
                    });
                    return false;
                },

                "brew" => {
                    if !self.expect_consume_space(
                        ls_iter,
                        CSTReport::EndOfFileDuring(Some(CSTNode::Brew{
                            indent_sz,
                            key: name.clone(),
                            ingr1: None,
                            plus: None,
                            ingr2: None,
                        }))
                    ) { return false }

                    let Some(ingr1) = self.parse_expr(ls_iter, 0) else { return false };
                    match ingr1 {
                        CSTNode::Label{ name: _ } | CSTNode::VarFetch{
                            at1: _,
                            var: _,
                            at2: _,
                        } => {},
                        _ => self.errs.push(CSTReport::InvalidExpressionX{ x: Some(ingr1.clone()), }),
                    }

                    self.consume_space(ls_iter);
                    let Some(plus) = self.expect_consume_token_or(
                        Token::Plus, ls_iter,
                        CSTReport::EndOfFileDuring(Some(CSTNode::Brew{
                            indent_sz,
                            key: name.clone(),
                            ingr1: Some(Box::new(ingr1.clone())),
                            plus: None,
                            ingr2: None,
                        }))
                    ) else { return false };

                    self.consume_space(ls_iter);
                    let Some(ingr2) = self.parse_expr(ls_iter, 0) else { return false };
                    match ingr2 {
                        CSTNode::Label{ name: _ } | CSTNode::VarFetch{
                            at1: _,
                            var: _,
                            at2: _,
                        } => {},
                        _ => self.errs.push(CSTReport::InvalidExpressionX{ x: Some(ingr1.clone()), }),
                    }
                    self.output.push(CSTNode::Brew{
                        indent_sz,
                        key: name.clone(),
                        ingr1: Some(Box::new(ingr1.clone())),
                        plus: Some(plus.clone()),
                        ingr2: Some(Box::new(ingr2.clone())),
                    });
                },

                "loadout" => {
                    if !self.expect_consume_space(
                        ls_iter,
                        CSTReport::EndOfFileDuring(Some(CSTNode::Loadout{
                            indent_sz, key: name.clone(), num: None,
                        })),
                    ) { return false }
                    let Some(num) = self.parse_expr(ls_iter, 0) else { return false };
                    match num {
                        CSTNode::Label{ name: _ } | CSTNode::VarFetch{
                            at1: _,
                            var: _,
                            at2: _,
                        } => {}
                        _ => self.errs.push(CSTReport::InvalidExpressionX{ x: Some(num.clone()) }),
                    }
                    self.output.push(CSTNode::Loadout{
                        indent_sz, key: name.clone(), num: Some(Box::new(num.clone())),
                    });
                },

                "activate" => {
                    if !self.expect_consume_space(
                        ls_iter,
                        CSTReport::EndOfFileDuring(Some(CSTNode::Activate{
                            indent_sz,
                            key: name.clone(),
                            opt: None
                        }))
                    ) { return false }
                    let Some(opt) = self.parse_expr(ls_iter, 0) else { return false };
                    match opt {
                        CSTNode::Label{ name: _ } | CSTNode::VarFetch{
                            at1: _,
                            var: _,
                            at2: _,
                        } => {},
                        _ => self.errs.push(CSTReport::InvalidExpressionX{ x: Some(opt.clone()) }),
                    }
                    self.output.push(CSTNode::Activate{
                        indent_sz, key: name.clone(), opt: Some(Box::new(opt.clone())),
                    });
                },

                "play" => {
                    if !self.expect_consume_space(
                        ls_iter,
                        CSTReport::EndOfFileDuring(Some(CSTNode::Play{
                            indent_sz,
                            key: name.clone(),
                            sound: None,
                            pitch: None,
                        }))
                    ) { return false }

                    let Some(sound) = self.parse_expr(ls_iter, 0) else { return false };
                    match sound {
                        CSTNode::Label{ name: _ } | CSTNode::VarFetch{
                            at1: _,
                            var: _,
                            at2: _,
                        } => {},
                        _ => {
                        },
                    }

                    if !self.consume_space(ls_iter) {
                        self.output.push(CSTNode::Play{
                            indent_sz,
                            key: name.clone(),
                            sound: Some(Box::new(sound.clone())),
                            pitch: None,
                        });
                        return true;
                    }

                    let Some(pitch) = self.parse_expr(ls_iter, 0) else { return false };
                    match pitch {
                        CSTNode::Label{ name: _ } | CSTNode::VarFetch{
                            at1: _,
                            var: _,
                            at2: _,
                        } => {},
                        _ => self.errs.push(CSTReport::InvalidExpressionX{ x: Some(pitch.clone()) }),
                    }

                    self.output.push(CSTNode::Play{
                        indent_sz,
                        key: name.clone(),
                        sound: Some(Box::new(sound.clone())),
                        pitch: Some(Box::new(pitch.clone())),
                    });
                },

                "equip" => return self.parse_equip(ls_iter, name.clone(), indent_sz),

                _ => {},
            }
        } else {
            match name.token {
                Token::Lesser => return self.parse_print(ls_iter, name.clone(), indent_sz),
                Token::Colon | Token::QuestMark => return self.parse_if_stmnt(ls_iter, name.clone(), indent_sz),
                _ => self.errs.push(CSTReport::ExpectedXGotY{
                    x: Token::Identifier,
                    y: name.clone()
                }),
            }
        }

        self.consume_space(ls_iter);
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
            Token::LParen => return self.parse_func_call(ls_iter, name.clone(), ltok.clone(), indent_sz),

			Token::Plus => {
                let Some(equal_or_plus) = ls_iter.peek() else {
                    self.errs.push(CSTReport::EndOfFileDuring(Some(CSTNode::BinOp{
                        lhs: Box::new(CSTNode::Label{ name }),
                        op: ltok.clone(),
                        rhs: None
                    })));
                    return false;
                };

                match equal_or_plus.token {
                    Token::Plus => {
                        self.output.push(CSTNode::VarMutator{
                            indent_sz, name,
                            operator: Some(OpType::Increment),
                            amount: None,
                        });
                        ls_iter.next();
                    },

                    Token::Equal => {
                        self.consume_space(ls_iter);
                        let Some(expr) = self.parse_expr(ls_iter, 0) else { return false };
                        self.output.push(CSTNode::VarMutator{
                            indent_sz, name,
                            operator: Some(OpType::AddAssign),
                            amount: Some(Box::new(expr)),
                        });
                        ls_iter.next();
                    },

                    _ => {
                        self.errs.push(CSTReport::ExpectedXGotY{
                    	    x: Token::Plus,
                    	    y: (*equal_or_plus).clone()
                    	});
                        return false;
                    },
                }

                return true;
            },

            Token::Dash => {
                let Some(equal_or_dash) = ls_iter.peek() else {
                    self.errs.push(CSTReport::EndOfFileDuring(Some(CSTNode::BinOp{
                        lhs: Box::new(CSTNode::Label{ name }),
                        op: ltok.clone(),
                        rhs: None,
                    })));
                    return false;
                };

                match equal_or_dash.token {
                    Token::Dash => {
                        self.output.push(CSTNode::VarMutator{
                            indent_sz, name,
                            operator: Some(OpType::Decrement),
                            amount: None,
                        });
                        ls_iter.next();
                    },

                    Token::Equal => {
                        self.consume_space(ls_iter);
                        let Some(expr) = self.parse_expr(ls_iter, 0) else { return false };
                        match expr {
                            CSTNode::Number{ num: _ } | CSTNode::VarFetch{
                                at1: _,
                                var: _,
                                at2: _,
                            } => {},
                            _ => {
                                self.errs.push(CSTReport::InvalidExpressionX{ x: Some(expr) });
                                return false;
                            },
                        }
                        self.output.push(CSTNode::VarMutator{
                            indent_sz, name,
                            operator: Some(OpType::MinusAssign),
                            amount: Some(Box::new(expr)),
                        });
                    },

                    _ => {
                        self.errs.push(CSTReport::ExpectedXGotY{
                            x: Token::Dash,
                            y: (*equal_or_dash).clone()
                        });
                        return false;
                    },
                }

                return true;
            },

            Token::Star => {
                let Some(equal) = self.expect_consume_token_or(
                    Token::Equal, ls_iter,
                    CSTReport::EndOfFileDuring(Some(CSTNode::BinOp{
                        lhs: Box::new(CSTNode::Label{ name: name.clone() }),
                        op: ltok.clone(),
                        rhs: None
                    }))
                ) else { return false };

                let Some(expr) = self.parse_expr(ls_iter, 0) else { return false };
                match expr {
                    CSTNode::Number{ num: _ } | CSTNode::VarFetch{
                        at1: _,
                        var: _,
                        at2: _,
                    } => {},
                    _ => self.errs.push(CSTReport::InvalidExpressionX{ x: Some(expr.clone()) }),
                }

                self.output.push(CSTNode::VarMutator{
                    indent_sz, name,
                    operator: Some(OpType::TimesAssign),
                    amount: Some(Box::new(expr)),
                });

                return true;
            },

            Token::Slash => {
				if self.expect_consume_token_or(
                	Token::Equal, ls_iter,
                	CSTReport::EndOfFileDuring(Some(CSTNode::BinOp{
                	    lhs: Box::new(CSTNode::Label{ name: name.clone() }),
                	    op: ltok.clone(),
                	    rhs: None
                	}
                ))).is_none() { return false };

                let Some(expr) = self.parse_expr(ls_iter, 0) else { return false };
                match expr {
                    CSTNode::Number{ num: _ } | CSTNode::VarFetch{
                        at1: _,
                        var: _,
                        at2: _,
                    } => {},
                    _ => self.errs.push(CSTReport::InvalidExpressionX{ x: Some(expr.clone()) }),
                }

                self.output.push(CSTNode::VarMutator{
                    indent_sz, name,
                    operator: Some(OpType::DivideAssign),
                    amount: Some(Box::new(expr)),
                });
                
                return true;
            },

            Token::Dot => {
                let Some(method) = self.parse_element(
                    ls_iter,
                    CSTNode::Label{ name: name.clone() },
                    ltok.clone(),
                    0,
                ) else { return false };
                match method {
                    CSTNode::VarElement{
                        indent_sz: _,
                        var: _,
                        dot: _,
                        element: _,
                    } => {
                        match ls_iter.peek() {
                            Some(ltok) => match ltok.token {
                                Token::Newline => {
                                    ls_iter.next();
                                    self.output.push(method.clone());
                                    return true;
                                },

                                Token::Space | Token::Equal => {},

                                _ => {
                                    self.errs.push(CSTReport::ExpectedXGotY{
                                        x: Token::Newline,
                                        y: (*ltok).clone(),
                                    });
                                    return false;
                                }
                            }

                            None => {
                                self.output.push(method);
                                return true;
                            },
                        }

                        let Some(ltok) = ls_iter.next() else {
                            self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee));
                            return false;
                        };

                        match ltok.token {
                            Token::Space => {
                                let Some(equal) = self.expect_consume_token_or(
                                    Token::Equal, ls_iter,
                                    CSTReport::ExpectedXGotY{
                                        x: Token::Newline,
                                        y: ltok.clone()
                                    },
                                ) else { return false; };
                                let Some(expr) = self.parse_expr(ls_iter, 0) else {
                                    self.output.push(CSTNode::VarDef{
                                        indent_sz,
                                        name: Box::new(method),
                                        equal: Some(equal),
                                        def: None
                                    });
                                    return false;
                                };
                                self.output.push(CSTNode::VarDef{
                                    indent_sz,
                                    name: Box::new(method),
                                    equal: Some(equal),
                                    def: Some(Box::new(expr)),
                                });
                            },

                            Token::Equal => {
                                self.consume_space(ls_iter);
                                let Some(expr) = self.parse_expr(ls_iter, 0) else {
                                    self.output.push(CSTNode::VarDef{
                                        indent_sz,
                                        name: Box::new(method),
                                        equal: Some(ltok.clone()),
                                        def: None
                                    });
                                    return false;
                                };
                                self.output.push(CSTNode::VarDef{
                                    indent_sz,
                                    name: Box::new(method),
                                    equal: Some(ltok.clone()),
                                    def: Some(Box::new(expr)),
                                });
                            },

                            _ => {
                                self.errs.push(CSTReport::ExpectedXGotY{
                                    x: Token::Newline,
                                    y: ltok.clone(),
                                });
                                return false;
                            },
                        }

                        return true;
                   },

                    CSTNode::VarMethod{
                        indent_sz: _,
                        var: _,
                        dot: _,
                        method: _,
                    } => {
                        if !self.expect_peek_token(Token::Newline, ls_iter) {
                            let Some(ltok) = ls_iter.next() else {
                                self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee));
                                return false;
                            };
                            self.errs.push(CSTReport::ExpectedXGotY{
                                x: Token::Newline,
                                y: ltok.clone(),
                            });
                            return false;
                        }
                        return true;
                    },

                    _ => {
                        self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee));
                        return false;
                    },
                }
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
                self.consume_space(ls_iter);
                let Some(equal) = self.expect_consume_token_or(
                    Token::Equal, ls_iter,
                    CSTReport::UnfinishedStatement(CSTNode::VarDef{
                        indent_sz,
                        name: Box::new(CSTNode::Label{ name: name.clone() }),
                        equal: None,
                        def: None,
                    })
                ) else { return false; };
                self.consume_space(ls_iter);
                let Some(expr) = self.parse_expr(ls_iter, 0) else { return false };
                self.output.push(CSTNode::VarDef{
                    indent_sz,
                    name: Box::new(CSTNode::Label{ name: name.clone() }),
                    equal: Some(equal),
                    def: Some(Box::new(expr)),
                });
                return true;
            }
        }
    }

    fn parse_print_param<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self,
        ls_iter: &mut Peekable<I>,
        print_node: &mut CSTNode<'linespan>,
    ) -> bool {
        let _print_node = print_node.clone();
        let CSTNode::Print{
            indent_sz: _,
            key: _,
            spcl_key,
            spcl_x_pos,
            spcl_y_pos,
            spcl_clr_key,
            spcl_clr_val,
            contents,
        } = print_node else {
            self.errs.push(CSTReport::InternalError(IntErrID::FetchCSTNode));
            return false;
        };

        /* NOTE: Since Print can be implicitly cast, we can prepare for it. */
        let Some(_spcl_key) = spcl_key else {
            self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee));
            return false;
        };

        self.consume_space(ls_iter);

        let Some(x_pos) = self.parse_expr(ls_iter, 0) else {
            *spcl_key = None;
            return false;
        };
        contents.push(x_pos.clone());
        match x_pos {
            CSTNode::Number{ num: _ } | CSTNode::VarFetch{
                at1: _,
                var: _,
                at2: _,
            } => *spcl_x_pos = Some(Box::new(x_pos.clone())),
            _ => {
                *spcl_key = None;
                self.errs.push(CSTReport::InvalidExpressionX{ x: Some(x_pos.clone()) });
                return false;
            }
        }

        self.consume_space(ls_iter);

        let Some(comma1) = self.expect_consume_token_or(
            Token::Comma, ls_iter,
            CSTReport::EndOfFileDuring(Some(_print_node.clone()))
        ) else {
            *spcl_x_pos = None;
            *spcl_key = None;
            return false;
        };
        contents.push(CSTNode::Label{ name: comma1 });

        self.consume_space(ls_iter);

        let Some(y_pos) = self.parse_expr(ls_iter, 0) else {
            *spcl_x_pos = None;
            *spcl_key = None;
            return false;
        };
        contents.push(y_pos.clone());
        match y_pos {
            CSTNode::Number{ num: _ } | CSTNode::VarFetch{
                at1: _,
                var: _,
                at2: _,
            } => *spcl_y_pos = Some(Box::new(y_pos.clone())),
            _ => {
                *spcl_x_pos = None;
                *spcl_key = None;
                self.errs.push(CSTReport::InvalidExpressionX{ x: Some(y_pos.clone()) });
                return false;
            }
        }

        self.consume_space(ls_iter);

        let Some(comma2) = self.expect_consume_token_or(
            Token::Comma, ls_iter,
            CSTReport::EndOfFileDuring(Some(_print_node))
        ) else {
            *spcl_y_pos = None;
            *spcl_x_pos = None;
            *spcl_key = None;
            return false;
        };
        contents.push(CSTNode::Label{ name: comma2 });

        match ls_iter.next() {
            Some(ltok) if ltok.token == Token::Hashtag
                => {
                    contents.push(CSTNode::Label{ name: ltok.clone() });
                    *spcl_clr_key = Some(ltok.clone())
                },
            None | Some(_) => {
                contents.clear();
                return true;
            },
        }

        match ls_iter.next() {
            Some(ltok)
                if ltok.token == Token::Identifier
                && ltok.span.str.len() == 6
            => {
                let mut lchars = ltok.span.str.chars();
                let rain: String = lchars.clone().take(4).collect();
                contents.push(CSTNode::Label{ name: ltok.clone() });
                if rain == "rain" {
                    let Some(fifth) = lchars.nth(4) else {
                        *spcl_clr_key = None;
                        *spcl_y_pos = None;
                        *spcl_x_pos = None;
                        *spcl_key = None;
                        self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee));
                        return false;
                    };
                    if !is_hexadecimal(fifth) {
                        *spcl_clr_key = None;
                        *spcl_y_pos = None;
                        *spcl_x_pos = None;
                        *spcl_key = None;
                        return false;
                    }
                    let Some(sixth) = lchars.nth(5) else {
                        *spcl_clr_key = None;
                        *spcl_y_pos = None;
                        *spcl_x_pos = None;
                        *spcl_key = None;
                        self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee));
                        return false;
                    };
                    if !is_hexadecimal(sixth) {
                        *spcl_clr_key = None;
                        *spcl_y_pos = None;
                        *spcl_x_pos = None;
                        *spcl_key = None;
                        return false;
                    }
                    *spcl_clr_val = Some(ltok.clone());
                } else {
                    if lchars.all(|c| is_hexadecimal(c)) {
                        *spcl_clr_val = Some(ltok.clone());
                    } else {
                        *spcl_clr_key = None;
                        *spcl_y_pos = None;
                        *spcl_x_pos = None;
                        *spcl_key = None;
                        return false;
                    }
                }
            },
            Some(ltok) => {
                *spcl_clr_key = None;
                *spcl_y_pos = None;
                *spcl_x_pos = None;
                *spcl_key = None;
                return false;
            },
            None => {
                let Some(clr_key) = spcl_clr_key.clone() else { return false };
                contents.push(CSTNode::Label{ name: clr_key });
                *spcl_clr_key = None;
            },
        }

        match ls_iter.next() {
            Some(ltok) if ltok.token == Token::Comma => {
                contents.clear();
                return true
            },
            Some(_) | None => {
                *spcl_clr_val = None;
                *spcl_clr_key = None;
                *spcl_y_pos = None;
                *spcl_x_pos = None;
                *spcl_key = None;
                return false;
            },
        }
    }

    fn parse_print<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self,
        ls_iter: &mut Peekable<I>,
        r_angle_brack: LexToken<'linespan>,
        indent_sz: usize,
    ) -> bool {
        let line_num = r_angle_brack.span.line.get_num();

        let mut print_node = CSTNode::Print{
            indent_sz,
            key: r_angle_brack.clone(),
            spcl_key: None,
            spcl_x_pos: None,
            spcl_y_pos: None,
            spcl_clr_key: None,
            spcl_clr_val: None,
            contents: vec![],
        };

        let mut _print_node = print_node.clone();

        let CSTNode::Print{
            indent_sz: _,
            key: _,
            ref mut spcl_key,
            spcl_x_pos: _,
            spcl_y_pos: _,
            spcl_clr_key: _,
            spcl_clr_val: _,
            ref mut contents,
        } = print_node else {
            self.errs.push(CSTReport::InternalError(IntErrID::FetchCSTNode));
            return false;
        };

        let mut parse_param = false;
        match ls_iter.peek() {
            Some(ltok) => match ltok.token {
                Token::Identifier => {
                    match ltok.span.str.as_str() {
                        "o" | "h" | "c" | "f" => parse_param = true,
                        _ => {}
                    }
                },

                Token::Tilde => parse_param = true,

                Token::LParen => {
                    let _ltok = ltok.clone();
                    drop(ltok);
                    let Some(face) = self.expect_consume_token_or(
                        Token::Identifier, ls_iter,
                        CSTReport::EndOfFileDuring(Some(_print_node.clone()))
                    ) else { return false };
                    if face.span.str.len() != 4 {
                        self.errs.push(CSTReport::ImplicitPrintCastInLine{ x: line_num });
                        contents.push(CSTNode::Label{ name: face.clone() });
                        contents.push(CSTNode::Label{ name: face.clone() });
                        ls_iter.next();
                    } else {
                        *spcl_key = Some(_ltok.clone());
                        self.output.push(print_node);
                        return true;
                    }
                },

                _ => {},
            },

            None => {
                self.output.push(print_node);
                return true;
            },
        }

        if parse_param && !self.parse_print_param(ls_iter, &mut print_node) {
            self.errs.push(CSTReport::ImplicitPrintCastInLine{ x: line_num });
        }

        let mut is_ascii = true;

        loop {
            let Some(peek) = ls_iter.peek() else { break };
            match peek.token {
                Token::Newline => {
                    ls_iter.next();
                    break;
                },

                Token::Continue => continue,

                Token::Identifier if peek.span.str == "ascii"  && is_ascii => {
                    let Some(ascii) = self.parse_expr(ls_iter, 0) else {
                        self.output.push(print_node);
                        return false;
                    };
                    if let CSTNode::Ascii{ key_start: _, lines: _, key_end: _ } = ascii {
                        break;
                    } else {
                        self.output.push(print_node);
                        return false;
                    }
                },

                _ => {
                    is_ascii = false;
                    self.output.push(CSTNode::Label{ name: (*peek).clone() })
                },
            }
        }
        self.output.push(print_node);

        true
    }

    /* NOTE: This implicitly supports else statements as well. */
    fn parse_if_stmnt<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self,
        ls_iter: &mut Peekable<I>,
        if_or_else: LexToken<'linespan>,
        indent_sz: usize
    ) -> bool {
        let mut is_else = false;
        if if_or_else.token == Token::Colon {
            match ls_iter.peek() {
                Some(peek) => match peek.token {
                    Token::Newline => {
                        self.output.push(CSTNode::Else{
                            indent_sz,
                            colon: if_or_else,
                            if_stmnt: None
                        });
                        return true;
                    },
                    Token::QuestMark => is_else = true,
                    _ => {
                        self.errs.push(CSTReport::ExpectedXGotY{
                            x: Token::Newline,
                            y: (*peek).clone(),
                        });
                        return false;
                    },
                },
                None => {
                    self.output.push(CSTNode::Else{
                        indent_sz,
                        colon: if_or_else,
                        if_stmnt: None
                    });
                    return true;
                },
            }
        } else if if_or_else.token != Token::QuestMark {
            self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee));
            return false;
        }

        let mut if_stmnt = CSTNode::If{
            indent_sz: if is_else { 0 } else { indent_sz },
            key: if !is_else { if_or_else.clone() } else {
                let Some(question_mark) = ls_iter.next() else {
                    self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee));
                    return false;
                };
                question_mark.clone()
            },
            bin_op: None,
        };
        let CSTNode::If{
            indent_sz: _,
            key: _,
            ref mut bin_op,
        } = if_stmnt else {
            self.errs.push(CSTReport::InternalError(IntErrID::FetchCSTNode));
            return false;
        };

        self.consume_space(ls_iter);
        let Some(lhs) = self.parse_expr(ls_iter, 0) else { return false };
        match ls_iter.peek() {
            Some(op) => match op.token {
                Token::Equal | Token::LessEq | Token::GreatEq | Token::Lesser | Token::Greater
                    => {
                        let _op = (*op).clone();
                        let Some(_bin_op) = self.parse_bin_op(ls_iter, lhs, _op) else { return false };
                        *bin_op = Some(Box::new(_bin_op));
                        self.output.push(if is_else {
                            CSTNode::Else{
                                indent_sz,
                                colon: if_or_else,
                                if_stmnt: Some(Box::new(if_stmnt)),
                            }
                        } else { if_stmnt });
                        return true;
                    },
                Token::Newline => {
                    self.output.push(if is_else {
                        CSTNode::Else{
                            indent_sz,
                            colon: if_or_else,
                            if_stmnt: Some(Box::new(if_stmnt)),
                        }
                    } else { if_stmnt });
                    return true;
                },
                _ => {
                    self.errs.push(CSTReport::ExpectedXGotY{
                        x: Token::Newline,
                        y: (*op).clone(),
                    });
                    return false;
                },
            },
            None => {
                self.errs.push(CSTReport::EndOfFileDuring(Some(if is_else {
                    CSTNode::Else{
                        indent_sz,
                        colon: if_or_else,
                        if_stmnt: Some(Box::new(if_stmnt)),
                    }
                } else { if_stmnt })));
                return false;
            },
        }
    }

    fn parse_equip<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self,
        ls_iter: &mut Peekable<I>,
        key: LexToken<'linespan>,
        indent_sz: usize
    ) -> bool {
        let mut equip_node = CSTNode::Equip{
            indent_sz,
            key: key.clone(),
            tool_name: None,
            plus: None,
            star: None,
            minus: None,
        };
        let CSTNode::Equip{
            indent_sz: _, key: _,
            ref mut tool_name,
            ref mut plus,
            ref mut star,
            ref mut minus,
        } = equip_node else {
            self.errs.push(CSTReport::InternalError(IntErrID::FetchCSTNode));
            return false;
        };
        let mut is_expect_space = true;

        loop {
            let Some(ltok) = ls_iter.peek() else {
                self.output.push(equip_node);
                return true;
            };

            match ltok.token {
                Token::Space => {
                    if is_expect_space {
                        self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee));
                    }
                    is_expect_space = false;
                    continue;
                },

                Token::Identifier => {
                    if tool_name.is_some() {
                        self.errs.push(CSTReport::ExpectedXGotY{
                            x: if plus.is_none() { Token::Plus }
                            else if star.is_some() { Token::Star }
                            else if minus.is_none() { Token::Dash }
                            else { Token::Newline },
                            y: (*ltok).clone(),
                        });
                        continue;
                    }
                    *tool_name = Some(Box::new(CSTNode::Label{ name: (*ltok).clone() }));
                    is_expect_space = true;
                    ls_iter.next();
                },

                Token::At => {
                    let Some(expr) = self.parse_expr(ls_iter, 0) else { continue };
                    match expr {
                        CSTNode::Label{ name: _ } | CSTNode::VarFetch{
                            at1: _,
                            var: _,
                            at2: _,
                        } => {},
                        _ => self.errs.push(CSTReport::InvalidExpressionX{ x: Some(expr.clone()) }),
                    }
                    *tool_name = Some(Box::new(expr.clone()));
                },

                Token::Plus => {
                    let Some(expr) = self.parse_expr(ls_iter, 0) else { continue };
                    match expr {
                        CSTNode::Label{ name: _ } | CSTNode::VarFetch{
                            at1: _,
                            var: _,
                            at2: _,
                        } => {},
                        _ => self.errs.push(CSTReport::InvalidExpressionX{ x: Some(expr.clone()) }),
                    }
                    *plus = Some(Box::new(expr.clone()));
                    ls_iter.next();
                },

                Token::Star => {
                    let Some(expr) = self.parse_expr(ls_iter, 0) else { continue };
                    match expr {
                        CSTNode::Number{ num: _ } | CSTNode::VarFetch{
                            at1: _,
                            var: _,
                            at2: _,
                        } => {},
                        _ => self.errs.push(CSTReport::InvalidExpressionX{ x: Some(expr.clone()) }),
                    }
                    *star = Some(Box::new(expr.clone()));
                    ls_iter.next();
                },

                Token::Dash => {
                    let Some(expr) = self.parse_expr(ls_iter, 0) else { continue };
                    match expr {
                        CSTNode::Label{ name: _ } | CSTNode::VarFetch{
                            at1: _,
                            var: _,
                            at2: _,
                        } => {},
                        _ => self.errs.push(CSTReport::InvalidExpressionX{ x: Some(expr.clone()) }),
                    }
                    *minus = Some(Box::new(expr.clone()));
                    ls_iter.next();
                },

                Token::Newline => break,

                _ => {
                    let expected_token: Token = if is_expect_space { Token::Space }
                        else if tool_name.is_none() { Token::Identifier }
                        else if star.is_none() { Token::Star }
                        else if plus.is_none() { Token::Plus }
                        else if minus.is_none() { Token::Dash }
                        else { Token::Newline };
                    self.errs.push(CSTReport::ExpectedXGotY{
                        x: expected_token,
                        y: (*ltok).clone(),
                    });
                    ls_iter.next();
                },
            }
        }

        true
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
        var: LexToken<'linespan>,
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
        let mut paren = CSTNode::Paren{
            lparen,
            contents: Vec::new(),
            rparen: None,
        };
        let CSTNode::Paren{
            ref lparen,
            ref mut contents,
            ref mut rparen,
        } = paren else {
            self.errs.push(CSTReport::InternalError(IntErrID::FetchCSTNode));
            return None;
        };

        loop {
            if self.expect_peek_token(Token::RParen, ls_iter) {
                let Some(ltok) = ls_iter.next() else {
                    self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee));
                    return None;
                };
                *rparen = Some(ltok.clone());
                break;
            }

            let mut item = CSTNode::Item{
                comma: None,
                var: None,
            };
            let CSTNode::Item{ ref mut comma, ref mut var } = item else {
                self.errs.push(CSTReport::InternalError(IntErrID::FetchCSTNode));
                return None;
            };

            self.consume_space(ls_iter);
            let Some(expr) = self.parse_expr(ls_iter, 0) else { return None };

            if !self.expect_peek_token(Token::Comma, ls_iter) {
                let Some(ltok) = ls_iter.next() else {
                    self.errs.push(CSTReport::EndOfFileDuring(Some(paren.clone())));
                    return Some(paren.clone());
                };
                self.errs.push(CSTReport::ExpectedXGotY{
                    x: Token::Comma,
                    y: ltok.clone(),
                });
                *var = Some(Box::new(expr.clone()));
            }
            self.consume_space(ls_iter);

            if let Some(found_comma) = self.expect_consume_token_or(
                Token::Comma, ls_iter,
                CSTReport::EndOfFileDuring(Some(CSTNode::Paren{
                    lparen: lparen.clone(),
                    contents: contents.clone(),
                    rparen: None,
                })),
            ) { *comma = Some(found_comma); };
            contents.push(item.clone());
        }

        Some(paren)
    }

    fn parse_bracket<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self, ls_iter: &mut Peekable<I>, lbrack: LexToken<'linespan>
    ) -> Option<CSTNode<'linespan>> {
        let mut paren = CSTNode::Brack{
            lbrack,
            contents: Vec::new(),
            rbrack: None,
        };
        let CSTNode::Brack{
            ref lbrack,
            ref mut contents,
            ref mut rbrack,
        } = paren else {
            self.errs.push(CSTReport::InternalError(IntErrID::FetchCSTNode));
            return None;
        };

        loop {
            if self.expect_peek_token(Token::RParen, ls_iter) {
                let Some(ltok) = ls_iter.next() else {
                    self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee));
                    return None;
                };
                *rbrack = Some(ltok.clone());
                break;
            }

            let mut item = CSTNode::Item{
                comma: None,
                var: None,
            };
            let CSTNode::Item{ ref mut comma, ref mut var } = item else {
                self.errs.push(CSTReport::InternalError(IntErrID::FetchCSTNode));
                return None;
            };

            self.consume_space(ls_iter);
            let Some(expr) = self.parse_expr(ls_iter, 0) else { return None };

            if !self.expect_peek_token(Token::Comma, ls_iter) {
                let Some(ltok) = ls_iter.next() else {
                    self.errs.push(CSTReport::EndOfFileDuring(Some(paren.clone())));
                    return Some(paren.clone());
                };
                self.errs.push(CSTReport::ExpectedXGotY{
                    x: Token::Comma,
                    y: ltok.clone(),
                });
                *var = Some(Box::new(expr.clone()));
            }
            self.consume_space(ls_iter);

            if let Some(found_comma) = self.expect_consume_token_or(
                Token::Comma, ls_iter,
                CSTReport::EndOfFileDuring(Some(CSTNode::Brack{
                    lbrack: lbrack.clone(),
                    contents: contents.clone(),
                    rbrack: None,
                })),
            ) { *comma = Some(found_comma); };
            contents.push(item.clone());
        }

        Some(paren)
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
                        Token::Plus | Token::Dash | Token::Star | Token::Slash
                            => {
                                ls_iter.next();
                                return self.parse_bin_op(ls_iter, num_node.clone(), (*op).clone());
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

            Token::At => {
                if self.consume_space(ls_iter) {
                    let Some(space) = ls_iter.next() else {
                        self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee));
                        return None;
                    };
                    self.errs.push(CSTReport::ExpectedXGotY{
                        x: Token::Identifier,
                        y: space.clone(),
                    });
                }
                let Some(expr) = self.parse_expr(ls_iter, 0) else { return None };
                self.consume_space(ls_iter);
                let Some(at) = ls_iter.next() else {
                    self.errs.push(CSTReport::EndOfFileDuring(Some(CSTNode::VarFetch{
                        at1: ltok.clone(),
                        var: Some(Box::new(expr)),
                        at2: None
                    })));
                    return Some(CSTNode::VarFetch{
                        at1: ltok.clone(),
                        var: Some(Box::new(expr)),
                        at2: None
                    });
                };
                if ltok.token != Token::At {
                    self.errs.push(CSTReport::ExpectedXGotY{
                        x: Token::At,
                        y: at.clone(),
                    });
                    return Some(CSTNode::VarFetch{
                        at1: ltok.clone(),
                        var: Some(Box::new(expr)),
                        at2: None
                    });
                }
                return Some(CSTNode::VarFetch{
                    at1: ltok.clone(),
                    var: Some(Box::new(expr)),
                    at2: None,
                });
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
                var: Box::new(var.clone()),
                dot: dot.clone(),
                element: None
            }))
        ) else { return None; };

        self.consume_space(ls_iter);

        let mut varmethod = CSTNode::VarElement{
            indent_sz,
            var: Box::new(var.clone()),
            dot: dot.clone(),
            element: Some(Box::new(CSTNode::Label{ name: ltok.clone() })),
        };

        self.consume_space(ls_iter);

        if self.expect_peek_token(Token::LParen, ls_iter) {
            let Some(lparen) = ls_iter.next() else {
                self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee));
                return None;
            };
            let Some(paren) = self.parse_paren(ls_iter, lparen.clone()) else { return None };
            varmethod = CSTNode::VarMethod{
                indent_sz,
                var: Box::new(var.clone()),
                dot: dot.clone(),
                method: Some(Box::new(CSTNode::FuncCall{
                    name: ltok.clone(),
                    paren: Some(Box::new(paren)),
                    indent_sz,
                })),
            };
        }

        match ls_iter.peek() {
            Some(peek) => match peek.token {
                Token::Newline => {
                    ls_iter.next();
                    return Some(CSTNode::VarElement{
                        indent_sz,
                        var: Box::new(var),
                        dot,
                        element: Some(Box::new(CSTNode::Label{ name: ltok.clone() })),
                    });
                },

                Token::Dot => return self.parse_element(ls_iter, varmethod, ltok.clone(), indent_sz,),

                _ => {
                    self.errs.push(CSTReport::ExpectedXGotY{
                        x: Token::Newline,
                        y: ltok.clone(),
                    });
                    return Some(CSTNode::VarElement{
                        indent_sz,
                        var: Box::new(var),
                        dot,
                        element: Some(Box::new(CSTNode::Label{ name: ltok.clone() })),
                    });
                },
            }

            None => return Some(CSTNode::VarElement{
                indent_sz,
                var: Box::new(var),
                dot,
                element: Some(Box::new(CSTNode::Label{ name: ltok.clone() })),
            }),
        }
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
                    if !self.check_consume_trailing(ls_iter) {
                    	self.errs.push(CSTReport::ExpectedXGotY{
                    	    x: Token::Newline,
                    	    y: ltok.clone(),
                    	});
                    }
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
        &mut self, ls_iter: &mut Peekable<I>, lhs: CSTNode<'linespan>, op: LexToken<'linespan>,
    ) -> Option<CSTNode<'linespan>> {
        let mut bin_op = CSTNode::BinOp{
            lhs: Box::new(lhs.clone()),
            op: op.clone(),
            rhs: None,
        };
        let CSTNode::BinOp{
            lhs: _,
            op: _,
            ref mut rhs
        } = bin_op else { return None };

        self.consume_space(ls_iter);
        match ls_iter.peek() {
            Some(ltok) => match ltok.token {
                Token::Identifier => {},
                Token::Number => {},
                _ => {},
            },
            None => {
                self.errs.push(CSTReport::EndOfFileDuring(Some(CSTNode::BinOp{
                    lhs: Box::new(lhs),
                    op,
                    rhs: None
                })));
            },
        }
        match self.parse_expr(ls_iter, 0) {
            Some(expr) => *rhs = Some(Box::new(expr)),
            None => return None,
        }

        self.consume_space(ls_iter);
        match ls_iter.peek() {
            Some(peek) => match peek.token {
                Token::Newline => {
                    ls_iter.next();
                    return Some(bin_op);
                },
                Token::Plus | Token::Dash | Token::Star | Token::Slash
                    => {
                        let _peek = (*peek).clone();
                        return self.parse_bin_op(ls_iter, bin_op, _peek)
                    },
                _ => {
                    self.errs.push(CSTReport::InvalidExpressionXAfterY{
                        x: (*peek).clone(),
                        y: Some(bin_op.clone()),
                    });
                    return Some(bin_op);
                }
            },
            None => {
                return Some(bin_op);
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

        self.consume_continue(ls_iter)
    }

    fn consume_indent_sz<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &self, ls_iter: &mut Peekable<I>
    ) -> usize {
        return match ls_iter.peek() {
            Some(ltok) if ltok.token == Token::Space
                => {
                    let len = ltok.span.str.len();
                    ls_iter.next();
                    return len;
                },
            _ => 0,
        }
    }

    fn check_consume_trailing<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &self,
        ls_iter: &mut Peekable<I>
    ) -> bool {
        loop {
            match ls_iter.peek() {
                Some(ltok) => match ltok.token {
                    Token::Newline => break,
                    Token::Space | Token::Comment | Token::MultiComment
                        => continue,
                    _ => return false,
                },
                None => {
                    ls_iter.next();
                    return false;
                },
            }
        }

        true
    }

    fn consume_continue<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &self,
        ls_iter: &mut Peekable<I>,
    ) -> bool {
        if self.expect_peek_token(Token::Continue, ls_iter) {
            ls_iter.next();
            return true;
        }

        false
    }

    fn expect_consume_space<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self,
        ls_iter: &mut Peekable<I>,
        err_or: CSTReport<'linespan>,
    ) -> bool {
        let Some(space) = ls_iter.peek() else {
            self.errs.push(err_or);
            return false;
        };
        if space.token != Token::Space {
            self.errs.push(CSTReport::ExpectedXGotY{
                x: Token::Space,
                y: (*space).clone(),
            });
            return false;
        }
        ls_iter.next();

        true
    }

    fn expect_consume_token_or<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self,
        token: Token,
        ls_iter: &mut Peekable<I>,
        or_err: CSTReport<'linespan>,
    ) -> Option<LexToken<'linespan>> {
        let Some(ltok) = ls_iter.peek() else {
            self.errs.push(or_err);
            return None;
        };

        if ltok.token != token {self.errs.push(CSTReport::ExpectedXGotY{
            x: token,
            y: (*ltok).clone(),
        })}

        let _ltok = (*ltok).clone();
        ls_iter.next();
        return Some(_ltok);
    }

    fn expect_peek_token<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &self,
        token: Token,
        ls_iter: &mut Peekable<I>
    ) -> bool {
        if let Some(ltok) = ls_iter.peek()
            && ltok.token == token {
                return true;
            }

         false
    }
}
