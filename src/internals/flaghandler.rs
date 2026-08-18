use std::env::args;

#[derive(Debug, Clone)]
enum FlagState {
    Start,
    Path,
    TabWidth,
    FMTMode,
    LNTRMode,
}

pub enum FormatterMode {
    Strict,
    Normal,
    Off,
}

pub enum LinterMode {
    Strict,
    Picky,
    Normal,
    Off,
}

pub struct GSConfig {
    pub file_paths: Vec<String>,
    pub tabspace_sz: i32,
    pub debug_flag: bool,
    pub fmt_mode: FormatterMode,
    pub lntr_mode: LinterMode,
}

pub fn handle_args() -> Option<GSConfig> {
    let mut fs = FlagState::Start;
    let mut gsc = GSConfig::new();
    for arg in args() {
        match (arg.as_str(), fs.clone()) {
            (_, FlagState::Start) => {
                fs = FlagState::Path;
                continue;
            },
            (a, FlagState::Path) => match a {
                "-L0" => gsc.lntr_mode = LinterMode::Off,
                "-L1" => gsc.lntr_mode = LinterMode::Normal, /* Default, but for later versions maybe not. */
                "-L2" => gsc.lntr_mode = LinterMode::Picky,
                "-L3" => gsc.lntr_mode = LinterMode::Strict,

                "-F0" => gsc.fmt_mode = FormatterMode::Off, /* Default, but for later versions maybe not. */
                "-F1" => gsc.fmt_mode = FormatterMode::Normal,
                "-F2" => gsc.fmt_mode = FormatterMode::Strict,

                "-L" | "--linter-mode" => fs = FlagState::LNTRMode,
                "-F" | "--formatter-mode" => fs = FlagState::FMTMode,
                "-t" | "--tab-spaces" => fs = FlagState::TabWidth,
                "-d" | "--debug" => gsc.debug_flag = true,
                "-h" | "--help" => {
                    let help_message: &str = r#"
- GrindStone -
Sharpen your MindStone.

A tool that checks for errors, outside of the game.
And also lints and formats.

- Flags -
    -h   | --help             : Prints this message.
    -L#  | --linter-mode #    : Sets the linter mode, by a number, or name of the mode.
    -F#  | --formatter-mode # : Sets the formatter mode, by a number or name of the mode.
    -t # | --tab-spaces #     : Sets the tab spaces of the indentation.
    -d   | --debug            : Makes more output. (For developers!)

- Linter modes -
    -L -> 0 : Off    } Turns off the linter
          1 : Normal } Adds structural enforcement [DEFAULT]
          2 : Picky  } Will only show code errors
          3 : Strict } Also forces documentation of code

- Formatter modes -
    -F -> 0 : Off    } Turns the formatter off, code will not be modified [DEFAULT]
          1 : Normal } Fixes structure of code.
          2 : Strict } Also adds extra comments for you to add.
"#;
                    println!("{help_message}");
                    return None;
                },

                _ => gsc.file_paths.push(arg.to_owned()),
            },

            (num, FlagState::LNTRMode) => match num.to_lowercase().as_str() {
                "0" => gsc.lntr_mode = LinterMode::Off,
                "1" => gsc.lntr_mode = LinterMode::Normal, /* Default, but for later versions maybe not. */
                "2" => gsc.lntr_mode = LinterMode::Picky,
                "3" => gsc.lntr_mode = LinterMode::Strict,

                "off" => gsc.lntr_mode = LinterMode::Off,
                "normal" => gsc.lntr_mode = LinterMode::Normal, /* Default, but for later versions maybe not. */
                "picky" => gsc.lntr_mode = LinterMode::Picky,
                "strict" => gsc.lntr_mode = LinterMode::Strict,

                _ => {
                    println!("[ERROR] {num} is invalid.");
                    println!("[TIP] Please only provide numbers from 0 to 3, like 2.");
                    return None;
                },
            },

            (num, FlagState::FMTMode) => match num {
                "0" => gsc.fmt_mode = FormatterMode::Off, /* Default, but for later versions maybe not. */
                "1" => gsc.fmt_mode = FormatterMode::Normal,
                "2" => gsc.fmt_mode = FormatterMode::Strict,

                "off" => gsc.fmt_mode = FormatterMode::Off, /* Default, but for later versions maybe not. */
                "normal" => gsc.fmt_mode = FormatterMode::Normal,
                "strict" => gsc.fmt_mode = FormatterMode::Strict,

                _ => {
                    println!("[ERROR] {num} is invalid.");
                    println!("[TIP] Please only provide numbers from 0 to 2, like 2.");
                    return None;
                }
            },

            (num, FlagState::TabWidth) => {
                let Ok(parsed_tabwidth) = num.parse::<i32>() else {
                    println!("[ERROR] Could not parse Tab width: {num}");
                    println!("[TIP] Maybe try a number? Or don't have this flag, it detects it automatically.");
                    return None;
                };
                if parsed_tabwidth < 1 {
                    println!("[ERROR] You entered a tab width lower than 1, which is not possible.");
                    return None;
                }
                gsc.tabspace_sz = parsed_tabwidth;
            },
        }
    }

    Some(gsc)
}

impl GSConfig {
    pub fn new() -> GSConfig {
        GSConfig{
            file_paths: Vec::new(),
            tabspace_sz: -1,
            debug_flag: false,
            fmt_mode: FormatterMode::Off,
            lntr_mode: LinterMode::Normal,
        }
    }
}
