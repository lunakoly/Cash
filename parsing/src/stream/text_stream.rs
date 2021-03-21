use crate::stream::*;
use crate::stream::buffered_stream::*;

/// Buffered stream that works with
/// characters.
pub trait TextStream : BufferedStream<Option<char>> {
    /// Builds a string from the buffer contents.
    fn get_text(&self) -> String;

    /// Returns the number of matching chars
    /// if contents of `next` match contents
    /// of the inner buffer or 0 othersize.
    /// Length of `next` must be <= buffer size.
    fn match_text(&self, next: &str) -> usize;

    /// Returns the grab().unwrap().to_string().
    fn grab_string(&mut self) -> String;
}

/// A trivial implementation of a TextStream.
pub struct SimpleTextStream<'a> {
    /// The backend for this stream that manages
    /// all the hard work.
    pub delegate: SimpleBufferedStream<'a, Option<char>>,
}

impl <'a> SimpleTextStream<'a> {
    pub fn new(
        delegate: SimpleBufferedStream<'a, Option<char>>
    ) -> SimpleTextStream<'a> {
        return SimpleTextStream::<'a> {
            delegate: delegate,
        };
    }
}

impl <'a> Stream<Option<char>> for SimpleTextStream<'a> {
    fn get_end_value(&self) -> Option<char> {
        return self.delegate.get_end_value();
    }

    fn peek(&mut self) -> Option<char> {
        return self.delegate.peek();
    }

    fn step(&mut self) {
        self.delegate.step();
    }

    fn get_offset(&self) -> usize {
        return self.delegate.get_offset();
    }
}

impl <'a> BufferedStream<Option<char>> for SimpleTextStream<'a> {
    fn lookahead(&self, position: usize) -> Option<char> {
        return self.delegate.lookahead(position);
    }

    fn get_buffer(&self) -> Vec<Option<char>> {
        return self.delegate.get_buffer();
    }
}

impl <'a> TextStream for SimpleTextStream<'a> {
    fn get_text(&self) -> String {
        let mut result = String::new();

        for it in 0..self.delegate.buffer_size {
            if let Some(that) = self.delegate.lookahead(it) {
                result.push(that.clone());
            }
        }

        return result;
    }

    fn match_text(&self, next: &str) -> usize {
        let mut iterator = next.chars().peekable();

        for it in self.delegate.peek_index..self.delegate.buffer_size {
            if let Some(&next) = iterator.peek() {
                if Some(next) == self.delegate.buffer[it] {
                    iterator.next();
                } else {
                    return 0;
                }
            } else {
                return it - self.delegate.peek_index;
            }
        }

        for it in 0..self.delegate.peek_index {
            if let Some(&next) = iterator.peek() {
                if Some(next) == self.delegate.buffer[it] {
                    iterator.next();
                } else {
                    return 0;
                }
             } else {
                return it + self.delegate.buffer_size - self.delegate.peek_index;
            }
        }

        return self.delegate.buffer_size;
    }

    fn grab_string(&mut self) -> String {
        return self.grab().unwrap().to_string();
    }
}
