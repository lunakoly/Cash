use crate::stream::*;

pub trait BurstStream : Stream<Option<char>> {}

/// This stream can mimic
/// user input with spaces
/// once it reaches `\n`.
/// This is handy, if we read user
/// input interactively, but subsequent
/// streams require some lookahead.
pub struct SimpleBurstStream<'a> {
    /// Delegate for all operations.
    pub backend: &'a mut (dyn Stream<Option<char>> + 'a),
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
    /// If this stream decides it wants
    /// to peek the value first, the value
    /// will be saved to the intermediate
    /// field (because the backend stream
    /// is not peekable, it may only `grab()`).
    pub last_peeked: Option<char>,
}

impl <'a> SimpleBurstStream<'a> {
    pub fn new(
        backend: &'a mut (dyn Stream<Option<char>> + 'a),
        maximum_lock_length: usize,
    ) -> SimpleBurstStream<'a> {
        return SimpleBurstStream::<'a> {
            backend: backend,
            locked: false,
            maximum_lock_length: maximum_lock_length,
            current_lock_size: 0,
            last_peeked: None,
        }
    }
}

impl <'a> Stream<Option<char>> for SimpleBurstStream<'a> {
    fn has_next(&self) -> bool {
        if let Some(_) = self.last_peeked {
            true
        } else {
            self.backend.has_next()
        }
    }

    fn get_offset(&self) -> usize {
        if let Some(_) = self.last_peeked {
            self.backend.get_offset() - 1
        } else {
            self.backend.get_offset()
        }
    }

    fn grab(&mut self) -> Option<char> {
        if let Some(peeked) = self.last_peeked {
            Some(peeked)
        } else {
            self.backend.grab()
        }
    }
}

impl <'a> BurstStream for SimpleBurstStream<'a> {

}
