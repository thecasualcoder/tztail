#[macro_use]
extern crate clap;
extern crate chrono;
extern crate chrono_tz;
#[macro_use(lazy_static)]
extern crate lazy_static;
extern crate regex;

mod parser;

use clap::{App, AppSettings, Arg};
use parser::Parser;
use std::io;
use std::io::BufRead;

fn run<P: Parser>(p: P) {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        match line {
            Ok(content) => println!("{}", p.parse(&content)),
            Err(err) => eprintln!("{}", err),
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
        ).arg(
            Arg::with_name("format")
                .long("format")
                .short("f")
                .value_name("FORMAT")
                .required(false)
                .takes_value(true)
                .help("Custom format for parsing dates"),
        );

    let matches = app.get_matches();
    let timezone = matches.value_of("timezone").expect("Please pass timezone");
    let custom_format = matches.value_of("format");

    match custom_format {
        Some(format) => run(parser::new_fixedformatutcparser(timezone, format)),
        None => run(parser::new_utcparser(timezone)),
    };
}
