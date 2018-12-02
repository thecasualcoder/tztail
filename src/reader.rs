use std::fs::File;
use std::io::{self, BufRead, BufReader, Stdin};

// Abstraction to read from source
// first_line is read first.
// This can be used to further optimization if needed like autodetecting the format before starting conversion.
pub struct InputReader<'a> {
    reader: Box<dyn BufRead + 'a>,
    pub first_line: String,
}

// Input can either be a single file or STDIN
pub enum Input<'a> {
    Stdin(&'a Stdin),
    File(&'a str),
}

impl<'a> InputReader<'a> {
    // Instantiate InputReader based on the type of Input
    pub fn new(input: Input) -> io::Result<InputReader> {
        match input {
            Input::Stdin(stdin) => {
                let mut reader = stdin.lock();

                let mut first_line = String::new();
                reader.read_line(&mut first_line)?;

                Ok(InputReader {
                    reader: Box::new(reader),
                    first_line: first_line,
                })
            }
            Input::File(filename) => {
                let file = File::open(filename)?;
                let mut reader = BufReader::new(file);

                let mut first_line = String::new();
                reader.read_line(&mut first_line)?;

                Ok(InputReader {
                    reader: Box::new(reader),
                    first_line: first_line,
                })
            }
        }
    }

    // Proxy call for read_line of the underling BufRead
    pub fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
        self.reader.read_line(buf)
    }

    // Getter for first_line
    pub fn first_line(&self) -> &str {
        &self.first_line
    }
}
