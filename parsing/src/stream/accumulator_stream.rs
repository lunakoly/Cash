use crate::stream::*;
use crate::stream::buffered_stream::*;
use crate::stream::text_stream::*;

/// Stream with an accumulator that saves
/// every single character it has seen.
pub trait AccumulatorStream : PeekableStream<Option<char>> {
    /// Clears the interal lexeme.
    fn clear(&mut self);

    /// Returns the string starting from
    /// the `position` and up to the current
    /// position.
    fn revise(&self, position: usize) -> String;

    /// Returns the whole string.
    fn revise_all(&self) -> String;

    /// Steps if `peek()` equals `next`.
    fn accept(&mut self, next: char) -> bool {
        if self.peek() == Some(next) {
            self.step();
            return true;
        }

        return false;
    }

    /// Returns true, if the `peek()`'ed character
    /// has it's u32 representation within the values
    /// of the given 2 characters (both including).
    fn expect_in(&mut self, from: char, to: char) -> bool {
        if let Some(symbol) = self.peek() {
            // from as u32 <= symbol as u32 && symbol as u32 <= to as u32
            (from..=to).contains(&symbol)
        } else {
            false
        }
    }
}

/// Accumulates the values of another
/// character stream
pub struct SimpleAccumulatorStream<'a> {
    /// Delegate for all operations.
    pub delegate: &'a mut (dyn TextStream + 'a),
    /// Accumulates the value
    pub accumulator: String,
}

impl <'a> SimpleAccumulatorStream<'a> {
    pub fn new(
        delegate: &'a mut (dyn TextStream + 'a)
    ) -> SimpleAccumulatorStream<'a> {
        return SimpleAccumulatorStream::<'a> {
            delegate: delegate,
            accumulator: String::new()
        };
    }
}

impl <'a> Stream<Option<char>> for SimpleAccumulatorStream<'a> {
    fn has_next(&self) -> bool {
        return self.delegate.has_next();
    }

    fn get_offset(&self) -> usize {
        return self.delegate.get_offset();
    }

    fn grab(&mut self) -> Option<char> {
        let it = self.peek();
        self.step();
        return it;
    }
}

impl <'a> PeekableStream<Option<char>> for SimpleAccumulatorStream<'a> {
    fn peek(&mut self) -> Option<char> {
        return self.delegate.peek();
    }

    fn step(&mut self) {
        let it = self.peek().unwrap();
        self.accumulator.push(it);
        self.delegate.step();
    }
}

impl <'a> BufferedStream<Option<char>> for SimpleAccumulatorStream<'a> {
    fn lookahead(&self, position: usize) -> Option<char> {
        return self.delegate.lookahead(position);
    }

    fn get_buffer(&self) -> Vec<Option<char>> {
        return self.delegate.get_buffer();
    }
}

impl <'a> TextStream for SimpleAccumulatorStream<'a> {
    fn get_text(&self) -> String {
        return self.delegate.get_text();
    }

    fn match_text(&self, next: &str) -> usize {
        return self.delegate.match_text(next);
    }

    fn grab_string(&mut self) -> String {
        return self.delegate.grab_string();
    }
}

impl <'a> AccumulatorStream for SimpleAccumulatorStream<'a> {
    fn clear(&mut self) {
        self.accumulator.clear();
    }

    fn revise(&self, position: usize) -> String {
        let distance = (self as &dyn TextStream).get_offset() - position;

        return self.accumulator.chars()
            .skip(self.accumulator.len() - distance)
            .take(distance)
            .collect();
    }

    fn revise_all(&self) -> String {
        return self.accumulator.clone();
    }
}
