use crate::stream::*;
use crate::stream::buffered_stream::*;

pub trait BurstStream : BufferedStream<Option<char>> {}

/// This stream can mimic
/// user input with spaces
/// once it reaches `\n`.
/// This is handy, if we read user
/// input interactively, but subsequent
/// streams require some lookahead.
pub struct SimpleBurstStream<'a> {
    /// Delegate for all operations.
    pub backend: &'a mut (dyn BufferedStream<Option<char>> + 'a),
    /// Used to make the stream
    /// return blanks instead of
    /// real characters when
    /// \n is reached. `true` activates
    /// the feature.
    pub locked: bool,
    /// The maximum number of spaces
    /// that can be received if locked.
    /// The lock is reset back to false
    /// once this is exceeded.
    pub maximum_lock_length: usize,
    /// The number of already returned spaces.
    pub current_lock_size: usize,
}

impl <'a> SimpleBurstStream<'a> {
    pub fn new(
        backend: &'a mut (dyn BufferedStream<Option<char>> + 'a),
        maximum_lock_length: usize,
    ) -> SimpleBurstStream<'a> {
        return SimpleBurstStream::<'a> {
            backend: backend,
            locked: false,
            maximum_lock_length: maximum_lock_length,
            current_lock_size: 0,
        }
    }
}

impl <'a> Stream<Option<char>> for SimpleBurstStream<'a> {
    fn get_end_value(&self) -> Option<char> {
        return self.backend.get_end_value();
    }

    fn peek(&mut self) -> Option<char> {
        if self.locked {
            return Some(' ');
        }

        return self.backend.peek();
    }

    fn step(&mut self) {
        if !self.locked {
            if self.backend.peek() == Some('\n') {
                self.locked = true;
            } else {
                self.backend.step();
            }
        } else {
            self.current_lock_size += 1;

            if self.current_lock_size >= self.maximum_lock_length {
                self.current_lock_size = 0;
                self.locked = false;
                self.backend.step();
            }
        }
    }

    fn get_offset(&self) -> usize {
        return self.backend.get_offset();
    }
}

impl <'a> BufferedStream<Option<char>> for SimpleBurstStream<'a> {
    fn lookahead(&self, position: usize) -> Option<char> {
        if let Some(value) = self.backend.lookahead(position) {
            Some(value)
        } else {
            Some(' ')
        }
    }

    fn get_buffer(&self) -> Vec<Option<char>> {
        return self.backend.get_buffer();
    }
}

impl <'a> BurstStream for SimpleBurstStream<'a> {

}
