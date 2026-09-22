# ISSUES.md

This is where known issues live, so I can fix them when I come back to this project.

## STRUCTURAL ISSUES

### The LineSpan lifetime issue

This issue stems from `StrSpan` being coupled with the
addresses of `LineSpan` in which it carries over to the
other files.

The solution to this is rather simple, which is to just
replace it with another step in the Lexing process. To
fetch all the lines, store them outside the loop, and
tokenize from there.

The current structure of course will not support this,
the current structure will make the semantic analyzer
inside the loop, which makes it limited to the current
file.

Fixing this issue will pull out the CST and AST variables
outside the loop, and can be analyzed outside the loop.

### File organization

This project was inheritly big, given that a linter's
complexity. Though at the time, this project I thought
of was kind of simple to implement, though I was quickly
proven wrong.

This could be resolved by the sorting of the files in a
folder.
