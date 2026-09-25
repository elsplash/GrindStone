mod internals;
use internals::{
    handle_args,
    LineSpan,
    LexerOutput,

    CSTOutput,

    ASTOutput,
    ast_output_parse_all,
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

    	let mut cst_tree = CSTOutput::new();
        if !cst_tree.parse_all(&mut lex_output) { is_end_early_by_err.push(file.clone()) }

    	for err in cst_tree.errs.iter() {
    	    println!("{err}\n");
    	}

        err_count += cst_tree.errs.len();

        let mut ast_tree = ASTOutput{
            output: Vec::new(),
            errs: Vec::new(),
        };

        ast_output_parse_all(&mut ast_tree, &cst_tree);

        for node in ast_tree.output.iter() {
            println!("{:#?}\n", node);
        }

        for err in ast_tree.errs.iter() {
            println!("{err}\n");
        }

        err_count += ast_tree.errs.len();

    	/* TODO: Semantic Analyzer */
    }


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
