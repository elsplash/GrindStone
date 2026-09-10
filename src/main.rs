mod internals;
use internals::{
    handle_args,
    LineSpan,
    LexerOutput,
    CSTOutput,
};

fn main() {
    let grindstone_config = match handle_args() {
        Some(gsc) => gsc,
        None => return,
    };

    /* let mut ast_tree = ASTOutput::new(); */

    /* err count var */

    for file in grindstone_config.file_paths.iter() {
        let mut line_spans: Vec<LineSpan> = Vec::new();
        let mut lex_output: LexerOutput = LexerOutput::new();

        if !lex_output.tokenize_file(file, &mut line_spans) { break; }

    	let mut cst_tree = CSTOutput::new();
        cst_tree.parse_all(&mut lex_output);

        /* AST Will merge all CST Trees */
        /* ast_tree.merge(cst_tree); */
    }

    // ...

    /* TODO: Semantic Analyzer */

    /*
     * Semantically Analyze the Parsed output
     * Linter logic function here
     *
     * You can implement the formatter after making Tokens
     * interchangeable to Strings.
     */

    /* display error count */
}

