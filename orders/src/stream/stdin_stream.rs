use crate::stream::*;
use crate::stream::buffered_stream::*;

use std::io::{Stdin, BufRead, BufReader};

/// A simple wrapper for the
/// std::io::Stdin.
pub struct StdinStream {
    /// Number of read values.
    pub offset: usize,
    /// The stream that provides the data.
    pub backend: BufReader<Stdin>,
    /// Buffer for decoding UTF-8.
    pub buffer: Vec<char>,
    /// next buffer item
    pub next: usize,
}

impl StdinStream {
    pub fn new() -> StdinStream {
        return StdinStream {
            offset: 0,
            backend: BufReader::new(std::io::stdin()),
            buffer: vec![],
            next: 0,
        }
    }

    fn can_read_more(&mut self) -> bool {
        if self.next >= self.buffer.len() {
            let mut buffer = vec![];

            if let Ok(_) = self.backend.read_until(b'\n', &mut buffer) {
                self.offset += self.buffer.len();
                self.buffer = String::from_utf8_lossy(&buffer).replace('\r', "").chars().collect();
                self.next = 0;
            }
        }

        return self.next < self.buffer.len();
    }
}

impl Stream<Option<char>> for StdinStream {
    fn get_end_value(&self) -> Option<char> {
        return None;
    }

    fn peek(&mut self) -> Option<char> {
        if self.can_read_more() {
            return Some(self.buffer[self.next]);
        }

        return None;
    }

    fn step(&mut self) {
        if self.can_read_more() {
            self.next += 1;
            // ensure the index is within the
            // buffer size or, at least, 0
            self.can_read_more();
        }
    }

    fn get_offset(&self) -> usize {
        println!("Cache: {:?}, next: {}", self.buffer, self.next);
        return self.next + self.offset;
    }
}

impl BufferedStream<Option<char>> for StdinStream {
    fn lookahead(&self, position: usize) -> Option<char> {
        if self.next + position < self.buffer.len() {
            Some(self.buffer[self.next + position])
        } else {
            None
        }
    }

    fn get_buffer(&self) -> Vec<Option<char>> {
        return self.buffer.iter().map(|it| Some(it.clone())).collect();
    }
}
