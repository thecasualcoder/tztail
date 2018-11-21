extern crate assert_cmd;
extern crate escargot;
#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate chrono_tz;

use assert_cmd::prelude::*;
use chrono::prelude::*;
use chrono::DateTime;

use escargot::CargoRun;
use std::process::Command;

lazy_static! {
    static ref CARGO_RUN: CargoRun = escargot::CargoBuild::new()
        .bin("tztail")
        .current_release()
        .run()
        .unwrap();
}

fn tztail() -> Command {
    CARGO_RUN.command()
}

fn convert_to_localtimezone(input: &str, format: &str) -> String {
    let local_timezone = Local::now().timezone();
    return DateTime::parse_from_str(input, format)
        .unwrap()
        .with_timezone(&local_timezone)
        .format(format)
        .to_string();
}

#[test]
fn test_stdin() {
    let remote_time = "19/Nov/2018:15:59:47 +0000";
    let local_time = convert_to_localtimezone(remote_time, "%d/%b/%Y:%H:%M:%S %z");
    tztail()
        .with_stdin()
        .buffer(format!("[{}] PUT /some/nginxurl", remote_time))
        .assert()
        .success()
        .stdout(format!("[{}] PUT /some/nginxurl\n", local_time))
        .stderr("");
}

#[test]
fn test_target_timezone() {
    tztail()
        .arg("--timezone")
        .arg("US/Pacific")
        .with_stdin()
        .buffer("2018-11-21T22:48:14+0100: This is a log")
        .assert()
        .success()
        .stdout("2018-11-21T13:48:14-0800: This is a log\n")
        .stderr("");
}

#[test]
fn test_read_from_file() {
    let local_time = convert_to_localtimezone("2018-11-21T17:26:30+0700", "%Y-%m-%dT%H:%M:%S%z");
    tztail()
        .arg("tests/inputs/test_read_from_file.txt")
        .assert()
        .success()
        .stdout(format!("{} postgres parameters not changed\n", local_time))
        .stderr("");
}

#[test]
fn test_custom_format() {
    let remote_time = "2018/11/21 22:48:14 +03:00";
    let local_time = convert_to_localtimezone(remote_time, "%Y/%m/%d %H:%M:%S %:z");
    tztail()
        .arg("--format")
        .arg("%Y/%m/%d %H:%M:%S %:z")
        .with_stdin()
        .buffer("2018/11/21 22:48:14 +03:00: This is a custom formatted log")
        .assert()
        .success()
        .stdout(format!("{}: This is a custom formatted log\n", local_time))
        .stderr("");
}

#[test]
fn test_default_utc() {
    tztail()
        .arg("-t")
        .arg("Asia/Kolkata")
        .arg("--format")
        .arg("%Y/%m/%dT%H:%M:%S")
        .with_stdin()
        .buffer("2018/11/21T22:48:14: This is a log in UTC")
        .assert()
        .success()
        .stdout("2018/11/22T04:18:14: This is a log in UTC\n")
        .stderr("");
}
