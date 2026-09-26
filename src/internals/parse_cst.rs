/*
 * NOTE: standardcombo, why the FUCK did you make this language as complex as it is?
 *       Please, I am curious as to why the fuck you ever made me come through this hellhole.
 *		 I ask of you, I sure as hell ask of you, to please translate this Parser code to C#,
 *		 and never speak of the old one. I BEG, dear standardcombo, your unrelenting implementations.
 *		 Why oh why have you made me carry such burden? You fucking bitch.
 * - ESplash (09.09.2026)
 */

use std::{
    fmt::{self, Display},
    iter::Peekable,
};

use crate::internals::{LexToken, LexerOutput, LineSpan, StrSpan, Token};

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
    Left,
    Right,
    Automatic,
}

#[derive(Clone, Debug)]
pub enum CSTNode<'linespan> {
    String {
        lquot: LexToken<'linespan>,
        contents: Option<LexToken<'linespan>>,
        rquot: Option<LexToken<'linespan>>,
    },
    Number {
        num: LexToken<'linespan>,
    },
    Label {
        name: LexToken<'linespan>,
    },

    Import {
        indent_sz: usize,
        key: LexToken<'linespan>,
        path: Option<Box<CSTNode<'linespan>>>,
    },
    New {
        indent_sz: usize,
        key: LexToken<'linespan>,
        path: Option<Box<CSTNode<'linespan>>>,
    },

    Item {
        var: Option<Box<CSTNode<'linespan>>>,
        comma: Option<LexToken<'linespan>>,
    },
    Paren {
        lparen: LexToken<'linespan>,
        contents: Vec<CSTNode<'linespan>>,
        rparen: Option<LexToken<'linespan>>,
    },
    Brack {
        lbrack: LexToken<'linespan>,
        contents: Vec<CSTNode<'linespan>>,
        rbrack: Option<LexToken<'linespan>>,
    },

    BinOp {
        lhs: Box<CSTNode<'linespan>>,
        op: LexToken<'linespan>,
        rhs: Option<Box<CSTNode<'linespan>>>,
    },

    VarDecl {
        indent_sz: usize,
        key: LexToken<'linespan>,
        name: Option<LexToken<'linespan>>,
        equal: Option<LexToken<'linespan>>,
        def: Option<Box<CSTNode<'linespan>>>,
    },
    VarDef {
        indent_sz: usize,
        name: Box<CSTNode<'linespan>>,
        equal: Option<LexToken<'linespan>>,
        def: Option<Box<CSTNode<'linespan>>>,
    },
    VarElement {
        indent_sz: usize,
        var: Box<CSTNode<'linespan>>,
        dot: LexToken<'linespan>,
        element: Option<Box<CSTNode<'linespan>>>,
    },
    VarMethod {
        indent_sz: usize,
        var: Box<CSTNode<'linespan>>,
        dot: LexToken<'linespan>,
        method: Option<Box<CSTNode<'linespan>>>,
    },

    VarFetch {
        at1: LexToken<'linespan>,
        var: Option<Box<CSTNode<'linespan>>>,
        at2: Option<LexToken<'linespan>>,
    },

    VarMutator {
        indent_sz: usize,
        name: Option<LexToken<'linespan>>,
        operator: Option<CSTOpType>,
        op_tok: Option<LexToken<'linespan>>,
        amount: Option<Box<CSTNode<'linespan>>>,
    },

    TableAccess {
        label: LexToken<'linespan>,
        brack: Option<Box<CSTNode<'linespan>>>,
    },

    FuncDecl {
        indent_sz: usize,
        key: LexToken<'linespan>,
        name: Option<LexToken<'linespan>>,
        paren: Option<Box<CSTNode<'linespan>>>,
    },
    FuncCall {
        indent_sz: usize,
        name: LexToken<'linespan>,
        paren: Option<Box<CSTNode<'linespan>>>,
    },

    If {
        indent_sz: usize,
        key: LexToken<'linespan>,
        bin_op: Option<Box<CSTNode<'linespan>>>,
    },
    Else {
        indent_sz: usize,
        colon: LexToken<'linespan>,
        if_stmnt: Option<Box<CSTNode<'linespan>>>,
    },

    Ascii {
        key_start: LexToken<'linespan>,
        lines: Vec<LineSpan>,
        key_end: Option<LexToken<'linespan>>,
    },

    Equip {
        indent_sz: usize,
        key: LexToken<'linespan>,
        equip_type: EquipType,
        tool_name: Vec<CSTNode<'linespan>>,
        minus: Vec<CSTNode<'linespan>>,
        star: Vec<CSTNode<'linespan>>,
        plus: Vec<CSTNode<'linespan>>,
    },

    Enable {
        indent_sz: usize,
        key: LexToken<'linespan>,
        opt: Option<Box<CSTNode<'linespan>>>,
        opt_opts: Option<Box<CSTNode<'linespan>>>,
    },
    Disable {
        indent_sz: usize,
        key: LexToken<'linespan>,
        opt: Option<Box<CSTNode<'linespan>>>,
        opt_opts: Option<Box<CSTNode<'linespan>>>,
    },

    Brew {
        indent_sz: usize,
        key: LexToken<'linespan>,
        ingr1: Option<Box<CSTNode<'linespan>>>,
        plus: Option<LexToken<'linespan>>,
        ingr2: Option<Box<CSTNode<'linespan>>>,
    },

    Loadout {
        indent_sz: usize,
        key: LexToken<'linespan>,
        num: Option<Box<CSTNode<'linespan>>>,
    },

    Activate {
        indent_sz: usize,
        key: LexToken<'linespan>,
        opt: Option<Box<CSTNode<'linespan>>>,
    },

    Play {
        indent_sz: usize,
        key: LexToken<'linespan>,
        sound: Option<Box<CSTNode<'linespan>>>,
        pitch: Option<Box<CSTNode<'linespan>>>,
    },

    Print {
        indent_sz: usize,
        key: LexToken<'linespan>,
        spcl_key: Option<LexToken<'linespan>>,
        spcl_x_pos: Option<Box<CSTNode<'linespan>>>,
        spcl_y_pos: Option<Box<CSTNode<'linespan>>>,
        spcl_clr_key: Option<LexToken<'linespan>>,
        spcl_clr_val: Option<LexToken<'linespan>>,
        contents: Vec<CSTNode<'linespan>>,
    },

    For {
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
pub enum CSTReport<'linespan> {
    InternalError(u16),
    ExpectedXGotY { x: Token, y: LexToken<'linespan> },
    EndOfFileDuring(CSTNode<'linespan>),
    ImplicitPrintCastInXBecauseY{ x: CSTNode<'linespan>, y: Box<CSTReport<'linespan>>, },
}

pub struct CSTOutput<'linespan> {
    pub output: Vec<CSTNode<'linespan>>,
    pub errs: Vec<CSTReport<'linespan>>,
}

fn fetch_help_code<'linespan>(err: &CSTReport<'linespan>) -> (String, String) {
    let help_message;
    let help_code;
    match err {
        CSTReport::ExpectedXGotY{x, y} => match x {
            Token::Newline => {
                help_message = "[HINT] You need to place a newline here.".to_string();
                let num_size = " ".repeat(y.span.line.num.to_string().len());
                let span_str = &y.clone().span.line.str;
                let modified_code: String = span_str.chars().take(y.span.clmn).collect();
                help_code = format!(
                    "{num_size} |\n{} - {}\n{} + {modified_code}\n{num_size} |",
                    y.span.line.num,
                    y.span.line.str,
                    y.span.line.num,
                );
            },

            Token::Space => {
                help_message = "[HINT] You may need to add a space here.".to_string();
                let num_size = " ".repeat(y.span.line.num.to_string().len());
                let lhs: String = y.span.str.chars().take(y.span.clmn).collect();
                let rhs: String = y.span.str.chars()
                    .skip(y.span.clmn).take(y.span.line.str.len() - y.span.clmn).collect();
                let modified_code = format!("{lhs} {rhs}");
                help_code = format!(
                    "{num_size} |\n{} - {}\n{} + {modified_code}\n{num_size} |",
                    y.span.line.num,
                    y.span.line.str,
                    y.span.line.num,
                );
            },

            /* TODO(V2): Make this peek ahead if there is a identifier. */
            Token::Identifier => {
                help_message = "[HINT] You may need to add a name here".to_string();
                let num_size = " ".repeat(y.span.line.num.to_string().len());
                let lhs: String = y.span.str.chars().take(y.span.clmn).collect();
                let rhs: String = y.span.str.chars()
                    .skip(y.span.clmn).take(y.span.line.str.len() - y.span.clmn).collect();
                let modified_code = format!("{lhs}[NAME]{rhs} <- You need to replace [NAME] with a name");
                help_code = format!(
                    "{num_size} |\n{} - {}\n{} + {modified_code}\n{num_size} |",
                    y.span.line.num,
                    y.span.line.str,
                    y.span.line.num,
                );
            },

            Token::Plus => {
                help_message = "[HINT] You may need to add a Plus (+) here".to_string();
                let num_size = " ".repeat(y.span.line.num.to_string().len());
                let lhs: String = y.span.str.chars().take(y.span.clmn).collect();
                let rhs: String = y.span.str.chars()
                    .skip(y.span.clmn).take(y.span.line.str.len() - y.span.clmn).collect();
                let modified_code = format!("{lhs}+{rhs} <- This is an incrementor");
                help_code = format!(
                    "{num_size} |\n{} - {}\n{} + {modified_code}\n{num_size} |",
                    y.span.line.num,
                    y.span.line.str,
                    y.span.line.num,
                );
            },

            Token::Dash => {
                help_message = "[HINT] You may need to add a Plus (-) here".to_string();
                let num_size = " ".repeat(y.span.line.num.to_string().len());
                let lhs: String = y.span.str.chars().take(y.span.clmn).collect();
                let rhs: String = y.span.str.chars()
                    .skip(y.span.clmn).take(y.span.line.str.len() - y.span.clmn).collect();
                let modified_code = format!("{lhs}+{rhs} <- This is an decrementor");
                help_code = format!(
                    "{num_size} |\n{} - {}\n{} + {modified_code}\n{num_size} |",
                    y.span.line.num,
                    y.span.line.str,
                    y.span.line.num,
                );
            },

            Token::Equal => {
                help_message = "[HINT] You may insert Equals (=), or assignment.".to_string();
                let num_size = " ".repeat(y.span.line.num.to_string().len());
                let lhs: String = y.span.str.chars().take(y.span.clmn).collect();
                let rhs: String = y.span.str.chars()
                    .skip(y.span.clmn).take(y.span.line.str.len() - y.span.clmn).collect();
                let modified_code = format!("{lhs}={rhs} <- This is an assigment");
                help_code = format!(
                    "{num_size} |\n{} - {}\n{} + {modified_code}\n{num_size} |",
                    y.span.line.num,
                    y.span.line.str,
                    y.span.line.num,
                );
            },

            Token::Comma => {
                help_message = "[HINT] You may insert a Comma (,)".to_string();
                let num_size = " ".repeat(y.span.line.num.to_string().len());
                let lhs: String = y.span.str.chars().take(y.span.clmn).collect();
                let rhs: String = y.span.str.chars()
                    .skip(y.span.clmn).take(y.span.line.str.len() - y.span.clmn).collect();
                let modified_code = format!("{lhs},{rhs}");
                help_code = format!(
                    "{num_size} |\n{} - {}\n{} + {modified_code}\n{num_size} |",
                    y.span.line.num,
                    y.span.line.str,
                    y.span.line.num,
                );
            },

            Token::Star => {
                help_message = "[HINT] You may need a Star (*) here".to_string();
                let num_size = " ".repeat(y.span.line.num.to_string().len());
                let lhs: String = y.span.str.chars().take(y.span.clmn).collect();
                let rhs: String = y.span.str.chars()
                    .skip(y.span.clmn).take(y.span.line.str.len() - y.span.clmn).collect();
                let modified_code = format!("{lhs}*{rhs}");
                help_code = format!(
                    "{num_size} |\n{} - {}\n{} + {modified_code}\n{num_size} |",
                    y.span.line.num,
                    y.span.line.str,
                    y.span.line.num,
                );
            },

            Token::Number => {
                help_message = "[HINT] You need to insert a Number here".to_string();
                let num_size = " ".repeat(y.span.line.num.to_string().len());
                let lhs: String = y.span.str.chars().take(y.span.clmn).collect();
                let rhs: String = y.span.str.chars()
                    .skip(y.span.clmn).take(y.span.line.str.len() - y.span.clmn).collect();
                let modified_code = format!("{lhs}1234{rhs} <- You can replace 1234 with a number.");
                help_code = format!(
                    "{num_size} |\n{} - {}\n{} + {modified_code}\n{num_size} |",
                    y.span.line.num,
                    y.span.line.str,
                    y.span.line.num,
                );
            },

            Token::Slash => {
                help_message = "[HINT] You need to put a Slash (/) here".to_string();
                let num_size = " ".repeat(y.span.line.num.to_string().len());
                let lhs: String = y.span.str.chars().take(y.span.clmn).collect();
                let rhs: String = y.span.str.chars()
                    .skip(y.span.clmn).take(y.span.line.str.len() - y.span.clmn).collect();
                let modified_code = format!("{lhs}/{rhs}");
                help_code = format!(
                    "{num_size} |\n{} - {}\n{} + {modified_code}\n{num_size} |",
                    y.span.line.num,
                    y.span.line.str,
                    y.span.line.num,
                );
            },

        	_ => (help_message, help_code) = ("Internal Error".to_string(), "Internal Error".to_string()),
        },

        /* TODO(V2): Add the "complete node" function */
        CSTReport::EndOfFileDuring(_) => {
            help_message = "[HINT] You need to finish this statement first.".to_string();
            help_code = String::new();
        },

        _ => (help_message, help_code) = ("Internal Error".to_string(), "Internal Error".to_string()),
    }

    (help_message, help_code)
}

