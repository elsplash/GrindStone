use crate::internals::{
    LexToken,
};

pub enum ExprCST<'linespan> {
    String{
        ldoublequot: LexToken<'linespan>,
        str: Vec<LexToken<'linespan>>,
        rdoublequot: LexToken<'linespan>,
    },
    Num{ num: LexToken<'linespan> },
    Comma{ comma: LexToken<'linespan> },
    Space{
        size: usize,
        space: LexToken<'linespan>
    }, /* Acts as both the indent level, and the filler in between */

    Parens{
        lparen: LexToken<'linespan>,
        items: Vec<Box<ExprCST<'linespan>>>,
        rparen: LexToken<'linespan>,
    },
    Bracket{
        lbrack: LexToken<'linespan>,
        items: Box<ExprCST<'linespan>>,
        rbrack: LexToken<'linespan>
    },

    If{
        lhs: Box<ExprCST<'linespan>>,
        op: LexToken<'linespan>,
        rhs: Box<ExprCST<'linespan>>,
    },
    Else{ if_stmnt: Option<Box<ExprCST<'linespan>>>},

    FuncDecl{
        func_key: LexToken<'linespan>,
        name: LexToken<'linespan>,
        paren: Box<ExprCST<'linespan>>,
    },
    VarDecl{
        var_key: LexToken<'linespan>,
        name: LexToken<'linespan>,
        equal: Option<LexToken<'linespan>>,
        var_def: Option<Box<ExprCST<'linespan>>>,
    },

    VarDef{
        name: LexToken<'linespan>,
        equal: LexToken<'linespan>,
        var_def: Box<ExprCST<'linespan>>,
    },
    FuncCall{
        name: LexToken<'linespan>,
        paren: Box<ExprCST<'linespan>>,
    },

    Equation{
        lhs: Box<ExprCST<'linespan>>,
        op: LexToken<'linespan>,
        rhs: Box<ExprCST<'linespan>>,
    },
    VarMethod{
        var: LexToken<'linespan>,
        dot: LexToken<'linespan>,
        method_name: LexToken<'linespan>,
        method_paren: Box<ExprCST<'linespan>>,
    },
}

pub struct FileCST<'linespan> {
    file_name: String,
    cst_expressions: Vec<ExprCST<'linespan>>,
}

pub struct CSTParser<'linespan>(Vec<FileCST>);

/* Implement the functions for these things... */
