use regex::{Match, Regex};

// Format holds a format and the regex to capture the format from
// a string. It also hold information on if its timezone aware format
pub struct Format {
    fmt: String,
    re: Regex,
    timezone_aware: bool,
}

impl Format {
    // Proxies request to re.find
    pub fn find<'a>(&self, input: &'a str) -> Option<Match<'a>> {
        return self.re.find(input);
    }

    // Getter for timezone aware
    pub fn is_timezone_aware(&self) -> bool {
        return self.timezone_aware;
    }

    // Getter for fmt
    pub fn fmt(&self) -> &str {
        return &self.fmt;
    }

    // To create a new Format type from a format string
    pub fn new(fmt: &str) -> Format {
        let re = create_re(fmt);

        return Format {
            fmt: String::from(fmt),
            re: re,
            timezone_aware: format_has_timezone(fmt),
        };
    }
}

// Utility function to create and compile a regex for the given format.
fn create_re(format: &str) -> Regex {
    let format_str = String::from(format);
    let regex_str = FORMAT_TO_REGEX
        .iter()
        .fold(format_str, |acc, item| acc.replace(item.0, item.1));
    return Regex::new(&regex_str).unwrap();
}

// Checks if the given format has any timezone specifiers
fn format_has_timezone(fmt: &str) -> bool {
    const TIMEZONE_SPECIFIERS: [&str; 4] = ["%Z", "%z", "%:z", "%#z"];
    for spec in TIMEZONE_SPECIFIERS.iter() {
        if fmt.contains(spec) {
            return true;
        }
    }
    return false;
}

const FORMAT_TO_REGEX: [(&'static str, &'static str); 51] = [
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
    ("%z", r"[\+\-]\d{4}"),
    ("%:z", r"\+\d{2}:\d{2}"),
    ("%#z", r"\+\d{2,4}"),
    //Date & Time Specifiers
    ("%c", r"\w{3} \w{3} \d+ \d{2}:\d{2}:\d{2} \d{4}"),
    (
        "%+",
        r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}.\d+\+\d{2}:\d{2}",
    ),
    ("%s", r"\d+"),
];

#[cfg(test)]
mod format_tests {
    use format::Format;
    #[test]
    fn test_new() {
        let fmt = Format::new("%Y-%m-%d %H:%M:%S");

        assert!(!fmt.is_timezone_aware());

        match fmt.re.find("2019-08-08 10:20:24") {
            Some(found) => {
                assert_eq!(found.as_str(), "2019-08-08 10:20:24");
                assert_eq!(found.start(), 0);
                assert_eq!(found.end(), "2019-08-08 10:20:24".len());
            }
            None => assert!(false),
        }

        match fmt.re.find("20190-08-08 10:20:24") {
            Some(found) => {
                assert_eq!(found.start(), 1);
            }
            None => assert!(false),
        }

        match fmt.re.find("some random string") {
            Some(_) => assert!(false),
            None => assert!(true),
        }
    }

    #[test]
    fn test_new_with_timezone() {
        let fmt = Format::new("%Y-%m-%d %H:%M:%S %z");

        assert!(fmt.is_timezone_aware());
        let valid_str = "2019-08-08 10:20:24 +0000";
        match fmt.re.find(valid_str) {
            Some(found) => {
                assert_eq!(found.as_str(), valid_str);
                assert_eq!(found.start(), 0);
                assert_eq!(found.end(), valid_str.len());
            }
            None => assert!(false),
        }

        let fmt = Format::new("%Y-%m-%d %H:%M:%S %Z");
        assert!(fmt.timezone_aware);
        match fmt.re.find("2019-08-08 10:20:24 IST") {
            Some(found) => {
                assert_eq!(found.as_str(), "2019-08-08 10:20:24 IST");
            }
            None => assert!(false),
        }
    }
}
