use chrono::prelude::*;
use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Tz;
use format::Format;
use std::vec::Vec;

// Converter can be used to convert all the datetimes present in a single line
//
// timezone represents the target timezone in which output should be printed.
// formats are the list of all formats the log is evaluated against
pub struct Converter {
    formats: Vec<Format>,
    timezone: Option<Tz>,
    local: DateTime<Local>,
}

// The default auto-detectable formats supported.
// Add standard formats here
// They get converted into Regexes and are validated
const DEFAULT_FORMATS: &[&str] = &[
    "%Y-%m-%dT%H:%M:%S%z",      // 2014-11-28T12:00:09+0000
    "%Y-%m-%d %H:%M:%S%z",      // 2014-11-28 12:00:09+0000
    "%Y-%m-%dT%H:%M:%S %z",     // 2014-11-28T12:00:09 +0000
    "%Y-%m-%d %H:%M:%S %z",     // 2014-11-28 12:00:09 +0000
    "%d/%b/%Y:%H:%M:%S %z",     // 04/Nov/2018:12:13:49 +0000 Nginx
    "%d/%b/%Y:%H:%M:%S%.3f %z", // 04/Nov/2018:12:13:49.334 +0000 Nginx
    "%d/%b/%Y:%H:%M:%S",        // 04/Nov/2018:12:13:49 HAProxy
    "%a, %d %b %Y %H:%M:%S %z", // Fri, 28 Nov 2014 12:00:09 +0000
    "%Y-%m-%dT%H:%M:%SZ",       // 2014-11-28T12:00:09Z
    "%Y-%m-%dT%H:%M:%S",        // 2014-11-28T12:00:09
    "%Y-%m-%d %H:%M:%S",        // 2014-11-28 12:00:09
];

// Parses timezone from a string
fn parse_timezone(tz_str: &str) -> Option<Tz> {
    match tz_str.parse() {
        Ok(tz) => return Some(tz),
        Err(err) => {
            eprintln!(
                "Using local timezone as given timezone is not valid: {}",
                err
            );
            return None;
        }
    };
}

impl Converter {
    // Public method to create a new Converter
    // Takes in two optional paramters
    //
    // 1. Timezone
    // 2. Fmt
    //
    // If `timezone` is not specified, the system's local timezone is used.
    // If `fmt` is not specified, the autodetectable default formats are used.
    pub fn new(tz_str: Option<&str>, fmt: Option<&str>) -> Result<Converter, String> {
        let timezone = match tz_str {
            Some(timezone) => parse_timezone(timezone),
            None => None,
        };

        let formats = match fmt {
            Some(fmt) => vec![Format::new(fmt)],
            None => DEFAULT_FORMATS.iter().map(|f| Format::new(f)).collect(),
        };

        Ok(Converter {
            formats: formats,
            timezone: timezone,
            local: Local::now(),
        })
    }

    // The method that converts a given string into the target timezone
    // It also tries to "detect" the source timezone if available, or it will assume UTC
    // TODO: the formats are looped sequentially. Use RegexSet to parallely match all expressions
    // TODO: If there is a hit in autodetected formats, prioritize it
    // TODO: Allow target timezone configurable
    pub fn convert(&self, input: &str) -> String {
        let original_str = String::from(input);
        for format in &self.formats {
            match format.find(input) {
                Some(found) => {
                    let source_datetime = found.as_str();

                    if format.is_timezone_aware() {
                        let dt = match DateTime::parse_from_str(source_datetime, format.fmt()) {
                            Ok(dt) => dt,
                            Err(err) => {
                                eprintln!(
                                    "Error when parsing from string that is datetime aware: {}",
                                    err
                                );
                                return original_str;
                            }
                        };

                        let target_datetime = match self.timezone {
                            Some(tz) => dt.with_timezone(&tz).format(format.fmt()).to_string(),
                            None => dt
                                .with_timezone(&self.local.timezone())
                                .format(format.fmt())
                                .to_string(),
                        };

                        return input.replace(source_datetime, &target_datetime);
                    } else {
                        let dt = match Utc.datetime_from_str(source_datetime, format.fmt()) {
                            Ok(dt) => dt,
                            Err(err) => {
                                eprintln!("Error when parsing using UTC: {}", err);
                                return original_str;
                            }
                        };

                        let target_datetime = match self.timezone {
                            Some(tz) => dt.with_timezone(&tz).format(format.fmt()).to_string(),
                            None => dt
                                .with_timezone(&self.local.timezone())
                                .format(format.fmt())
                                .to_string(),
                        };
                        return input.replace(source_datetime, &target_datetime);
                    }
                }
                None => {
                    continue;
                }
            }
        }
        return original_str;
    }
}

// A function to test various formats
fn _chrono(input: &str) -> String {
    let dt = match DateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S %z") {
        Ok(dt) => dt,
        Err(err) => {
            println!("Error: {}", err);
            return String::from(input);
        }
    };
    let local: DateTime<Local> = Local::now();
    let tz = local.timezone();
    return dt
        .with_timezone(&tz)
        .format("%Y-%m-%d %H:%M:%S %z")
        .to_string();
}

