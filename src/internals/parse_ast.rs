/*
 * Oh boy! I sure do hope this shit won't take as long as the CST!
 * - ESplash (09.17.26)
 */

use crate::internals::{
    StrSpan,
    CSTNode,
};

pub enum OpType {
    Plus, Minus, Times, Divide, Modulo,
    AddAssign, SubAssign, MultAssign, DivAssign,
    Increment, Decrement,
    Equal,
    Greater, Lesser, GreatEq, LessEq,
    And, Or, Not,
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
        op: OpType,
        rhs: Box<ASTNode<'linespan>>,
    },

    UnOp{
        var: Box<ASTNode<'linespan>>,
        op: OpType,
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
}

pub struct ASTBlock<'linespan>{
    output: Vec<ASTNode<'linespan>>,
    errs: Vec<ASTReport<'linespan>>,
}

pub enum ASTReport<'linespan> {
    InternalError(u8),
    MissingXInY{
        x: String,
        y: CSTNode<'linespan>,
    }
}

/* TODO LIST 2000: Since I am not a perfect robot.
 * 1. Do Bracket Parsing
 * 2. Variable parsing
 */
impl<'linespan> ASTBlock<'linespan> {
    pub fn new() -> ASTBlock<'linespan> {
        ASTBlock{
            output: Vec::new(),
            errs: Vec::new(),
    	}
    }

    fn parse_expr(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTNode<'linespan>> {
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
                    self.errs.push(ASTReport::InternalError(100));
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
                        y: cnode.clone(),
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
                        y: cnode.clone(),
                    });
                }
                return self.parse_brack(cnode);
            },

            _ => {
                self.errs.push(ASTReport::InternalError(101));
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
            self.errs.push(ASTReport::InternalError(102));
            return None;
        };

        let mut ast_contents: Vec<ASTNode<'linespan>> = Vec::new();

        for node in contents.iter() {
            let CSTNode::Item{
                var,
                comma,
            } = node else {
                self.errs.push(ASTReport::InternalError(103));
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

        return Some(ASTNode::Paren(ast_contents));
    }

    fn parse_bracket(&mut self, cnode: CSTNode<'linespan>) -> Option<ASTNode<'linespan>> {
        let CSTNode::Brack{
            lbrack: _,
            contents,
            rbrack: _,
        } = cnode else {
            self.errs.push(ASTReport::InternalError(104));
            return None;
        };

        let mut ast_contents: Vec<ASTNode<'linespan>> = Vec::new();

        for node in contents.iter() {
            let CSTNode::Item{
                var,
                comma,
            } = node else {
                self.errs.push(ASTReport::InternalError(105));
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

        return Some(ASTNode::Brack(ast_contents));
    }
}
