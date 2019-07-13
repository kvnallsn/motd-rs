//! MOTD Utility
//!
//! Parses motd templates (*.motd) and displays them to the console

mod commands;
mod error;
mod motd;

use clap::{App, Arg, ArgMatches};
use log::{error, LevelFilter};

/// Setups the up the command line arguments to process for
/// generating a message of the day
fn get_arguments() -> ArgMatches<'static> {
    App::new("Message of the Day Generator")
        .author("Kevin Allison <kvnallsn@gmail.com>")
        .about("Create custom and detailed MotDs from template files")
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets verbosity level, up to -vvvv for everything"),
        )
        .get_matches()
}

/// Configures the log level to show for debugging purposes
///
/// # Arguments
///
/// * `level` - The logging level to filter at (will only show level and below)
fn configure_logging(level: LevelFilter) {
    let res = fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                record.target(),
                record.level(),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        .apply();

    if let Err(e) = res {
        error!("Failed to configure logging: {:?}", e);
    }
}

fn main() {
    let args = get_arguments();

    let log_level = match args.occurrences_of("v") {
        0 => LevelFilter::Off,
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        4 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    configure_logging(log_level);

    let motd = motd::Motd::new();
    let template = motd.render("templates/falcon.motd").unwrap();

    println!("{}", template);
}
