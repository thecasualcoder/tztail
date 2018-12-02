use atty::Stream;
use clap::ArgMatches;
use output_formatter::OutputFormatter;

type Result<T> = ::std::result::Result<T, String>;

// To represent command line arguments
pub struct Args<'a> {
    pub filename: Option<&'a str>,
    pub custom_format: Option<&'a str>,
    pub timezone: Option<&'a str>,
    pub should_follow: bool,
    pub color_choice: ColorChoice,
}

impl<'a> Args<'a> {
    // Parses ArgMatches into Args
    pub fn parse(matches: &'a ArgMatches) -> Result<Args<'a>> {
        Ok(Args {
            filename: matches.value_of("FILE"),
            custom_format: matches.value_of("format"),
            timezone: matches.value_of("timezone"),
            should_follow: matches.is_present("follow"),
            color_choice: ColorChoice::new(matches.value_of("color")),
        })
    }
}

// ColorChoice can be made from the command line
// Auto is to decide automatically. If auto is selected, and stdout is a tty, it is colored else it is not.
// Always to force using color
// Never to force not using color
pub enum ColorChoice {
    Auto,
    Always,
    Never,
}

impl ColorChoice {
    // Instantiate ColorChoice based on cli option chosen
    fn new(choice: Option<&str>) -> ColorChoice {
        match choice {
            Some("always") => ColorChoice::Always,
            Some("never") => ColorChoice::Never,
            _ => ColorChoice::Auto,
        }
    }

    // Builder for output formatter from a ColorChoice
    pub fn build_formatter(&self) -> OutputFormatter {
        match self {
            ColorChoice::Auto => {
                return if atty::is(Stream::Stdout) {
                    OutputFormatter::colored()
                } else {
                    OutputFormatter::plain()
                }
            }
            ColorChoice::Never => OutputFormatter::plain(),
            ColorChoice::Always => OutputFormatter::colored(),
        }
    }
}
