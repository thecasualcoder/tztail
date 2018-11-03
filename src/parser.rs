extern crate chrono;
extern crate chrono_tz;
extern crate regex;

use self::chrono::{TimeZone, Utc};
use self::chrono_tz::Tz;
use self::regex::Regex;

pub trait Parser {
    fn parse(&self, s: &String) -> String;
}

pub struct UTCParser {
    pub target_tz: String,
}

impl Parser for UTCParser {
    fn parse(&self, s: &String) -> String {
        let format = "%Y-%m-%d %H:%M:%S.%6f";
        let target_timezone: Tz = self.target_tz.parse().unwrap();
        let (output, time) = pluck_time_from_log(s);

        let source_datetime = Utc
            .datetime_from_str(&time, format)
            .expect("Not able to parse time");

        let target_datetime = source_datetime
            .with_timezone(&target_timezone)
            .format(format)
            .to_string();

        return output.replace("{}", &target_datetime);
    }
}

fn pluck_time_from_log(s: &String) -> (String, String) {
    //2018-11-03 12:19:36.359337
    let re = Regex::new(r"(\d{4}-\d{1,2}-\d{1,2} \d{1,2}:\d{1,2}:\d{1,2}\.?\d+)").unwrap();
    let time = match re.captures(&s) {
        Some(capture) => capture.get(0).map_or("", |m| m.as_str()),
        None => "",
    };
    let replaced = re.replace(&s, "{}");

    return (replaced.into_owned(), String::from(time));
}

#[cfg(test)]
mod parser_tests {
    use parser::Parser;
    #[test]
    fn test_parse() {
        let p = super::UTCParser {
            target_tz: String::from("Asia/Kolkata"),
        };

        struct TestCase {
            input: String,
            output: String,
        }

        let testcases = vec![
            // TestCase {
            //     input: String::from("2018-11-03 12:19:36 I | some log"),
            //     output: String::from("2018-11-03 17:49:36 I | some log"),
            // },
            TestCase {
                input: String::from("2018-11-03 12:19:36.361297 I | some log"),
                output: String::from("2018-11-03 17:49:36.361297 I | some log"),
            },
        ];

        for t in testcases.iter() {
            let actual = p.parse(&(t.input));

            assert_eq!(actual, t.output);
        }
    }

    #[test]
    fn test_pluck_time_from_log() {
        let log = String::from("2018-11-03 12:19:36.361297 I | some log");

        let (actual_log, actual_time) = super::pluck_time_from_log(&log);
        let (expected_log, expected_time) = (
            String::from("{} I | some log"),
            String::from("2018-11-03 12:19:36.361297"),
        );

        assert_eq!(actual_log, expected_log);
        assert_eq!(actual_time, expected_time);
    }

    #[test]
    fn test_pluck_time_from_log_for_failing_cases() {
        let log = String::from("2018-11-03 I | some log");

        let (actual_log, actual_time) = super::pluck_time_from_log(&log);
        let (expected_log, expected_time) =
            (String::from("2018-11-03 I | some log"), String::from(""));

        assert_eq!(actual_log, expected_log);
        assert_eq!(actual_time, expected_time);
    }
}
