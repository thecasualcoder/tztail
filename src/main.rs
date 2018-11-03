#[macro_use]
extern crate clap;

mod parser;

use clap::{App, AppSettings, Arg};
use parser::{Parser, UTCParser};
use std::io;
use std::io::BufRead;

fn run(timezone: &str) {
    let stdin = io::stdin();

    let p = UTCParser {
        target_tz: String::from(timezone),
    };

    for line in stdin.lock().lines() {
        match line {
            Ok(content) => println!("{}", p.parse(&content)),
            Err(err) => println!("{}", err),
        }
    }
}

fn main() {
    let app = App::new(crate_name!())
        .setting(AppSettings::ColorAuto)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::UnifiedHelpMessage)
        .version(crate_version!())
        .about("tztail (TimeZoneTAIL) allows you to view logs in the timezone you want")
        .arg(
            Arg::with_name("timezone")
                .long("timezone")
                .short("t")
                .value_name("TIMEZONE")
                .required(true)
                .takes_value(true)
                .help("Sets the timezone in which output should be printed"),
        );

    let matches = app.get_matches();
    let timezone = matches.value_of("timezone").expect("Please pass timezone");

    run(timezone);
}
