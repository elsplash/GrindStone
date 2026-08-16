mod internals;
use internals::{
    handle_args,
    LineSpan,
    LexUnknownSymbols,
    LexerOutput,
};

fn main() {
    let grindstone_config = match handle_args() {
        Some(gsc) => gsc,
        None => return,
    };

    /* err count var */

    for file in grindstone_config.file_paths.iter() {
        let mut line_spans: Vec<LineSpan> = Vec::new();
        let mut lex_output: LexerOutput = LexerOutput::new();

        if !lex_output.tokenize_file(file, &mut line_spans) { break; }

        /* TOOD: Parse the CST, the CST should be outside of this loop. */
    }

    /* TODO: Parse the CST to an AST. */

    /*
     * Semantically Analyze the Parsed output
     * Linter logic function here
     *
     * You can implement the formatter after making Tokens
     * interchangeable to Strings.
     */

    /* display error count */
}

