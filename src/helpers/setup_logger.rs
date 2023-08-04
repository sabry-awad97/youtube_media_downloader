use fern::colors::{Color, ColoredLevelConfig};
use std::io::stderr;
use std::io::stdout;

pub fn setup_logger(
    level: log::LevelFilter,
    log_filename: Option<&str>,
) -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red)
        .debug(Color::White)
        .trace(Color::BrightBlack);

    let mut output = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{timestamp}] {level} in {module}: {message}",
                timestamp = chrono::Local::now().format("%H:%M:%S"),
                level = colors.color(record.level()),
                module = record.module_path().unwrap_or("<unnamed>"),
                message = message
            ))
        })
        .level(level)
        .chain(stdout())
        .chain(stderr());

    if let Some(filename) = log_filename {
        output = output.chain(fern::log_file(filename)?);
    }

    output.apply()?;

    Ok(())
}
