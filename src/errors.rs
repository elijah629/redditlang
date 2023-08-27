use crate::Rule;
use colored::Colorize;
use core::fmt;
use pest::error::{InputLocation, LineColLocation};

pub fn format_error(error: pest::error::Error<Rule>) -> Option<String> {
    let code = error.line();

    let pos = match error.line_col {
        LineColLocation::Pos(x) => x,
        LineColLocation::Span(x, _) => x,
    };

    let arrow_size = match error.location {
        InputLocation::Pos(_) => 1,
        InputLocation::Span((a, b)) => b - a,
    };

    // fast count digits, zero is not counted because there is no "line zero"
    let line_padding = " ".repeat(pos.0.ilog10() as usize + 1);
    let error_arrow_padding = " ".repeat(pos.1);

    let message = error.variant.message().bold();

    let path = error.path()?;
    let line = pos.0;
    let col = pos.1;
    let colored_line = line.to_string().blue().bold();

    let colored_bar = "|".blue().bold();

    let colored_arrow = "-->".blue().bold();
    let colored_error_arrow = "^".repeat(arrow_size).red().bold();
    return Some(format!(
        r"{message}
{line_padding}{colored_arrow} {path}:{line}:{col}
{line_padding} {colored_bar}
{colored_line} {colored_bar} {code}
{line_padding} {colored_bar}{error_arrow_padding}{colored_error_arrow}
{line_padding} {colored_bar}"
    ));
}

pub fn syntax_error(syntax_error: pest::error::Error<Rule>) -> ! {
    error!("{}", format_error(syntax_error).unwrap());
}

const ERR_BUG: &str =
    "Error! This is a bug, please report this at https://github.com/elijah629/redditlang/issues. Include your code and any other context.";

pub fn _bug(args: fmt::Arguments) -> ! {
    error!("{}{}", ERR_BUG, args);
}

pub fn _error(args: fmt::Arguments) -> ! {
    log::error!("{}", args);
    std::process::exit(1);
}

// log:error but exits
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        $crate::errors::_error(std::format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! bug {
    ($($arg:tt)*) => {{
        $crate::errors::_bug(std::format_args!($($arg)*));
    }};
}

pub(crate) use error;
