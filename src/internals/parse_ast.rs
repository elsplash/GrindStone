/*
 * Oh boy! I sure do hope this shit won't take as long as the CST!
 * - ESplash (09.17.26)
 */

use crate::internals::{
    StrSpan,
    LexToken,
    Token,
    LineSpan,

    CSTNode,
    CSTOpType,
};

#[derive(Copy, Clone)]
pub enum ASTOpType {
    Plus, Minus, Times, Divide, Modulo,
    Greater, Lesser, GreatEq, LessEq,
    And, Or, Not,
    Equal,

    Increment, Decrement,
    AddAssign, SubAssign, MultAssign, DivAssign,
}

pub enum ASTNode<'linespan> {
    Number{
        val: f64,
        span: StrSpan<'linespan>,
    },
    String(Option<StrSpan<'linespan>>),
    Label(StrSpan<'linespan>),

    Paren(Vec<ASTNode<'linespan>>),
    Brack(Vec<ASTNode<'linespan>>),

    BinOp{
        lhs: Box<ASTNode<'linespan>>,
        op: ASTOpType,
        rhs: Box<ASTNode<'linespan>>,
    },

    UnOp{
        var: Box<ASTNode<'linespan>>,
        op: ASTOpType,
    },

    VarFetch{
        var: Box<ASTNode<'linespan>>,
        fetch: StrSpan<'linespan>,
    },

    VarMethod{
        var: Box<ASTNode<'linespan>>,
        m_name: StrSpan<'linespan>,
        m_paren: Box<ASTNode<'linespan>>,
    },

    TableAccess{
        var: StrSpan<'linespan>,
        index: Box<ASTNode<'linespan>>,
    },

    Ascii(Vec<LineSpan>),

    Func{
        name: StrSpan<'linespan>,
        paren: Box<ASTNode<'linespan>>,
    },

    VarDecl(Box<ASTNode<'linespan>>),
    FuncDecl(Box<ASTNode<'linespan>>),

    If(Box<ASTNode<'linespan>>),
    Else(Option<Box<ASTNode<'linespan>>>),

    Import(Box<ASTNode<'linespan>>),
    New(Box<ASTNode<'linespan>>),
}

pub struct ASTBlock<'linespan> {
    indent_lv: usize,
    block: Vec<ASTNode<'linespan>>,
    errs: Vec<ASTReport<'linespan>>,
    /* NOTE: With errors collected on this block, need to be cleared when passed over to ASTOutput's errs */
}

pub struct ASTOutput<'linespan> {
    output: Vec<ASTBlock<'linespan>>,
    errs: Vec<ASTReport<'linespan>>,
    /* NOTE: errs are all errors output has. */
}

/*
 * NOTE: We need to replace some of the MissingXInY with different errors
 *       since we need to be more verbose about it, and the fucking help
 *       messages too.
 */
pub enum ASTReport<'linespan> {
    InternalError(u16),

    MissingXInY{
        x: String,
        y: CSTNode<'linespan>,
    }
}

impl ASTOpType {
    pub fn default() -> ASTOpType { ASTOpType::Equal }

    pub fn translate_c_op(&mut self, c_op: CSTOpType) -> ASTOpType {
        match c_op {
            CSTOpType::Increment => *self = ASTOpType::Increment,
            CSTOpType::Decrement => *self = ASTOpType::Decrement,
            CSTOpType::AddAssign => *self = ASTOpType::AddAssign,
            CSTOpType::SubAssign => *self = ASTOpType::SubAssign,
            CSTOpType::MultAssign => *self = ASTOpType::MultAssign,
            CSTOpType::DivAssign => *self = ASTOpType::DivAssign,
        }

        return *self;
    }

    pub fn translate_ltok(&mut self, ltok: LexToken) -> Option<ASTOpType> {
        match ltok.token {
            Token::Plus => *self = ASTOpType::Plus,
            Token::Dash => *self = ASTOpType::Minus,
            Token::Star => *self = ASTOpType::Times,
            Token::Slash => *self = ASTOpType::Divide,
            Token::Percent => *self = ASTOpType::Modulo,

            Token::And => *self = ASTOpType::And,
            Token::ExclMark => *self = ASTOpType::Not,
            Token::Bar => *self = ASTOpType::Or,

            Token::Equal => *self = ASTOpType::Equal,

            _ => return None,
        }

        return Some(*self);
    }
}

/*
 * TODO LIST 2000:
 * Since I cannot keep track of everything...
 * (Should be done in order)
 *
 * [ASTBlock] -> 1. Do the Equip Node first.
 *               2. Boring nodes (Enable, Brew, etc)
 *               3. For loop
 *               4. The main AST Block parser caller
 *
 * [ASTOutput] -> 1. We only need a public function, not an implementation.
 *
 * [ASTNode] -> 1. StrSpan fetcher for each node, recursive descent.
 * 				2. LineSpan fetcher.
 *				3. ASTReport helpers, like replace, remove, etc.
 *
 * [ASTReport] -> 1. Replace all the MissingXInY with special cases, and run helper code for them.
 *				  2. Replace the CSTNode placeholders.
 */
impl<'linespan> ASTBlock<'linespan> {
    pub fn new() -> ASTBlock<'linespan> {
        ASTBlock{
            indent_lv: 0,
            block: Vec::new(),
            errs: Vec::new(),
        }
    }

    /* NOTE: The else parse function is small enough to inline. */

    fn parse_if_stmnt(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTBlock<'linespan>> {
        let _cnode = cnode.clone();
        let CSTNode::If{
            key: _,
            indent_sz,
            bin_op,
        } = cnode else {
            self.errs.push(ASTReport::InternalError(100));
            return None;
        };

        if self.indent_lv < indent_sz { return None }
        else if self.indent_lv > indent_sz {
            return ASTBlock{
                indent_lv: indent_sz,
                block: Vec::new(),
                errs: Vec::new(),
            }.parse_if_stmnt(_cnode);
        }

        let Some(_bin_op) = bin_op else {
            self.errs.push(ASTReport::MissingXInY{
                x: "Check".to_string(),
                y: _cnode,
            });
            return None;
        };

        let Some(__bin_op) = self.parse_expr(*_bin_op) else { return None };

        self.block.push(ASTNode::If(Box::new(__bin_op)));

        None
    }

    fn parse_var_decl(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTBlock<'linespan>> {
        let _cnode = cnode.clone();
        let CSTNode::VarDecl{
            key: _,
            indent_sz,
            name,
            equal,
            def,
        } = cnode else {
            self.errs.push(ASTReport::InternalError(101));
            return None;
        };

        if self.indent_lv < indent_sz { return None }
        else if self.indent_lv > indent_sz {
            return ASTBlock{
                indent_lv: indent_sz,
                block: Vec::new(),
                errs: Vec::new(),
            }.parse_var_decl(_cnode);
        }

        let Some(_name) = name else {
            self.errs.push(ASTReport::MissingXInY{
                x: "Name".to_string(),
                y: _cnode,
            });
            return None;
        };

        if equal.is_none() {
            self.errs.push(ASTReport::MissingXInY{
                x: "Equal".to_string(),
                y: _cnode.clone(),
            });
        }

        let Some(_def) = def else {
            self.errs.push(ASTReport::MissingXInY{
                x: "Definition".to_string(),
                y: _cnode,
            });
            return None;
        };

        let Some(__def) = self.parse_expr(*_def) else { return None };

        self.block.push(ASTNode::VarDecl(Box::new(ASTNode::BinOp{
            lhs: Box::new(ASTNode::Label(_name.span)),
            op: ASTOpType::Equal,
            rhs: Box::new(__def),
        })));

        None
    }

    fn parse_func_decl(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTBlock<'linespan>> {
        let _cnode = cnode.clone();
        let CSTNode::FuncDecl{
            key: _,
            indent_sz,
            name,
            paren,
        } = cnode else {
            self.errs.push(ASTReport::InternalError(102));
            return None;
        };

        if self.indent_lv < indent_sz { return None }
        else if self.indent_lv > indent_sz {
            return ASTBlock{
                indent_lv: indent_sz,
                block: Vec::new(),
                errs: Vec::new(),
            }.parse_func_decl(_cnode);
        }

        let Some(_name) = name else {
            self.errs.push(ASTReport::MissingXInY{
                x: "Name".to_string(),
                y: _cnode,
            });
            return None;
        };

        let Some(_paren) = paren else {
            self.errs.push(ASTReport::MissingXInY{
                x: "Parenthesis".to_string(),
                y: _cnode,
            });
            return None;
        };

        let Some(__paren) = self.parse_expr(*_paren) else { return None };

        self.block.push(ASTNode::FuncDecl(Box::new(ASTNode::Func{
            name: _name.span,
            paren: Box::new(__paren),
        })));

        None
    }

    fn parse_var_mutator(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTBlock<'linespan>> {
        let _cnode = cnode.clone();
        let CSTNode::VarMutator{
            indent_sz,
            name,
            operator,
            amount,
        } = cnode else {
            self.errs.push(ASTReport::InternalError(103));
            return None;
        };

        if self.indent_lv < indent_sz { return None }
        else if self.indent_lv > indent_sz {
            return ASTBlock{
                indent_lv: indent_sz,
                block: Vec::new(),
                errs: Vec::new(),
            }.parse_var_mutator(_cnode);
        }

        let Some(_name) = name else {
            self.errs.push(ASTReport::MissingXInY{
                x: "Variable".to_string(),
                y: _cnode,
            });
            return None;
        };

        let Some(_operator) = operator else {
            self.errs.push(ASTReport::MissingXInY{
                x: "Operator".to_string(),
                y: _cnode,
            });
            return None;
        };

        let ast_op = ASTOpType::default().translate_c_op(_operator);

        let Some(_amount) = amount else {
            self.block.push(ASTNode::UnOp{
                var: Box::new(ASTNode::Label(_name.span)),
                op: ast_op,
            });
            return None;
        };

        let Some(__amount) = self.parse_expr(*_amount) else { return None };

        self.block.push(ASTNode::BinOp{
            lhs: Box::new(ASTNode::Label(_name.span)),
            op: ast_op,
            rhs: Box::new(__amount),
        });

        None
    }

    fn parse_expr(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTNode<'linespan>> {
        let _cnode = cnode.clone();
        match cnode {
            CSTNode::Label{ name } => return Some(ASTNode::Label(name.span)),

            CSTNode::String{
                lquot: _,
                contents,
                rquot: _,
            } => {
                let Some(_contents) = contents else { return Some(ASTNode::String(None)) };
                return Some(ASTNode::String(Some(_contents.span)));
            },

            CSTNode::Number{ num } => {
                let Ok(val) = num.span.str.parse::<f64>() else {
                    self.errs.push(ASTReport::InternalError(104));
                    return None;
                };
                return Some(ASTNode::Number{
                    val,
                    span: num.span
                });
            },

            CSTNode::Paren{
                lparen: _,
                contents: _,
                ref rparen,
            } => {
                if rparen.is_none() {
                    self.errs.push(ASTReport::MissingXInY{
                        x: ")".to_string(),
                        y: _cnode,
                    });
                }
                return self.parse_paren(cnode);
            },

            CSTNode::Brack{
                lbrack: _,
                contents: _,
                ref rbrack,
            } => {
                if rbrack.is_none() {
                    self.errs.push(ASTReport::MissingXInY{
                        x: "]".to_string(),
                        y: _cnode,
                    });
                }
                return self.parse_bracket(cnode);
            },

            CSTNode::VarFetch{
                at1: _,
                var: _,
                at2: _,
            } => return self.parse_var_fetch(cnode),

            CSTNode::Item{
                var,
                comma,
            } => {
                if comma.is_none() {
                    self.errs.push(ASTReport::MissingXInY{
                        x: ",".to_string(),
                        y: _cnode.clone(),
                    });
                }
                let Some(_var) = var else {
                    self.errs.push(ASTReport::MissingXInY{
                        x: "Variable".to_string(),
                        y: _cnode,
                    });
                    return None;
                };
                return self.parse_expr(*_var);
            },

            CSTNode::BinOp{
                lhs: _,
                op: _,
                rhs: _,
            } => return self.parse_bin_op(cnode),

            CSTNode::Ascii{
                key_start: _,
                lines: _,
                key_end: _,
            } => return self.parse_ascii(cnode),

            CSTNode::TableAccess{
                label: _,
                brack: _,
            } => return self.parse_table_access(cnode),

            CSTNode::VarMethod{
                indent_sz: _,
                var: _,
                dot: _,
                method: _,
            } => return self.parse_var_method(cnode),

            CSTNode::FuncCall{
                indent_sz: _,
                name: _,
                paren: _,
            } => return self.parse_func(cnode),

            CSTNode::Import{
                indent_sz: _,
                key: _,
                path: _,
            } => return self.parse_import(cnode),

            CSTNode::New{
                indent_sz: _,
                key: _,
                path: _,
            } => return self.parse_new(cnode),

            _ => {
                self.errs.push(ASTReport::InternalError(105));
                return None;
            }
        }
    }

    fn parse_paren(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTNode<'linespan>> {
        let CSTNode::Paren{
            lparen: _,
            contents,
            rparen: _,
        } = cnode else {
            self.errs.push(ASTReport::InternalError(106));
            return None;
        };

        let mut ast_contents: Vec<ASTNode<'linespan>> = Vec::new();

        for node in contents.iter() {
            let CSTNode::Item{
                var,
                comma,
            } = node else {
                self.errs.push(ASTReport::InternalError(107));
                return None;
            };
            if comma.is_none() {
                self.errs.push(ASTReport::MissingXInY{
                    x: ",".to_string(),
                    y: node.clone(),
                });
            }
            let Some(ref _var) = *var else {
                self.errs.push(ASTReport::MissingXInY{
                    x: "Identifier".to_string(),
                    y: node.clone(),
                });
                continue;
            };
            let Some(__var) = self.parse_expr(*(_var.clone())) else { continue };
            ast_contents.push(__var);
        }

        Some(ASTNode::Paren(ast_contents))
    }

    fn parse_bracket(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTNode<'linespan>> {
        let CSTNode::Brack{
            lbrack: _,
            contents,
            rbrack: _,
        } = cnode else {
            self.errs.push(ASTReport::InternalError(108));
            return None;
        };

        let mut ast_contents: Vec<ASTNode<'linespan>> = Vec::new();

        for node in contents.iter() {
            let CSTNode::Item{
                var,
                comma,
            } = node else {
                self.errs.push(ASTReport::InternalError(109));
                return None;
            };
            if comma.is_none() {
                self.errs.push(ASTReport::MissingXInY{
                    x: ",".to_string(),
                    y: node.clone(),
                });
            }
            let Some(ref _var) = *var else {
                self.errs.push(ASTReport::MissingXInY{
                    x: "Identifier".to_string(),
                    y: node.clone(),
                });
                continue;
            };
            let Some(__var) = self.parse_expr(*(_var.clone())) else { continue };
            ast_contents.push(__var);
        }

        Some(ASTNode::Brack(ast_contents))
    }

    fn parse_bin_op(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTNode<'linespan>> {
        let _cnode = cnode.clone();
        let CSTNode::BinOp{
            lhs,
            op,
            rhs,
        } = cnode else {
            self.errs.push(ASTReport::InternalError(110));
            return None;
        };

        let Some(_lhs) = self.parse_expr(*lhs) else { return None };

        let Some(ast_op) = ASTOpType::default().translate_ltok(op) else {
            self.errs.push(ASTReport::InternalError(111));
            return None;
        };

        let Some(_rhs) = rhs else {
            self.errs.push(ASTReport::MissingXInY{
                x: "Right expression".to_string(),
                y: _cnode,
            });
            return None;
        };

        let Some(__rhs) = self.parse_expr(*_rhs) else { return None };

        Some(ASTNode::BinOp{
            lhs: Box::new(_lhs),
            op: ast_op,
            rhs: Box::new(__rhs),
        })
    }

    fn parse_var_fetch(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTNode<'linespan>> {
        let _cnode = cnode.clone();
        let CSTNode::VarFetch{
            mut at1,
            var,
            at2,
        } = cnode else {
            self.errs.push(ASTReport::InternalError(112));
            return None;
        };

        if at2.is_none() {
            self.errs.push(ASTReport::MissingXInY{
                x: "@".to_string(),
                y: _cnode.clone(),
            });
        }

        let Some(_var) = var else {
            self.errs.push(ASTReport::MissingXInY{
                x: "Variable fetch".to_string(),
                y: _cnode,
            });
            return None;
        };

        let Some(__var) = self.parse_expr(*_var) else { return None };

        /*
         * NOTE: Since I'm too lazy to think of a solution to make the CSTNodes return their spans
         *       I will instead make a stupid looking shitty workaround.
         */
        for c in at1.span.line.str.chars().skip(at1.span.clmn) {
            at1.span.str.push(c);
            if c == '@' {
                break;
            }
        }

        Some(ASTNode::VarFetch{var: Box::new(__var), fetch: at1.span})
    }

    /* NOTE: For future implementation, please check the indent_sz */
    fn parse_var_method(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTNode<'linespan>> {
        let _cnode = cnode.clone();
        let CSTNode::VarMethod{
            indent_sz: _,
            dot: _,
            var,
            method,
        } = cnode else {
            self.errs.push(ASTReport::InternalError(109));
            return None;
        };

        let Some(_var) = self.parse_expr(*var) else { return None };
        let Some(_method) = method else {
            self.errs.push(ASTReport::MissingXInY{
                x: "Method".to_string(),
                y: _cnode
            });
            return None;
        };

        let CSTNode::FuncCall{
            indent_sz: _,
            name,
            paren,
        } = *_method else {
            self.errs.push(ASTReport::InternalError(113));
            return None;
        };

        let Some(_paren) = paren else {
            self.errs.push(ASTReport::InternalError(114));
            return None;
        };
        let Some(__paren) = self.parse_expr(*_paren) else { return None };

        Some(ASTNode::VarMethod{
            var: Box::new(_var),
            m_name: name.span,
            m_paren: Box::new(__paren),
        })
    }

    fn parse_table_access(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTNode<'linespan>> {
        let _cnode = cnode.clone();
        let CSTNode::TableAccess{
            label,
            brack,
        } = cnode else {
            self.errs.push(ASTReport::InternalError(115));
            return None;
        };

        let Some(_brack) = brack else {
            self.errs.push(ASTReport::MissingXInY{
                x: "Bracket".to_string(),
                y: _cnode,
            });
            return None;
        };

        let Some(__brack) = self.parse_expr(*_brack) else { return None };

        Some(ASTNode::TableAccess{
            var: label.span,
            index: Box::new(__brack),
        })
    }

    fn parse_ascii(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTNode<'linespan>> {
        let _cnode = cnode.clone();
        let CSTNode::Ascii{
            key_start: _,
            lines,
            key_end,
        } = cnode else {
            self.errs.push(ASTReport::InternalError(116));
            return None;
        };

        if key_end.is_none() {
            self.errs.push(ASTReport::MissingXInY{
                x: "asciiend".to_string(),
                y: _cnode,
            });
        }

        Some(ASTNode::Ascii(lines))
    }

    /* NOTE: For future implementation, please check the indent_sz */
    fn parse_func(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTNode<'linespan>> {
        let _cnode = cnode.clone();
        let CSTNode::FuncCall{
            indent_sz: _,
            name,
            paren,
        } = cnode else {
            self.errs.push(ASTReport::InternalError(117));
            return None;
        };

        let Some(_paren) = paren else {
            self.errs.push(ASTReport::MissingXInY{
                x: "Parenthesis".to_string(),
                y: _cnode,
            });
            return None;
        };

        let Some(__paren) = self.parse_expr(*_paren) else { return None };

        Some(ASTNode::Func{
            name: name.span,
            paren: Box::new(__paren),
        })
    }

    /* NOTE: For future implementation, please check the indent_sz */
    fn parse_import(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTNode<'linespan>> {
        let _cnode = cnode.clone();
        let CSTNode::Import{
            indent_sz: _,
            key: _,
            path,
        } = cnode else {
            self.errs.push(ASTReport::InternalError(118));
            return None;
        };

        let Some(_path) = path else {
            self.errs.push(ASTReport::MissingXInY{
                x: "Path".to_string(),
                y: _cnode.clone(),
            });
            return None;
        };

        let Some(__path) = self.parse_expr(*_path) else { return None };

        Some(ASTNode::Import(Box::new(__path)))
    }

    /* NOTE: For future implementation, please check the indent_sz */
    fn parse_new(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTNode<'linespan>> {
        let _cnode = cnode.clone();
        let CSTNode::New{
            indent_sz: _,
            key: _,
            path,
        } = cnode else {
            self.errs.push(ASTReport::InternalError(119));
            return None;
        };

        let Some(_path) = path else {
            self.errs.push(ASTReport::MissingXInY{
                x: "Path".to_string(),
                y: _cnode.clone(),
            });
            return None;
        };

        let Some(__path) = self.parse_expr(*_path) else { return None };

        Some(ASTNode::New(Box::new(__path)))
    }
}

/* Might need some functions in ASTNode for the analyzer */
