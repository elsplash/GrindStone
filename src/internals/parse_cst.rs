use crate::internals::{
    Token,
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
    Comment{
        cmnt_start: LexToken<'linespan>,
        contents: Vec<LexToken<'linespan>>,
    },
    MultiComment{
        multicmnt_start: LexToken<'linespan>,
        contents: Vec<LexToken<'linespan>>,
        multicmnt_end: LexToken<'linespan>,
    },

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

pub enum ParserError<'linespan> {
    NumberAsStatement(LexToken<'linespan>),
    InvalidStatement(LexToken<'linespan>),
    ExpectedXGotY(Token, LexToken<'linespan>),
}

pub struct FileCST<'linespan> {
    file_name: String,
    cst_expressions: Vec<ExprCST<'linespan>>,
}

pub struct CSTParser<'linespan>(Vec<FileCST<'linespan>>);

impl<'linespan> CSTParser<'linespan> {
    pub fn new() -> CSTParser<'linespan> { CSTParser(Vec::new()) }

    pub fn parse_file(&mut self) {}

    fn find_and_parse_type(&mut self, first_token: &LexToken<'linespan>, line_section: &Vec<LexToken<'linespan>>) -> Option<ParserError> {
        match first_token.token {
            Token::Identifier => match first_token.span.str.as_str() {
                "var" => {/* Get Variable. */},
                "func" => {/* Get the function statement */},
                /* Probably more here. */
                _ => {/* Get the identifier, and call the function again. */},
            },

            Token::QuestMark => {/* The if statement */},
            Token::Colon => {/* Check if there's a QuestMark token, else, it's an else. */},

            /* Probably more here. */

            _ => return Some(
                ParserError::ExpectedXGotY(Token::Identifier, (*first_token).clone())
            ),
        }
        None
    }
}
