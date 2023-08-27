use colored::{self, Color, Colorize};
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

struct ColoredLogger;

fn level_to_color(level: Level) -> Color {
    match level {
        Level::Error => Color::Red,
        Level::Warn => Color::Yellow,
        Level::Info => Color::Blue,
        Level::Debug => Color::Green,
        Level::Trace => Color::White,
    }
}

impl log::Log for ColoredLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!(
                "{}{} {}",
                record
                    .level()
                    .to_string()
                    .to_lowercase()
                    .color(level_to_color(record.level()))
                    .bold(),
                ":".bold(),
                //":".color(level_to_color(record.level())).bold(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

static LOGGER: ColoredLogger = ColoredLogger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}
