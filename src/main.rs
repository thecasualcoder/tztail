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

fn run(filename: Option<&str>, tz: Option<&str>, fmt: Option<&str>) -> Result<bool, String> {
    let c = Converter::new(tz, fmt)?;
    let stdin = io::stdin();

    let reader: Box<dyn BufRead> = match filename {
        Some(name) => {
            let file = File::open(name).expect("Error opening file");
            Box::new(BufReader::new(file))
        }
        None => Box::new(stdin.lock()),
    };

    for line in reader.lines() {
        match line {
            Ok(content) => println!("{}", c.convert(&content)),
            Err(err) => {
                eprintln!("{}: {}", "Exited while reading lines", err.description());
                return Err(format!("Exited while reading lines: {}", err));
            }
        }
    }

    return Ok(true);
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
            Arg::with_name("format")
                .long("format")
                .short("f")
                .value_name("FORMAT")
                .required(false)
                .takes_value(true)
                .help("Custom format for parsing dates. (Default: autodetected patterns)"),
        );

    let matches = app.get_matches();
    let timezone = matches.value_of("timezone");
    let custom_format = matches.value_of("format");
    let filename = matches.value_of("FILE");

    let result = run(filename, timezone, custom_format);

    match result {
        Err(error) => {
            eprintln!("{}: {}", "Exited non-successfully", error);
            process::exit(1);
        }
        Ok(false) => process::exit(1),
        Ok(true) => process::exit(0),
    }
}
