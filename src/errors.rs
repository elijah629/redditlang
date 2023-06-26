use colorize::AnsiColor;
use pest::error::LineColLocation;

use crate::Rule;

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

pub fn error(error: pest::error::Error<Rule>) -> ! {
    println!("{}", format_error(error));
    std::process::exit(1);
}

pub const ERR_BUG: &str =
    "Unexpected error. This is a bug, please report this at https://github.com/elijah629/redditlang/issues";
