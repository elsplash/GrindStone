use std::{
    path::Path,
    fs::File,
    io::{BufReader, BufRead},
    fmt::{Display, self},
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Token {
    Identifier, Number,

    Newline, Space,

    Dot, Comma, Quote, Tilde, Hashtag, Caret,

    QuestMark, Colon, Equal, Lesser, Greater, LessEq, GreatEq,

    And, Bar, ExclMark,

    Plus, Minus, Star, Slash, Percent, At,

    LParen, RParen, LBracket, RBracket,

    Comment, MultiComment,

    Continue,
}

#[derive(Clone)]
enum LexStat {
    Identifier,
    Number,
    MultiComment,
}

#[derive(Clone)]
pub struct LineSpan {
    str: String,
    num: usize,
}

#[derive(Clone)]
pub struct StrSpan<'linespan> {
    pub line: &'linespan LineSpan,
    pub str: String,
    pub clmn: usize,
}

#[derive(Clone)]
pub struct LexToken<'linespan> {
    pub token: Token,
    pub span: StrSpan<'linespan>,
}

pub struct LexerOutput<'linespan>(pub Vec<LexToken<'linespan>>);

impl LineSpan {
    pub fn new(string: String, num: usize) -> LineSpan {
        LineSpan{
            str: string,
            num: num,
        }
    }
}

impl<'linespan> StrSpan<'linespan> {
    pub fn new(
        s: String,
        line: &'linespan LineSpan,
        clmn_start: usize,
    ) -> StrSpan {
        StrSpan{
            str: s,
            line: line,
            clmn: clmn_start,
        }
    }
}

impl<'linespan> LexToken<'linespan> {
    pub fn concat(&mut self, tok: LexToken<'linespan>) -> bool {
        if self.span.line.num != tok.span.line.num { return false }
        match self.token {
            Token::Identifier => {/* Ignore */},
            Token::Number if tok.token == Token::Number => {
                self.span.str += tok.span.str.as_str();
                return true;
            }
            _ => self.token = Token::Identifier,
        }
        self.span.str += tok.span.str.as_str();

        true
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display: &str = match self {
            Token::Identifier => "Identifier",
            Token::Number => "Number",

            Token::Newline => "Newline",
            Token::Space => "Space",

            Token::Dot => "Dot",
            Token::Comma => "Comma",
            Token::Quote => "Quote",
            Token::Tilde => "Tilde",
            Token::Hashtag => "Hashtag",
            Token::Caret => "Caret",

            Token::QuestMark => "Question Mark",
            Token::Colon => "Colon",
            Token::Equal => "Equal",
            Token::Lesser => "Lesser Than",
            Token::Greater => "Greater Than",
            Token::LessEq => "Lesser or Equal Than",
            Token::GreatEq => "Greater or Equal Than",

            Token::And => "Ampersand",
            Token::Bar => "Bar",
            Token::ExclMark => "Exclamation Mark",

            Token::Plus => "Plus",
            Token::Minus => "Minus",
            Token::Star => "Star",
            Token::Slash => "Slash",
            Token::Percent => "Percent",
            Token::At => "At, or Ki",

            Token::LParen => "Left Parenthesis",
            Token::RParen => "Right Parenthesis",
            Token::LBracket => "Left Bracket",
            Token::RBracket => "Right Bracket",

            Token::Comment => "Comment",
            Token::MultiComment => "Multi-line Comment",
            Token::Continue => "Caret",
        };
        write!(f, "{display}")
    }
}

impl Display for LineSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} | {}", self.num, self.str)
    }
}

impl<'linespan> Display for StrSpan<'linespan> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lnum_sz = (self.line.num / 10) + 1;
        write!(f, "{}\n{} | {}{}",
            self.line,
            " ".repeat(lnum_sz),
            " ".repeat(self.clmn),
            "^".repeat(self.str.len())
        )
    }
}

impl<'linespan> LexerOutput<'linespan> {
    pub fn new() -> LexerOutput<'linespan> { LexerOutput{0: Vec::new()} }

    fn tok_push(&mut self, t: &Token, c: char, clmn: usize, line: &'linespan LineSpan) {
        if let Some(lt) = self.0.last_mut() && lt.token == *t {
            if *t == Token::Identifier
                || *t == Token::Number
        || *t == Token::Space {
                lt.span.str.push(c);
                lt.span.clmn += 1;
                return;
            }
        };

        if let Some(lt) = self.0.last_mut() {match (lt.token.clone(), t) {
            (Token::Number, Token::Identifier) => {
                let new_span = StrSpan::new(
                    format!("{}{}", lt.span.str, c),
                    line, lt.span.clmn,
                );
                lt.token = Token::Identifier;
                lt.span = new_span;
                return;
            },

            (Token::Identifier, Token::Number) => {
                let new_span = StrSpan::new(
                    format!("{}{}", lt.span.str, c),
                    line, lt.span.clmn,
                );
                lt.span = new_span;
            },

            _ => {}
        }}

        self.0.push(LexToken{
            token: t.clone(),
            span: StrSpan::new(String::from(c), line, clmn),
        });
    }

