use colored::*;
use converter::TimedLog;

// OutputFormatter can either format the target time as a colored
// string or a plain string based on a flag
pub struct OutputFormatter {
    colored: bool,
}

impl OutputFormatter {
    pub fn plain() -> OutputFormatter {
        OutputFormatter { colored: false }
    }

    pub fn colored() -> OutputFormatter {
        OutputFormatter { colored: true }
    }

    // format does a string replace of the original_time in log
    // TODO: Is string replace slow? Is there a better way?
    pub fn format(&self, t: TimedLog) -> String {
        if t.converted {
            let target_time = if self.colored {
                format!("{}", t.target_time.unwrap().red())
            } else {
                t.target_time.unwrap()
            };

            return t.log.replace(&t.original_time.unwrap(), &target_time);
        }

        String::from(t.log)
    }
}
