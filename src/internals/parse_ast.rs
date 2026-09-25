/*
 * Oh boy! I sure do hope this shit won't take as long as the CST!
 * - ESplash (09.17.26)
 *
 * I was fucking wrong.
 * - ESplash (09.21.26)
 */

use crate::internals::{
    CSTNode, CSTOpType, CSTOutput, EquipType, LexToken, LineSpan, StrSpan, Token,
};

use std::{
    fmt::{self, Display},
    iter::Peekable,
};

#[derive(Copy, Clone)]
pub enum ASTOpType {
    Plus,
    Minus,
    Times,
    Divide,
    Modulo,
    Greater,
    Lesser,
    GreatEq,
    LessEq,
    And,
    Or,
    Not,
    Equal,

    Increment,
    Decrement,
    AddAssign,
    SubAssign,
    MultAssign,
    DivAssign,
}

/* These are based on the coordinates so we can track when something can go offscreen */
#[derive(Clone, PartialEq, Eq)]
pub enum PrintType {
    Normal,
    Advanced,
    Centered,
    Relative,
    BigHead,
}

#[derive(Clone)]
pub enum ASTNode<'linespan> {
    Number {
        val: f64,
        span: StrSpan<'linespan>,
    },
    String(Option<StrSpan<'linespan>>),
    Label(StrSpan<'linespan>),

    Paren(Vec<ASTNode<'linespan>>),
    Brack(Vec<ASTNode<'linespan>>),

    BinOp {
        lhs: Box<ASTNode<'linespan>>,
        op: ASTOpType,
        rhs: Box<ASTNode<'linespan>>,
    },

    UnOp {
        var: Box<ASTNode<'linespan>>,
        op: ASTOpType,
    },

    VarFetch {
        var: Box<ASTNode<'linespan>>,
        fetch: StrSpan<'linespan>,
    },

    VarMethod {
        var: Box<ASTNode<'linespan>>,
        m_name: StrSpan<'linespan>,
        m_paren: Box<ASTNode<'linespan>>,
    },

    TableAccess {
        var: StrSpan<'linespan>,
        index: Box<ASTNode<'linespan>>,
    },

    Ascii(Vec<LineSpan>),

    Func {
        name: StrSpan<'linespan>,
        paren: Box<ASTNode<'linespan>>,
    },

    VarDecl(Box<ASTNode<'linespan>>),
    FuncDecl(Box<ASTNode<'linespan>>),

    If(Box<ASTNode<'linespan>>),
    Else(Option<Box<ASTNode<'linespan>>>),

    Import(Box<ASTNode<'linespan>>),
    New(Box<ASTNode<'linespan>>),

    Equip {
        etype: EquipType,
        tool_name: Vec<ASTNode<'linespan>>,
        plus: Vec<ASTNode<'linespan>>,
        minus: Vec<ASTNode<'linespan>>,
        star: Vec<ASTNode<'linespan>>,
    },

    Enable {
        opt: Box<ASTNode<'linespan>>,
        opt_opts: Option<Box<ASTNode<'linespan>>>,
    },

    Disable {
        opt: Box<ASTNode<'linespan>>,
        opt_opts: Option<Box<ASTNode<'linespan>>>,
    },

    Brew {
        lhs: Option<Box<ASTNode<'linespan>>>,
        op: Option<ASTOpType>,
        rhs: Option<Box<ASTNode<'linespan>>>,
    },

    Loadout(Box<ASTNode<'linespan>>),

    Activate(Box<ASTNode<'linespan>>),

    Play {
        sound: Box<ASTNode<'linespan>>,
        pitch: Option<Box<ASTNode<'linespan>>>,
    },

    For {
        var: StrSpan<'linespan>,
        start: Box<ASTNode<'linespan>>,
        end: Box<ASTNode<'linespan>>,
    },

    Print {
        ptype: PrintType,
        spcl_x: Option<Box<ASTNode<'linespan>>>,
        spcl_y: Option<Box<ASTNode<'linespan>>>,
        spcl_clr: Option<StrSpan<'linespan>>,
        contents: Vec<ASTNode<'linespan>>,
    },
}

pub struct ASTBlock<'linespan> {
    indent_lv: usize,
    block: Vec<ASTNode<'linespan>>,
    errs: Vec<ASTReport<'linespan>>,
}

pub struct ASTOutput<'linespan> {
    output: Vec<ASTBlock<'linespan>>,
    errs: Vec<ASTReport<'linespan>>,
}

pub enum ARMissing {
    /* Print Statements */
    XPos,
    YPos,
    ClrKey,
    ClrVal,

    /* Variable */
    Var,
    VarName,
    VarDef,

    /* Function */
    Func,
    FuncName,
    FuncParen,

    /* For Statements */
    StartNum,
    Dot1,
    Dot2,
    EndNum,

    /* Play*/
    Sound,

    /* Brew */
    Ingr1,
    Ingr2,

    /* If */
    Condition,

    /* Equip */
    ToolName,

    /* Bin Op */
    Operator,
    RHSExpr,

    /* Misc */
    Equal,
    Number,

    Paren,
    RParen,

    Bracket,
    RBracket,
    LBracket,

    AsciiEnd,

    Comma,
    Option,
    At,
    Path,
}

pub enum ASTReport<'linespan> {
    InternalError(u16),

    MissingXInY {
        x: ARMissing,
        y: CSTNode<'linespan>,
    }, /* NOTE: This is still needed by the way */

    MissingXInYAfterZ {
        x: ARMissing,
        y: CSTNode<'linespan>,

        /* NOTE: Could be a LexToken or CSTNode. */
        z: CSTNode<'linespan>,
    },

    ImplicitPrintCastInXBecauseY{
        x: CSTNode<'linespan>,
        y: Box<ASTReport<'linespan>>,
    },
}

impl ASTOpType {
    pub fn default() -> ASTOpType {
        ASTOpType::Equal
    }

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

/* NOTE: bool is "Succeeded?". */
pub fn parse_all<'linespan>(
    ast_output: &mut ASTOutput<'linespan>,
    cst_output: &'linespan CSTOutput<'linespan>,
) -> bool {
    let mut cn_iter = cst_output.output.iter().peekable();
    let mut prev_indent_lvs: Vec<usize> = vec![0];
    let mut curr_ast_block = ASTBlock {
        indent_lv: 0,
        block: Vec::new(),
        errs: Vec::new(),
    };

    loop {
        match curr_ast_block.parse_block(&mut cn_iter) {
            Some(new_block) => {
                let ASTBlock {
                    ref mut indent_lv,
                    ref mut block,
                    ref mut errs,
                } = curr_ast_block;
                prev_indent_lvs.push(*indent_lv);
                block.clear();
                ast_output.errs.append(errs);

                ast_output.output.push(curr_ast_block);
                curr_ast_block = new_block;
            }

            None => {
                ast_output.output.push(curr_ast_block);
                let Some(prev) = prev_indent_lvs.last() else {
                    break;
                };
                if *prev == 0 {
                    break;
                }
                curr_ast_block = ASTBlock {
                    indent_lv: *prev,
                    block: Vec::new(),
                    errs: Vec::new(),
                };
            }
        }
    }

    true
}

impl<'linespan> Display for ASTReport<'linespan> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (err_msg, line, hint, help) = match self {
            ASTReport::InternalError(code) => (
                "Internal Error, this is not your fault.".to_string(),
                ";)".to_string(),
                format!("You may report this to ESplash, ERRCODE: {}", *code),
                None::<String>,
            ),

            _ => todo!(),
        };
        write!(
            f,
            "[ERROR] {err_msg}\n{line}\n[HINT] {hint}\n{}",
            match help {
                Some(s) => s,
                None => "".to_string(),
            }
        )
    }
}

/*
 * TODO LIST 2000:
 * Since I cannot keep track of everything...
 * (Should be done in order)
 *
 * [ASTReport] -> 1. Replace all the MissingXInY with special cases, and run helper code for them.
 *                2. Replace the CSTNode placeholders.
 */
impl<'linespan> ASTBlock<'linespan> {
    pub fn new() -> ASTBlock<'linespan> {
        ASTBlock {
            indent_lv: 0,
            block: Vec::new(),
            errs: Vec::new(),
        }
    }

    /* NOTE: When it returns some, please rerun this function on the block who returned the block. */
    fn parse_block<I: Iterator<Item = &'linespan CSTNode<'linespan>>>(
        &mut self,
        cn_iter: &mut Peekable<I>,
    ) -> Option<ASTBlock<'linespan>> {
        loop {
            let Some(cnode) = cn_iter.peek() else { break };
            let err_count = self.errs.len();
            let block_count = self.block.len();
            let rval: Option<ASTBlock<'linespan>>;

            match cnode {
                CSTNode::Print { .. } => rval = self.parse_print((*cnode).clone()),
                CSTNode::For { .. } => rval = self.parse_for((*cnode).clone()),
                CSTNode::Play { .. } => rval = self.parse_for((*cnode).clone()),
                CSTNode::Brew { .. } => rval = self.parse_brew((*cnode).clone()),
                CSTNode::Disable { .. } => rval = self.parse_disable((*cnode).clone()),
                CSTNode::Enable { .. } => rval = self.parse_enable((*cnode).clone()),
                CSTNode::Equip { .. } => rval = self.parse_equip((*cnode).clone()),
                CSTNode::If { .. } => rval = self.parse_if_stmnt((*cnode).clone()),
                CSTNode::VarDecl { .. } => rval = self.parse_var_decl((*cnode).clone()),
                CSTNode::FuncDecl { .. } => rval = self.parse_func_decl((*cnode).clone()),
                CSTNode::VarMutator { .. } => rval = self.parse_var_mutator((*cnode).clone()),
                CSTNode::VarDef { .. } => rval = self.parse_var_def((*cnode).clone()),

                /* NOTE: As of now, I realize it's probably better if we couple this to a function instead. */
                CSTNode::Loadout { indent_sz, num, .. } => {
                    let _cnode = (*cnode).clone();

                    if self.indent_lv < *indent_sz {
                        break;
                    } else if self.indent_lv < *indent_sz {
                        return ASTBlock {
                            indent_lv: *indent_sz,
                            block: Vec::new(),
                            errs: Vec::new(),
                        }
                        .parse_block(cn_iter);
                    }

                    let Some(_num) = num else {
                        self.errs.push(ASTReport::MissingXInY {
                            x: ARMissing::Number,
                            y: _cnode,
                        });
                        continue;
                    };

                    let Some(__num) = self.parse_expr(*(_num.clone())) else {
                        continue;
                    };

                    self.block.push(ASTNode::Loadout(Box::new(__num)));

                    cn_iter.next();
                    continue;
                }

                _ => {
                    let Some(expr) = self.parse_expr((*cnode).clone()) else {
                        continue;
                    };
                    self.block.push(expr);
                    cn_iter.next();
                    continue;
                }
            }

            cn_iter.next();
            match rval {
                Some(new_block) => return Some(new_block),

                None => {
                    if err_count < self.errs.len() || block_count < self.block.len() {
                        continue;
                    } else {
                        break;
                    }
                }
            }
        }

        None
    }

    fn parse_special_print_contents(&mut self, c: &Vec<CSTNode<'linespan>>) -> Vec<ASTNode<'linespan>> {
        let mut output = Vec::<ASTNode<'linespan>>::new();

        for node in c.iter() {
            let Some(_node) = self.parse_expr(node.clone()) else { break };
            output.push(_node);
        }

        output
    }

    fn parse_print(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTBlock<'linespan>> {
        let _cnode = cnode.clone();
        let CSTNode::Print {
            indent_sz,
            spcl_key,
            spcl_x_pos,
            spcl_y_pos,
            spcl_clr_key,
            spcl_clr_val,
            contents,
            ..
        } = cnode
        else {
            self.errs.push(ASTReport::InternalError(100));
            return None;
        };

        if self.indent_lv < indent_sz {
            return None;
        } else if self.indent_lv > indent_sz {
            return ASTBlock {
                indent_lv: indent_sz,
                block: Vec::new(),
                errs: Vec::new(),
            }
            .parse_print(_cnode);
        }

        let mut ptype = PrintType::Normal;
        let mut spcl_x = None::<Box<ASTNode<'linespan>>>;
        let mut spcl_y = None::<Box<ASTNode<'linespan>>>;
        let mut spcl_clr = None::<StrSpan<'linespan>>;

        if let Some(_spcl_key) = spcl_key {
            match _spcl_key.span.str.as_str() {
                "c" => ptype = PrintType::Centered,

                "`" => ptype = PrintType::Advanced,

                "h" | "o" | "f" => ptype = PrintType::Relative,

                "(" => ptype = PrintType::BigHead,

                _ => self.errs.push(ASTReport::InternalError(101)),
            }
        }

        if ptype != PrintType::Normal || ptype != PrintType::BigHead {
            let Some(_spcl_x_pos) = spcl_x_pos else {
                self.errs.push(ASTReport::ImplicitPrintCastInXBecauseY {
                    x: _cnode.clone(),
                	y: Box::new(ASTReport::MissingXInY {
                	    x: ARMissing::XPos,
                	    y: _cnode,
                	}),
                });
                return None;
            };

            let Some(__spcl_x_pos) = self.parse_expr(*_spcl_x_pos) else {
                return None;
            };

            spcl_x = Some(Box::new(__spcl_x_pos));

            let Some(_spcl_y_pos) = spcl_y_pos else {
                self.errs.push(ASTReport::MissingXInY {
                    x: ARMissing::YPos,
                    y: _cnode,
                });
                return None;
            };

            let Some(__spcl_y_pos) = self.parse_expr(*_spcl_y_pos) else {
                return None;
            };

            spcl_y = Some(Box::new(__spcl_y_pos));

            if let Some(_spcl_clr_key) = spcl_clr_key {
                let Some(_spcl_clr_val) = spcl_clr_val else {
                    self.errs.push(ASTReport::MissingXInY {
                        x: ARMissing::ClrVal,
                        y: _cnode,
                    });
                    return None;
                };

                spcl_clr = Some(_spcl_clr_val.span);
            }
        }

        let ast_contents = self.parse_special_print_contents(&contents);

        self.block.push(ASTNode::Print{
            ptype, spcl_x, spcl_y, spcl_clr, contents: ast_contents,
        });

        None
    }

    fn parse_for(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTBlock<'linespan>> {
        let _cnode = cnode.clone();
        let CSTNode::For {
            indent_sz,
            var,
            start,
            end,

            equal,
            dot1,
            dot2,
            ..
        } = cnode
        else {
            self.errs.push(ASTReport::InternalError(100));
            return None;
        };

        if self.indent_lv < indent_sz {
            return None;
        } else if self.indent_lv > indent_sz {
            return ASTBlock {
                indent_lv: indent_sz,
                block: Vec::new(),
                errs: Vec::new(),
            }
            .parse_for(_cnode);
        }

        let Some(_var) = var else {
            self.errs.push(ASTReport::MissingXInY {
                x: ARMissing::VarName,
                y: _cnode,
            });
            return None;
        };

        let Some(_equal) = equal else {
            self.errs.push(ASTReport::MissingXInYAfterZ {
                x: ARMissing::Equal,
                y: _cnode.clone(),
                z: CSTNode::Label { name: _var.clone() },
            });
            return None;
        };

        let Some(_start) = start else {
            self.errs.push(ASTReport::MissingXInYAfterZ {
                x: ARMissing::StartNum,
                y: _cnode,
                z: CSTNode::Label {
                    name: _equal.clone(),
                },
            });
            return None;
        };

        let Some(__start) = self.parse_expr(*(_start.clone())) else {
            return None;
        };

        let Some(_dot1) = dot1 else {
            self.errs.push(ASTReport::MissingXInYAfterZ {
                x: ARMissing::Dot1,
                y: _cnode.clone(),
                z: (*_start).clone(),
            });
            return None;
        };

        let Some(_dot2) = dot2 else {
            self.errs.push(ASTReport::MissingXInYAfterZ {
                x: ARMissing::Dot2,
                y: _cnode.clone(),
                z: CSTNode::Label { name: _dot1 },
            });
            return None;
        };

        let Some(_end) = end else {
            self.errs.push(ASTReport::MissingXInYAfterZ {
                x: ARMissing::EndNum,
                y: _cnode,
                z: CSTNode::Label { name: _dot2 },
            });
            return None;
        };

        let Some(__end) = self.parse_expr(*_end) else {
            return None;
        };

        self.block.push(ASTNode::For {
            var: _var.span,
            start: Box::new(__start),
            end: Box::new(__end),
        });

        None
    }

    fn parse_play(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTBlock<'linespan>> {
        let _cnode = cnode.clone();
        let CSTNode::Play {
            indent_sz,
            key,
            sound,
            pitch,
        } = cnode
        else {
            self.errs.push(ASTReport::InternalError(100));
            return None;
        };

        if self.indent_lv < indent_sz {
            return None;
        } else if self.indent_lv > indent_sz {
            return ASTBlock {
                indent_lv: indent_sz,
                block: Vec::new(),
                errs: Vec::new(),
            }
            .parse_play(_cnode);
        }

        let Some(_sound) = sound else {
            self.errs.push(ASTReport::MissingXInYAfterZ {
                x: ARMissing::Sound,
                y: _cnode,
                z: CSTNode::Label { name: key },
            });
            return None;
        };

        let Some(__sound) = self.parse_expr(*_sound) else {
            return None;
        };

        let Some(_pitch) = pitch else {
            self.block.push(ASTNode::Play {
                sound: Box::new(__sound),
                pitch: None,
            });
            return None;
        };

        let Some(__pitch) = self.parse_expr(*_pitch) else {
            return None;
        };

        self.block.push(ASTNode::Play {
            sound: Box::new(__sound),
            pitch: Some(Box::new(__pitch)),
        });

        None
    }

    /* Both parse_loadout() and parse_activate() are small enough to be inlined */

    fn parse_brew(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTBlock<'linespan>> {
        let _cnode = cnode.clone();
        let CSTNode::Brew {
            indent_sz,
            key,
            ingr1,
            plus,
            ingr2,
        } = cnode
        else {
            self.errs.push(ASTReport::InternalError(100));
            return None;
        };

        if self.indent_lv < indent_sz {
            return None;
        } else if self.indent_lv > indent_sz {
            return ASTBlock {
                indent_lv: indent_sz,
                block: Vec::new(),
                errs: Vec::new(),
            }
            .parse_brew(_cnode);
        }

        let mut ast_node = ASTNode::Brew {
            lhs: None,
            op: None,
            rhs: None,
        };

        let ASTNode::Brew {
            ref mut lhs,
            ref mut op,
            ref mut rhs,
        } = ast_node
        else {
            self.errs.push(ASTReport::InternalError(101));
            return None;
        };

        let Some(_ingr1) = ingr1 else {
            self.errs.push(ASTReport::MissingXInYAfterZ {
                x: ARMissing::Ingr1,
                y: _cnode.clone(),
                z: CSTNode::Label{ name: key },
            });

            let Some(_plus) = plus else { return None };

            let Some(_ingr2) = ingr2 else {
            	self.errs.push(ASTReport::MissingXInYAfterZ {
            	    x: ARMissing::Ingr2,
            	    y: _cnode,
            	    z: CSTNode::Label{ name: _plus },
            	});
                return None;
            };

            let Some(__ingr2) = self.parse_expr(*_ingr2) else {
                return None;
            };

            *rhs = Some(Box::new(__ingr2));

            self.errs.pop();
            self.block.push(ast_node);

            return None;
        };

        let Some(__ingr1) = self.parse_expr(*_ingr1) else { return None };
        *lhs = Some(Box::new(__ingr1.clone()));

        let Some(_plus) = plus else {
            self.block.push(ASTNode::Brew {
                lhs: Some(Box::new(__ingr1)),
                op: None,
                rhs: None,
            });
            return None;
        };

        let Some(__plus) = ASTOpType::default().translate_ltok(_plus.clone()) else {
            return None;
        };
        *op = Some(__plus);

        let Some(_ingr2) = ingr2 else {
            self.errs.push(ASTReport::MissingXInYAfterZ {
                x: ARMissing::Ingr2,
                y: _cnode,
                z: CSTNode::Label{ name: _plus },
            });
            return None;
        };

        let Some(__ingr2) = self.parse_expr(*_ingr2) else {
            return None;
        };
        *rhs = Some(Box::new(__ingr2));

        self.block.push(ast_node);

        None
    }

    fn parse_disable(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTBlock<'linespan>> {
        let _cnode = cnode.clone();
        let CSTNode::Disable {
            indent_sz,
            key,
            opt,
            opt_opts,
        } = cnode
        else {
            self.errs.push(ASTReport::InternalError(100));
            return None;
        };

        if self.indent_lv < indent_sz {
            return None;
        } else if self.indent_lv > indent_sz {
            return ASTBlock {
                indent_lv: indent_sz,
                block: Vec::new(),
                errs: Vec::new(),
            }
            .parse_disable(_cnode);
        }

        let Some(_opt) = opt else {
            self.errs.push(ASTReport::MissingXInYAfterZ {
                x: ARMissing::Option,
                y: _cnode,
                z: CSTNode::Label{ name: key },
            });
            return None;
        };

        let Some(__opt) = self.parse_expr(*_opt) else {
            return None;
        };

        let Some(_opt_opts) = opt_opts else {
            self.block.push(ASTNode::Disable {
                opt: Box::new(__opt),
                opt_opts: None,
            });
            return None;
        };

        let Some(__opt_opts) = self.parse_expr(*_opt_opts) else {
            self.block.push(ASTNode::Disable {
                opt: Box::new(__opt),
                opt_opts: None,
            });
            return None;
        };

        self.block.push(ASTNode::Disable {
            opt: Box::new(__opt),
            opt_opts: Some(Box::new(__opt_opts)),
        });

        None
    }

    fn parse_enable(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTBlock<'linespan>> {
        let _cnode = cnode.clone();
        let CSTNode::Enable {
            indent_sz,
            key,
            opt,
            opt_opts,
        } = cnode
        else {
            self.errs.push(ASTReport::InternalError(100));
            return None;
        };

        if self.indent_lv < indent_sz {
            return None;
        } else if self.indent_lv > indent_sz {
            return ASTBlock {
                indent_lv: indent_sz,
                block: Vec::new(),
                errs: Vec::new(),
            }
            .parse_enable(_cnode);
        }

        let Some(_opt) = opt else {
            self.errs.push(ASTReport::MissingXInYAfterZ {
                x: ARMissing::Option,
                y: _cnode,
                z: CSTNode::Label{ name: key },
            });
            return None;
        };

        let Some(__opt) = self.parse_expr(*_opt) else {
            return None;
        };

        let Some(_opt_opts) = opt_opts else {
            self.block.push(ASTNode::Enable {
                opt: Box::new(__opt),
                opt_opts: None,
            });
            return None;
        };

        let Some(__opt_opts) = self.parse_expr(*_opt_opts) else {
            self.block.push(ASTNode::Enable {
                opt: Box::new(__opt),
                opt_opts: None,
            });
            return None;
        };

        self.block.push(ASTNode::Enable {
            opt: Box::new(__opt),
            opt_opts: Some(Box::new(__opt_opts)),
        });

        None
    }

    fn parse_equip(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTBlock<'linespan>> {
        let _cnode = cnode.clone();
        let CSTNode::Equip {
            indent_sz,
            equip_type,
            tool_name,
            plus,
            minus,
            star,
            ..
        } = cnode
        else {
            self.errs.push(ASTReport::InternalError(100));
            return None;
        };

        if self.indent_lv < indent_sz {
            return None;
        } else if self.indent_lv > indent_sz {
            return ASTBlock {
                indent_lv: indent_sz,
                block: Vec::new(),
                errs: Vec::new(),
            }
            .parse_equip(_cnode);
        }

        let mut ast_tool_name: Vec<ASTNode<'linespan>> = Vec::new();
        let mut ast_plus: Vec<ASTNode<'linespan>> = Vec::new();
        let mut ast_minus: Vec<ASTNode<'linespan>> = Vec::new();
        let mut ast_star: Vec<ASTNode<'linespan>> = Vec::new();

        for node in tool_name {
            let Some(expr) = self.parse_expr(node) else {
                break;
            };
            ast_tool_name.push(expr);
        }

        for node in plus {
            let Some(expr) = self.parse_expr(node) else {
                break;
            };
            ast_plus.push(expr);
        }

        for node in minus {
            let Some(expr) = self.parse_expr(node) else {
                break;
            };
            ast_minus.push(expr);
        }

        for node in star {
            let Some(expr) = self.parse_expr(node) else {
                break;
            };
            ast_star.push(expr);
        }

        if ast_tool_name.is_empty() {
            self.errs.push(ASTReport::MissingXInY{
                x: ARMissing::ToolName,
                y: _cnode,
            });
        }

        self.block.push(ASTNode::Equip {
            etype: equip_type,
            tool_name: ast_tool_name,
            plus: ast_plus,
            minus: ast_minus,
            star: ast_star,
        });

        None
    }

    /* NOTE: The parse_else function is small enough to inlined. */

    fn parse_if_stmnt(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTBlock<'linespan>> {
        let _cnode = cnode.clone();
        let CSTNode::If {
            indent_sz, key, bin_op,
        } = cnode
        else {
            self.errs.push(ASTReport::InternalError(100));
            return None;
        };

        if self.indent_lv < indent_sz {
            return None;
        } else if self.indent_lv > indent_sz {
            return ASTBlock {
                indent_lv: indent_sz,
                block: Vec::new(),
                errs: Vec::new(),
            }
            .parse_if_stmnt(_cnode);
        }

        let Some(_bin_op) = bin_op else {
            self.errs.push(ASTReport::MissingXInYAfterZ {
                x: ARMissing::Condition,
                y: _cnode,
                z: CSTNode::Label{ name: key },
            });
            return None;
        };

        let Some(__bin_op) = self.parse_expr(*_bin_op) else {
            return None;
        };

        self.block.push(ASTNode::If(Box::new(__bin_op)));

        None
    }

    fn parse_var_decl(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTBlock<'linespan>> {
        let _cnode = cnode.clone();
        let CSTNode::VarDecl {
            indent_sz,
            key,
            name,
            equal,
            def,
        } = cnode
        else {
            self.errs.push(ASTReport::InternalError(101));
            return None;
        };

        if self.indent_lv < indent_sz {
            return None;
        } else if self.indent_lv > indent_sz {
            return ASTBlock {
                indent_lv: indent_sz,
                block: Vec::new(),
                errs: Vec::new(),
            }
            .parse_var_decl(_cnode);
        }

        let Some(_name) = name else {
            self.errs.push(ASTReport::MissingXInYAfterZ {
                x: ARMissing::VarName,
                y: _cnode,
                z: CSTNode::Label{ name: key },
            });
            return None;
        };

        let Some(_equal) = equal else {
            self.block.push(ASTNode::VarDecl(Box::new(ASTNode::Label(_name.span))));
            return None;
        };

        let Some(_def) = def else {
            self.errs.push(ASTReport::MissingXInYAfterZ {
                x: ARMissing::VarDef ,
                y: _cnode,
                z: CSTNode::Label{ name: _equal },
            });
            return None;
        };

        let Some(__def) = self.parse_expr(*_def) else {
            return None;
        };

        self.block.push(ASTNode::VarDecl(Box::new(ASTNode::BinOp {
            lhs: Box::new(ASTNode::Label(_name.span)),
            op: ASTOpType::Equal,
            rhs: Box::new(__def),
        })));

        None
    }

    fn parse_func_decl(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTBlock<'linespan>> {
        let _cnode = cnode.clone();
        let CSTNode::FuncDecl {
            indent_sz,
            key,
            name,
            paren,
        } = cnode
        else {
            self.errs.push(ASTReport::InternalError(102));
            return None;
        };

        if self.indent_lv < indent_sz {
            return None;
        } else if self.indent_lv > indent_sz {
            return ASTBlock {
                indent_lv: indent_sz,
                block: Vec::new(),
                errs: Vec::new(),
            }
            .parse_func_decl(_cnode);
        }

        let Some(_name) = name else {
            self.errs.push(ASTReport::MissingXInYAfterZ {
                x: ARMissing::FuncName,
                y: _cnode,
                z: CSTNode::Label{ name: key },
            });
            return None;
        };

        let Some(_paren) = paren else {
            self.errs.push(ASTReport::MissingXInYAfterZ {
                x: ARMissing::FuncParen,
                y: _cnode,
                z: CSTNode::Label{ name: _name },
            });
            return None;
        };

        let Some(__paren) = self.parse_expr(*_paren) else { return None };

        self.block.push(ASTNode::FuncDecl(Box::new(ASTNode::Func {
            name: _name.span,
            paren: Box::new(__paren),
        })));

        None
    }

    fn parse_var_mutator(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTBlock<'linespan>> {
        let _cnode = cnode.clone();
        let CSTNode::VarMutator {
            indent_sz,
            name,
            operator,
            amount,
            ..
        } = cnode
        else {
            self.errs.push(ASTReport::InternalError(103));
            return None;
        };

        if self.indent_lv < indent_sz {
            return None;
        } else if self.indent_lv > indent_sz {
            return ASTBlock {
                indent_lv: indent_sz,
                block: Vec::new(),
                errs: Vec::new(),
            }
            .parse_var_mutator(_cnode);
        }

        let Some(_name) = name else {
            self.errs.push(ASTReport::MissingXInY {
                x: ARMissing::VarName,
                y: _cnode,
            });
            return None;
        };

        let Some(_operator) = operator else {
            self.errs.push(ASTReport::MissingXInY {
                x: ARMissing::Operator,
                y: _cnode,
            });
            return None;
        };

        let ast_op = ASTOpType::default().translate_c_op(_operator);

        let Some(_amount) = amount else {
            self.block.push(ASTNode::UnOp {
                var: Box::new(ASTNode::Label(_name.span)),
                op: ast_op,
            });
            return None;
        };

        let Some(__amount) = self.parse_expr(*_amount) else {
            return None;
        };

        self.block.push(ASTNode::BinOp {
            lhs: Box::new(ASTNode::Label(_name.span)),
            op: ast_op,
            rhs: Box::new(__amount),
        });

        None
    }

    fn parse_var_def(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTBlock<'linespan>> {
        let _cnode = cnode.clone();
        let CSTNode::VarDef {
            indent_sz,
            name,
            equal,
            def,
        } = cnode
        else {
            return None;
        };

        if self.indent_lv < indent_sz {
            return None;
        } else if self.indent_lv > indent_sz {
            return ASTBlock {
                indent_lv: indent_sz,
                block: Vec::new(),
                errs: Vec::new(),
            }
            .parse_var_def(_cnode);
        }

        let Some(_name) = self.parse_expr(*(name.clone())) else {
            return None;
        };

        let Some(_equal) = equal else {
            self.errs.push(ASTReport::MissingXInYAfterZ {
                x: ARMissing::Operator,
                y: _cnode.clone(),
                z: *(name.clone()),
            });
            return None;
        };

        let Some(_def) = def else {
            self.errs.push(ASTReport::MissingXInYAfterZ {
                x: ARMissing::VarDef,
                y: _cnode,
                z: CSTNode::Label{ name: _equal },
            });
            return None;
        };

        let Some(__def) = self.parse_expr(*_def) else {
            return None;
        };

        self.block.push(ASTNode::BinOp {
            lhs: Box::new(_name),
            op: ASTOpType::Equal,
            rhs: Box::new(__def),
        });

        None
    }

    fn parse_expr(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTNode<'linespan>> {
        let _cnode = cnode.clone();
        match cnode {
            CSTNode::Label { name } => return Some(ASTNode::Label(name.span)),

            CSTNode::String { contents, .. } => {
                let Some(_contents) = contents else {
                    return Some(ASTNode::String(None));
                };
                return Some(ASTNode::String(Some(_contents.span)));
            }

            CSTNode::Number { num } => {
                let Ok(val) = num.span.str.parse::<f64>() else {
                    self.errs.push(ASTReport::InternalError(104));
                    return None;
                };
                return Some(ASTNode::Number {
                    val,
                    span: num.span,
                });
            }

            CSTNode::Paren { ref rparen, .. } => {
                if rparen.is_none() {
                    self.errs.push(ASTReport::MissingXInY {
                        x: ARMissing::RParen,
                        y: _cnode,
                    });
                }
                return self.parse_paren(cnode);
            }

            CSTNode::Brack { ref rbrack, .. } => {
                if rbrack.is_none() {
                    self.errs.push(ASTReport::MissingXInY {
                        x: ARMissing::LBracket,
                        y: _cnode,
                    });
                }
                return self.parse_bracket(cnode);
            }

            CSTNode::VarFetch { .. } => return self.parse_var_fetch(cnode),

            CSTNode::Item { var, comma } => {
                if comma.is_none() {
                    self.errs.push(ASTReport::MissingXInY {
                        x: ARMissing::Comma,
                        y: _cnode.clone(),
                    });
                }
                let Some(_var) = var else {
                    self.errs.push(ASTReport::MissingXInY {
                        x: ARMissing::Var,
                        y: _cnode
                    });
                    return None;
                };
                return self.parse_expr(*_var);
            }

            CSTNode::BinOp { .. } => return self.parse_bin_op(cnode),
            CSTNode::Ascii { .. } => return self.parse_ascii(cnode),
            CSTNode::TableAccess { .. } => return self.parse_table_access(cnode),
            CSTNode::VarMethod { .. } => return self.parse_var_method(cnode),
            CSTNode::FuncCall { .. } => return self.parse_func(cnode),
            CSTNode::Import { .. } => return self.parse_import(cnode),
            CSTNode::New { .. } => return self.parse_new(cnode),

            _ => {
                self.errs.push(ASTReport::InternalError(105));
                return None;
            }
        }
    }

    fn parse_paren(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTNode<'linespan>> {
        let CSTNode::Paren { contents, .. } = cnode else {
            self.errs.push(ASTReport::InternalError(106));
            return None;
        };

        let mut ast_contents: Vec<ASTNode<'linespan>> = Vec::new();

        for node in contents.iter() {
            let CSTNode::Item { var, comma } = node else {
                self.errs.push(ASTReport::InternalError(107));
                return None;
            };

            if comma.is_none() {
                self.errs.push(ASTReport::MissingXInY {
                    x: ARMissing::Comma,
                    y: node.clone(),
                });
            }

            let Some(ref _var) = *var else {
                self.errs.push(ASTReport::MissingXInY {
                    x: ARMissing::Var,
                    y: node.clone(),
                });
                continue;
            };

            let Some(__var) = self.parse_expr(*(_var.clone())) else {
                continue;
            };
            ast_contents.push(__var);
        }

        Some(ASTNode::Paren(ast_contents))
    }

    fn parse_bracket(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTNode<'linespan>> {
        let CSTNode::Brack { contents, .. } = cnode else {
            self.errs.push(ASTReport::InternalError(108));
            return None;
        };

        let mut ast_contents: Vec<ASTNode<'linespan>> = Vec::new();

        for node in contents.iter() {
            let CSTNode::Item { var, comma } = node else {
                self.errs.push(ASTReport::InternalError(109));
                return None;
            };

            if comma.is_none() {
                self.errs.push(ASTReport::MissingXInY {
                    x: ARMissing::Comma,
                    y: node.clone(),
                });
            }

            let Some(ref _var) = *var else {
                self.errs.push(ASTReport::MissingXInY {
                    x: ARMissing::Var,
                    y: node.clone(),
                });
                continue;
            };

            let Some(__var) = self.parse_expr(*(_var.clone())) else {
                continue;
            };
            ast_contents.push(__var);
        }

        Some(ASTNode::Brack(ast_contents))
    }

    fn parse_bin_op(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTNode<'linespan>> {
        let _cnode = cnode.clone();
        let CSTNode::BinOp { lhs, op, rhs } = cnode else {
            self.errs.push(ASTReport::InternalError(110));
            return None;
        };

        let Some(_lhs) = self.parse_expr(*lhs) else {
            return None;
        };

        let Some(ast_op) = ASTOpType::default().translate_ltok(op) else {
            self.errs.push(ASTReport::InternalError(111));
            return None;
        };

        let Some(_rhs) = rhs else {
            self.errs.push(ASTReport::MissingXInY {
                x: ARMissing::RHSExpr,
                y: _cnode,
            });
            return None;
        };

        let Some(__rhs) = self.parse_expr(*_rhs) else {
            return None;
        };

        Some(ASTNode::BinOp {
            lhs: Box::new(_lhs),
            op: ast_op,
            rhs: Box::new(__rhs),
        })
    }

    fn parse_var_fetch(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTNode<'linespan>> {
        let _cnode = cnode.clone();
        let CSTNode::VarFetch { mut at1, var, at2 } = cnode else {
            self.errs.push(ASTReport::InternalError(112));
            return None;
        };

        if at2.is_none() {
            self.errs.push(ASTReport::MissingXInY {
                x: ARMissing::At,
                y: _cnode.clone(),
            });
        }

        let Some(_var) = var else {
            self.errs.push(ASTReport::MissingXInY {
                x: ARMissing::Var,
                y: _cnode,
            });
            return None;
        };

        let Some(__var) = self.parse_expr(*_var) else {
            return None;
        };

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

        Some(ASTNode::VarFetch {
            var: Box::new(__var),
            fetch: at1.span,
        })
    }

    /* NOTE: For future implementation, please check the indent_sz */
    fn parse_var_method(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTNode<'linespan>> {
        let _cnode = cnode.clone();
        let CSTNode::VarMethod { var, method, .. } = cnode else {
            self.errs.push(ASTReport::InternalError(109));
            return None;
        };

        let Some(_var) = self.parse_expr(*var) else {
            return None;
        };
        let Some(_method) = method else {
            self.errs.push(ASTReport::MissingXInY {
                x: ARMissing::Func,
                y: _cnode,
            });
            return None;
        };

        let CSTNode::FuncCall {
            indent_sz: _,
            name,
            paren,
        } = *_method
        else {
            self.errs.push(ASTReport::InternalError(113));
            return None;
        };

        let Some(_paren) = paren else {
            self.errs.push(ASTReport::InternalError(114));
            return None;
        };
        let Some(__paren) = self.parse_expr(*_paren) else {
            return None;
        };

        Some(ASTNode::VarMethod {
            var: Box::new(_var),
            m_name: name.span,
            m_paren: Box::new(__paren),
        })
    }

    fn parse_table_access(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTNode<'linespan>> {
        let _cnode = cnode.clone();
        let CSTNode::TableAccess { label, brack } = cnode else {
            self.errs.push(ASTReport::InternalError(115));
            return None;
        };

        let Some(_brack) = brack else {
            self.errs.push(ASTReport::MissingXInY {
                x: ARMissing::Bracket,
                y: _cnode,
            });
            return None;
        };

        let Some(__brack) = self.parse_expr(*_brack) else {
            return None;
        };

        Some(ASTNode::TableAccess {
            var: label.span,
            index: Box::new(__brack),
        })
    }

    fn parse_ascii(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTNode<'linespan>> {
        let _cnode = cnode.clone();
        let CSTNode::Ascii { lines, key_end, .. } = cnode else {
            self.errs.push(ASTReport::InternalError(116));
            return None;
        };

        if key_end.is_none() {
            self.errs.push(ASTReport::MissingXInY {
                x: ARMissing::AsciiEnd,
                y: _cnode,
            });
        }

        Some(ASTNode::Ascii(lines))
    }

    /* NOTE: For future implementation, please check the indent_sz */
    fn parse_func(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTNode<'linespan>> {
        let _cnode = cnode.clone();
        let CSTNode::FuncCall { name, paren, .. } = cnode else {
            self.errs.push(ASTReport::InternalError(117));
            return None;
        };

        let Some(_paren) = paren else {
            self.errs.push(ASTReport::MissingXInY {
                x: ARMissing::Paren,
                y: _cnode,
            });
            return None;
        };

        let Some(__paren) = self.parse_expr(*_paren) else {
            return None;
        };

        Some(ASTNode::Func {
            name: name.span,
            paren: Box::new(__paren),
        })
    }

    /* NOTE: For future implementation, please check the indent_sz */
    fn parse_import(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTNode<'linespan>> {
        let _cnode = cnode.clone();
        let CSTNode::Import {
            key,
            path,
            ..
        } = cnode else {
            self.errs.push(ASTReport::InternalError(118));
            return None;
        };

        let Some(_path) = path else {
            self.errs.push(ASTReport::MissingXInYAfterZ {
                x: ARMissing::Path,
                y: _cnode.clone(),
                z: CSTNode::Label{ name: key },
            });
            return None;
        };

        let Some(__path) = self.parse_expr(*_path) else { return None };

        Some(ASTNode::Import(Box::new(__path)))
    }

    /* NOTE: For future implementation, please check the indent_sz */
    fn parse_new(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTNode<'linespan>> {
        let _cnode = cnode.clone();
        let CSTNode::New { path, .. } = cnode else {
            self.errs.push(ASTReport::InternalError(119));
            return None;
        };

        let Some(_path) = path else {
            self.errs.push(ASTReport::MissingXInY {
                x: ARMissing::Path,
                y: _cnode.clone(),
            });
            return None;
        };

        let Some(__path) = self.parse_expr(*_path) else {
            return None;
        };

        Some(ASTNode::New(Box::new(__path)))
    }
}
