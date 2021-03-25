pub mod buffered_stream;
pub mod text_stream;
pub mod accumulator_stream;
pub mod analyzable_stream;
pub mod stdin_stream;
pub mod burst_stream;
pub mod wrapper_stream;

/// A sequence of some values of
/// type T.
pub trait Stream<T: Eq> {
    /// Returns the value meaning
    /// the end of the stream.
    fn get_end_value(&self) -> T;

    /// Returns the next unread
    /// item without going forward.
    /// Should return a special value
    /// meaning "the end" if has_next()
    /// is false.
    fn peek(&mut self) -> T;

    /// Returns true if there're some
    /// other values left.
    fn has_next(&mut self) -> bool {
        return self.peek() != self.get_end_value();
    }

    ///  Skips the current item.
    fn step(&mut self);

    /// Skips several items.
    fn step_all(&mut self, count: usize) {
        for _ in 0..count {
            self.step();
        }
    }

    ///  Returns the next unread
    ///  item and goes forward
    ///  (peek() && step()).
    fn grab(&mut self) -> T {
        let it = self.peek();
        self.step();
        return it;
    }

    /// Returns the number of read values.
    fn get_offset(&self) -> usize;
}