    pub fn tokenize_file(&mut self, file_path: &str, line_spans: &'linespan mut Vec<LineSpan>) -> bool {
        let path = Path::new(file_path);
        let file: File;
        match File::open(path) {
            Ok(f) => file = f,
            Err(_) => {
                println!("[ERROR] Could not open file {file_path}.");
                return false;
            },
        }
        let freader: BufReader<File> = BufReader::new(file);

        let mut line_num: usize = 0;
        for might_be_line in freader.lines() {
            let Ok(line) = might_be_line else {
                println!("[ERROR] Could not read from file {file_path}.");
                return false;
            };
            line_spans.push(LineSpan::new(line, line_num));
            line_num += 1;
        }

        let mut line_stat = LexStat::Identifier;
        for span in line_spans.iter() {
            self.tokenize_line(span, &mut line_stat);
        }

        true
    }

    fn tokenize_line(&mut self, line: &'linespan LineSpan, stat: &mut LexStat) {
        let mut lchars = line.str.chars().peekable();
        let mut clmn: usize = 0;
        loop {
            let Some(current) = lchars.next() else { break; };
            match stat.clone() {
                LexStat::Identifier => {
                      if current.is_alphabetic()
                          || current == '#'
                          || current == '_' {
                          self.tok_push(&Token::Identifier, current, clmn, line);
                          continue;
                      } else if current.is_numeric() {
                          *stat = LexStat::Number;
                          self.tok_push(&Token::Number, current, clmn, line);
                          continue;
                      }
                  },

                LexStat::Number => {
                      if current.is_numeric() || current == '.' {
                          self.tok_push(&Token::Number, current, clmn, line);
                          continue;
                      } else if current.is_alphabetic() {
                          *stat = LexStat::Identifier;
                          self.tok_push(&Token::Identifier, current, clmn, line);
                          continue;
                      }
                },

                LexStat::MultiComment => {
                    if current == '*' {
                        let Some(peek) = lchars.peek() else { return };
                        if *peek == '/' { *stat = LexStat::Identifier; }
                    }
                    continue;
                },
            }

            match current {
                ' ' => self.tok_push(&Token::Space, current, clmn, line),
                '.' => self.tok_push(&Token::Dot, current, clmn, line),
                ',' => self.tok_push(&Token::Comma, current, clmn, line),
                '"' => self.tok_push(&Token::Quote, current, clmn, line),
                '`' => self.tok_push(&Token::Tilde, current, clmn, line),
                '#' => self.tok_push(&Token::Hashtag, current, clmn, line),

                '^' => {
                    if let Some(ltok) = self.0.last() && ltok.token == Token::Newline {
                        self.0.pop();
                        continue;
                    }
                    self.tok_push(&Token::Caret, current, clmn, line)
                },

                '?' => self.tok_push(&Token::QuestMark, current, clmn, line),
                ':' => self.tok_push(&Token::Colon, current, clmn, line),
                '=' => self.tok_push(&Token::Equal, current, clmn, line),

                '<' => self.tok_push(
                    if let Some('=') = lchars.peek() {
                        lchars.next();
                        &Token::LessEq
                    } else { &Token::Lesser},
                    current, clmn, line
                ),
                '>' => self.tok_push(
                    if let Some('=') = lchars.peek() {
                        lchars.next();
                        &Token::GreatEq
                    } else { &Token::Greater },
                    current, clmn, line
                ),

                '&' => self.tok_push(&Token::And, current, clmn, line),
                '|' => self.tok_push(&Token::Bar, current, clmn, line),
                '!' => self.tok_push(&Token::ExclMark, current, clmn, line),

                '+' => self.tok_push(&Token::Plus, current, clmn, line),
                '-' => self.tok_push(&Token::Minus, current, clmn, line),
                '*' => self.tok_push(&Token::Star, current, clmn, line),

                '/' => {
                    if let Some(peek) = lchars.peek() {match *peek {
                        '/' => {
                            lchars.next();
                            self.tok_push(&Token::Comment, current, clmn, line);
                            break;
                        },
                        '*' => {
                            lchars.next();
                            self.tok_push(&Token::MultiComment, current, clmn, line);
                            *stat = LexStat::MultiComment;
                            continue;
                        },
                        _ => {},
                    }}
                    self.tok_push(&Token::Slash, current, clmn, line)
                },

                '@' => self.tok_push(&Token::At, current, clmn, line),
                '%' => self.tok_push(&Token::Percent, current, clmn, line),

                '(' => self.tok_push(&Token::LParen, current, clmn, line),
                ')' => self.tok_push(&Token::RParen, current, clmn, line),
                '[' => self.tok_push(&Token::LBracket, current, clmn, line),
                ']' => self.tok_push(&Token::RBracket, current, clmn, line),

                _ => self.tok_push(&Token::Identifier, current, clmn, line),
            }
            clmn += 1;
        }

        self.tok_push(&Token::Newline, '\n', clmn, line);
    }
}
