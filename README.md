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

### BUILDING / INSTALLING

You may compile from source, by just building it.

```sh
cargo build -r
```

Or you may install pre-compiled executables in the
releases tab.

### USING GRINDSTONE

After installing GrindStone, you may use the `--help`
flag to see what flags are available, and how to
use them.

When you're ready to know the errors of your
Stonescript code, you may run:

```sh
./GrindStone yourcode.txt # Linux/MacOS

GrindStone yourcode.txt # Windows
```

Now you'll see your beautiful errors, without opening the
app.

## Future implementations

This should just be a list of things I should implement
after I stopped working on the project, which is updated
in 09.25.2026 currently.

### Manual References

Make errors have attacked Manual references like:

```none
[ERROR] Expected Equal (=) got Identifier.
23 | for v in 1..2
   |       ^^
[HINT] You may replace `in` with `=`.
23 - for v in 1..2
23 + for v = 1..2
[HELP] For proper `for` statements, go to https://stonestoryrpg.com/stonescript/manual.html#loops
```

So people can know where to go.

## Review

> TBA
