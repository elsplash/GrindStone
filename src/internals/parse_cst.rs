/*
 * NOTE: standardcombo, why the FUCK did you make this language as complex as it is?
 *       Please, I am curious as to why the fuck you ever made me come through this hellhole.
 *		 I ask of you, I sure as hell ask of you, to please translate this Parser code to C#,
 *		 and never speak of the old one. I BEG, dear standardcombo, your unrelenting implementations.
 *		 Why oh why have you made me carry such burden? You fucking bitch.
 * - ESplash (09.09.2026)
 */

use std::{
    iter::Peekable,
    fmt::{self, Display}
};

use crate::internals::{
    LexerOutput,
    LineSpan,
    LexToken,
    Token,
};

#[derive(Clone, Debug)]
pub enum CSTOpType {
    Increment,
    Decrement,
    AddAssign,
    SubAssign,
    MultAssign,
    DivAssign,
}

#[derive(Clone, Debug)]
pub enum EquipType {
    Left, Right, Automatic
}

#[derive(Clone, Debug)]
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
        name: Option<LexToken<'linespan>>,
        operator: Option<CSTOpType>,
        amount: Option<Box<CSTNode<'linespan>>>,
    },

    TableAccess{
        label: LexToken<'linespan>,
        brack: Option<Box<CSTNode<'linespan>>>,
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
        equip_type: EquipType,
        tool_name: Vec<CSTNode<'linespan>>,
        minus: Vec<CSTNode<'linespan>>,
        star: Vec<CSTNode<'linespan>>,
        plus: Vec<CSTNode<'linespan>>,
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
    },

    For{
        indent_sz: usize,
        key: LexToken<'linespan>,
        var: Option<LexToken<'linespan>>,
        equal: Option<LexToken<'linespan>>,
        start: Option<Box<CSTNode<'linespan>>>,
        dot1: Option<LexToken<'linespan>>,
        dot2: Option<LexToken<'linespan>>,
        end: Option<Box<CSTNode<'linespan>>>,
    },
}

#[derive(Debug)]
pub enum IntErrID {
    FetchCSTNode{ code: u16 },
    ExpectedGuarantee{ code: u16 },
}

/*
 * TODO: After doing all AST parsing errors, you may remove some of these now.
 */
