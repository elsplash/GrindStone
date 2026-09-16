mod internals;
use internals::{
    handle_args,
    Token,
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

    let mut err_count: usize = 0;
    let mut is_end_early_by_err: Vec<String> = vec![];

    for file in grindstone_config.file_paths.iter() {
        let mut line_spans: Vec<LineSpan> = Vec::new();
        let mut lex_output: LexerOutput = LexerOutput::new();

        if !lex_output.tokenize_file(file, &mut line_spans) { break; }

        // for ltok in lex_output.0.iter() {
        //     print!("{} ", ltok.token);
        //     if ltok.token == Token::Newline {
        //         println!("")
        //     }
        // }

    	let mut cst_tree = CSTOutput::new();
        if !cst_tree.parse_all(&mut lex_output) { is_end_early_by_err.push(file.clone()) }

        // for node in cst_tree.output.iter() {
        //     println!("{:#?}\n", node);
        // }

    	for err in cst_tree.errs.iter() {
    	    println!("{err}\n");
    	}

        err_count += cst_tree.errs.len();

        /* AST Will merge all CST Trees */
        /* ast_tree.merge(cst_tree); */
    }

    /* TODO: Semantic Analyzer */

    /*
     * Semantically Analyze the Parsed output
     * Linter logic function here
     *
     * You can implement the formatter after making Tokens
     * interchangeable to Strings.
     */

    if err_count != 0 {
        println!("[REPORT] {} errors reported.", err_count);
    }

    for ended_early in is_end_early_by_err.iter() {
        println!("[NOTE] File `{ended_early}` was not read completely by an error.");
        println!("[NOTE] You may fix the errors from `{ended_early}` and rerun GrindStone.\n");
    }
}
