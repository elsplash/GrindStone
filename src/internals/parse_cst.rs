use crate::internals::{
    Token,
    LexToken,
};

pub enum ExprCST<'linespan> {
    String{
        ldoublequot: Option<LexToken<'linespan>>,
        str: Vec<LexToken<'linespan>>,
        rdoublequot: Option<LexToken<'linespan>>,
    },
    Num{ num: Option<LexToken<'linespan>> },
    Comma{ comma: Option<LexToken<'linespan>> },
    Space{
        size: usize,
        space: Option<LexToken<'linespan>>,
    }, /* Acts as both the indent level, and the filler in between */

    Comment{
        cmnt_start: Option<LexToken<'linespan>>,
        contents: Vec<LexToken<'linespan>>,
    },
    MultiComment{
        multicmnt_start: Option<LexToken<'linespan>>,
        contents: Vec<LexToken<'linespan>>,
        multicmnt_end: Option<LexToken<'linespan>>,
    },

    Parens{
        lparen: Option<LexToken<'linespan>>,
        items: Vec<Box<ExprCST<'linespan>>>,
        rparen: Option<LexToken<'linespan>>,
    },
    Bracket{
        lbrack: Option<LexToken<'linespan>>,
        items: Box<ExprCST<'linespan>>,
        rbrack: Option<LexToken<'linespan>>
    },

    If{
        lhs: Option<Box<ExprCST<'linespan>>>,
        op: Option<LexToken<'linespan>>,
        rhs: Option<Box<ExprCST<'linespan>>>,
    },
    Else{ if_stmnt: Option<Box<ExprCST<'linespan>>>},

    FuncDecl{
        func_key: Option<LexToken<'linespan>>,
        name: Option<LexToken<'linespan>>,
        paren: Option<Box<ExprCST<'linespan>>>,
    },
    VarDecl{
        var_key: Option<LexToken<'linespan>>,
        name: Option<LexToken<'linespan>>,
        equal: Option<LexToken<'linespan>>,
        var_def: Option<Box<ExprCST<'linespan>>>,
    },

    VarDef{
        name: Option<LexToken<'linespan>>,
        equal: Option<LexToken<'linespan>>,
        var_def: Option<Box<ExprCST<'linespan>>>,
    },
    FuncCall{
        name: Option<LexToken<'linespan>>,
        paren: Option<Box<ExprCST<'linespan>>>,
    },

    Equation{
        lhs: Option<Box<ExprCST<'linespan>>>,
        op: Option<LexToken<'linespan>>,
        rhs: Option<Box<ExprCST<'linespan>>>,
    },

    VarMethod{
        var: Option<LexToken<'linespan>>,
        dot: Option<LexToken<'linespan>>,
        method_name: Option<LexToken<'linespan>>,
        method_paren: Option<Box<ExprCST<'linespan>>>,
    },
}

pub enum ParserError<'linespan> {
    ExpectedXGotY(Token, LexToken<'linespan>),
    UselessStatement(ExprCST<'linespan>),
    UnclosedDelimiter(Token, ExprCST<'linespan>),
    InternalError,
}

pub struct FileCST<'linespan> {
    file_name: String,
    cst_expressions: Vec<ExprCST<'linespan>>,
}

pub struct CSTParser<'linespan>(Vec<FileCST<'linespan>>);

impl<'linespan> FileCST<'linespan> {
    pub fn new(file_name: &str) -> FileCST {
        FileCST{
            file_name: String::from(file_name),
            cst_expressions: Vec::new(),
        }
    }
}

impl<'linespan> CSTParser<'linespan> {
    pub fn new() -> CSTParser<'linespan> { CSTParser(Vec::new()) }

    pub fn parse_file(&mut self) {}

    fn find_and_parse_type(
        &mut self,
        first_token: &LexToken<'linespan>,
        line_section: &Vec<LexToken<'linespan>>
    ) -> Vec<ParserError> {
        let mut errs: Vec<ParserError> = Vec::new();
        match first_token.token {
            Token::Identifier => match first_token.span.str.as_str() {
                "var" => {
                    self.parse_var(line_section, &mut errs);
                    return errs;
                },

                "func" => {/* Get the function statement */},

                /* Probably more here. */
                _ => {/* Get the identifier, and call the function again. */},
            },

            Token::QuestMark => {/* The if statement */},
            Token::Colon => {/* Check if there's a QuestMark token, else, it's an else. */},

            /* Probably more here. */

            _ => return errs,
        }
        
        errs
    }

    fn parse_var(&mut self, line_section: &Vec<LexToken<'linespan>>, errs: &mut Vec<ParserError>) -> Option<ExprCST<'linespan>> {
        let expr: ExprCST<'linespan> = ExprCST::VarDecl{
            var_key: None,
            name: None,
            equal: None,
            var_def: None,
        };
        let ls_iter = line_section.iter().peekable();

        let ExprCST::VarDecl() = expr else {
            return None;
        };

        let Some(var) = ls_iter.next() else { return Some(expr) };
        _var_key = var.clone();
        drop(var);

        let Some(name) = ls_iter.next() else {
            errs.push(ParserError::ExpectedXGotY(Token::Identifier, None));
            return Some(expr)
        };
        if name.token != Token::Identifier {
            return Some(expr);
        }
        expr.name = name.clone();
        drop(name);

        let Some(equal) = ls_iter.peek() else { return Some(expr) };
        if equal.token == Token::Newline { return Some(expr); }
        else if equal.token != Token::Equal {
            errs.push(ParserError::ExpectedXGotY(Token::Equal, Some(equal.clone())));
            return Some(expr);
        }
        ls_iter.next();
        expr.equal = equal.clone();
        drop(equal);

         let Some(var_def) = ls_iter.next() else {
            errs.push(ParserError::ExpectedXGotY(Token::Identifier, None));
            return Some(expr);
        };
        expr.var_def = var_def.clone();
        drop(var_def);

        Some(expr)
        }


    fn push_expr_cur_file(&mut self, expr: ExprCST<'linespan>) -> Option<ParserError> {
        if let Some(last_fcst) = self.0.last() {
            match expr {
                ExprCST::Comment(cmnt_start, contents) => {
                    if cmnt_start.is_none() {
                        return Some(ParserError::InternalError);
                    }

                    last_fcst.push(expr.clone());
                    if contents.is_empty() { return Some(ParserError::UselessStatement(expr.clone())) }
                    return None;
                },

                ExprCST::MultiComment(
                    multicmnt_start,
                    contents,
                    multicmnt_end
                ) => {
                    if multicmnt_start.is_none() {
                        return Some(ParserError::InternalError);
                    }

                    last_fcst.push(expr.clone());
                    if multicmnt_end.is_none() {
                        return Some(ParserError::UnclosedDelimiter(
                            Token::MulticommentStart, expr.clone()
                        ));
                    } if contents.is_empty() { return Some(ParserError::UselessStatement(expr.clone())) }
                },

				ExprCST::Paren(
                    lparen,
                    items,
                    rparen,
                ) => {
                    if lparen.is_none() {
                        return Some(ParserError::InternalError);
                    }

                    last_fcst.push(expr.clone());
                    if rparen.is_none() {
                        return Some(ParserError::UnclosedDelim(Token::LParen, expr.clone()));
                    } if items.is_empty() { return Some(ParserError::UselessStatement(expr.clone())) }
                },

                ExprCST::Bracket() => {},

                _ => {},
            }
            /*   ^ If this is false turn into an err_expr */
            /*   ^ Else push into the Vec<FileCST>. */
            /* Else push into the Vec<FileCST>. */
        }
        self.0.push(FileCST::new());
        self.push_token_cur_file(expr);
    }
}