#[cfg(test)]
mod converter_tests {
    use chrono::DateTime;

    #[test]
    fn test_new() {
        match super::Converter::new(Some("Random/str"), None) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        };

        match super::Converter::new(Some("Asia/Kolkata"), None) {
            Ok(c) => {
                assert!(true);
                assert_eq!(c.formats.len(), super::DEFAULT_FORMATS.len());
            }
            Err(_) => assert!(false),
        };

        match super::Converter::new(Some("Asia/Kolkata"), Some("%Y-%m-%d %H:%M:%S %z")) {
            Ok(c) => {
                assert!(true);
                assert_eq!(c.formats.len(), 1);
            }
            Err(_) => assert!(false),
        };
    }

    #[test]
    fn test_convert() {
        struct TestCase<'a> {
            timezone: Option<&'a str>,
            format: Option<&'a str>,
            inputs: Vec<&'a str>,
            outputs: Vec<&'a str>,
        };

        fn convert_utc_to_localtimezone(input: &str, format: &str) -> String {
            use chrono::TimeZone;

            let local_timezone = super::Local::now().timezone();
            return super::Utc
                .datetime_from_str(input, format)
                .unwrap()
                .with_timezone(&local_timezone)
                .format(format)
                .to_string();
        };

        fn convert_to_localtimezone(input: &str, format: &str) -> String {
            let local_timezone = super::Local::now().timezone();
            return DateTime::parse_from_str(input, format)
                .unwrap()
                .with_timezone(&local_timezone)
                .format(format)
                .to_string();
        };

        let local_timezone_case_1 =
            convert_utc_to_localtimezone("2002-10-02 15:00:00", "%Y-%m-%d %H:%M:%S");
        let local_timezone_case_2 =
            convert_to_localtimezone("2012-07-24T23:14:29-0700", "%Y-%m-%dT%H:%M:%S%z");

        let testcases = vec![
            TestCase {
                timezone: Some("Asia/Kolkata"),
                format: Some("%Y-%m-%d %H:%M:%S %z"),
                inputs: vec![
                    "2018-08-08 10:32:15 +0000",
                    "2018-03-03 10:32:15 +0700",
                    "2018-08-08 10:32:15 -0200",
                ],
                outputs: vec![
                    "2018-08-08 16:02:15 +0530",
                    "2018-03-03 09:02:15 +0530",
                    "2018-08-08 18:02:15 +0530",
                ],
            },
            TestCase {
                timezone: Some("Asia/Kolkata"),
                format: Some("%Y-%m-%d %H:%M:%S"),
                inputs: vec!["2018-11-03 22:39:33 Some random log"],
                outputs: vec!["2018-11-04 04:09:33 Some random log"],
            },
            TestCase {
                timezone: Some("Europe/Paris"),
                format: None,
                inputs: vec![
                    "Fri, 28 Nov 2014 12:00:09 +0000",
                    "Thu, 27 Nov 2014 01:00:09 +0530",
                    "14/Nov/2018:22:14:27 -0800",
                    "2014-11-28T12:00:09+0500",
                    "2014-11-28 12:00:09+0500",
                    "2014-11-28T12:00:09 +0500",
                    "2014-11-28 12:00:09 +0500",
                    "04/Nov/2018:12:13:49 +0500 Nginx",
                    "04/Nov/2018:12:13:49.334 +0500 Nginx",
                    "04/Nov/2018:12:13:49 HAProxy",
                ],
                outputs: vec![
                    "Fri, 28 Nov 2014 13:00:09 +0100",
                    "Wed, 26 Nov 2014 20:30:09 +0100",
                    "15/Nov/2018:07:14:27 +0100",
                    "2014-11-28T08:00:09+0100",
                    "2014-11-28 08:00:09+0100",
                    "2014-11-28T08:00:09 +0100",
                    "2014-11-28 08:00:09 +0100",
                    "04/Nov/2018:08:13:49 +0100 Nginx",
                    "04/Nov/2018:08:13:49.334 +0100 Nginx",
                    "04/Nov/2018:13:13:49 HAProxy",
                ],
            },
            TestCase {
                timezone: None,
                format: None,
                inputs: vec!["2002-10-02 15:00:00", "2012-07-24T23:14:29-0700"],
                outputs: vec![&local_timezone_case_1, &local_timezone_case_2],
            },
        ];

        for test in testcases {
            let converter = match super::Converter::new(test.timezone, test.format) {
                Ok(c) => c,
                Err(_) => {
                    assert!(false);
                    return;
                }
            };

            for i in 0..test.inputs.len() {
                let input = test.inputs[i];
                let expected_output = test.outputs[i];

                let output = converter.convert(input);

                assert_eq!(output, expected_output);
            }
        }
    }

    // #[test]
    // fn test_chrono() {
    //     let input = "2018-08-08 10:10:10 +0000";
    //     let output = super::chrono(input);
    //     let expected_output = "2018-08-08 14:40:10 +05300";

    //     assert_eq!(output, expected_output)
    // }
}
