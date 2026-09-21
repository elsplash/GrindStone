# GrindStone

## ANNOUNCEMENTS

This project has gone way too big, and now the schedules are colliding.
I will have to cut some features of this project. I will have to cut
these features:

- Linter
- Formatter

As they have gone way out of the schedule, you may thank the CST parser
for this hassle.

This project will still have error checking, don't worry. I am trying
my best to not make this project a unfinished one where it's forever
 working with a large codebase
of Stonescript, like [this one](https://github.com/Eunomiac/stone-story).

In which you shouldn't cite your code line by line or be
disappointed when the code errors out on you on your phone.

> Insert finishing bad code and running it through GrindStone

### A Linter with 4 Modes
**OFF** (self-explanatory)

This will turn off linter features, but not the syntax checker,
you will only get actual errors, not warnings.

This includes indentation errors, unused imports, and
unreachable if conditions, so you won't be getting them as
warnings.

**Normal** [DEFAULT]

This will include the standard set of warnings, indentation
errors, unused imports, unreachable conditions, including
some help messages.

> Insert normal linter warnings

**Picky**

This includes the set of Normal mode and naming conventions,
so it would enforce the names of variables, and functions
to follow snake_case naming,

> Insert picky linter warnings

**Strict**

This will enforce comments to document the code you have,
for more maintainability.

> Insert strict linter warnings

### A Formatter with 3 Modes

**OFF** [default]

Will not format your code after linting.

**Normal**

Will fix some of the linter errors, and some syntactic
errors, though it will override your current code,
unless given an output file.

**Strict**

Will add placeholder comments between separated
variables and functions as Strict mode, and also
fixes the naming conventions for you.

## Usage

> IN DEVELOPMENT!

## Future implementations

This should just be a list of things I should implement
after I stopped working on the project, which is updated
in [insert date] currently.

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

## Review

> TBA
