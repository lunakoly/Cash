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
    /// Returns true if there're some
    /// other values left.
    fn has_next(&self) -> bool;

    ///  Returns the next unread
    ///  item and goes forward
    ///  (peek() && step()).
    fn grab(&mut self) -> T;

    /// Returns the number of read values.
    fn get_offset(&self) -> usize;
}

/// A sequence of some values of
/// type T with the ability to preview
/// the value before grabbing it.
pub trait PeekableStream<T: Eq> : Stream<T> {
    /// Returns the next unread
    /// item without going forward.
    /// Should return a special value
    /// meaning "the end" if has_next()
    /// is false.
    fn peek(&mut self) -> T;

    ///  Skips the current item.
    fn step(&mut self);

    /// Skips several items.
    fn step_all(&mut self, count: usize) {
        for _ in 0..count {
            self.step();
        }
    }
}
