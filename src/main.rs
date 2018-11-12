#[macro_use]
extern crate clap;
extern crate chrono;
extern crate chrono_tz;
extern crate hourglass;
extern crate regex;
mod converter;
mod format;

use clap::{App, AppSettings, Arg};
use converter::Converter;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::process;

struct AppArgs<'a> {
    filename: Option<&'a str>,
    custom_format: Option<&'a str>,
    timezone: Option<&'a str>,
    should_follow: bool,
}

fn run(args: AppArgs) -> Result<bool, String> {
    let AppArgs {
        filename,
        custom_format: fmt,
        timezone: tz,
        should_follow: follow
    } = args;

    let c = Converter::new(tz, fmt)?;
    let stdin = io::stdin();

    let mut reader: Box<dyn BufRead> = match filename {
        Some(name) => {
            let file = File::open(name).expect("Error opening file");
            Box::new(BufReader::new(file))
        }
        None => Box::new(stdin.lock()),
    };

    if follow {
        let mut buf = String::new();

        loop {
            match reader.read_line(&mut buf) {
                Ok(bytes) if bytes > 0 => {
                    println!("{}", c.convert(&buf.trim_end()));
                    buf.clear();
                },
                Ok(_) => (),
                Err(err) => return handle_err(err)
            }
        }
    } else {
        for line in reader.lines() {
            match line {
                Ok(content) => println!("{}", c.convert(&content)),
                Err(err) => return handle_err(err)
            }
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
        );

    let matches = app.get_matches();
    let args = AppArgs {
        filename: matches.value_of("FILE"),
        custom_format: matches.value_of("format"),
        timezone: matches.value_of("timezone"),
        should_follow: matches.is_present("follow")
    };

    let result = run(args);

    match result {
        Err(error) => {
            eprintln!("{}: {}", "Exited non-successfully", error);
            process::exit(1);
        }
        Ok(false) => process::exit(1),
        Ok(true) => process::exit(0),
    }
}
