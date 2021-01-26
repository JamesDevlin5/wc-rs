use std::cmp;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug)]
pub struct Count {
    lines: usize,
    words: usize,
    chars: usize,
    bytes: usize,
    line_len: usize,
}

impl Count {
    pub fn new() -> Self {
        Count {
            lines: 0,
            words: 0,
            chars: 0,
            bytes: 0,
            line_len: 0,
        }
    }
}

impl Count {
    pub fn process<R: Read>(&mut self, reader: R) {
        let mut buf = BufReader::new(reader);
        let mut line = String::new();

        loop {
            line.clear();
            let amt_read = buf.read_line(&mut line).expect("Could not read line...");
            if amt_read > 0 {
                self.lines += 1;
                self.bytes += amt_read;
                self.line_len = cmp::max(self.line_len, line.len());
                self.words += line.split_whitespace().count();
                self.chars += line.chars().count();
            } else {
                break;
            }
        }
    }
}
