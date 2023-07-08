use core::fmt;

use colored::Colorize;
use pest::error::LineColLocation;

use crate::{errors, Rule};

pub fn format_error(error: pest::error::Error<Rule>) -> String {
    let code = error.line();
    let pos = match error.line_col {
        LineColLocation::Pos(x) => x,
        LineColLocation::Span(x, _) => x,
    };
    let line_padding = " ".repeat(pos.0.to_string().len());
    let error_arrow_padding = " ".repeat(pos.1);

    let message = error.variant.message().to_string().red().bold();

    let colored_line = pos.0.to_string().blue().bold();
    let colored_col = pos.1.to_string().blue().bold();

    let colored_error_position = format!("{}{}{}", colored_line, ":".blue().bold(), colored_col);

    let colored_bar = "|".blue().bold();
    let colored_eq = "=".blue().bold();

    let colored_arrow = "-->".blue().bold();
    let colored_error_arrow = "^".red().bold();
    return format!(
        r"
{line_padding}{colored_arrow} {colored_error_position}
{line_padding} {colored_bar}
{colored_line} {colored_bar} {code}
{line_padding} {colored_bar}{error_arrow_padding}{colored_error_arrow}
{line_padding} {colored_bar}
{line_padding} {colored_eq} {message}
"
    );
}

pub fn syntax_error(syntax_error: pest::error::Error<Rule>) -> ! {
    error!("{}", format_error(syntax_error));
}

const ERR_BUG: &str =
    "Error! This is a bug, please report this at https://github.com/elijah629/redditlang/issues. Make sure to include your code! Additional Information: ";

pub fn _bug(args: fmt::Arguments) -> ! {
    error!("{}{}", ERR_BUG, args);
}

pub fn _error(args: fmt::Arguments) -> ! {
    log::error!("{}", args);
    std::process::exit(1);
}

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
