use crate::stream::*;

/// Stream with an accumulator that saves
/// every single character it has seen.
pub trait AccumulatorStream : Stream<Option<char>> {
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
