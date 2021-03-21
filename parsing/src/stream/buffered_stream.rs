use crate::stream::*;

/// Stream with lookaheads. Reads
/// a bunch of extra values beforehand.
pub trait BufferedStream<T : Eq> : Stream<T> {
    /// Allows to look at the specified
    /// position in the buffer.
    fn lookahead(&self, position: usize) -> T;

    /// Returns the inner buffer copy.
    fn get_buffer(&self) -> Vec<T>;
}

/// A trivial implementation of a BufferedStream<T>.
pub struct SimpleBufferedStream<'a, T : Eq + Copy> {
    /// The size of the inner buffer.
    pub buffer_size: usize,
    /// The number of visited characters
    /// to save.
    pub buffer_indent: usize,
    /// The stream used as a backend.
    pub backend: &'a mut (dyn Stream<T> + 'a),
    /// Points to the element that
    /// will be returned at the next
    /// peek() call. peek_index should
    /// cycle over the buffer.
    pub peek_index: usize,
    /// Points to the element that
    /// will be updated after the next step.
    /// fill_index should cycle over the buffer.
    pub fill_index: usize,
    /// Internal storage for collecting
    /// lookahead values.
    pub buffer: Vec<T>,
}

impl<'a, T : Eq + Copy> SimpleBufferedStream<'a, T> {
    pub fn new(
        backend: &'a mut (dyn Stream<T> + 'a),
        buffer_size: usize,
        buffer_indent: usize,
        default: T
    ) -> SimpleBufferedStream<'a, T> {
        let mut that = SimpleBufferedStream::<'a> {
            buffer_size: buffer_size,
            buffer_indent: buffer_indent,
            backend: backend,
            peek_index: buffer_indent,
            fill_index: 0,
            buffer: vec![default.clone(); buffer_size]
        };

        for it in that.peek_index..that.buffer_size {
            that.buffer[it] = that.backend.peek();
            that.backend.step();
        }

        return that;
    }
}

impl<'a, T : Eq + Copy> Stream<T> for SimpleBufferedStream<'a, T> {
    fn get_end_value(&self) -> T {
        return self.backend.get_end_value();
    }

    fn peek(&mut self) -> T {
        return self.buffer[self.peek_index].clone();
    }

    fn has_next(&mut self) -> bool {
        return self.buffer[self.peek_index] != self.get_end_value();
    }

    fn step(&mut self) {
        self.buffer[self.fill_index] = self.backend.peek();
        self.backend.step();

        if self.peek_index != self.buffer_size - 1 {
            self.peek_index += 1;
        } else {
            self.peek_index = 0;
        }

        if self.fill_index != self.buffer_size - 1 {
            self.fill_index += 1;
        } else {
            self.fill_index = 0;
        }
    }

    fn get_offset(&self) -> usize {
        return self.backend.get_offset() + self.buffer_indent - self.buffer_size;
    }
}

impl<'a, T: Eq + Copy> BufferedStream<T> for SimpleBufferedStream<'a, T> {
    fn lookahead(&self, position: usize) -> T {
        let mut index = self.peek_index + position;

        if index >= self.buffer_size {
            index -= self.buffer_size;
        }

        return self.buffer[index].clone();
    }

    fn get_buffer(&self) -> Vec<T> {
        return self.buffer.clone();
    }
}
