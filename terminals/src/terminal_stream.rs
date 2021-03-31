use crate::{read_line, has_next};

use parsing::stream::*;
use parsing::stream::buffered_stream::*;
use parsing::stream::text_stream::*;

use std::io::{Stdin, BufRead, BufReader};

pub struct TerminalStream {
    /// Number of read values.
    pub offset: usize,
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

impl TerminalStream {
    pub fn new() -> TerminalStream {
        return TerminalStream {
            offset: 0,
            buffer: vec![],
            next: 0,
            should_read: true,
        }
    }

    fn read_next_line(&mut self) {
        self.next = 0;
        self.buffer = read_line().replace('\r', "").chars().collect();
        self.offset += self.buffer.len();
        self.should_read = false;
    }
}

impl Stream<Option<char>> for TerminalStream {
    fn has_next(&self) -> bool {
        return has_next();
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

impl PeekableStream<Option<char>> for TerminalStream {
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

impl BufferedStream<Option<char>> for TerminalStream {
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

impl TextStream for TerminalStream {
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
