use chrono::{TimeZone, Utc};
use chrono_tz::Tz;
use regex::Regex;

pub trait Parser {
    fn parse(&self, s: &str) -> String;
}

pub struct UTCParser {
    target_tz: Tz,
}

static SUPPORTED_FORMATS: &'static [&'static str] = &[
    "%Y-%m-%d %H:%M:%S.%6f",
    "%d/%b/%Y:%H:%M:%S %z",     // 04/Nov/2018:12:13:49
    "%Y-%m-%dT%H:%M:%S%:z",     // "2014-11-28T12:00:09+00:00"
    "%a, %d %b %Y %H:%M:%S %z", // "Fri, 28 Nov 2014 12:00:09 +0000 I | some log"
];

lazy_static! {
    static ref COMPILED_REGEX: Vec<Regex> = vec![
        Regex::new(r"(\d{4}-\d{1,2}-\d{1,2} \d{1,2}:\d{1,2}:\d{1,2}\.\d{6})").unwrap(),
        Regex::new(r"(\d{2}/\w+/\d{4}:\d{1,2}:\d{1,2}:\d{1,2} \+\d{4})").unwrap(),
        Regex::new(r"(\d{4}-\d{1,2}-\d{1,2}T\d{1,2}:\d{1,2}:\d{1,2}\+\d{2}:\d{2})").unwrap(),
        Regex::new(r"(\w+, \d{1,2} \w+ \d{4} \d{1,2}:\d{1,2}:\d{1,2} \+\d{4})").unwrap(),
    ];
}

pub fn new_utcparser(target_tz: &str) -> UTCParser {
    return UTCParser {
        target_tz: target_tz.parse().unwrap(),
    };
}

impl Parser for UTCParser {
    fn parse(&self, s: &str) -> String {
        let original_str = String::from(s);
        let mut index = 0;
        for re in COMPILED_REGEX.iter() {
            match re.find(s) {
                Some(found) => {
                    let source_datetime = found.as_str();
                    let format = SUPPORTED_FORMATS[index];
                    let datetime = match Utc.datetime_from_str(source_datetime, format) {
                        Ok(dt) => dt,
                        Err(err) => {
                            println!("{}", err);
                            return original_str;
                        }
                    };

                    let target_datetime = datetime
                        .with_timezone(&self.target_tz)
                        .format(format)
                        .to_string();

                    return s.replace(source_datetime, &target_datetime);
                }
                None => {
                    index += 1;
                    continue;
                }
            };
        }

        return original_str;
    }
}

#[cfg(test)]
mod parser_tests {

    use parser::Parser;

    #[test]
    fn test_parse() {
        let p = super::new_utcparser("Asia/Kolkata");

        struct TestCase {
            input: String,
            output: String,
        }

        let testcases = vec![
            TestCase {
                input: String::from("x.x.x.x - [x.x.x.x] - - [04/Nov/2018:12:13:49 +0000] \"PUT /some/path/url HTTP/2.0\" 200 0 \"-\" \"okhttp/3.10.0\" 478 0.004 [default-80] 10.32.34.208:8000 0 0.004 200"),
                output: String::from("x.x.x.x - [x.x.x.x] - - [04/Nov/2018:17:43:49 +0530] \"PUT /some/path/url HTTP/2.0\" 200 0 \"-\" \"okhttp/3.10.0\" 478 0.004 [default-80] 10.32.34.208:8000 0 0.004 200"),
            },
            TestCase {
                input: String::from("2018-11-03 12:19:36.361297 I | some log"),
                output: String::from("2018-11-03 17:49:36.361297 I | some log"),
            },
            TestCase {
                input: String::from("2014-11-28T12:00:09+00:00 I | some log"),
                output: String::from("2014-11-28T17:30:09+05:30 I | some log"),
            },
            TestCase {
                input: String::from("Fri, 28 Nov 2014 12:00:09 +0000 I | some log"),
                output: String::from("Fri, 28 Nov 2014 17:30:09 +0530 I | some log"),
            },
        ];

        for t in testcases.iter() {
            let actual = p.parse(&(t.input));

            assert_eq!(actual, t.output);
        }
    }
}