impl<'linespan> Display for CSTReport<'linespan> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err: String;
        let origin: String;
        let line: String;
        let help_message: String;
        let help_code: String;
        match self {
            CSTReport::InternalError(id) => {
                err = "[ERROR] Internal Error, this is not your fault.".to_string();
                origin = String::new();
                line = ";)".to_string();
                help_message = format!("[HINT] You may report this to the devs ( ERRCODE: {id} )");
                help_code = String::new();
            }

            CSTReport::ExpectedXGotY { x, y } => {
                err = format!("[ERROR] Expected {x} got {}", y.token);
                origin = format!(" -> {}:{}:{}", y.span.line.file, y.span.line.num, y.span.clmn);
                let num_size = " ".repeat(y.span.line.num.to_string().len());
                line = format!("{num_size} |\n{}\n{num_size} |", y.span);
                (help_message, help_code) = fetch_help_code(&self);
            }

            CSTReport::EndOfFileDuring(x) => {
                err = "[ERROR] End of file during x.".to_string();
                let spans = x.fetch_all_strspan();
                if let Some(x_span) = spans.first() {
                    origin = format!(" -> {}:{}:{}", x_span.line.file, x_span.line.num, x_span.clmn);
                    let l = x_span.line;
                    let num_size = " ".repeat(l.num.to_string().len());
                    let str_size = "^".repeat(l.str.len());
                	line = format!("{num_size} |\n{l}\n{num_size} | {str_size}\n{num_size} |");
                } else {
                    origin = " -> Internal Error".to_string();
                    line = String::new();
                }
                (help_message, help_code) = fetch_help_code(&self);
            },

            CSTReport::ImplicitPrintCastInXBecauseY{x, y} => {
                err = "[WARNING] This print statement became a normal print.".to_string();
                let spans = x.fetch_all_strspan();
                if let Some(x_span) = spans.first() {
                    origin = format!(" -> {}:{}:{}", x_span.line.file, x_span.line.num, x_span.clmn);
                } else { origin = " -> Internal Error".to_string() }
                line = match x.fetch_linespan() {
                    Some(l) => format!(
                        "{} |\n{l}\n{} |\n{} | {}\n{} |",
                        " ".repeat(l.num.to_string().len()),
                        " ".repeat(l.num.to_string().len()),
                        " ".repeat(l.num.to_string().len()),
                        "~".repeat(l.str.len()),
                        " ".repeat(l.num.to_string().len()),
                    ),
                    None => "If you're seeing this, I messed up.".to_string(),
                };
                help_message = format!(
                    "[HINT] It happened because of:\n{}",
                    *(y.clone())
                );
                help_code = String::new();
            },
        }
        write!(f, "{err}\n{origin}\n{line}\n{help_message}\n{help_code}")
    }
}

fn push_if_some_ltok<'linespan>(
    output: &mut Vec<StrSpan<'linespan>>,
    opt: &Option<LexToken<'linespan>>,
) {
    if let Some(_opt) = opt {
        output.push(_opt.span.clone())
    }
}

impl<'linespan> CSTNode<'linespan> {
    pub fn fetch_all_strspan(&self) -> Vec<StrSpan<'linespan>> {
        let mut output: Vec<StrSpan> = Vec::new();
        /* NOTE: Since of the burden of the <'linesapn> lifetime, I am not able to make the cnode extractor a function. */
        match self {
            CSTNode::String {
                lquot,
                contents,
                rquot,
            } => {
                output.push(lquot.span.clone());
                push_if_some_ltok(&mut output, contents);
                push_if_some_ltok(&mut output, rquot);
            }

            CSTNode::Import { key, path, .. } => {
                output.push(key.span.clone());
                if let Some(_path) = path {
                    output.extend(_path.fetch_all_strspan())
                }
            }

            CSTNode::New { key, path, .. } => {
                output.push(key.span.clone());
                if let Some(_path) = path {
                    output.extend(_path.fetch_all_strspan())
                }
            }

            CSTNode::Item { var, comma } => {
                if let Some(_var) = var {
                    output.extend(_var.fetch_all_strspan())
                }
                push_if_some_ltok(&mut output, comma);
            }

            CSTNode::Paren {
                lparen,
                contents,
                rparen,
            } => {
                output.push(lparen.span.clone());
                for node in contents.iter() {
                    output.extend(node.fetch_all_strspan());
                }
                push_if_some_ltok(&mut output, rparen);
            }

            CSTNode::Brack {
                lbrack,
                contents,
                rbrack,
            } => {
                output.push(lbrack.span.clone());
                for node in contents.iter() {
                    output.extend(node.fetch_all_strspan());
                }
                push_if_some_ltok(&mut output, rbrack);
            }

            CSTNode::BinOp { lhs, op, rhs } => {
                output.extend(lhs.fetch_all_strspan());
                output.push(op.span.clone());
                if let Some(_rhs) = rhs {
                    output.extend(_rhs.fetch_all_strspan())
                }
            }

            CSTNode::VarDecl {
                key,
                name,
                equal,
                def,
                ..
            } => {
                output.push(key.span.clone());
                push_if_some_ltok(&mut output, name);
                push_if_some_ltok(&mut output, equal);
                if let Some(_def) = def {
                    output.extend(_def.fetch_all_strspan())
                }
            }

            CSTNode::VarDef {
                name, equal, def, ..
            } => {
                output.extend(name.fetch_all_strspan());
                push_if_some_ltok(&mut output, equal);
                if let Some(_def) = def {
                    output.extend(_def.fetch_all_strspan())
                }
            }

            CSTNode::VarElement {
                var, dot, element, ..
            } => {
                output.extend(var.fetch_all_strspan());
                output.push(dot.span.clone());
                if let Some(_element) = element {
                    output.extend(_element.fetch_all_strspan())
                }
            }

            CSTNode::VarMethod {
                var, dot, method, ..
            } => {
                output.extend(var.fetch_all_strspan());
                output.push(dot.span.clone());
                if let Some(_method) = method {
                    output.extend(_method.fetch_all_strspan())
                }
            }

            CSTNode::VarFetch { at1, var, at2 } => {
                output.push(at1.span.clone());
                if let Some(_var) = var {
                    output.extend(_var.fetch_all_strspan())
                }
                push_if_some_ltok(&mut output, at2);
            }

            CSTNode::VarMutator {
                name,
                op_tok,
                amount,
                ..
            } => {
                push_if_some_ltok(&mut output, name);
                push_if_some_ltok(&mut output, op_tok);
                if let Some(_amount) = amount {
                    output.extend(_amount.fetch_all_strspan())
                }
            }

            CSTNode::TableAccess { label, brack } => {
                output.push(label.span.clone());
                if let Some(_brack) = brack {
                    output.extend(_brack.fetch_all_strspan())
                }
            }

            CSTNode::FuncDecl {
                key, name, paren, ..
            } => {
                output.push(key.span.clone());
                push_if_some_ltok(&mut output, name);
                if let Some(_paren) = paren {
                    output.extend(_paren.fetch_all_strspan())
                }
            }

            CSTNode::FuncCall { name, paren, .. } => {
                output.push(name.span.clone());
                if let Some(_paren) = paren {
                    output.extend(_paren.fetch_all_strspan())
                }
            }

            CSTNode::If { key, bin_op, .. } => {
                output.push(key.span.clone());
                if let Some(_bin_op) = bin_op {
                    output.extend(_bin_op.fetch_all_strspan())
                }
            }

            CSTNode::Else {
                colon, if_stmnt, ..
            } => {
                output.push(colon.span.clone());
                if let Some(_if_stmnt) = if_stmnt {
                    output.extend(_if_stmnt.fetch_all_strspan())
                }
            }

            /* NOTE: Since the Ascii node is mainly composed of lines, we can simply ignore it. */
            CSTNode::Ascii {
                key_start, key_end, ..
            } => {
                output.push(key_start.span.clone());
                push_if_some_ltok(&mut output, key_end);
            }

            /*
             * NOTE: Since the equip parser is unstable, and doesn't keep the order,
             *       this will make the strspan order them the way the program sees it.
             */
            CSTNode::Equip {
                key,
                tool_name,
                plus,
                star,
                minus,
                ..
            } => {
                output.push(key.span.clone());

                for node in tool_name.iter() {
                    output.extend(node.fetch_all_strspan())
                }

                for node in plus.iter() {
                    output.extend(node.fetch_all_strspan())
                }

                for node in star.iter() {
                    output.extend(node.fetch_all_strspan())
                }

                for node in minus.iter() {
                    output.extend(node.fetch_all_strspan())
                }
            }

            CSTNode::Enable {
                key, opt, opt_opts, ..
            } => {
                output.push(key.span.clone());
                if let Some(_opt) = opt {
                    output.extend(_opt.fetch_all_strspan())
                }
                if let Some(_opt_opts) = opt_opts {
                    output.extend(_opt_opts.fetch_all_strspan())
                }
            }

            CSTNode::Disable {
                key, opt, opt_opts, ..
            } => {
                output.push(key.span.clone());
                if let Some(_opt) = opt {
                    output.extend(_opt.fetch_all_strspan())
                }
                if let Some(_opt_opts) = opt_opts {
                    output.extend(_opt_opts.fetch_all_strspan())
                }
            }

            CSTNode::Brew {
                key,
                ingr1,
                plus,
                ingr2,
                ..
            } => {
                output.push(key.span.clone());
                if let Some(_ingr1) = ingr1 {
                    output.extend(_ingr1.fetch_all_strspan())
                }
                push_if_some_ltok(&mut output, plus);
                if let Some(_ingr2) = ingr2 {
                    output.extend(_ingr2.fetch_all_strspan())
                }
            }

            CSTNode::Loadout { key, num, .. } => {
                output.push(key.span.clone());
                if let Some(_num) = num {
                    output.extend(_num.fetch_all_strspan())
                }
            }

            CSTNode::Activate { key, opt, .. } => {
                output.push(key.span.clone());
                if let Some(_opt) = opt {
                    output.extend(_opt.fetch_all_strspan())
                }
            }

            CSTNode::Play {
                key, sound, pitch, ..
            } => {
                output.push(key.span.clone());
                if let Some(_sound) = sound {
                    output.extend(_sound.fetch_all_strspan())
                }
                if let Some(_pitch) = pitch {
                    output.extend(_pitch.fetch_all_strspan())
                }
            }

            CSTNode::Print {
                key,
                spcl_key,
                spcl_x_pos,
                spcl_y_pos,
                spcl_clr_key,
                spcl_clr_val,
                contents,
                ..
            } => {
                output.push(key.span.clone());

                push_if_some_ltok(&mut output, spcl_key);
                if let Some(_spcl_x_pos) = spcl_x_pos {
                    output.extend(_spcl_x_pos.fetch_all_strspan())
                }
                if let Some(_spcl_y_pos) = spcl_y_pos {
                    output.extend(_spcl_y_pos.fetch_all_strspan())
                }

                push_if_some_ltok(&mut output, spcl_clr_key);
                push_if_some_ltok(&mut output, spcl_clr_val);

                for node in contents.iter() {
                    output.extend(node.fetch_all_strspan())
                }
            }

            CSTNode::For {
                key,
                var,
                equal,
                start,
                dot1,
                dot2,
                end,
                ..
            } => {
                output.push(key.span.clone());

                push_if_some_ltok(&mut output, var);
                push_if_some_ltok(&mut output, equal);

                if let Some(_start) = start {
                    output.extend(_start.fetch_all_strspan())
                }

                push_if_some_ltok(&mut output, dot1);
                push_if_some_ltok(&mut output, dot2);

                if let Some(_end) = end {
                    output.extend(_end.fetch_all_strspan())
                }
            }

            CSTNode::Number { num } => output.push(num.span.clone()),
            CSTNode::Label { name } => output.push(name.span.clone()),
        }

