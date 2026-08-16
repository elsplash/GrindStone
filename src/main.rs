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

    let mut err_count: usize = 0;

    for file in grindstone_config.file_paths.iter() {
        let mut lexer_errors: Vec<LexUnknownSymbols> = Vec::new();
        let mut line_spans: Vec<LineSpan> = Vec::new();
        let mut lex_output: LexerOutput = LexerOutput::new();

        if !lex_output.tokenize_file(file, &mut lexer_errors, &mut line_spans) { break; }

        if let Some(front) = lexer_errors.first() && front.clmns.len() > 0 {
            for err in lexer_errors.iter() {
                err_count += err.clmns.len();
                err.print();
            }
        }

        /* Parser!! */
    }

    /*
     * Semantically Analyze the Parsed output
     * Linter logic function here
     *
     * You can implement the formatter after making Tokens
     * interchangeable to Strings.
     */

	if err_count != 0 {
        println!("[GRINDSTONE] {err_count} errors generated. Fix them, as the code won't run.");
    }
}

