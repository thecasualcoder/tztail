use std::fs::File;
use std::io::{self, BufRead, BufReader, Stdin};

pub struct InputReader<'a> {
    reader: Box<dyn BufRead + 'a>,
    pub first_line: String,
}

pub enum Input<'a> {
    Stdin(&'a Stdin),
    File(&'a str),
}

impl<'a> InputReader<'a> {
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

    pub fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
        self.reader.read_line(buf)
    }

    pub fn first_line(&self) -> &str {
        &self.first_line
    }
}