        output
    }

    pub fn fetch_merge_strspan(&self) -> Option<StrSpan<'linespan>> {
        let spans = self.fetch_all_strspan();
        let Some(_span) = spans.first() else { return None };
        let mut span = _span.clone();

        for s_append in spans.iter().skip(1) {
            span.str += s_append.str.as_str();
        }

        Some(span.clone())
    }

    pub fn fetch_linespan(&self) -> Option<LineSpan> {
        let spans = self.fetch_all_strspan();
        let Some(last) = spans.last() else { return None };

        Some(last.line.clone())
    }
}

/* TODO LIST:
 * 1. Replace all ExpectedXGotY to pass over to the AST
 *    Or find a way to implement it here.
 */
impl<'linespan> CSTOutput<'linespan> {
    pub fn new() -> CSTOutput<'linespan> {
        CSTOutput {
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
                Token::Plus | Token::Dash | Token::Identifier => {
                    if is_expect_newline {
                        self.errs.push(CSTReport::ExpectedXGotY {
                            x: Token::Newline,
                            y: (*peek).clone(),
                        });
                        self.consume_remaining(&mut ls_iter);
                        continue;
                    }
                    let is_err = self.parse_identifier(&mut ls_iter);
                    if !is_err { return false }
                }

                Token::Greater => {
                    if is_expect_newline {
                        self.errs.push(CSTReport::ExpectedXGotY {
                            x: Token::Newline,
                            y: (*peek).clone(),
                        });
                        self.consume_remaining(&mut ls_iter);
                        continue;
                    }
                    is_expect_newline = true;
                    let _peek = (*peek).clone();
                    ls_iter.next();
                    if !self.parse_print(&mut ls_iter, _peek, 0) {
                        return false;
                    }
                }

                Token::Colon | Token::QuestMark => {
                    if is_expect_newline {
                        self.errs.push(CSTReport::ExpectedXGotY {
                            x: Token::Newline,
                            y: (*peek).clone(),
                        });
                        self.consume_remaining(&mut ls_iter);
                        continue;
                    }
                    is_expect_newline = true;
                    let _peek = (*peek).clone();
                    ls_iter.next();
                    if !self.parse_if_stmnt(&mut ls_iter, _peek, 0) {
                        return false;
                    }
                }

                Token::MultiComment | Token::Space => {
                    if !is_expect_newline {
                        let is_err = self.parse_identifier(&mut ls_iter);
                        if !is_err { return false }
                    } else {
                        ls_iter.next();
                        continue;
                    }
                },

                Token::Comment | Token::Newline => {
                    ls_iter.next();
                    is_expect_newline = false;
                    continue;
                }

                _ => {
                    self.errs.push(CSTReport::ExpectedXGotY {
                        x: if is_expect_newline {
                            Token::Newline
                        } else {
                            Token::Identifier
                        },
                        y: (*peek).clone(),
                    });
                    ls_iter.next();
                }
            }
            self.consume_space(&mut ls_iter);
        }

        true
    }

    fn parse_identifier<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self,
        ls_iter: &mut Peekable<I>,
    ) -> bool {
        let indent_sz = self.consume_indent_sz(ls_iter);

        let Some(opt_key) = ls_iter.peek() else {
            self.errs.push(CSTReport::InternalError(1));
            return false;
        };
        let key = (*opt_key).clone();

        if key.span.str == "var" || key.span.str == "const" {
            ls_iter.next();
            return self.parse_var_decl(ls_iter, key, indent_sz);
        } else if key.span.str == "func" {
            ls_iter.next();
            return self.parse_func_decl(ls_iter, key.clone(), indent_sz);
        }

        if self.check_consume_trailing(ls_iter) {
            return true;
        }

        let Some(name) = ls_iter.next() else {
            self.errs.push(CSTReport::InternalError(2));
            return false;
        };

        if name.token == Token::Identifier {
            match name.span.str.as_str() {
                "enable" => {
                    let key = name.clone();
                    let mut opt = None::<Box<CSTNode<'linespan>>>;
                    let mut opt_opts = None::<Box<CSTNode<'linespan>>>;

                    match ls_iter.next() {
                        Some(space) => match space.token {
                            Token::Space => {}
                            _ => {
                                self.output.push(CSTNode::Enable {
                                    indent_sz,
                                    key,
                                    opt,
                                    opt_opts,
                                });
                                self.errs.push(CSTReport::ExpectedXGotY {
                                    x: Token::Space,
                                    y: space.clone(),
                                });
                                return true;
                            }
                        },
                        None => {
                            self.output.push(CSTNode::Enable {
                                indent_sz,
                                key: key.clone(),
                                opt: opt.clone(),
                                opt_opts: opt_opts.clone(),
                            });
                            self.errs
                                .push(CSTReport::EndOfFileDuring(CSTNode::Enable {
                                    indent_sz,
                                    key,
                                    opt,
                                    opt_opts,
                                }));
                            return true;
                        }
                    }

                    let Some(ident) = self.parse_expr(ls_iter, 0, false) else {
                        self.output.push(CSTNode::Enable {
                            indent_sz,
                            key,
                            opt,
                            opt_opts,
                        });
                        return false;
                    };
                    opt = Some(Box::new(ident));

                    if self.consume_space(ls_iter) {
                        let Some(opt) = self.parse_expr(ls_iter, 0, false) else {
                            self.output.push(CSTNode::Enable {
                                indent_sz,
                                key,
                                opt,
                                opt_opts,
                            });
                            return false;
                        };
                        opt_opts = Some(Box::new(opt));
                    }

                    self.output.push(CSTNode::Enable {
                        indent_sz,
                        key,
                        opt,
                        opt_opts,
                    });
                    return true;
                }

                "disable" => {
                    let key = name.clone();
                    let mut opt = None::<Box<CSTNode<'linespan>>>;
                    let mut opt_opts = None::<Box<CSTNode<'linespan>>>;

                    self.expect_consume_token_or(
                        Token::Space,
                        ls_iter,
                        CSTReport::EndOfFileDuring(CSTNode::Disable {
                            indent_sz,
                            key: key.clone(),
                            opt: opt.clone(),
                            opt_opts: opt_opts.clone(),
                        }),
                    );

                    let Some(ident) = self.parse_expr(ls_iter, 0, false) else {
                        self.output.push(CSTNode::Disable {
                            indent_sz,
                            key,
                            opt,
                            opt_opts,
                        });
                        return false;
                    };

                    opt = Some(Box::new(ident));

                    if self.consume_space(ls_iter) {
                        let Some(_opt) = self.parse_expr(ls_iter, 0, false) else {
                            self.output.push(CSTNode::Disable {
                                indent_sz,
                                key,
                                opt,
                                opt_opts,
                            });
                            return false;
                        };
                        opt_opts = Some(Box::new(_opt));
                    }

                    self.output.push(CSTNode::Disable {
                        indent_sz,
                        key,
                        opt,
                        opt_opts,
                    });
                    return true;
                }

                "brew" => {
                    let key = name.clone();
                    let mut ingr1 = None::<Box<CSTNode<'linespan>>>;
                    let mut plus = None::<LexToken<'linespan>>;
                    let mut ingr2 = None::<Box<CSTNode<'linespan>>>;

                    self.expect_consume_token_or(
                        Token::Space,
                        ls_iter,
                        CSTReport::EndOfFileDuring(CSTNode::Brew {
                            indent_sz,
                            key: key.clone(),
                            ingr1: ingr1.clone(),
                            plus: plus.clone(),
                            ingr2: ingr2.clone(),
                        }),
                    );

                    let Some(_ingr1) = self.parse_expr(ls_iter, 0, false) else {
                        self.output.push(CSTNode::Brew {
                            indent_sz,
                            key: key.clone(),
                            ingr1: ingr1.clone(),
                            plus: plus.clone(),
                            ingr2: ingr2.clone(),
                        });
                        return true;
                    };
                    ingr1 = Some(Box::new(_ingr1));

                    self.consume_space(ls_iter);
                    let Some(_plus) = self.expect_consume_token_or(
                        Token::Plus,
                        ls_iter,
                        CSTReport::EndOfFileDuring(CSTNode::Brew {
                            indent_sz,
                            key: key.clone(),
                            ingr1: ingr1.clone(),
                            plus: plus.clone(),
                            ingr2: ingr2.clone(),
                        }),
                    ) else {
                        self.output.push(CSTNode::Brew {
                            indent_sz,
                            key,
                            ingr1,
                            plus,
                            ingr2,
                        });
                        return true;
                    };
                    plus = Some(_plus);

                    self.consume_space(ls_iter);
                    let Some(_ingr2) = self.parse_expr(ls_iter, 0, false) else {
                        self.output.push(CSTNode::Brew {
                            indent_sz,
                            key,
                            ingr1,
                            plus,
                            ingr2,
                        });
                        return true;
                    };
                    ingr2 = Some(Box::new(_ingr2));

                    self.output.push(CSTNode::Brew {
                        indent_sz,
                        key,
                        ingr1,
                        plus,
                        ingr2,
                    });
                    return true;
                }

                "loadout" => {
                    let key = name.clone();
                    let mut num = None::<Box<CSTNode<'linespan>>>;

                    self.expect_consume_token_or(
                        Token::Space,
                        ls_iter,
                        CSTReport::EndOfFileDuring(CSTNode::Loadout {
                            indent_sz,
                            key: key.clone(),
                            num: num.clone(),
                        }),
                    );

                    let Some(_num) = self.parse_expr(ls_iter, 0, false) else {
                        self.output.push(CSTNode::Loadout {
                            indent_sz,
                            key,
                            num,
                        });
                        return true;
                    };
                    num = Some(Box::new(_num));

                    self.output.push(CSTNode::Loadout {
                        indent_sz,
                        key,
                        num,
                    });
                    return true;
                }

                "activate" => {
                    let key = name.clone();
                    let mut opt = None::<Box<CSTNode<'linespan>>>;

                    self.expect_consume_token_or(
                        Token::Space,
                        ls_iter,
                        CSTReport::EndOfFileDuring(CSTNode::Activate {
                            indent_sz,
                            key: key.clone(),
                            opt: opt.clone(),
                        }),
                    );

                    let Some(_opt) = self.parse_expr(ls_iter, 0, false) else {
                        self.output.push(CSTNode::Activate {
                            indent_sz,
                            key,
                            opt,
                        });
                        return true;
                    };
                    opt = Some(Box::new(_opt));

                    self.output.push(CSTNode::Activate {
                        indent_sz,
                        key,
                        opt,
                    });
                    return true;
                }

                "play" => {
                    let key = name.clone();
                    let mut sound = None::<Box<CSTNode<'linespan>>>;
                    let mut pitch = None::<Box<CSTNode<'linespan>>>;

                    self.expect_consume_token_or(
                        Token::Space,
                        ls_iter,
                        CSTReport::EndOfFileDuring(CSTNode::Play {
                            indent_sz,
                            key: key.clone(),
                            sound: sound.clone(),
                            pitch: pitch.clone(),
                        }),
                    );

                    let Some(_sound) = self.parse_expr(ls_iter, 0, false) else {
                        self.output.push(CSTNode::Play {
                            indent_sz,
                            key,
                            sound,
                            pitch,
                        });
                        return true;
                    };
                    sound = Some(Box::new(_sound));

                    self.expect_consume_token_or(
                        Token::Space,
                        ls_iter,
                        CSTReport::EndOfFileDuring(CSTNode::Play {
                            indent_sz,
                            key: key.clone(),
                            sound: sound.clone(),
                            pitch: pitch.clone(),
                        }),
                    );

                    let Some(_pitch) = self.parse_expr(ls_iter, 0, false) else {
                        return true;
                    };
                    pitch = Some(Box::new(_pitch));

                    self.output.push(CSTNode::Play {
                        indent_sz,
                        key,
                        sound,
                        pitch,
                    });
                    return true;
                }

                "equipL" | "equipR" | "equip" => {
                    return self.parse_equip(ls_iter, name.clone(), indent_sz);
                }

                "for" => {
                    let key = name.clone();
                    let mut var = None::<LexToken<'linespan>>;
                    let mut equal = None::<LexToken<'linespan>>;
                    let mut start = None::<Box<CSTNode<'linespan>>>;
                    let mut dot1 = None::<LexToken<'linespan>>;
                    let mut dot2 = None::<LexToken<'linespan>>;
                    let mut end = None::<Box<CSTNode<'linespan>>>;

                    self.expect_consume_token_or(
                        Token::Space,
                        ls_iter,
                        CSTReport::EndOfFileDuring(CSTNode::For {
                            indent_sz,
                            key: key.clone(),
                            var: var.clone(),
                            equal: equal.clone(),
                            start: start.clone(),
                            dot1: dot1.clone(),
                            dot2: dot2.clone(),
                            end: end.clone(),
                        }),
                    );

                    let Some(id) = self.expect_consume_token_or(
                        Token::Identifier,
                        ls_iter,
                        CSTReport::EndOfFileDuring(CSTNode::For {
                            indent_sz,
                            key: key.clone(),
                            var: var.clone(),
                            equal: equal.clone(),
                            start: start.clone(),
                            dot1: dot1.clone(),
                            dot2: dot2.clone(),
                            end: end.clone(),
                        }),
                    ) else {
                        self.output.push(CSTNode::For {
                            indent_sz,
                            key,
                            var,
                            equal,
                            start,
                            dot1,
                            dot2,
                            end,
                        });
                        return true;
                    };
                    var = Some(id);
                    self.consume_space(ls_iter);

                    let Some(_equal) = self.expect_consume_token_or(
                        Token::Equal,
                        ls_iter,
                        CSTReport::EndOfFileDuring(CSTNode::For {
                            indent_sz,
                            key: key.clone(),
                            var: var.clone(),
                            equal: equal.clone(),
                            start: start.clone(),
                            dot1: dot1.clone(),
                            dot2: dot2.clone(),
                            end: end.clone(),
                        }),
                    ) else {
                        self.output.push(CSTNode::For {
                            indent_sz,
                            key,
                            var,
                            equal,
                            start,
                            dot1,
                            dot2,
                            end,
                        });
                        return true;
                    };
                    equal = Some(_equal);
                    self.consume_space(ls_iter);

                    let Some(expr) = self.parse_special_for_expr(ls_iter) else {
                        self.output.push(CSTNode::For {
                            indent_sz,
                            key: key.clone(),
                            var: var.clone(),
                            equal: equal.clone(),
                            start: start.clone(),
                            dot1: dot1.clone(),
                            dot2: dot2.clone(),
                            end: end.clone(),
                        });
                        return true;
                    };
                    start = Some(Box::new(expr));
                    self.consume_space(ls_iter);

                    let Some(dot) = self.expect_consume_token_or(
                        Token::Dot,
                        ls_iter,
                        CSTReport::EndOfFileDuring(CSTNode::For {
                            indent_sz,
                            key: key.clone(),
                            var: var.clone(),
                            equal: equal.clone(),
                            start: start.clone(),
                            dot1: dot1.clone(),
                            dot2: dot2.clone(),
                            end: end.clone(),
                        }),
                    ) else {
                        self.output.push(CSTNode::For {
                            indent_sz,
                            key,
                            var,
                            equal,
                            start,
                            dot1,
                            dot2,
                            end,
                        });
                        return true;
                    };
                    dot1 = Some(dot);

                    let Some(dot) = self.expect_consume_token_or(
                        Token::Dot,
                        ls_iter,
                        CSTReport::EndOfFileDuring(CSTNode::For {
                            indent_sz,
                            key: key.clone(),
                            var: var.clone(),
                            equal: equal.clone(),
                            start: start.clone(),
                            dot1: dot1.clone(),
                            dot2: dot2.clone(),
                            end: end.clone(),
                        }),
                    ) else {
                        self.output.push(CSTNode::For {
                            indent_sz,
                            key,
                            var,
                            equal,
                            start,
                            dot1,
                            dot2,
                            end,
                        });
                        return true;
                    };
                    dot2 = Some(dot);
                    self.consume_space(ls_iter);

                    let Some(expr2) = self.parse_special_for_expr(ls_iter) else {
                        self.output.push(CSTNode::For {
                            indent_sz,
                            key: key.clone(),
                            var: var.clone(),
                            equal: equal.clone(),
                            start: start.clone(),
                            dot1: dot1.clone(),
                            dot2: dot2.clone(),
                            end: end.clone(),
                        });
                        return false;
                    };
                    end = Some(Box::new(expr2));

                    self.output.push(CSTNode::For {
                        indent_sz,
                        key,
                        var,
                        equal,
                        start,
                        dot1,
                        dot2,
                        end,
                    });
                    return true;
                }

                "import" => {
                    let key = name.clone();
                    let mut path = None::<Box<CSTNode<'linespan>>>;

                    self.consume_space(ls_iter);

                    let Some(file_path) = self.parse_file_path(ls_iter) else {
                        self.output.push(CSTNode::Import {
                            indent_sz,
                            key,
                            path,
                        });
                        return true;
                    };
                    path = Some(Box::new(file_path));

                    self.output.push(CSTNode::Import {
                        indent_sz,
                        key,
                        path,
                    });
                    return true;
                }

                "new" => {
                    let key = name.clone();
                    let mut path = None::<Box<CSTNode<'linespan>>>;
                    self.consume_space(ls_iter);

                    let Some(file_path) = self.parse_file_path(ls_iter) else {
                        self.output.push(CSTNode::New{
                            indent_sz,
                            key,
                            path,
                        });
                        return true;
                    };
                    path = Some(Box::new(file_path));

                    self.output.push(CSTNode::New{
                        indent_sz,
                        key,
                        path,
                    });
                    return true;
                }

                _ => {}
            }
        } else {
            match name.token {
                Token::Greater => return self.parse_print(ls_iter, name.clone(), indent_sz),
                Token::Colon | Token::QuestMark => return self.parse_if_stmnt(ls_iter, name.clone(), indent_sz),

                Token::Plus => match ls_iter.peek() {
                    Some(peek) if peek.token == Token::Plus => {
                        let mut op_tok = name.clone();
                        op_tok.concat((*peek).clone());

                        ls_iter.next();
                        let Some(ident) = self.expect_consume_token_or(
                            Token::Identifier,
                            ls_iter,
                            CSTReport::EndOfFileDuring(CSTNode::VarMutator {
                                indent_sz,
                                name: None,
                                operator: Some(CSTOpType::Increment),
                                op_tok: Some(op_tok.clone()),
                                amount: None,
                            }),
                        ) else {
                            return true;
                        };
                        self.output.push(CSTNode::VarMutator {
                            indent_sz,
                            name: Some(ident.clone()),
                            operator: Some(CSTOpType::Increment),
                            op_tok: Some(op_tok),
                            amount: None,
                        });
                        ls_iter.next();
                        return true;
                    }

                    _ => {
                        self.errs.push(CSTReport::ExpectedXGotY {
                            x: Token::Identifier,
                            y: name.clone(),
                        });
                        return true;
                    }
                },

                Token::Dash => match ls_iter.peek() {
                    Some(peek) if peek.token == Token::Dash => {
                        let mut op_tok = name.clone();
                        op_tok.concat((*peek).clone());
                        ls_iter.next();
                        let Some(ident) = self.expect_consume_token_or(
                            Token::Identifier,
                            ls_iter,
                            CSTReport::EndOfFileDuring(CSTNode::VarMutator {
                                indent_sz,
                                name: None,
                                operator: Some(CSTOpType::Decrement),
                                op_tok: Some(op_tok.clone()),
                                amount: None,
                            }),
                        ) else {
                            return true;
                        };
                        self.output.push(CSTNode::VarMutator {
                            indent_sz,
                            name: Some(ident.clone()),
                            operator: Some(CSTOpType::Decrement),
                            op_tok: Some(op_tok),
                            amount: None,
                        });
                        ls_iter.next();
                        return true;
                    }
                    _ => {
                        self.errs.push(CSTReport::ExpectedXGotY {
                            x: Token::Identifier,
                            y: name.clone(),
                        });
                        return true;
                    }
                },

                _ => self.errs.push(CSTReport::ExpectedXGotY {
                    x: Token::Identifier,
                    y: name.clone(),
                }),
            }
        }

        self.consume_space(ls_iter);
        let Some(ltok) = ls_iter.next() else {
            self.errs
                .push(CSTReport::EndOfFileDuring(CSTNode::VarDef {
                    indent_sz,
                    name: Box::new(CSTNode::Label { name: name.clone() }),
                    equal: None,
                    def: None,
                }));
            return false;
        };
        match ltok.token {
            Token::LParen => {
                return self.parse_func_call(ls_iter, name.clone(), ltok.clone(), indent_sz);
            }

            Token::Plus => {
                let Some(equal_or_plus) = ls_iter.peek() else {
                    let bin_op_node = CSTNode::BinOp {
                        lhs: Box::new(CSTNode::Label { name: name.clone() }),
                        op: ltok.clone(),
                        rhs: None,
                    };
                    self.output.push(bin_op_node.clone());
                    self.errs
                        .push(CSTReport::EndOfFileDuring(bin_op_node));
                    return true;
                };

                match equal_or_plus.token {
                    Token::Plus => {
                        let mut op_tok = ltok.clone();
                        op_tok.concat((*equal_or_plus).clone());
                        self.output.push(CSTNode::VarMutator {
                            indent_sz,
                            name: Some(name.clone()),
                            operator: Some(CSTOpType::Increment),
                            op_tok: Some(op_tok),
                            amount: None,
                        });
                        ls_iter.next();
                    }

                    Token::Equal => {
                        let mut op_tok = ltok.clone();
                        op_tok.concat((*equal_or_plus).clone());
                        self.consume_space(ls_iter);
                        let Some(expr) = self.parse_expr(ls_iter, 0, false) else {
                            return false;
                        };
                        self.output.push(CSTNode::VarMutator {
                            indent_sz,
                            name: Some(name.clone()),
                            operator: Some(CSTOpType::AddAssign),
                            op_tok: Some(op_tok),
                            amount: Some(Box::new(expr)),
                        });
                        ls_iter.next();
                    }

                    _ => {
                        self.errs.push(CSTReport::ExpectedXGotY {
                            x: Token::Plus,
                            y: (*equal_or_plus).clone(),
                        });
                        return true;
                    }
                }

                return true;
            }

            Token::Dash => {
                let Some(equal_or_dash) = ls_iter.peek() else {
                    let bin_op_node = CSTNode::BinOp {
                        lhs: Box::new(CSTNode::Label { name: name.clone() }),
                        op: ltok.clone(),
                        rhs: None,
                    };
                    self.output.push(bin_op_node.clone());
                    self.errs
                        .push(CSTReport::EndOfFileDuring(bin_op_node));
                    return true;
                };

                match equal_or_dash.token {
                    Token::Dash => {
                        let mut op_tok = ltok.clone();
                        op_tok.concat((*equal_or_dash).clone());
                        self.output.push(CSTNode::VarMutator {
                            indent_sz,
                            name: Some(name.clone()),
                            operator: Some(CSTOpType::Decrement),
                            op_tok: Some(op_tok),
                            amount: None,
                        });
                        ls_iter.next();
                    }

                    Token::Equal => {
                        let mut op_tok = ltok.clone();
                        op_tok.concat((*equal_or_dash).clone());
                        self.consume_space(ls_iter);
                        let Some(expr) = self.parse_expr(ls_iter, 0, false) else {
                            return false;
                        };

                        self.output.push(CSTNode::VarMutator {
                            indent_sz,
                            name: Some(name.clone()),
                            operator: Some(CSTOpType::SubAssign),
                            op_tok: Some(op_tok),
                            amount: Some(Box::new(expr)),
                        });
                    }

                    _ => {
                        self.errs.push(CSTReport::ExpectedXGotY {
                            x: Token::Dash,
                            y: (*equal_or_dash).clone(),
                        });
                        return true;
                    }
                }

                return true;
            }

            Token::Star => {
                let bin_op_node = CSTNode::BinOp {
                    lhs: Box::new(CSTNode::Label { name: name.clone() }),
                    op: ltok.clone(),
                    rhs: None,
                };
                let Some(equal) = self.expect_consume_token_or(
                    Token::Equal,
                    ls_iter,
                    CSTReport::EndOfFileDuring(bin_op_node.clone()),
                ) else {
                    self.output.push(bin_op_node);
                    return true;
                };
                let _ = bin_op_node;

                let mut op_tok = ltok.clone();
                op_tok.concat(equal);

                let mut varmut_node = CSTNode::VarMutator {
                    indent_sz,
                    name: Some(name.clone()),
                    operator: Some(CSTOpType::MultAssign),
                    op_tok: Some(op_tok),
                    amount: None,
                };

                let CSTNode::VarMutator { ref mut amount, .. } = varmut_node else {
                    self.errs.push(CSTReport::InternalError(3));
                    return false;
                };

                let Some(expr) = self.parse_expr(ls_iter, 0, false) else {
                    return false;
                };
                *amount = Some(Box::new(expr));

                self.output.push(varmut_node);

                return true;
            }

            Token::Slash => {
                let bin_op_node = CSTNode::BinOp {
                    lhs: Box::new(CSTNode::Label { name: name.clone() }),
                    op: ltok.clone(),
                    rhs: None,
                };

                let Some(equal) = self.expect_consume_token_or(
                    Token::Equal,
                    ls_iter,
                    CSTReport::EndOfFileDuring(bin_op_node.clone()),
                ) else {
                    self.output.push(bin_op_node);
                    return true;
                };
                let _ = bin_op_node;

                let mut op_tok = ltok.clone();
                op_tok.concat(equal);

                let mut varmut_node = CSTNode::VarMutator {
                    indent_sz,
                    name: Some(name.clone()),
                    operator: Some(CSTOpType::DivAssign),
                    op_tok: Some(op_tok),
                    amount: None,
                };

                let CSTNode::VarMutator { ref mut amount, .. } = varmut_node else {
                    self.errs.push(CSTReport::InternalError(4));
                    return false;
                };

                let Some(expr) = self.parse_expr(ls_iter, 0, false) else {
                    return true;
                };
                *amount = Some(Box::new(expr));

                self.output.push(varmut_node);

                return true;
            }

            Token::Dot => {
                let var = Box::new(CSTNode::Label { name: name.clone() });
                let dot = ltok.clone();
                let element = None::<Box<CSTNode<'linespan>>>;

                let Some(method) = self.parse_element(
                    ls_iter,
                    CSTNode::Label { name: name.clone() },
                    ltok.clone(),
                    0,
                ) else {
                    self.output.push(CSTNode::VarElement{
                        indent_sz,
                        var,
                        dot,
                        element,
                    });
                    return true;
                };

                /* TODO: Implement the binding method here. */
                match method {
                    CSTNode::VarElement { .. } => {
                        match ls_iter.peek() {
                            Some(ltok) => match ltok.token {
                                Token::Newline => {
                                    ls_iter.next();
                                    self.output.push(method.clone());
                                    return true;
                                }

                                Token::Space | Token::Equal => {}

                                _ => {
                                    self.errs.push(CSTReport::ExpectedXGotY {
                                        x: Token::Newline,
                                        y: (*ltok).clone(),
                                    });
                                    return true;
                                }
                            },

                            None => {
                                self.output.push(method);
                                return true;
                            }
                        }

                        let Some(ltok) = ls_iter.next() else {
                            self.errs.push(CSTReport::InternalError(5));
                            return true;
                        };

                        match ltok.token {
                            Token::Space => {
                                let Some(equal) = self.expect_consume_token_or(
                                    Token::Equal,
                                    ls_iter,
                                    CSTReport::ExpectedXGotY {
                                        x: Token::Newline,
                                        y: ltok.clone(),
                                    },
                                ) else {
                                    return true;
                                };
                                let Some(expr) = self.parse_expr(ls_iter, 0, false) else {
                                    self.output.push(CSTNode::VarDef {
                                        indent_sz,
                                        name: Box::new(method),
                                        equal: Some(equal),
                                        def: None,
                                    });
                                    return true;
                                };
                                self.output.push(CSTNode::VarDef {
                                    indent_sz,
                                    name: Box::new(method),
                                    equal: Some(equal),
                                    def: Some(Box::new(expr)),
                                });
                            }

                            Token::Equal => {
                                self.consume_space(ls_iter);
                                let Some(expr) = self.parse_expr(ls_iter, 0, false) else {
                                    self.output.push(CSTNode::VarDef {
                                        indent_sz,
                                        name: Box::new(method),
                                        equal: Some(ltok.clone()),
                                        def: None,
                                    });
                                    return true;
                                };
                                self.output.push(CSTNode::VarDef {
                                    indent_sz,
                                    name: Box::new(method),
                                    equal: Some(ltok.clone()),
                                    def: Some(Box::new(expr)),
                                });
                            }

                            _ => {
                                self.errs.push(CSTReport::ExpectedXGotY {
                                    x: Token::Newline,
                                    y: ltok.clone(),
                                });
                                return true;
                            }
                        }

                        return true;
                    }

                    CSTNode::VarMethod { .. } => {
                        if !self.expect_peek_token(Token::Newline, ls_iter) {
                            let Some(ltok) = ls_iter.next() else {
                                self.errs.push(CSTReport::InternalError(6));
                                return true;
                            };
                            self.errs.push(CSTReport::ExpectedXGotY {
                                x: Token::Newline,
                                y: ltok.clone(),
                            });
                            return true;
                        }
                        return true;
                    }

                    _ => {
                        self.errs.push(CSTReport::InternalError(7));
                        return true;
                    }
                }
            }

            Token::Newline => {
                let node = CSTNode::VarDef {
                    indent_sz,
                    name: Box::new(CSTNode::Label { name: name.clone() }),
                    equal: None,
                    def: None,
                };
                self.output.push(node.clone());
                self.errs.push(CSTReport::ExpectedXGotY {
                    x: Token::Equal,
                    y: ltok.clone(),
                });
                return true;
            }

            _ => {
                self.consume_space(ls_iter);
                let Some(equal) = self.expect_consume_token_or(
                    Token::Equal,
                    ls_iter,
                    CSTReport::ExpectedXGotY {
                        x: Token::Equal,
                        y: ltok.clone(),
                    },
                ) else {
                    return true;
                };

                self.consume_space(ls_iter);
                let Some(expr) = self.parse_expr(ls_iter, 0, false) else {
                    return true;
                };

                self.output.push(CSTNode::VarDef {
                    indent_sz,
                    name: Box::new(CSTNode::Label { name: name.clone() }),
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
                Token::Number => return Some(CSTNode::Number { num: ltok.clone() }),

                Token::At => {
                    let Some(expr) = self.parse_expr(ls_iter, 0, true) else {
                        return None;
                    };
                    self.consume_space(ls_iter);

                    let Some(at2) = self.expect_consume_token_or(
                        Token::At,
                        ls_iter,
                        CSTReport::EndOfFileDuring(CSTNode::VarFetch {
                            at1: ltok.clone(),
                            var: Some(Box::new(expr.clone())),
                            at2: None,
                        }),
                    ) else {
                        return None;
                    };

                    return Some(CSTNode::VarFetch {
                        at1: ltok.clone(),
                        var: Some(Box::new(expr)),
                        at2: Some(at2),
                    });
                }

                Token::Identifier => return Some(CSTNode::Label { name: ltok.clone() }),

                _ => {}
            },

            None => return None,
        }

        None
    }

    fn parse_print_param<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self,
        ls_iter: &mut Peekable<I>,

        /* We need them to be separated not in a CSTNode::Print beforehand, else it will break. */
        indent_sz: usize,
        key: LexToken<'linespan>,

        spcl_key: &mut Option<LexToken<'linespan>>,
        spcl_x_pos: &mut Option<Box<CSTNode<'linespan>>>,
        spcl_y_pos: &mut Option<Box<CSTNode<'linespan>>>,
        spcl_clr_key: &mut Option<LexToken<'linespan>>,
        spcl_clr_val: &mut Option<LexToken<'linespan>>,
        contents: &mut Vec<CSTNode<'linespan>>,
    ) -> bool {
        self.consume_space(ls_iter);

        let Some(_spcl_key) = spcl_key else {
            self.errs.push(CSTReport::InternalError(8));
            return false;
        };
        let _ = _spcl_key;

        let Some(x_pos) = self.parse_expr(ls_iter, 0, false) else { return true };
        *spcl_x_pos = Some(Box::new(x_pos.clone()));

        self.consume_space(ls_iter);

        match ls_iter.next() {
            Some(ltok) => if ltok.token != Token::Comma {
                self.errs.push(CSTReport::ImplicitPrintCastInXBecauseY{
                    x: CSTNode::Print{
                        indent_sz,
                        key,
                        spcl_key: spcl_key.clone(),
                        spcl_x_pos: spcl_x_pos.clone(),
                        spcl_y_pos: spcl_y_pos.clone(),
                        spcl_clr_key: spcl_clr_key.clone(),
                        spcl_clr_val: spcl_clr_val.clone(),
                        contents: contents.clone(),
                    },
                    y: Box::new(CSTReport::ExpectedXGotY{
                        x: Token::Comma,
                        y: ltok.clone(),
                    }),
                });
                return false;
            },
            None => {
                let node = CSTNode::Print{
                    indent_sz,
                    key,
                    spcl_key: spcl_key.clone(),
                    spcl_x_pos: spcl_x_pos.clone(),
                    spcl_y_pos: spcl_y_pos.clone(),
                    spcl_clr_key: spcl_clr_key.clone(),
                    spcl_clr_val: spcl_clr_val.clone(),
                    contents: contents.clone(),
                };
                self.errs.push(CSTReport::ImplicitPrintCastInXBecauseY{
                    x: node.clone(),
                    y: Box::new(CSTReport::EndOfFileDuring(node)),
                });
                return false;
            },
        };

        self.consume_space(ls_iter);

        let Some(y_pos) = self.parse_expr(ls_iter, 0, false) else { return false };
        *spcl_y_pos = Some(Box::new(y_pos.clone()));

        self.consume_space(ls_iter);

        match ls_iter.next() {
            Some(ltok) => if ltok.token != Token::Comma {
                self.errs.push(CSTReport::ImplicitPrintCastInXBecauseY{
                    x: CSTNode::Print{
                        indent_sz,
                        key,
                        spcl_key: spcl_key.clone(),
                        spcl_x_pos: spcl_x_pos.clone(),
                        spcl_y_pos: spcl_y_pos.clone(),
                        spcl_clr_key: spcl_clr_key.clone(),
                        spcl_clr_val: spcl_clr_val.clone(),
                        contents: contents.clone(),
                    },
                    y: Box::new(CSTReport::ExpectedXGotY{
                        x: Token::Comma,
                        y: ltok.clone(),
                    }),
                });
                return false;
            },
            None => {
                let node = CSTNode::Print{
                    indent_sz,
                    key,
                    spcl_key: spcl_key.clone(),
                    spcl_x_pos: spcl_x_pos.clone(),
                    spcl_y_pos: spcl_y_pos.clone(),
                    spcl_clr_key: spcl_clr_key.clone(),
                    spcl_clr_val: spcl_clr_val.clone(),
                    contents: contents.clone(),
                };
                self.errs.push(CSTReport::ImplicitPrintCastInXBecauseY{
                    x: node.clone(),
                    y: Box::new(CSTReport::EndOfFileDuring(node)),
                });
                return false;
            },
        };

        match ls_iter.peek() {
            Some(ltok) if ltok.token == Token::Hashtag
                => *spcl_clr_key = Some((*ltok).clone()),
            None | Some(_) => return true,
        }
        ls_iter.next();

        match ls_iter.next() {
            Some(ltok) if ltok.token == Token::Identifier && ltok.span.str.chars().count() == 6 => {
                let mut lchars = ltok.span.str.chars();
                let rain: String = lchars.clone().take(4).collect();
                contents.push(CSTNode::Label { name: ltok.clone() });
                if rain == "rain" {
                    if lchars.skip(4).take(2).all(|c| c.is_digit(16)) {
                        *spcl_clr_val = Some(ltok.clone());
                    } else { return false }
                } else {
                    if lchars.all(|c| c.is_digit(16)) {
                        *spcl_clr_val = Some(ltok.clone());
                    } else { return false }
                }
            }
            None | Some(_) => return false,
        }

        match ls_iter.next() {
            Some(ltok) if ltok.token == Token::Comma => return true,
            Some(_) | None => return false,
        }
    }

    fn parse_print<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self,
        ls_iter: &mut Peekable<I>,
        r_angle_brack: LexToken<'linespan>,
        indent_sz: usize,
    ) -> bool {
        let key = r_angle_brack;

        let mut spcl_key = None::<LexToken<'linespan>>;
        let mut spcl_x_pos = None::<Box<CSTNode<'linespan>>>;
        let mut spcl_y_pos = None::<Box<CSTNode<'linespan>>>;
        let mut spcl_clr_key = None::<LexToken<'linespan>>;
        let mut spcl_clr_val = None::<LexToken<'linespan>>;

        let mut contents = Vec::<CSTNode<'linespan>>::new();

        match ls_iter.peek() {
            Some(ltok) => match ltok.token {
                Token::Identifier => match ltok.span.str.as_str() {
                    "o" | "h" | "c" | "f" => {
                        spcl_key = Some((*ltok).clone());
                        ls_iter.next();
                    }
                    _ => {}
                },

                Token::Tilde => {
                    spcl_key = Some((*ltok).clone());
                    ls_iter.next();
                }

                Token::LParen => {
                    let spcl_key = Some((*ltok).clone());
                    let _ = ltok;
                    ls_iter.next();
                    let Some(face) = self.expect_consume_token_or(
                        Token::Identifier,
                        ls_iter,
                        CSTReport::EndOfFileDuring(CSTNode::Print{
                            indent_sz,
                            key: key.clone(),
                            spcl_key: spcl_key.clone(),
                            contents: contents.clone(),

                            spcl_x_pos: spcl_x_pos.clone(),
                            spcl_y_pos: spcl_y_pos.clone(),
                            spcl_clr_key: spcl_clr_key.clone(),
                            spcl_clr_val: spcl_clr_val.clone(),
                        }),
                    ) else {
                        self.output.push(CSTNode::Print{
                            indent_sz,
                            key,
                            spcl_key,
                            contents,

                            spcl_x_pos,
                            spcl_y_pos,
                            spcl_clr_key,
                            spcl_clr_val,
                        });
                        return true;
                    };
                    if face.span.str.chars().count() != 4 {
                        contents.push(CSTNode::Label { name: face.clone() });
                        ls_iter.next();
                    } else {
                        contents.push(CSTNode::Label { name: face.clone() });
                        self.output.push(CSTNode::Print{
                            indent_sz,
                            key,
                            spcl_key,
                            contents,

                            spcl_x_pos,
                            spcl_y_pos,
                            spcl_clr_key,
                            spcl_clr_val,
                        });
                        return true;
                    }
                }

                _ => {}
            },

            None => {
                self.output.push(CSTNode::Print{
                    indent_sz,
                    key,
                    contents,

                    spcl_key,
                    spcl_x_pos,
                    spcl_y_pos,
                    spcl_clr_key,
                    spcl_clr_val,
                });
                return true;
            }
        }

        if spcl_key.is_some() && !self.parse_print_param(
            ls_iter,

            indent_sz,
            key.clone(),

            &mut spcl_key,
            &mut spcl_x_pos,
            &mut spcl_y_pos,
            &mut spcl_clr_key,
            &mut spcl_clr_val,
            &mut contents,
        ) {
            self.output.push(CSTNode::Print{
                indent_sz,
                key,
                contents,

                spcl_key,
                spcl_x_pos,
                spcl_y_pos,
                spcl_clr_key,
                spcl_clr_val,
            });
            return false;
        }

        let mut is_ascii = true;

        loop {
            let Some(peek) = ls_iter.peek() else { break };
            match peek.token {
                Token::Newline => break,

                Token::Space | Token::Continue => {
                    ls_iter.next();
                    continue;
                }

                Token::Identifier if peek.span.str == "ascii" && is_ascii => {
                    let Some(ascii) = self.parse_expr(ls_iter, 0, false) else {
                        self.output.push(CSTNode::Print{
                            indent_sz,
                            key,
                            contents,

                            spcl_key,
                            spcl_x_pos,
                            spcl_y_pos,
                            spcl_clr_key,
                            spcl_clr_val,
                        });
                        return true;
                    };
                    ls_iter.next();
                    if let CSTNode::Ascii { .. } = ascii {
                        break;
                    } else {
                        self.output.push(CSTNode::Print{
                            indent_sz,
                            key,
                            contents,

                            spcl_key,
                            spcl_x_pos,
                            spcl_y_pos,
                            spcl_clr_key,
                            spcl_clr_val,
                        });
                        return true;
                    }
                }

                _ => {
                    is_ascii = false;
                    contents.push(CSTNode::Label {
                        name: (*peek).clone(),
                    });
                    ls_iter.next();
                }
            }
        }
        self.output.push(CSTNode::Print{
            indent_sz,
            key,
            contents,

            spcl_key,
            spcl_x_pos,
            spcl_y_pos,
            spcl_clr_key,
            spcl_clr_val,
        });

        true
    }

    /* NOTE: This implicitly supports else statements as well. */
    fn parse_if_stmnt<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self,
        ls_iter: &mut Peekable<I>,
        if_or_else: LexToken<'linespan>,
        indent_sz: usize,
    ) -> bool {
        let mut is_else = false;

        if if_or_else.token == Token::Colon {
            match ls_iter.peek() {
                Some(peek) => match peek.token {
                    Token::Newline => {
                        self.output.push(CSTNode::Else {
                            indent_sz,
                            colon: if_or_else,
                            if_stmnt: None,
                        });
                        return true;
                    }

                    Token::QuestMark => is_else = true,

                    _ => {
                        self.errs.push(CSTReport::ExpectedXGotY {
                            x: Token::Newline,
                            y: (*peek).clone(),
                        });
                        return true;
                    }
                },

                None => {
                    self.output.push(CSTNode::Else {
                        indent_sz,
                        colon: if_or_else,
                        if_stmnt: None,
                    });
                    return true;
                }
            }
        } else if if_or_else.token != Token::QuestMark {
            self.errs.push(CSTReport::InternalError(9));
            return true;
        }

        let mut if_stmnt = CSTNode::If {
            indent_sz: if is_else { 0 } else { indent_sz },
            key: if !is_else {
                if_or_else.clone()
            } else {
                let Some(question_mark) = ls_iter.next() else {
                    self.errs.push(CSTReport::InternalError(10));
                    return true;
                };
                question_mark.clone()
            },
            bin_op: None,
        };
        let CSTNode::If { ref mut bin_op, .. } = if_stmnt else {
            self.errs.push(CSTReport::InternalError(11));
            return true;
        };

        self.consume_space(ls_iter);

        let Some(lhs) = self.parse_expr(ls_iter, 0, false) else {
            self.output.push(if is_else {
                CSTNode::Else {
                    indent_sz,
                    colon: if_or_else,
                    if_stmnt: Some(Box::new(if_stmnt)),
                }
            } else {
                if_stmnt
            });
            return true;
        };

        self.consume_space(ls_iter);
        match ls_iter.peek() {
            Some(op) => match op.token {
                Token::Equal | Token::LessEq | Token::GreatEq | Token::Lesser | Token::Greater => {
                    let _op = (*op).clone();
                    ls_iter.next();
                    let Some(_bin_op) = self.parse_bin_op(ls_iter, lhs, _op) else {
                        return true;
                    };
                    *bin_op = Some(Box::new(_bin_op));
                    self.output.push(if is_else {
                        CSTNode::Else {
                            indent_sz,
                            colon: if_or_else,
                            if_stmnt: Some(Box::new(if_stmnt)),
                        }
                    } else {
                        if_stmnt
                    });
                    return true;
                }

                Token::Newline => {
                    self.output.push(if is_else {
                        CSTNode::Else {
                            indent_sz,
                            colon: if_or_else,
                            if_stmnt: Some(Box::new(if_stmnt)),
                        }
                    } else {
                        if_stmnt
                    });
                    return true;
                }

                _ => {
                    self.errs.push(CSTReport::ExpectedXGotY {
                        x: Token::Newline,
                        y: (*op).clone(),
                    });
                    self.output.push(if is_else {
                        CSTNode::Else {
                            indent_sz,
                            colon: if_or_else,
                            if_stmnt: Some(Box::new(if_stmnt)),
                        }
                    } else {
                        if_stmnt
                    });
                    return true;
                }
            },

            None => {
                self.errs.push(CSTReport::EndOfFileDuring(if is_else {
                    CSTNode::Else {
                        indent_sz,
                        colon: if_or_else.clone(),
                        if_stmnt: Some(Box::new(if_stmnt.clone())),
                    }
                } else {
                    if_stmnt.clone()
                }));
                self.output.push(if is_else {
                    CSTNode::Else {
                        indent_sz,
                        colon: if_or_else,
                        if_stmnt: Some(Box::new(if_stmnt)),
                    }
                } else {
                    if_stmnt
                });
                return false;
            }
        }
    }

    fn parse_equip<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self,
        ls_iter: &mut Peekable<I>,
        key: LexToken<'linespan>,
        indent_sz: usize,
    ) -> bool {
        let mut equip_node = CSTNode::Equip {
            indent_sz,
            key: key.clone(),
            equip_type: match key.span.str.as_str() {
                "equipL" => EquipType::Left,
                "equipR" => EquipType::Right,
                "equip" => EquipType::Automatic,
                _ => {
                    self.errs.push(CSTReport::InternalError(12));
                    EquipType::Automatic
                }
            },
            tool_name: vec![],
            plus: vec![],
            star: vec![],
            minus: vec![],
        };

        let CSTNode::Equip {
            ref mut tool_name,
            ref mut plus,
            ref mut star,
            ref mut minus,
            ..
        } = equip_node
        else {
            self.errs.push(CSTReport::InternalError(13));
            return true;
        };

        let mut is_expect_space = true;

        loop {
            let Some(peek) = ls_iter.peek() else { break };

            match peek.token {
                Token::Space => {
                    if !is_expect_space {
                        self.errs.push(CSTReport::InternalError(14));
                    }
                    is_expect_space = false;
                    ls_iter.next();
                }

                Token::Identifier => {
                    let Some(expr) = self.parse_expr(ls_iter, 0, false) else {
                        continue;
                    };
                    tool_name.push(expr);
                    ls_iter.next();
                }

                Token::At => {
                    let Some(expr) = self.parse_expr(ls_iter, 0, false) else {
                        continue;
                    };
                    tool_name.push(expr);
                    ls_iter.next();
                }

                Token::Plus => {
                    let Some(expr) = self.parse_special_equip_num(ls_iter) else {
                        continue;
                    };
                    plus.push(expr);
                }

                Token::Star => {
                    let Some(expr) = self.parse_special_equip_num(ls_iter) else {
                        continue;
                    };
                    star.push(expr);
                }

                Token::Dash => {
                    let Some(expr) = self.parse_special_equip_num(ls_iter) else {
                        continue;
                    };
                    minus.push(expr);
                }

                Token::Comment | Token::Newline => {
                    ls_iter.next();
                    break;
                }

                Token::MultiComment => continue,

                _ => {
                    let expected_token: Token = if is_expect_space {
                        Token::Space
                    } else if tool_name.is_empty() {
                        Token::Identifier
                    } else if star.is_empty() {
                        Token::Star
                    } else if plus.is_empty() {
                        Token::Plus
                    } else if minus.is_empty() {
                        Token::Dash
                    } else {
                        Token::Newline
                    };
                    self.errs.push(CSTReport::ExpectedXGotY {
                        x: expected_token,
                        y: (*peek).clone(),
                    });
                    ls_iter.next();
                }
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
                    return Some(CSTNode::Label { name: _ltok });
                }

                Token::At => return self.parse_expr(ls_iter, 0, false),

                Token::Number => {
                    let _ltok = (*ltok).clone();
                    ls_iter.next();
                    return Some(CSTNode::Number { num: _ltok });
                }

                _ => {
                    let _ltok = (*ltok).clone();
                    ls_iter.next();
                    self.errs.push(CSTReport::ExpectedXGotY {
                        x: Token::Number,
                        y: _ltok,
                    });
                    return None;
                }
            },
            None => return None,
        }
    }

    fn parse_func_decl<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self,
        ls_iter: &mut Peekable<I>,
        key: LexToken<'linespan>,
        indent_sz: usize,
    ) -> bool {
        let mut name = None::<LexToken<'linespan>>;
        let mut paren = None::<Box<CSTNode<'linespan>>>;

        self.expect_consume_token_or(
            Token::Space,
            ls_iter,
            CSTReport::EndOfFileDuring(CSTNode::FuncDecl{
                indent_sz,
                key: key.clone(),
                name: name.clone(),
                paren: paren.clone(),
            }),
        );

        let Some(_name) = self.expect_consume_token_or(
            Token::Identifier,
            ls_iter,
            CSTReport::EndOfFileDuring(CSTNode::FuncDecl{
                indent_sz,
                key: key.clone(),
                name: name.clone(),
                paren: paren.clone(),
            }),
        ) else {
            self.output.push(CSTNode::FuncDecl{
                indent_sz, key, name, paren,
            });
            return true;
        };
        self.consume_space(ls_iter);
        name = Some(_name);

        let Some(lparen) = self.expect_consume_token_or(
            Token::LParen,
            ls_iter,
            CSTReport::EndOfFileDuring(CSTNode::FuncDecl{
                indent_sz,
                key: key.clone(),
                name: name.clone(),
                paren: paren.clone(),
            }),
        ) else {
            return true;
        };

        match self.parse_paren(ls_iter, lparen.clone()) {
            Some(_paren) => {
                paren = Some(Box::new(_paren));
                self.output.push(CSTNode::FuncDecl{
                    indent_sz, key, name, paren,
                });
            }

            None => {
                self.output.push(CSTNode::FuncDecl{
                    indent_sz, key, name, paren,
                });
                return true;
            }
        }

        true
    }

    fn parse_var_decl<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self,
        ls_iter: &mut Peekable<I>,
        var: LexToken<'linespan>,
        indent_sz: usize,
    ) -> bool {
        if self
            .expect_consume_token_or(
                Token::Space,
                ls_iter,
                CSTReport::EndOfFileDuring(CSTNode::VarDecl {
                    indent_sz,
                    key: var.clone(),
                    name: None,
                    equal: None,
                    def: None,
                }),
            )
            .is_none()
        {
            return true;
        }

        let Some(name) =
            self.expect_consume_token_or(Token::Identifier, ls_iter, CSTReport::InternalError(15))
        else {
            return true;
        };

        self.consume_space(ls_iter);

        let Some(equal) = self.expect_consume_token_or(
            Token::Equal,
            ls_iter,
            CSTReport::EndOfFileDuring(CSTNode::VarDecl {
                indent_sz,
                key: var.clone(),
                name: Some(name.clone()),
                equal: None,
                def: None,
            }),
        ) else {
            return true;
        };

        self.consume_space(ls_iter);

        let Some(expr) = self.parse_expr(ls_iter, 0, false) else {
            return true;
        };

        self.output.push(CSTNode::VarDecl {
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
        indent_sz: usize,
    ) -> bool {
        match self.parse_paren(ls_iter, lparen) {
            Some(paren) => self.output.push(CSTNode::FuncCall {
                indent_sz,
                name,
                paren: Some(Box::new(paren)),
            }),
            None => return true,
        }

        true
    }

    fn parse_paren<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self,
        ls_iter: &mut Peekable<I>,
        lparen: LexToken<'linespan>,
    ) -> Option<CSTNode<'linespan>> {
        let mut paren = CSTNode::Paren {
            lparen,
            contents: Vec::new(),
            rparen: None,
        };
        let CSTNode::Paren {
            ref lparen,
            ref mut contents,
            ref mut rparen,
        } = paren
        else {
            self.errs.push(CSTReport::InternalError(16));
            return None;
        };

        loop {
            if self.expect_peek_token(Token::RParen, ls_iter) {
                let Some(ltok) = ls_iter.next() else {
                    self.errs.push(CSTReport::InternalError(17));
                    return None;
                };
                *rparen = Some(ltok.clone());
                break;
            }

            let mut item = CSTNode::Item {
                comma: None,
                var: None,
            };
            let CSTNode::Item {
                ref mut comma,
                ref mut var,
            } = item
            else {
                self.errs.push(CSTReport::InternalError(17));
                return None;
            };

            self.consume_space(ls_iter);
            let Some(expr) = self.parse_expr(ls_iter, 0, false) else {
                return None;
            };

            if !self.expect_peek_token(Token::Comma, ls_iter) {
                let Some(ltok) = ls_iter.next() else {
                    self.errs
                        .push(CSTReport::EndOfFileDuring(paren.clone()));
                    return Some(paren.clone());
                };
                self.errs.push(CSTReport::ExpectedXGotY {
                    x: Token::Comma,
                    y: ltok.clone(),
                });
                *var = Some(Box::new(expr.clone()));
            }
            self.consume_space(ls_iter);

            if let Some(found_comma) = self.expect_consume_token_or(
                Token::Comma,
                ls_iter,
                CSTReport::EndOfFileDuring(CSTNode::Paren {
                    lparen: lparen.clone(),
                    contents: contents.clone(),
                    rparen: None,
                }),
            ) {
                *comma = Some(found_comma);
            };
            contents.push(item.clone());
        }

        Some(paren)
    }

    fn parse_bracket<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self,
        ls_iter: &mut Peekable<I>,
        lbrack: LexToken<'linespan>,
    ) -> Option<CSTNode<'linespan>> {
        let mut paren = CSTNode::Brack {
            lbrack,
            contents: Vec::new(),
            rbrack: None,
        };
        let CSTNode::Brack {
            ref lbrack,
            ref mut contents,
            ref mut rbrack,
        } = paren
        else {
            self.errs.push(CSTReport::InternalError(18));
            return None;
        };

        loop {
            if self.expect_peek_token(Token::RParen, ls_iter) {
                let Some(ltok) = ls_iter.next() else {
                    self.errs.push(CSTReport::InternalError(19));
                    return None;
                };
                *rbrack = Some(ltok.clone());
                break;
            }

            let mut item = CSTNode::Item {
                comma: None,
                var: None,
            };
            let CSTNode::Item {
                ref mut comma,
                ref mut var,
            } = item
            else {
                self.errs.push(CSTReport::InternalError(20));
                return None;
            };

            self.consume_space(ls_iter);
            let Some(expr) = self.parse_expr(ls_iter, 0, false) else {
                return None;
            };

            if !self.expect_peek_token(Token::Comma, ls_iter) {
                let Some(ltok) = ls_iter.next() else {
                    self.errs
                        .push(CSTReport::EndOfFileDuring(paren.clone()));
                    return Some(paren.clone());
                };
                self.errs.push(CSTReport::ExpectedXGotY {
                    x: Token::Comma,
                    y: ltok.clone(),
                });
                *var = Some(Box::new(expr.clone()));
            }
            self.consume_space(ls_iter);

            if let Some(found_comma) = self.expect_consume_token_or(
                Token::Comma,
                ls_iter,
                CSTReport::EndOfFileDuring(CSTNode::Brack {
                    lbrack: lbrack.clone(),
                    contents: contents.clone(),
                    rbrack: None,
                }),
            ) {
                *comma = Some(found_comma);
            };
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
                            let Some(file_path) = self.parse_file_path(ls_iter) else {
                                return None;
                            };

                            return Some(CSTNode::Import {
                                indent_sz,
                                key: ltok.clone(),
                                path: Some(Box::new(file_path)),
                            });
                        }

                        "new" => {
                            self.consume_space(ls_iter);
                            let Some(file_path) = self.parse_file_path(ls_iter) else {
                                return None;
                            };

                            return Some(CSTNode::New {
                                indent_sz,
                                key: ltok.clone(),
                                path: Some(Box::new(file_path)),
                            });
                        }

                        "ascii" if self.check_consume_trailing(ls_iter) => {
                            return self.parse_ascii(ls_iter, ltok.clone());
                        }

                        _ => {}
                    }

                    match ls_iter.peek() {
                        Some(peek) => match peek.token {
                            Token::Continue | Token::MultiComment | Token::Space => {
                                self.consume_space(ls_iter);
                            }

                            Token::At if is_varfetch => {
                                return Some(CSTNode::Label { name: ltok.clone() });
                            }

                            _ => return Some(CSTNode::Label { name: ltok.clone() }),
                        },
                        None => return Some(CSTNode::Label { name: ltok.clone() }),
                    }

                    match ls_iter.peek() {
                        Some(check) => match check.token {
                            Token::LBracket => {
                                let _check = (*check).clone();
                                ls_iter.next();
                                let Some(brack) = self.parse_bracket(ls_iter, _check) else {
                                    return None;
                                };
                                return Some(CSTNode::TableAccess {
                                    label: ltok.clone(),
                                    brack: Some(Box::new(brack)),
                                });
                            }

                            Token::Dot => {
                                let _check = (*check).clone();
                                ls_iter.next();
                                return self.parse_element(
                                    ls_iter,
                                    CSTNode::Label { name: ltok.clone() },
                                    _check,
                                    0,
                                );
                            }

                            Token::Plus
                            | Token::Dash
                            | Token::Star
                            | Token::Slash
                            | Token::ExclMark
                            | Token::Percent => {
                                let _check = (*check).clone();
                                ls_iter.next();
                                return self.parse_bin_op(
                                    ls_iter,
                                    CSTNode::Label { name: ltok.clone() },
                                    _check,
                                );
                            }

                            Token::At if is_varfetch => {
                                return Some(CSTNode::Label { name: ltok.clone() });
                            }

                            _ => {}
                        },

                        None => return Some(CSTNode::Label { name: ltok.clone() }),
                    }

                    let mut label = ltok.clone();

                    loop {
                        match ls_iter.peek() {
                            Some(peek) => match ltok.token {
                                Token::Identifier => {
                                    label.concat((*peek).clone());
                                }

                                Token::Continue | Token::Space => {
                                    label.span.str.push(' ');
                                }

                                _ => break,
                            },

                            None => break,
                        }
                        ls_iter.next();
                    }

                    return Some(CSTNode::Label {
                        name: label.clone(),
                    });
                }

                Token::At => {
                    if is_varfetch {
                        return None;
                    }
                    let at1 = ltok;

                    let Some(expr) = self.parse_expr(ls_iter, 0, true) else {
                        return None;
                    };

                    let Some(at2) = self.expect_consume_token_or(
                        Token::At,
                        ls_iter,
                        CSTReport::EndOfFileDuring(CSTNode::VarFetch {
                            at1: at1.clone(),
                            var: Some(Box::new(expr.clone())),
                            at2: None,
                        }),
                    ) else {
                        return None;
                    };

                    return Some(CSTNode::VarFetch {
                        at1: at1.clone(),
                        var: Some(Box::new(expr)),
                        at2: Some(at2),
                    });
                }

                Token::Number => {
                    let Some(check) = ls_iter.peek() else {
                        return Some(CSTNode::Number { num: ltok.clone() });
                    };
                    let _check = (*check).clone();
                    let _ = check;
                    match check.token {
                        Token::Space | Token::Continue => {
                            ls_iter.next();
                        }

                        Token::Plus
                        | Token::Dash
                        | Token::Star
                        | Token::Slash
                        | Token::ExclMark
                        | Token::Percent => {
                            let _check = (*check).clone();
                            ls_iter.next();
                            return self.parse_bin_op(
                                ls_iter,
                                CSTNode::Number { num: ltok.clone() },
                                _check,
                            );
                        }

                        Token::At if is_varfetch => return Some(CSTNode::Number { num: _check }),

                        Token::Newline => return Some(CSTNode::Number { num: _check }),

                        _ => return Some(CSTNode::Number { num: _check }),
                    }

                    self.consume_space(ls_iter);
                    match ls_iter.next() {
                        Some(op) => match op.token {
                            Token::Plus
                            | Token::Dash
                            | Token::Star
                            | Token::Slash
                            | Token::ExclMark
                            | Token::Percent => {
                                ls_iter.next();
                                return self.parse_bin_op(
                                    ls_iter,
                                    CSTNode::Number { num: ltok.clone() },
                                    _check,
                                );
                            }

                            _ => return Some(CSTNode::Number { num: ltok.clone() }),
                        },

                        None => return Some(CSTNode::Number { num: ltok.clone() }),
                    }
                }

                Token::Plus => {
                    self.consume_space(ls_iter);
                    let Some(mut expr) = self.parse_expr(ls_iter, 0, is_varfetch) else {
                        return None;
                    };
                    match expr {
                        CSTNode::Number { num } => {
                            let mut new_ltok = ltok.clone();
                            new_ltok.concat(num);
                            return Some(CSTNode::Number { num: new_ltok });
                        }

                        CSTNode::BinOp { ref mut lhs, .. } => {
                            let mut current_lhs: CSTNode<'_> = *(lhs.clone());
                            loop {
                                match current_lhs {
                                    CSTNode::BinOp { ref lhs, .. } => {
                                        current_lhs = *(lhs.clone());
                                        continue;
                                    }

                                    CSTNode::Number { ref mut num } => {
                                        *num = ltok.clone();
                                        break;
                                    }

                                    CSTNode::Label { ref mut name } => {
                                        let mut new_ltok = ltok.clone();
                                        new_ltok.concat(name.clone());
                                        *name = ltok.clone();
                                        break;
                                    }

                                    CSTNode::VarFetch { ref at1, .. } => {
                                        self.errs.push(CSTReport::ExpectedXGotY {
                                            x: Token::Number,
                                            y: at1.clone(),
                                        });
                                        return Some(current_lhs);
                                    }

                                    _ => {
                                        self.errs.push(CSTReport::InternalError(21));
                                        return None;
                                    }
                                }
                            }

                            return Some(current_lhs);
                        }

                        _ => return Some(CSTNode::Label { name: ltok.clone() }),
                    }
                }

                Token::Dash => {
                    self.consume_space(ls_iter);
                    let Some(mut expr) = self.parse_expr(ls_iter, 0, is_varfetch) else {
                        return None;
                    };
                    match expr {
                        CSTNode::Number { num } => {
                            let mut new_ltok = ltok.clone();
                            new_ltok.concat(num);
                            return Some(CSTNode::Number { num: new_ltok });
                        }

                        CSTNode::BinOp { ref mut lhs, .. } => {
                            let mut current_lhs: CSTNode<'_> = *(lhs.clone());
                            loop {
                                match current_lhs {
                                    CSTNode::BinOp { ref lhs, .. } => {
                                        current_lhs = *(lhs.clone());
                                        continue;
                                    }

                                    CSTNode::Number { ref mut num } => {
                                        *num = ltok.clone();
                                        break;
                                    }

                                    CSTNode::Label { ref mut name } => {
                                        let mut new_ltok = ltok.clone();
                                        new_ltok.concat(name.clone());
                                        *name = ltok.clone();
                                        break;
                                    }

                                    CSTNode::VarFetch { ref at1, .. } => {
                                        self.errs.push(CSTReport::ExpectedXGotY {
                                            x: Token::Number,
                                            y: at1.clone(),
                                        });
                                        return Some(current_lhs);
                                    }

                                    _ => {
                                        self.errs.push(CSTReport::InternalError(22));
                                        return None;
                                    }
                                }
                            }

                            return Some(current_lhs);
                        }

                        _ => return Some(CSTNode::Label { name: ltok.clone() }),
                    }
                }

                /* Also possible that we can put this in a separate function. */
                Token::Quote => {
                    let mut quot = CSTNode::String {
                        lquot: ltok.clone(),
                        contents: None,
                        rquot: None,
                    };
                    let CSTNode::String {
                        ref mut contents,
                        ref mut rquot,
                        ..
                    } = quot
                    else {
                        self.errs.push(CSTReport::InternalError(23));
                        return None;
                    };
                    loop {
                        let Some(next_tok) = ls_iter.peek() else {
                            return None;
                        };
                        if next_tok.token == Token::Continue {
                            continue;
                        }
                        if next_tok.token == Token::Quote {
                            *rquot = Some((*next_tok).clone());
                            ls_iter.next();
                            break;
                        } else if next_tok.token == Token::Newline {
                            ls_iter.next();
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

                    match ls_iter.peek() {
                        Some(peek) => match peek.token {
                            Token::Plus => {
                                let _peek = (*peek).clone();
                                ls_iter.next();
                                self.consume_space(ls_iter);
                                return self.parse_bin_op(ls_iter, quot, _peek);
                            }

                            Token::Space | Token::Continue | Token::MultiComment => { ls_iter.next(); },

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
                        }

                        _ => return Some(quot),
                    }
                }

                Token::LParen => return self.parse_paren(ls_iter, ltok.clone()),
                Token::LBracket => return self.parse_bracket(ls_iter, ltok.clone()),

                Token::Space | Token::Continue | Token::MultiComment => {
                    self.consume_space(ls_iter);
                    return self.parse_expr(ls_iter, indent_sz, is_varfetch);
                }

                Token::Comment | Token::Newline => return None,

                _ => return None,
            },

            None => {
                self.errs.push(CSTReport::InternalError(24));
                return None;
            }
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
            Token::Identifier,
            ls_iter,
            CSTReport::EndOfFileDuring(CSTNode::VarElement {
                indent_sz,
                var: Box::new(var.clone()),
                dot: dot.clone(),
                element: None,
            }),
        ) else {
            return None;
        };

        self.consume_space(ls_iter);

        let mut varmethod = CSTNode::VarElement {
            indent_sz,
            var: Box::new(var.clone()),
            dot: dot.clone(),
            element: Some(Box::new(CSTNode::Label { name: ltok.clone() })),
        };

        self.consume_space(ls_iter);

        if self.expect_peek_token(Token::LParen, ls_iter) {
            let Some(lparen) = ls_iter.next() else {
                self.errs.push(CSTReport::InternalError(25));
                return None;
            };
            let Some(paren) = self.parse_paren(ls_iter, lparen.clone()) else {
                return None;
            };
            varmethod = CSTNode::VarMethod {
                indent_sz,
                var: Box::new(var.clone()),
                dot: dot.clone(),
                method: Some(Box::new(CSTNode::FuncCall {
                    name: ltok.clone(),
                    paren: Some(Box::new(paren)),
                    indent_sz,
                })),
            };
        }

        match ls_iter.peek() {
            Some(peek) => match peek.token {
                Token::Newline => {
                    return Some(CSTNode::VarElement {
                        indent_sz,
                        var: Box::new(var),
                        dot,
                        element: Some(Box::new(CSTNode::Label { name: ltok.clone() })),
                    });
                }

                Token::Dot => {
                    let _peek = (*peek).clone();
                    ls_iter.next();
                    return self.parse_element(ls_iter, varmethod, _peek, indent_sz);
                }

                _ => return Some(varmethod),
            },

            None => {
                return Some(CSTNode::VarElement {
                    indent_sz,
                    var: Box::new(var),
                    dot,
                    element: Some(Box::new(CSTNode::Label { name: ltok.clone() })),
                });
            }
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
                path = Some(CSTNode::Label { name: ltok.clone() });
                is_expect_slash = true;
                continue;
            };

            let CSTNode::Label { name } = _path else {
                self.errs.push(CSTReport::InternalError(26));
                return None;
            };

            match ltok.token {
                Token::Identifier => {
                    if is_expect_slash {
                        self.errs.push(CSTReport::ExpectedXGotY {
                            x: Token::Slash,
                            y: ltok.clone(),
                        });
                    }
                    is_expect_slash = true;
                    if !name.concat(ltok.clone()) {
                        break;
                    }
                }

                Token::Slash => {
                    if !is_expect_slash {
                        self.errs.push(CSTReport::ExpectedXGotY {
                            x: Token::Identifier,
                            y: ltok.clone(),
                        });
                    }
                    is_expect_slash = false;
                    name.concat(ltok.clone());
                }

                Token::Newline => break,

                Token::Space => {
                    if !self.check_consume_trailing(ls_iter) {
                        self.errs.push(CSTReport::ExpectedXGotY {
                            x: Token::Newline,
                            y: ltok.clone(),
                        });
                    }
                    break;
                }

                _ => {
                    if is_expect_slash {
                        self.errs.push(CSTReport::ExpectedXGotY {
                            x: Token::Slash,
                            y: ltok.clone(),
                        });
                    } else {
                        self.errs.push(CSTReport::ExpectedXGotY {
                            x: Token::Identifier,
                            y: ltok.clone(),
                        });
                    }
                }
            }
        }

        path
    }

    fn parse_bin_op<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self,
        ls_iter: &mut Peekable<I>,
        lhs: CSTNode<'linespan>,
        op: LexToken<'linespan>,
    ) -> Option<CSTNode<'linespan>> {
        let mut bin_op = CSTNode::BinOp {
            lhs: Box::new(lhs.clone()),
            op: op.clone(),
            rhs: None,
        };
        let CSTNode::BinOp { ref mut rhs, .. } = bin_op else {
            return None;
        };

        self.consume_space(ls_iter);
        match ls_iter.peek() {
            Some(ltok) => match ltok.token {
                Token::Identifier => {}
                Token::Number => {}
                _ => {}
            },
            None => {
                self.errs
                    .push(CSTReport::EndOfFileDuring(CSTNode::BinOp {
                        lhs: Box::new(lhs),
                        op,
                        rhs: None,
                    }));
            }
        }
        match self.parse_expr(ls_iter, 0, false) {
            Some(expr) => *rhs = Some(Box::new(expr)),
            None => return None,
        }

        if self.check_consume_trailing(ls_iter) {
            return Some(bin_op);
        }
        match ls_iter.peek() {
            Some(peek) => match peek.token {
                Token::Newline => {
                    ls_iter.next();
                    return Some(bin_op);
                }

                Token::Plus
                | Token::Dash
                | Token::Star
                | Token::Slash
                | Token::Percent
                | Token::ExclMark => {
                    let _peek = (*peek).clone();
                    return self.parse_bin_op(ls_iter, bin_op, _peek);
                }

                _ => return Some(bin_op),
            },
            None => {
                return Some(bin_op);
            }
        }
    }

    fn parse_ascii<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &mut self,
        ls_iter: &mut Peekable<I>,
        key: LexToken<'linespan>,
    ) -> Option<CSTNode<'linespan>> {
        let mut ascii = CSTNode::Ascii {
            key_start: key,
            lines: Vec::new(),
            key_end: None,
        };
        let CSTNode::Ascii {
            ref mut lines,
            ref mut key_end,
            ..
        } = ascii
        else {
            self.errs.push(CSTReport::InternalError(27));
            return None;
        };
        let mut is_asciiend_check = true;

        loop {
            let Some(ltok) = ls_iter.next() else {
                return Some(ascii);
            };
            if !is_asciiend_check {
                if ltok.token == Token::Newline {
                    is_asciiend_check = true;
                }
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
        ls_iter: &mut Peekable<I>,
    ) {
        loop {
            let Some(peek) = ls_iter.peek() else { break };
            if peek.token != Token::Comment || peek.token != Token::Newline {
                ls_iter.next();
            } else {
                break;
            }
        }
    }

    fn consume_space<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &self,
        ls_iter: &mut Peekable<I>,
    ) -> bool {
        if self.expect_peek_token(Token::Space, ls_iter) {
            ls_iter.next();
            return true;
        }

        self.consume_continue(ls_iter)
    }

    fn consume_indent_sz<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &self,
        ls_iter: &mut Peekable<I>,
    ) -> usize {
        return match ls_iter.peek() {
            Some(ltok) if ltok.token == Token::Space => {
                let len = ltok.span.str.chars().count();
                ls_iter.next();
                return len;
            }
            _ => 0,
        };
    }

    fn check_consume_trailing<I: Iterator<Item = &'linespan LexToken<'linespan>>>(
        &self,
        ls_iter: &mut Peekable<I>,
    ) -> bool {
        loop {
            match ls_iter.peek() {
                Some(ltok) => match ltok.token {
                    Token::Newline => break,
                    Token::Space | Token::Comment | Token::MultiComment => {
                        ls_iter.next();
                        continue;
                    }
                    _ => return false,
                },
                None => {
                    ls_iter.next();
                    return false;
                }
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
            self.errs.push(CSTReport::ExpectedXGotY {
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
            self.errs.push(CSTReport::ExpectedXGotY {
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
        ls_iter: &mut Peekable<I>,
    ) -> bool {
        if let Some(ltok) = ls_iter.peek()
            && ltok.token == token
        {
            return true;
        }

        false
    }
}
