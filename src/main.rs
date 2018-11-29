#[macro_use]
extern crate clap;
extern crate atty;
extern crate chrono;
extern crate chrono_tz;
extern crate colored;
extern crate regex;
mod args;
mod converter;
mod format;
mod output_formatter;
mod reader;

use args::Args;
use clap::{App, AppSettings, Arg};
use converter::Converter;
use reader::*;
use std::error::Error;
use std::io;
use std::io::Write;
use std::process;

fn run(args: Args) -> Result<bool, String> {
    let Args {
        filename,
        custom_format: fmt,
        timezone: tz,
        should_follow: follow,
        color_choice,
    } = args;

    let c = Converter::new(tz, fmt)?;
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut writer = stdout.lock();

    let reader = match filename {
        Some("-") => InputReader::new(Input::Stdin(&stdin)),
        Some(name) => InputReader::new(Input::File(&name)),
        None => InputReader::new(Input::Stdin(&stdin)),
    };

    let mut reader = match reader {
        Ok(r) => r,
        Err(err) => return handle_err(err),
    };

    let mut has_next = true;
    let mut buf = String::new();

    let formatter = color_choice.build_formatter();

    match write!(
        writer,
        "{}",
        formatter.format(c.convert(reader.first_line()))
    ) {
        Ok(_) => (),
        Err(err) => return handle_err(err),
    };

    while follow || has_next {
        match reader.read_line(&mut buf) {
            Ok(bytes) if bytes > 0 => {
                match write!(writer, "{}", formatter.format(c.convert(&buf))) {
                    Ok(_) => (),
                    Err(err) => return handle_err(err),
                }

                buf.clear();
                has_next = true;
            }
            Ok(_) => {
                has_next = false;
            }
            Err(err) => return handle_err(err),
        }
    }

    return Ok(true);
}

fn handle_err(err: std::io::Error) -> Result<bool, String> {
    eprintln!("{}: {}", "Exited while reading lines", err.description());
    return Err(format!("Exited while reading lines: {}", err));
}

fn main() {
    let app = App::new(crate_name!())
        .setting(AppSettings::ColorAuto)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::UnifiedHelpMessage)
        .version(crate_version!())
        .about("tztail (TimeZoneTAIL) allows you to view logs in the timezone you want")
        .arg(Arg::with_name("FILE").help("File to tail. STDIN by default"))
        .arg(
            Arg::with_name("timezone")
                .long("timezone")
                .short("t")
                .value_name("TIMEZONE")
                .required(false)
                .takes_value(true)
                .help("Sets the timezone in which output should be printed. (Default: local timezone)"),
        ).arg(
            Arg::with_name("follow")
                .long("follow")
                .short("f")
                .value_name("FOLLOW")
                .required(false)
                .takes_value(false)
                .help("Follow the file indefinitely as changes are added. (Default: Off)"),
        ).arg(
            Arg::with_name("format")
                .long("format")
                .value_name("FORMAT")
                .required(false)
                .takes_value(true)
                .help("Custom format for parsing dates. (Default: autodetected patterns)")
        ).arg(
            Arg::with_name("color")
                .long("color")
                .value_name("COLOR_CHOICE")
                .possible_values(&["never", "auto", "always"])
                .required(false)
                .help("Controls when to use color")
        );

    let result = Args::parse(&app.get_matches()).and_then(run);

    match result {
        Err(error) => {
            eprintln!("{}: {}", "Exited non-successfully", error);
            process::exit(1);
        }
        Ok(false) => process::exit(1),
        Ok(true) => process::exit(0),
    }
}
