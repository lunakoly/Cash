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
}
