use colored::*;
use converter::TimedLog;

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