#[derive(Debug)]
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

    ImplicitPrintCastInLine{ x: LineSpan },

    EndOfFileDuring(Option<CSTNode<'linespan>>),
    UnfinishedStatement(CSTNode<'linespan>),
    UnclosedDelimiter(CSTNode<'linespan>),
}

pub struct CSTOutput<'linespan>{
    pub output: Vec<CSTNode<'linespan>>,
    pub errs: Vec<CSTReport<'linespan>>,
}

impl<'linespan> Display for CSTReport<'linespan> {
    /* TODO: Replace this with better insight in the AST instead. */
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err: String;
        let line: String;
        let help_message: String;
        match self {
            CSTReport::InternalError(id) => {
                err = "[ERROR] Internal Error, this is not your fault.".to_string();
                line = ";)".to_string();
                help_message = match id {
                    IntErrID::ExpectedGuarantee{code} | IntErrID::FetchCSTNode{code} => format!("[HELP] You may report this to the devs. ErrCode: {code}"),
                };
            },

            CSTReport::ExpectedXGotY{
                x, y
            } => {
                err = format!("[ERROR] Expected {x} got {}", y.token);
                line = format!("{}", y.span);
                help_message = format!("[HINT] You may need to replace `{}` with a {x}", y.span.str);
            },

            CSTReport::InvalidExpressionXAfterY{x, y: _} => {
                err = format!("[ERROR] Invalid expression `{}`, after expression", x.span.str);
                line = format!("{} <- Is an invalid expression.", x.span);
                help_message = "[HINT] TODO: Fix this error, by passing it over to the AST".to_string();
            }

            CSTReport::InvalidExpressionX{x} => {
                err = format!("[ERROR] Invalid expression.");
                line = format!("{:#?}", x);
                help_message = "[HINT] TODO: Fix this error, by passing it over to the AST.".to_string();
            }

            CSTReport::ImplicitPrintCastInLine{x} => {
                err = format!("[WARNING] Implicit Print was cast into a Normal Print.");
                line = format!(
                    "{}\n{} | {} <- Is made into `>`.",
                    x,
                    " ".repeat(x.get_num().to_string().len()),
                    "~".repeat(x.get_line().len()),
                );
                help_message = "[HINT] Fixing the previous error(s) will fix it.".to_string();
            },

            CSTReport::EndOfFileDuring(x) => {
                err = "[ERROR] End of file during x.".to_string();
                line = format!("{:#?}", x);
                help_message = "[HINT] TODO: Fix this error, by passing it over to the AST.".to_string();
            },
            CSTReport::UnfinishedStatement(x) => {
                err = "[ERROR] Unfinished Statement x.".to_string();
                line = format!("{:#?}", x);
                help_message = "[HINT] TODO: Fix this error, by passing it over to the AST.".to_string();
            },
            CSTReport::UnclosedDelimiter(x) => {
                err = "[ERROR] Unclosed Delimiter x.".to_string();
                line = format!("{:#?}", x);
                help_message = "[HINT] TODO: Fix this error, by passing it over to the AST.".to_string();
            }
        }
        write!(f, "{}\n{}\n{}", err, line, help_message)
    }
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
                Token::Plus | Token::Dash | Token::Space | Token::Identifier
                    => {
                        if is_expect_newline {
                            self.errs.push(CSTReport::ExpectedXGotY{
                                x: Token::Newline,
                                y: (*peek).clone(),
                            });
                            self.consume_remaining(&mut ls_iter);
                            continue;
                        }
                        let is_err = self.parse_identifier(&mut ls_iter);
                        if !is_err { return false }
                    },

                Token::Greater => {
                    if is_expect_newline {
                        self.errs.push(CSTReport::ExpectedXGotY{
                            x: Token::Newline,
                            y: (*peek).clone(),
                        });
                        self.consume_remaining(&mut ls_iter);
                        continue;
                    }
                    is_expect_newline = true;
                    let _peek = (*peek).clone();
                    ls_iter.next();
                    if !self.parse_print(&mut ls_iter, _peek, 0) { return false }
                },

                Token::Colon | Token::QuestMark => {
                    if is_expect_newline {
                        self.errs.push(CSTReport::ExpectedXGotY{
                            x: Token::Newline,
                            y: (*peek).clone(),
                        });
                        self.consume_remaining(&mut ls_iter);
                        continue;
                    }
                    is_expect_newline = true;
                    let _peek = (*peek).clone();
                    ls_iter.next();
                    if !self.parse_if_stmnt(&mut ls_iter, _peek, 0) { return false }
                },

                Token::Comment | Token::MultiComment | Token::Newline => {
                    ls_iter.next();
                    is_expect_newline = false;
                    continue;
                },

                _ => {
                    self.errs.push(CSTReport::ExpectedXGotY{
                        x: if is_expect_newline { Token::Newline } else { Token::Identifier },
                        y: (*peek).clone(),
                    });
                    ls_iter.next();
                },
            }
            self.consume_space(&mut ls_iter);
        }

        true
    }

    fn parse_identifier<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self, ls_iter: &mut Peekable<I>
    ) -> bool {
        let indent_sz = self.consume_indent_sz(ls_iter);

        let Some(opt_key) = ls_iter.peek() else {
            self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee{ code: 100 }));
            return false;
        };
        let key  = (*opt_key).clone();

        if key.span.str == "var" || key.span.str == "const" {
            ls_iter.next();
            return self.parse_var_decl(ls_iter, key, indent_sz);
        } else if key.span.str == "func" {
            ls_iter.next();
            return self.parse_func_decl(ls_iter, key.clone(), indent_sz);
        }

        if self.check_consume_trailing(ls_iter) { return true }

        let Some(name) = ls_iter.next() else {
            self.errs.push(CSTReport::EndOfFileDuring(None));
            return false;
        };

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
                                self.consume_remaining(ls_iter);
                                return true;
                            }
                        },
                        None => {
                            let enable_node = CSTNode::Enable{
                                indent_sz, key: name.clone(), opt: None, opt_opts: None
                            };
                            self.output.push(enable_node.clone());
                            self.errs.push(CSTReport::EndOfFileDuring(Some(enable_node)));
                            self.consume_remaining(ls_iter);
                            return true;
                        },
                    }

                    let Some(ident) = self.parse_expr(ls_iter, 0, false) else { return false };
                    match ident {
                        CSTNode::Label{ name: _ } | CSTNode::VarFetch{
                            at1: _,
                            var: _,
                            at2: _,
                        } => {},
                        _ => self.errs.push(CSTReport::InvalidExpressionX{ x: Some(ident.clone()) }),
                    }

                    if self.consume_space(ls_iter) {
                        let Some(opt) = self.parse_expr(ls_iter, 0, false) else { return false };
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
                        return true;
                    }

                    self.output.push(CSTNode::Enable{
                        indent_sz, key: name.clone(), opt: Some(Box::new(ident.clone())), opt_opts: None
                    });
                    return true;
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
                                self.consume_remaining(ls_iter);
                                return true;
                            }
                        },
                        None => {
                            let disable_node = CSTNode::Disable{
                                indent_sz, key: name.clone(), opt: None, opt_opts: None
                            };
                            self.output.push(disable_node.clone());
                            self.errs.push(CSTReport::EndOfFileDuring(Some(disable_node)));
                            self.consume_remaining(ls_iter);
                            return true;
                        },
                    }

                    let Some(ident) = self.parse_expr(ls_iter, 0, false) else { return false };
                    match ident {
                        CSTNode::Label{ name: _ } | CSTNode::VarFetch{
                            at1: _,
                            var: _,
                            at2: _,
                        } => {},
                        _ => self.errs.push(CSTReport::InvalidExpressionX{ x: Some(ident.clone()) }),
                    }

                    if self.consume_space(ls_iter) {
                        let Some(opt) = self.parse_expr(ls_iter, 0, false) else { return false };
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
                        return true;
                    }

                    self.output.push(CSTNode::Disable{
                        indent_sz, key: name.clone(), opt: Some(Box::new(ident.clone())), opt_opts: None
                    });
                    return true;
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
                    ) {
                        self.consume_remaining(ls_iter);
                        return true;
                    }

                    let Some(ingr1) = self.parse_expr(ls_iter, 0, false) else { return false };
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
                    ) else {
                        self.consume_remaining(ls_iter);
                        return true;
                    };

                    self.consume_space(ls_iter);
                    let Some(ingr2) = self.parse_expr(ls_iter, 0, false) else { return false };
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
                    return true;
                },

                "loadout" => {
                    if !self.expect_consume_space(
                        ls_iter,
                        CSTReport::EndOfFileDuring(Some(CSTNode::Loadout{
                            indent_sz, key: name.clone(), num: None,
                        })),
                    ) {
                        self.consume_remaining(ls_iter);
                        return true;
                    }
                    let Some(num) = self.parse_expr(ls_iter, 0, false) else {
                        self.consume_remaining(ls_iter);
                        return true;
                    };
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
                    return true;
                },

                "activate" => {
                    if !self.expect_consume_space(
                        ls_iter,
                        CSTReport::EndOfFileDuring(Some(CSTNode::Activate{
                            indent_sz,
                            key: name.clone(),
                            opt: None
                        }))
                    ) {
                        self.consume_remaining(ls_iter);
                        return true;
                    }
                    let Some(opt) = self.parse_expr(ls_iter, 0, false) else {
                        self.consume_remaining(ls_iter);
                        return true;
                    };
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
                    return true;
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
                    ) {
                        self.consume_remaining(ls_iter);
                        return true;
                    }

                    let Some(sound) = self.parse_expr(ls_iter, 0, false) else {
                        self.consume_remaining(ls_iter);
                        return true;
                    };
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

                    let Some(pitch) = self.parse_expr(ls_iter, 0, false) else {
                        self.consume_remaining(ls_iter);
                        return true;
                    };
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
                    return true;
                },

                "equipL" | "equipR" | "equip" => return self.parse_equip(ls_iter, name.clone(), indent_sz),

                "for" => {
                    let Some(ltok) = self.expect_consume_token_or(
                        Token::Space, ls_iter,
                        CSTReport::EndOfFileDuring(Some(CSTNode::For{
                            indent_sz,
                            key: name.clone(),
                            var: None, equal: None, start: None, dot1: None, dot2: None, end: None,
                        }))
                    ) else {
                        self.consume_remaining(ls_iter);
                        return true;
                    };
                    drop(ltok);

                    let Some(id) = self.expect_consume_token_or(
                        Token::Identifier, ls_iter,
                        CSTReport::EndOfFileDuring(Some(CSTNode::For{
                            indent_sz,
                            key: name.clone(),
                            var: None, equal: None, start: None, dot1: None, dot2: None, end: None,
                        })),
                    ) else {
                        self.consume_remaining(ls_iter);
                        return true;
                    };
                    self.consume_space(ls_iter);

                    let Some(equal) = self.expect_consume_token_or(
                        Token::Equal, ls_iter,
                        CSTReport::EndOfFileDuring(Some(CSTNode::For{
                            indent_sz,
                            key: name.clone(),
                            var: Some(id.clone()),
                            equal: None, start: None, dot1: None, dot2: None, end: None,
                        })),
                    ) else {
                        self.consume_remaining(ls_iter);
                        return true;
                    };
                    self.consume_space(ls_iter);

                    let Some(expr1) = self.parse_special_for_expr(ls_iter) else {
                        self.consume_remaining(ls_iter);
                        return true;
                    };
                    self.consume_space(ls_iter);

                    let Some(dot1) = self.expect_consume_token_or(
                        Token::Dot, ls_iter,
                        CSTReport::EndOfFileDuring(Some(CSTNode::For{
                            indent_sz,
                            key: name.clone(),
                            var: Some(id.clone()),
                            equal: Some(equal.clone()),
                            start: Some(Box::new(expr1.clone())),
                            dot1: None, dot2: None, end: None,
                        }))
                    ) else {
                        self.consume_remaining(ls_iter);
                        return true;
                    };

                    let Some(dot2) = self.expect_consume_token_or(
                        Token::Dot, ls_iter,
                        CSTReport::EndOfFileDuring(Some(CSTNode::For{
                            indent_sz,
                            key: name.clone(),
                            var: Some(id.clone()),
                            equal: Some(equal.clone()),
                            start: Some(Box::new(expr1.clone())),
                            dot1: Some(dot1.clone()),
                            dot2: None, end: None,
                        }))
                    ) else {
                        self.consume_remaining(ls_iter);
                        return true;
                    };
                    self.consume_space(ls_iter);

                    let Some(expr2) = self.parse_special_for_expr(ls_iter) else { return false };

                    self.output.push(CSTNode::For{
                        indent_sz,
                        key: name.clone(),
                        var: Some(id.clone()),
                        equal: Some(equal.clone()),
                        start: Some(Box::new(expr1.clone())),
                        dot1: Some(dot1.clone()),
                        dot2: Some(dot2.clone()),
                        end: Some(Box::new(expr2.clone())),
                    });
                    return true;
                },

                "import" => {
                    self.consume_space(ls_iter);
                    let Some(file_path) = self.parse_file_path(ls_iter) else {
                        self.consume_remaining(ls_iter);
                        return true;
                    };
                    self.output.push(CSTNode::Import{
                        indent_sz,
                        key: name.clone(),
                        path: Some(Box::new(file_path)),
                    });
                    return true;
                },

                "new" => {
                    self.consume_space(ls_iter);
                    let Some(file_path) = self.parse_file_path(ls_iter) else {
                        self.consume_remaining(ls_iter);
                        return true;
                    };
                    self.output.push(CSTNode::New{
                        indent_sz,
                        key: name.clone(),
                        path: Some(Box::new(file_path)),
                    });
                    return true;
                },

                _ => {},
            }
        } else {
            match name.token {
                Token::Greater => return self.parse_print(ls_iter, name.clone(), indent_sz),

                Token::Colon | Token::QuestMark => return self.parse_if_stmnt(ls_iter, name.clone(), indent_sz),

                Token::Plus => {
                    match ls_iter.peek() {
                        Some(peek) if peek.token == Token::Plus => {
                            ls_iter.next();
                            let Some(ident) = self.expect_consume_token_or(
                                Token::Identifier, ls_iter,
                                CSTReport::EndOfFileDuring(Some(CSTNode::VarMutator{
                                    indent_sz,
                                    name: None,
                                    operator: Some(CSTOpType::Increment),
                                    amount: None,
                                }))
                            ) else {
                                self.consume_remaining(ls_iter);
                                return true;
                            };
                            self.output.push(CSTNode::VarMutator{
                                indent_sz,
                                name: Some(ident.clone()),
                                operator: Some(CSTOpType::Increment),
                                amount: None,
                            });
                            ls_iter.next();
                            return true;
                        },
                        _ => {
                            self.errs.push(CSTReport::ExpectedXGotY{
                                x: Token::Identifier,
                                y: name.clone(),
                            });
                            self.consume_remaining(ls_iter);
                            return true;
                        },
                    }
                },

                Token::Dash => {
                    match ls_iter.peek() {
                        Some(peek) if peek.token == Token::Dash => {
                            ls_iter.next();
                            let Some(ident) = self.expect_consume_token_or(
                                Token::Identifier, ls_iter,
                                CSTReport::EndOfFileDuring(Some(CSTNode::VarMutator{
                                    indent_sz,
                                    name: None,
                                    operator: Some(CSTOpType::Decrement),
                                    amount: None,
                                }))
                            ) else {
                                self.consume_remaining(ls_iter);
                                return true;
                            };
                            self.output.push(CSTNode::VarMutator{
                                indent_sz,
                                name: Some(ident.clone()),
                                operator: Some(CSTOpType::Decrement),
                                amount: None,
                            });
                            ls_iter.next();
                            return true;
                        },
                        _ => {
                            self.errs.push(CSTReport::ExpectedXGotY{
                                x: Token::Identifier,
                                y: name.clone(),
                            });
                            self.consume_remaining(ls_iter);
                            return true;
                        },
                    }
                },

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
            self.consume_remaining(ls_iter);
            return false;
        };
        match ltok.token {
            Token::LParen => return self.parse_func_call(ls_iter, name.clone(), ltok.clone(), indent_sz),

            Token::Plus => {
                let Some(equal_or_plus) = ls_iter.peek() else {
                    self.errs.push(CSTReport::EndOfFileDuring(Some(CSTNode::BinOp{
                        lhs: Box::new(CSTNode::Label{ name: name.clone() }),
                        op: ltok.clone(),
                        rhs: None
                    })));
                    self.consume_remaining(ls_iter);
                    return true;
                };

                match equal_or_plus.token {
                    Token::Plus => {
                        self.output.push(CSTNode::VarMutator{
                            indent_sz, name: Some(name.clone()),
                            operator: Some(CSTOpType::Increment),
                            amount: None,
                        });
                        ls_iter.next();
                    },

                    Token::Equal => {
                        self.consume_space(ls_iter);
                        let Some(expr) = self.parse_expr(ls_iter, 0, false) else { return false };
                        self.output.push(CSTNode::VarMutator{
                            indent_sz, name: Some(name.clone()),
                            operator: Some(CSTOpType::AddAssign),
                            amount: Some(Box::new(expr)),
                        });
                        ls_iter.next();
                    },

                    _ => {
                        self.errs.push(CSTReport::ExpectedXGotY{
                            x: Token::Plus,
                            y: (*equal_or_plus).clone()
                        });
                        self.consume_remaining(ls_iter);
                        return true;
                    },
                }

                return true;
            },

            Token::Dash => {
                let Some(equal_or_dash) = ls_iter.peek() else {
                    self.errs.push(CSTReport::EndOfFileDuring(Some(CSTNode::BinOp{
                        lhs: Box::new(CSTNode::Label{ name: name.clone() }),
                        op: ltok.clone(),
                        rhs: None,
                    })));
                    self.consume_remaining(ls_iter);
                    return true;
                };

                match equal_or_dash.token {
                    Token::Dash => {
                        self.output.push(CSTNode::VarMutator{
                            indent_sz, name: Some(name.clone()),
                            operator: Some(CSTOpType::Decrement),
                            amount: None,
                        });
                        ls_iter.next();
                    },

                    Token::Equal => {
                        self.consume_space(ls_iter);
                        let Some(expr) = self.parse_expr(ls_iter, 0, false) else { return false };
                        match expr {
                            CSTNode::Number{ num: _ } | CSTNode::VarFetch{
                                at1: _,
                                var: _,
                                at2: _,
                            } => {},
                            _ => {
                                self.errs.push(CSTReport::InvalidExpressionX{ x: Some(expr) });
                                self.consume_remaining(ls_iter);
                                return true;
                            },
                        }
                        self.output.push(CSTNode::VarMutator{
                            indent_sz, name: Some(name.clone()),
                            operator: Some(CSTOpType::SubAssign),
                            amount: Some(Box::new(expr)),
                        });
                    },

                    _ => {
                        self.errs.push(CSTReport::ExpectedXGotY{
                            x: Token::Dash,
                            y: (*equal_or_dash).clone()
                        });
                        self.consume_remaining(ls_iter);
                        return true;
                    },
                }

                return true;
            },

            Token::Star => {
                if self.expect_consume_token_or(
                    Token::Equal, ls_iter,
                    CSTReport::EndOfFileDuring(Some(CSTNode::BinOp{
                        lhs: Box::new(CSTNode::Label{ name: name.clone() }),
                        op: ltok.clone(),
                        rhs: None
                    }))
                ).is_none() {
                    self.consume_remaining(ls_iter);
                    return true;
                };

                let Some(expr) = self.parse_expr(ls_iter, 0, false) else { return false };
                match expr {
                    CSTNode::Number{ num: _ } | CSTNode::VarFetch{
                        at1: _,
                        var: _,
                        at2: _,
                    } => {},
                    _ => self.errs.push(CSTReport::InvalidExpressionX{ x: Some(expr.clone()) }),
                }

                self.output.push(CSTNode::VarMutator{
                    indent_sz, name: Some(name.clone()),
                    operator: Some(CSTOpType::MultAssign),
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
                ))).is_none() {
                    self.consume_remaining(ls_iter);
                    return true;
                };

                let Some(expr) = self.parse_expr(ls_iter, 0, false) else {
                    self.consume_remaining(ls_iter);
                    return true;
                };
                match expr {
                    CSTNode::Number{ num: _ } | CSTNode::VarFetch{
                        at1: _,
                        var: _,
                        at2: _,
                    } => {},
                    _ => self.errs.push(CSTReport::InvalidExpressionX{ x: Some(expr.clone()) }),
                }

                self.output.push(CSTNode::VarMutator{
                    indent_sz, name: Some(name.clone()),
                    operator: Some(CSTOpType::DivAssign),
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
                ) else {
                    self.consume_remaining(ls_iter);
                    return true;
                };
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
                                    self.consume_remaining(ls_iter);
                                    return true;
                                }
                            }

                            None => {
                                self.output.push(method);
                                return true;
                            },
                        }

                        let Some(ltok) = ls_iter.next() else {
                            self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee{ code: 101 }));
                            self.consume_remaining(ls_iter);
                            return true;
                        };

                        match ltok.token {
                            Token::Space => {
                                let Some(equal) = self.expect_consume_token_or(
                                    Token::Equal, ls_iter,
                                    CSTReport::ExpectedXGotY{
                                        x: Token::Newline,
                                        y: ltok.clone()
                                    },
                                ) else {
                                    self.consume_remaining(ls_iter);
                                    return true;
                                };
                                let Some(expr) = self.parse_expr(ls_iter, 0, false) else {
                                    self.output.push(CSTNode::VarDef{
                                        indent_sz,
                                        name: Box::new(method),
                                        equal: Some(equal),
                                        def: None
                                    });
                                    self.consume_remaining(ls_iter);
                                    return true;
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
                                let Some(expr) = self.parse_expr(ls_iter, 0, false) else {
                                    self.output.push(CSTNode::VarDef{
                                        indent_sz,
                                        name: Box::new(method),
                                        equal: Some(ltok.clone()),
                                        def: None
                                    });
                                    self.consume_remaining(ls_iter);
                                    return true;
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
                                self.consume_remaining(ls_iter);
                                return true;
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
                                self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee{ code: 102 }));
                                self.consume_remaining(ls_iter);
                                return true;
                            };
                            self.errs.push(CSTReport::ExpectedXGotY{
                                x: Token::Newline,
                                y: ltok.clone(),
                            });
                            self.consume_remaining(ls_iter);
                            return true;
                        }
                        return true;
                    },

                    _ => {
                        self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee{ code: 103 }));
                        self.consume_remaining(ls_iter);
                        return true;
                    },
                }
            },

            Token::Newline => {
                let node = CSTNode::VarDef{
                        indent_sz,
                        name: Box::new(CSTNode::Label{ name: name.clone() }),
                        equal: None,
                        def: None,
                    };
                self.output.push(node.clone());
                self.errs.push(CSTReport::UnfinishedStatement(node));
                self.consume_remaining(ls_iter);
                return true;
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
                ) else {
                    self.consume_remaining(ls_iter);
                    return true;
                };

                self.consume_space(ls_iter);
                let Some(expr) = self.parse_expr(ls_iter, 0, false) else {
                    self.consume_remaining(ls_iter);
                    return true;
                };

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

    fn parse_special_for_expr<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self,
        ls_iter: &mut Peekable<I>,
    ) -> Option<CSTNode<'linespan>> {
        match ls_iter.next() {
            Some(ltok) => match ltok.token {
                Token::Number => return Some(CSTNode::Number{ num : ltok.clone() }),

                Token::At => {
                    let Some(expr) = self.parse_expr(ls_iter, 0, true) else { return None };
                    self.consume_space(ls_iter);

                    let Some(at2) = self.expect_consume_token_or(
                        Token::At, ls_iter,
                        CSTReport::EndOfFileDuring(Some(CSTNode::VarFetch{
                            at1: ltok.clone(),
                            var: Some(Box::new(expr.clone())),
                            at2: None
                        }))
                    ) else { return None };

                    return Some(CSTNode::VarFetch{
                        at1: ltok.clone(),
                        var: Some(Box::new(expr)),
                        at2: Some(at2),
                    });
                },

                Token::Identifier => return Some(CSTNode::Label{ name: ltok.clone() }),

                _ => {}
            },

            None => return None,
        }

        None
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
            self.errs.push(CSTReport::InternalError(IntErrID::FetchCSTNode{ code: 201 }));
            self.consume_remaining(ls_iter);
            return true;
        };

        /* NOTE: Since Print can be implicitly cast, we can prepare for it. */
        let Some(_spcl_key) = spcl_key else {
            self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee{ code: 104 }));
            return false;
        };

        self.consume_space(ls_iter);

        let Some(x_pos) = self.parse_expr(ls_iter, 0, false) else {
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

        let Some(y_pos) = self.parse_expr(ls_iter, 0, false) else {
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
                    if lchars.skip(4).take(2).all(|c| c.is_digit(16)) {
                        *spcl_clr_val = Some(ltok.clone());
                    } else {
                        *spcl_clr_key = None;
                        *spcl_y_pos = None;
                        *spcl_x_pos = None;
                        *spcl_key = None;
                        return false;
                    }
                } else {
                    if lchars.all(|c| c.is_digit(16)) {
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
            Some(_) => {
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
        let line_span = r_angle_brack.span.line.clone();

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
            self.errs.push(CSTReport::InternalError(IntErrID::FetchCSTNode{ code: 202 }));
            self.consume_remaining(ls_iter);
            return true;
        };

        match ls_iter.peek() {
            Some(ltok) => match ltok.token {
                Token::Identifier => {
                    match ltok.span.str.as_str() {
                        "o" | "h" | "c" | "f" => {
                            *spcl_key = Some((*ltok).clone());
                            ls_iter.next();
                        },
                        _ => {}
                    }
                },

                Token::Tilde => {
                    *spcl_key = Some((*ltok).clone());
                    ls_iter.next();
                },

                Token::LParen => {
                    let _ltok = (*ltok).clone();
                    let _ = ltok;
                    ls_iter.next();
                    let Some(face) = self.expect_consume_token_or(
                        Token::Identifier, ls_iter,
                        CSTReport::EndOfFileDuring(Some(_print_node.clone()))
                    ) else {
                        self.consume_remaining(ls_iter);
                        return true;
                    };
                    if face.span.str.len() != 4 {
                        self.errs.push(CSTReport::ImplicitPrintCastInLine{ x: face.span.line.clone() });
                        contents.push(CSTNode::Label{ name: face.clone() });
                        contents.push(CSTNode::Label{ name: face.clone() });
                        ls_iter.next();
                    } else {
                        *spcl_key = Some(_ltok.clone());
                        contents.push(CSTNode::Label{ name: face.clone() });
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

        if spcl_key.is_some() && !self.parse_print_param(ls_iter, &mut print_node) {
            self.errs.push(CSTReport::ImplicitPrintCastInLine{ x: line_span });
        }

        let mut is_ascii = true;

        loop {
            let Some(peek) = ls_iter.peek() else { break };
            match peek.token {
                Token::Newline => break,

                Token::Space | Token::Continue => {
                    ls_iter.next();
                    continue;
                },

                Token::Identifier if peek.span.str == "ascii"  && is_ascii => {
                    let Some(ascii) = self.parse_expr(ls_iter, 0, false) else {
                        self.output.push(print_node);
                        self.consume_remaining(ls_iter);
                        return true;
                    };
                    ls_iter.next();
                    if let CSTNode::Ascii{ key_start: _, lines: _, key_end: _ } = ascii {
                        break;
                    } else {
                        self.output.push(print_node);
                        self.consume_remaining(ls_iter);
                        return true;
                    }
                },

                _ => {
                    is_ascii = false;
                    self.output.push(CSTNode::Label{ name: (*peek).clone() });
                    ls_iter.next();
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
                        self.consume_remaining(ls_iter);
                        return true;
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
            self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee{ code: 107 }));
            self.consume_remaining(ls_iter);
            return true;
        }

        let mut if_stmnt = CSTNode::If{
            indent_sz: if is_else { 0 } else { indent_sz },
            key: if !is_else { if_or_else.clone() } else {
                let Some(question_mark) = ls_iter.next() else {
                    self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee{ code: 108 }));
                    self.consume_remaining(ls_iter);
                    return true;
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
            self.errs.push(CSTReport::InternalError(IntErrID::FetchCSTNode{ code: 203 }));
            self.consume_remaining(ls_iter);
            return true;
        };

        self.consume_space(ls_iter);

        let Some(lhs) = self.parse_expr(ls_iter, 0, false) else {
            self.consume_remaining(ls_iter);
            return true;
        };

        self.consume_space(ls_iter);
        match ls_iter.peek() {
            Some(op) => match op.token {
                Token::Equal | Token::LessEq | Token::GreatEq | Token::Lesser | Token::Greater
                    => {
                        let _op = (*op).clone();
                        ls_iter.next();
                        let Some(_bin_op) = self.parse_bin_op(ls_iter, lhs, _op) else {
                            self.consume_remaining(ls_iter);
                            return true;
                        };
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
                    self.consume_remaining(ls_iter);
                    return true;
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
            equip_type: match key.span.str.as_str() {
                "equipL" => EquipType::Left,
                "equipR" => EquipType::Right,
                "equip" => EquipType::Automatic,
                _ => {
                    self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee{ code: 109 }));
                    EquipType::Automatic
                },
            },
            tool_name: vec![],
            plus: vec![],
            star: vec![],
            minus: vec![],
        };
        let CSTNode::Equip{
            indent_sz: _, key: _, equip_type: _,
            ref mut tool_name,
            ref mut plus,
            ref mut star,
            ref mut minus,
        } = equip_node else {
            self.errs.push(CSTReport::InternalError(IntErrID::FetchCSTNode{ code: 204 }));
            self.consume_remaining(ls_iter);
            return true;
        };
        let mut is_expect_space = true;

        loop {
            let Some(peek) = ls_iter.peek() else { break };

            match peek.token {
                Token::Space => {
                    if !is_expect_space {
                        self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee{ code: 110 }));
                    }
                    is_expect_space = false;
                    ls_iter.next();
                },

                Token::Identifier => {
                    let Some(expr) = self.parse_expr(ls_iter, 0, false) else { continue };
                    tool_name.push(expr);
                    ls_iter.next();
                },

                Token::At => {
                    let Some(expr) = self.parse_expr(ls_iter, 0, false) else { continue };
                    tool_name.push(expr);
                    ls_iter.next();
                },

                Token::Plus => {
                    let Some(expr) = self.parse_special_equip_num(ls_iter) else {
                        continue
                    };
                    plus.push(expr);
                },

                Token::Star => {
                    let Some(expr) = self.parse_special_equip_num(ls_iter) else {
                        continue
                    };
                    star.push(expr);
                },

                Token::Dash => {
                    let Some(expr) = self.parse_special_equip_num(ls_iter) else {
                        continue
                    };
                    minus.push(expr);
                },

                Token::Comment | Token::Newline => {
                    ls_iter.next();
                    break;
                },

                Token::MultiComment => continue,

                _ => {
                    let expected_token: Token = if is_expect_space { Token::Space }
                        else if tool_name.is_empty() { Token::Identifier }
                        else if star.is_empty() { Token::Star }
                        else if plus.is_empty() { Token::Plus }
                        else if minus.is_empty() { Token::Dash }
                        else { Token::Newline };
                    self.errs.push(CSTReport::ExpectedXGotY{
                        x: expected_token,
                        y: (*peek).clone(),
                    });
                    ls_iter.next();
                },
            }
        }

        self.output.push(equip_node);

        true
    }

    fn parse_special_equip_num<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self,
        ls_iter: &mut Peekable<I>,
    ) -> Option<CSTNode<'linespan>> {
        match ls_iter.peek() {
            Some(ltok) => match ltok.token {
                Token::Identifier => {
                    let _ltok = (*ltok).clone();
                    ls_iter.next();
                    return Some(CSTNode::Label{ name: _ltok });
                },

                Token::At => return self.parse_expr(ls_iter, 0, false),

                Token::Number => {
                    let _ltok = (*ltok).clone();
                    ls_iter.next();
                    return Some(CSTNode::Number{ num: _ltok });
                },

                _ => {
                    let _ltok = (*ltok).clone();
                    ls_iter.next();
                    self.errs.push(CSTReport::ExpectedXGotY{
                        x: Token::Number,
                        y: _ltok,
                    });
                    return None;
                },
            },
            None => return None,
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
        ).is_none() {
            self.consume_remaining(ls_iter);
            return true;
        }

        let Some(name) = self.expect_consume_token_or(
            Token::Identifier, ls_iter,
            CSTReport::EndOfFileDuring(Some(CSTNode::FuncDecl{
                indent_sz,
                key: func_key.clone(),
                name: None,
                paren: None,
            }))
        ) else {
            self.consume_remaining(ls_iter);
            return true;
        };
        self.consume_space(ls_iter);

        let Some(lparen) = self.expect_consume_token_or(
            Token::LParen, ls_iter,
            CSTReport::EndOfFileDuring(Some(CSTNode::FuncDecl{
                indent_sz,
                key: func_key.clone(),
                name: Some(name.clone()),
                paren: None,
            }))
        ) else {
            self.consume_remaining(ls_iter);
            return true;
        };

        match self.parse_paren(ls_iter, lparen.clone()) {
            Some(paren) => self.output.push(CSTNode::FuncDecl{
                indent_sz,
                key: func_key.clone(),
                name: Some(name.clone()),
                paren: Some(Box::new(paren.clone())),
            }),
            None => {
                self.consume_remaining(ls_iter);
                return true;
            },
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
        ).is_none() {
            self.consume_remaining(ls_iter);
            return true;
        }

        let Some(name) = self.expect_consume_token_or(
            Token::Identifier, ls_iter,
            CSTReport::InternalError(IntErrID::ExpectedGuarantee{ code: 111 })
        ) else {
            self.consume_remaining(ls_iter);
            return true;
        };

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
        ) else {
            self.consume_remaining(ls_iter);
            return true;
        };

        self.consume_space(ls_iter);

        let Some(expr) = self.parse_expr(ls_iter, 0, false) else {
            self.consume_remaining(ls_iter);
            return true;
        };

        self.output.push(CSTNode::VarDecl{
            indent_sz,
            key: var.clone(),
            name: Some(name),
            equal: Some(equal),
            def: Some(Box::new(expr)),
        });

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
            None => {
                self.consume_remaining(ls_iter);
                return true;
            },
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
            self.errs.push(CSTReport::InternalError(IntErrID::FetchCSTNode{ code: 205 }));
            return None;
        };

        loop {
            if self.expect_peek_token(Token::RParen, ls_iter) {
                let Some(ltok) = ls_iter.next() else {
                    self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee{ code: 112 }));
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
                self.errs.push(CSTReport::InternalError(IntErrID::FetchCSTNode{ code: 206 }));
                return None;
            };

            self.consume_space(ls_iter);
            let Some(expr) = self.parse_expr(ls_iter, 0, false) else {
                return None;
            };

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
            self.errs.push(CSTReport::InternalError(IntErrID::FetchCSTNode{ code: 207 }));
            return None;
        };

        loop {
            if self.expect_peek_token(Token::RParen, ls_iter) {
                let Some(ltok) = ls_iter.next() else {
                    self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee{ code: 113 }));
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
                self.errs.push(CSTReport::InternalError(IntErrID::FetchCSTNode{ code: 208 }));
                return None;
            };

            self.consume_space(ls_iter);
            let Some(expr) = self.parse_expr(ls_iter, 0, false) else {
                return None
            };

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
        &mut self,
        ls_iter: &mut Peekable<I>,
        indent_sz: usize,
        is_varfetch: bool,
    ) -> Option<CSTNode<'linespan>> {
        match ls_iter.next() {
            Some(ltok) => match ltok.token {
                Token::Identifier => {
                    match ltok.span.str.as_str() {
                        "import" => {
                            self.consume_space(ls_iter);
                            let Some(file_path) = self.parse_file_path(ls_iter) else { return None };

                            return Some(CSTNode::Import{indent_sz, key: ltok.clone(), path: Some(Box::new(file_path))});
                        },

                        "new" => {
                            self.consume_space(ls_iter);
                            let Some(file_path) = self.parse_file_path(ls_iter) else { return None };

                            return Some(CSTNode::New{indent_sz, key: ltok.clone(), path: Some(Box::new(file_path))});
                        },

                        "ascii" if self.check_consume_trailing(ls_iter) => return self.parse_ascii(ls_iter, ltok.clone()),

                        _ => {},
                    }

                    match ls_iter.peek() {
                        Some(peek) => match peek.token {
                            Token::Continue | Token::MultiComment | Token::Space => { self.consume_space(ls_iter); },

                            Token::At if is_varfetch => return Some(CSTNode::Label{ name: ltok.clone() }),

                            _ => return Some(CSTNode::Label{ name: ltok.clone() }),
                        },
                        None => return Some(CSTNode::Label{ name: ltok.clone() }),
                    }

                    match ls_iter.peek() {
                        Some(check) => match check.token {
                            Token::LBracket => {
                                let _check = (*check).clone();
                                ls_iter.next();
                                let Some(brack) = self.parse_bracket(ls_iter, _check) else { return None };
                                return Some(CSTNode::TableAccess{
                                    label: ltok.clone(),
                                    brack: Some(Box::new(brack)),
                                });
                            },

                            Token::Dot => {
                                let _check = (*check).clone();
                                ls_iter.next();
                                return self.parse_element(ls_iter, CSTNode::Label{ name: ltok.clone() }, _check, 0);
                            },

                            Token::Plus | Token::Dash | Token::Star | Token::Slash | Token::ExclMark | Token::Percent
                                => {
                                    let _check = (*check).clone();
                                    ls_iter.next();
                                    return self.parse_bin_op(ls_iter, CSTNode::Label{ name: ltok.clone() }, _check);
                                },

                            Token::At if is_varfetch => return Some(CSTNode::Label{ name: ltok.clone() }),

                            _ => {},
                        },

                        None => return Some(CSTNode::Label{ name: ltok.clone() }),
                    }

                    let mut label = ltok.clone();
                    label.span.str.push(' ');

                    loop {
                        match ls_iter.peek() {
                            Some(peek) => match ltok.token {
                                Token::Identifier => {
                                    label.concat((*peek).clone());
                                },

                                Token::Continue | Token::Space => {
                                    label.span.str.push(' ');
                                },

                                _ => break,
                            },

                            None => break,
                        }
                        ls_iter.next();
                    }

                    return Some(CSTNode::Label{ name: label.clone() });
                },

                Token::At => {
                    if is_varfetch {
                        self.errs.push(CSTReport::InvalidExpressionXAfterY{
                            x: ltok.clone(),
                            y: None,
                        });
                        return None;
                    }
                    let at1 = ltok;

                    let Some(expr) = self.parse_expr(ls_iter, 0, true) else { return None };

                    let Some(at2) = self.expect_consume_token_or(
                        Token::At, ls_iter,
                        CSTReport::EndOfFileDuring(Some(CSTNode::VarFetch{
                            at1: at1.clone(),
                            var: Some(Box::new(expr.clone())),
                            at2: None
                        }))
                    ) else { return None };

                    return Some(CSTNode::VarFetch{
                        at1: at1.clone(),
                        var: Some(Box::new(expr)),
                        at2: Some(at2),
                    });
                },

                Token::Number => {
                    let Some(check) = ls_iter.peek() else { return Some(CSTNode::Number{ num: ltok.clone() }) };
                    let _check = (*check).clone();
                    let _ = check;
                    match check.token {
                        Token::Space | Token::Continue => { ls_iter.next(); },

                        Token::Plus | Token::Dash | Token::Star | Token::Slash | Token::ExclMark | Token::Percent
                            => {
                                let _check = (*check).clone();
                                ls_iter.next();
                                return self.parse_bin_op(ls_iter, CSTNode::Number{ num: ltok.clone() }, _check);
                            },

                        Token::At if is_varfetch => return Some(CSTNode::Number{ num: _check }),

                        Token::Newline => return Some(CSTNode::Number{ num: _check }),


                        _ => return Some(CSTNode::Number{ num: _check }),
                    }

                    self.consume_space(ls_iter);
                    match ls_iter.next() {
                        Some(op) => match op.token {
                            Token::Plus | Token::Dash | Token::Star | Token::Slash | Token::ExclMark | Token::Percent
                                => {
                                    ls_iter.next();
                                    return self.parse_bin_op(ls_iter, CSTNode::Number{ num: ltok.clone() }, _check);
                                },

                            _ => return Some(CSTNode::Number{ num: ltok.clone() }),
                        },

                        None => return Some(CSTNode::Number{ num: ltok.clone() }),
                    }
                },

                Token::Plus => {
                    self.consume_space(ls_iter);
                    let Some(mut expr) = self.parse_expr(ls_iter, 0, is_varfetch) else { return None };
                    match expr {
                        CSTNode::Number{ num } => {
                            let mut new_ltok = ltok.clone();
                            new_ltok.concat(num);
                            return Some(CSTNode::Number{ num: new_ltok });
                        },

                        CSTNode::BinOp{ ref mut lhs, op: _, rhs: _ } => {
                            let mut current_lhs: CSTNode<'_> = *(lhs.clone());
                            loop {
                                match current_lhs {
                                    CSTNode::BinOp{
                                        ref lhs,
                                        op: _,
                                        rhs: _,
                                    } => {
                                        current_lhs = *(lhs.clone());
                                        continue;
                                    },

                                    CSTNode::Number{ ref mut num } => {
                                        *num = ltok.clone();
                                        break;
                                    },

                                    CSTNode::Label{ ref mut name } => {
                                        let mut new_ltok = ltok.clone();
                                        new_ltok.concat(name.clone());
                                        *name = ltok.clone();
                                        break;
                                    },

                                    CSTNode::VarFetch{
                                        ref at1,
                                        var: _,
                                        at2: _,
                                    } => {
                                        self.errs.push(CSTReport::ExpectedXGotY{
                                            x: Token::Number,
                                            y: at1.clone(),
                                        });
                                        return Some(current_lhs);
                                    },

                                    _ => {
                                        self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee{ code: 114 }));
                                        return None;
                                    },
                                }
                            }

                            return Some(current_lhs);
                        }

                        _ => return Some(CSTNode::Label{ name: ltok.clone() }),
                    }
                },

                Token::Dash => {
                    self.consume_space(ls_iter);
                    let Some(mut expr) = self.parse_expr(ls_iter, 0, is_varfetch) else { return None };
                    match expr {
                        CSTNode::Number{ num } => {
                            let mut new_ltok = ltok.clone();
                            new_ltok.concat(num);
                            return Some(CSTNode::Number{ num: new_ltok });
                        },

                        CSTNode::BinOp{ ref mut lhs, op: _, rhs: _ } => {
                            let mut current_lhs: CSTNode<'_> = *(lhs.clone());
                            loop {
                                match current_lhs {
                                    CSTNode::BinOp{
                                        ref lhs,
                                        op: _,
                                        rhs: _,
                                    } => {
                                        current_lhs = *(lhs.clone());
                                        continue;
                                    },

                                    CSTNode::Number{ ref mut num } => {
                                        *num = ltok.clone();
                                        break;
                                    },

                                    CSTNode::Label{ ref mut name } => {
                                        let mut new_ltok = ltok.clone();
                                        new_ltok.concat(name.clone());
                                        *name = ltok.clone();
                                        break;
                                    },

                                    CSTNode::VarFetch{
                                        ref at1,
                                        var: _,
                                        at2: _,
                                    } => {
                                        self.errs.push(CSTReport::ExpectedXGotY{
                                            x: Token::Number,
                                            y: at1.clone(),
                                        });
                                        return Some(current_lhs);
                                    },

                                    _ => {
                                        self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee{ code: 114 }));
                                        return None;
                                    },
                                }
                            }

                            return Some(current_lhs);
                        }

                        _ => return Some(CSTNode::Label{ name: ltok.clone() }),
                    }
                },

                /* Also possible that we can put this in a separate function. */
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
                        self.errs.push(CSTReport::InternalError(IntErrID::FetchCSTNode{ code: 209 }));
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
                            break;
                        } else if next_tok.token == Token::Newline {
                            ls_iter.next();
                            self.errs.push(CSTReport::UnclosedDelimiter(quot));
                            return None;
                        }
                        let Some(stok) = contents else {
                            *contents = Some((*next_tok).clone());
                            ls_iter.next();
                            continue;
                        };
                        stok.concat((*next_tok).clone());
                        ls_iter.next();
                    }

                    let opt_space: LexToken<'linespan>;

                    match ls_iter.peek() {
                        Some(peek) => match peek.token {
                            Token::Plus => {
                                let _peek = (*peek).clone();
                                ls_iter.next();
                                self.consume_space(ls_iter);
                                return self.parse_bin_op(ls_iter, quot, _peek);
                            },

                            Token::Space | Token::Continue | Token::MultiComment
                                => {
                                    opt_space = (*peek).clone();
                                    ls_iter.next();
                                },

                            _ => return Some(quot),
                        },

                        None => return Some(quot),
                    }

                    match ls_iter.peek() {
                        Some(peek) if peek.token == Token::Plus => {
                            let _peek = (*peek).clone();
                            ls_iter.next();
                            self.consume_space(ls_iter);
                            return self.parse_bin_op(ls_iter, quot, _peek);
                        },

                        _ => {
                            self.errs.push(CSTReport::ExpectedXGotY{
                                x: Token::Newline,
                                y: opt_space,
                            });
                            self.consume_remaining(ls_iter);
                            return Some(quot)
                        },
                    }
                },

                Token::LParen => return self.parse_paren(ls_iter, ltok.clone()),
                Token::LBracket => return self.parse_bracket(ls_iter, ltok.clone()),

                Token::Space | Token::Continue | Token::MultiComment => {
                    self.consume_space(ls_iter);
                    return self.parse_expr(ls_iter, indent_sz, is_varfetch);
                },

                Token::Comment | Token::Newline => return None,

                _ => {
                    self.errs.push(CSTReport::InvalidExpressionXAfterY{
                        x: ltok.clone(),
                        y: None,
                    });
                    return None;
                },
            },

            None => {
                self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee{ code: 116 }));
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
        ) else { return None };

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
                self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee{ code: 117 }));
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
                    return Some(CSTNode::VarElement{
                        indent_sz,
                        var: Box::new(var),
                        dot,
                        element: Some(Box::new(CSTNode::Label{ name: ltok.clone() })),
                    });
                },

                Token::Dot => {
                    let _peek = (*peek).clone();
                    ls_iter.next();
                    return self.parse_element(ls_iter, varmethod, _peek, indent_sz);
                },

                _ => return Some(varmethod),
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
            let Some(ltok) = ls_iter.next() else { break };

            let Some(ref mut _path) = path else {
                path = Some(CSTNode::Label{
                    name: ltok.clone(),
                });
                is_expect_slash = true;
                continue;
            };

            let CSTNode::Label{name} = _path else {
                self.errs.push(CSTReport::InternalError(IntErrID::ExpectedGuarantee{ code: 118 }));
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
                    name.concat(ltok.clone());
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
        match self.parse_expr(ls_iter, 0, false) {
            Some(expr) => *rhs = Some(Box::new(expr)),
            None => return None,
        }

        if self.check_consume_trailing(ls_iter) { return Some(bin_op) }
        match ls_iter.peek() {
            Some(peek) => match peek.token {
                Token::Newline => {
                    ls_iter.next();
                    return Some(bin_op);
                },

                Token::Plus | Token::Dash | Token::Star | Token::Slash | Token::Percent | Token::ExclMark
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
            self.errs.push(CSTReport::InternalError(IntErrID::FetchCSTNode{ code: 212 }));
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

    /* Consumes everything up until a newline for errors */
    fn consume_remaining<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self,
        ls_iter: &mut Peekable<I>
    ) {
        loop {
            let Some(peek) = ls_iter.peek() else { break };
            if peek.token != Token::Comment || peek.token != Token::Newline { ls_iter.next(); }
            else { break }
        }
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
                        => {
                            ls_iter.next();
                            continue;
                        },
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

        let _ltok = (*ltok).clone();
        if _ltok.token != token {
            self.errs.push(CSTReport::ExpectedXGotY{
                x: token,
                y: _ltok.clone(),
            });
            return Some(_ltok);
        }

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
