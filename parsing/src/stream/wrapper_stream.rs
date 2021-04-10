use crate::stream::*;
use crate::stream::buffered_stream::*;
use crate::stream::text_stream::*;

use std::io::{BufRead};

/// A simple wrapper for any
/// std::io::BufRead.
pub struct WrapperStream<T: BufRead> {
    /// Number of the read values.
    pub offset: usize,
    /// The stream that provides the data.
    pub backend: T,
    /// Buffer for decoding UTF-8.
    pub buffer: Vec<char>,
    /// Next buffer item.
    pub next: usize,
    /// If true, a new line should be
    /// read.
    ///
    /// The old approach was to read
    /// a new line as soon as the old one is
    /// finished (we meet `\n`), so the buffer
    /// would store a new line. But when we work
    /// with user interactive input, this means,
    /// we force the user to enter one more
    /// line in order to finish processing the
    /// previous one at its `\n`.
    ///
    /// The new approach first sets the flag and
    /// only reads the next line if the first
    /// character of it is needed.
    pub should_read: bool,
}

impl <T: BufRead> WrapperStream<T> {
    pub fn new(backend: T) -> WrapperStream<T> {
        return WrapperStream {
            offset: 0,
            backend: backend,
            buffer: vec![],
            next: 0,
            should_read: true,
        }
    }

    fn read_next_line(&mut self) {
        self.next = 0;
        let mut buffer = vec![];

        if let Ok(_) = self.backend.read_until(b'\n', &mut buffer) {
            self.offset += self.buffer.len();
            self.buffer = String::from_utf8_lossy(&buffer).replace('\r', "").chars().collect();
        }

        self.should_read = false;
    }
}

impl <T: BufRead> Stream<Option<char>> for WrapperStream<T> {
    fn has_next(&self) -> bool {
        return self.next < self.buffer.len() || self.should_read;
    }

    fn get_offset(&self) -> usize {
        return self.next + self.offset;
    }

    fn grab(&mut self) -> Option<char> {
        let it = self.peek();
        self.step();
        return it;
    }
}

impl <T: BufRead> PeekableStream<Option<char>> for WrapperStream<T> {
    fn peek(&mut self) -> Option<char> {
        if self.should_read {
            self.read_next_line();
        }

        if self.next >= self.buffer.len() {
            return None;
        }

        return Some(self.buffer[self.next]);
    }

    fn step(&mut self) {
        if self.should_read {
            self.read_next_line();
        }

        if self.next >= self.buffer.len() - 1 {
            self.should_read = true;
        } else {
            self.next += 1;
        }
    }
}

impl <T: BufRead> BufferedStream<Option<char>> for WrapperStream<T> {
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

impl <T: BufRead> TextStream for WrapperStream<T> {
    fn get_text(&self) -> String {
        return self.buffer.iter().collect();
    }

    fn match_text(&self, next: &str) -> usize {
        let mut iterator = next.chars().peekable();

        for it in self.next..self.buffer.len() {
            if let Some(&next) = iterator.peek() {
                if next == self.buffer[it] {
                    iterator.next();
                } else {
                    return 0;
                }
            } else {
                return it - self.next;
            }
        }

        return self.buffer.len() - self.next;
    }
}
