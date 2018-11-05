use chrono::{TimeZone, Utc};
use chrono_tz::Tz;
use regex::Regex;

// Trait for any Parser to implement
pub trait Parser {
    fn parse(&self, s: &str) -> String;
}

// UTCParser can only parse UTC times
// It supports only a limited number of formats.
// It can auto detect a format using regex.
pub struct UTCParser {
    target_tz: Tz,
    re: Vec<Regex>,
}

// Various formats supported by the auto detecing UTCParser
static SUPPORTED_FORMATS: &'static [&'static str] = &[
    "%Y-%m-%d %H:%M:%S.%6f",
    "%d/%b/%Y:%H:%M:%S %z",     // 04/Nov/2018:12:13:49
    "%Y-%m-%dT%H:%M:%S%:z",     // "2014-11-28T12:00:09+00:00"
    "%a, %d %b %Y %H:%M:%S %z", // "Fri, 28 Nov 2014 12:00:09 +0000 I | some log"
];

// Implementation of parse method for UTCParser
// It uses a fixed number of supported formats and auto detects
// the datetime format. It can also parse only UTC timestamp.
// If the datetime is not parsable, it will return the original string
impl Parser for UTCParser {
    fn parse(&self, s: &str) -> String {
        let original_str = String::from(s);
        let mut index = 0;
        for re in self.re.iter() {
            match re.find(s) {
                Some(found) => {
                    let source_datetime = found.as_str();
                    let format = SUPPORTED_FORMATS[index];
                    let datetime = match Utc.datetime_from_str(source_datetime, format) {
                        Ok(dt) => dt,
                        Err(err) => {
                            eprintln!("{}", err);
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

// FixedFormatUTCParser can be used if the format of datetime is known beforehand
pub struct FixedFormatUTCParser {
    target_tz: Tz,
    format: String,
    re: Regex,
}

// Implementation of Parse method for FixedFormatUTCParser
// It reads only UTC based datetime
impl Parser for FixedFormatUTCParser {
    fn parse(&self, s: &str) -> String {
        let original_str = String::from(s);
        match self.re.find(s) {
            Some(found) => {
                let source_datetime = found.as_str();
                let datetime = match Utc.datetime_from_str(source_datetime, &self.format) {
                    Ok(dt) => dt,
                    Err(err) => {
                        eprintln!("{}", err);
                        return original_str;
                    }
                };

                let target_datetime = datetime
                    .with_timezone(&self.target_tz)
                    .format(&self.format)
                    .to_string();

                return s.replace(source_datetime, &target_datetime);
            }
            None => return original_str,
        }
    }
}

// Entire map of Datetime format to their regex
// Its based on format::strftime
// Source: https://docs.rs/chrono/0.4.6/chrono/format/strftime/index.html#specifiers
lazy_static! {
    static ref FORMAT_TO_REGEX: Vec<(&'static str, &'static str)> = vec![
        // Date Specifiers
        ("%Y", r"\d{4}"),
        ("%C", r"\d{2}"),
        ("%y", r"\d{2}"),
        ("%m", r"\d{2}"),
        ("%b", r"\w{3}"),
        ("%B", r"\w+"),
        ("%h", r"\w{3}"),
        ("%d", r"\d{2}"),
        ("%e", r"\d+"),
        ("%e", r"\d+"),
        ("%a", r"\w{3}"),
        ("%A", r"\w+"),
        ("%w", r"\d"),
        ("%u", r"\d"),
        ("%U", r"\d{2}"),
        ("%W", r"\d{2}"),
        ("%G", r"\d{4}"),
        ("%g", r"\d{2}"),
        ("%V", r"\d{2}"),
        ("%j", r"\d{3}"),
        ("%D", r"\d{2}/\d{2}/\d{2}"),
        ("%x", r"\d{2}/\d{2}/\d{2}"),
        ("%F", r"\d{4}-\d{2}-\d{2}"),
        ("%v", r"\d{2}-\w{3}-\d{4}"),
        // Time Specifiers
        ("%H", r"\d{2}"),
        ("%k", r"\d+"),
        ("%I", r"\d{2}"),
        ("%l", r"\d{1,2}"),
        ("%P", r"[ap]m)"),
        ("%p", r"[AP]M)"),
        ("%M", r"\d{2}"),
        ("%S", r"\d{2}"),
        ("%f", r"\d+"),
        ("%3f", r"\d{3}"),
        ("%6f", r"\d{6}"),
        ("%9f", r"\d{9}"),
        ("%.f", r"\.\d+"),
        ("%.3f", r"\.\d{3}"),
        ("%.6f", r"\.\d{6}"),
        ("%.9f", r"\.\d{9}"),
        ("%R", r"\d{2}:\d{2}"),
        ("%T", r"\d{2}:\d{2}:\d{2}"),
        ("%X", r"\d{2}:\d{2}:\d{2}"),
        ("%r", r"\d{2}:\d{2}:\d{2} [AP]M"),
        // Timezone Specifiers
        ("%Z", r"[A-Z]+"),
        ("%z", r"\+\d{4}"),
        ("%:z", r"\+\d{2}:\d{2}"),
        ("%#z", r"\+\d{2,4}"),
        //Date & Time Specifiers
        ("%c", r"\w{3} \w{3} \d+ \d{2}:\d{2}:\d{2} \d{4}"),
        ("%+", r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}.\d+\+\d{2}:\d{2}"),
        ("%s", r"\d+"),
    ];
}

// Utility function to create and compile a regex for the given datetime format.
fn create_re(format: &str) -> Regex {
    let format_str = String::from(format);
    let regex_str = FORMAT_TO_REGEX
        .iter()
        .fold(format_str, |acc, item| acc.replace(item.0, item.1));
    return Regex::new(&regex_str).unwrap();
}

// Public method that creates a UTCParser.
pub fn new_utcparser(target_tz: &str) -> UTCParser {
    let re = SUPPORTED_FORMATS.iter().map(|i| create_re(i)).collect();
    return UTCParser {
        target_tz: target_tz.parse().unwrap(),
        re: re,
    };
}

// Public method that creates a FixedFormatUTCParser.
pub fn new_fixedformatutcparser(target_tz: &str, format: &str) -> FixedFormatUTCParser {
    return FixedFormatUTCParser {
        target_tz: target_tz.parse().unwrap(),
        format: String::from(format),
        re: create_re(format),
    };
}

#[cfg(test)]
mod parser_tests {
    struct TestCase {
        input: String,
        output: String,
    }

    mod utcparser_tests {
        use super::TestCase;
        use parser::Parser;
        #[test]
        fn test_parse() {
            let p = super::super::new_utcparser("Asia/Kolkata");

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

    mod fixedformatutcparser_tests {
        use super::TestCase;
        use parser::Parser;
        #[test]
        fn test_parse() {
            let p = super::super::new_fixedformatutcparser("Asia/Kolkata", "%Y-%m-%d %H:%M:%S");

            let testcases = vec![TestCase {
                input: String::from("service: 2018-11-03 12:19:36 I some log"),
                output: String::from("service: 2018-11-03 17:49:36 I some log"),
            }];

            for t in testcases.iter() {
                let actual = p.parse(&(t.input));

                assert_eq!(actual, t.output);
            }
        }
    }

    #[test]
    fn test_create_re() {
        let re = super::create_re("%Y-%m-%d");
        let valid_input = "2004-08-08";
        let invalid_input = "204-08-08";
        let invalid_input_with_match = "20004-08-08";

        match re.find(valid_input) {
            Some(m) => {
                assert_eq!(m.start(), 0);
                assert_eq!(m.end(), valid_input.len());
            }
            None => {
                assert!(false);
            }
        }

        match re.find(invalid_input) {
            Some(_) => assert!(false),
            None => assert!(true),
        }

        match re.find(invalid_input_with_match) {
            Some(m) => assert_ne!(m.start(), 0),
            None => assert!(false),
        }
    }

}
