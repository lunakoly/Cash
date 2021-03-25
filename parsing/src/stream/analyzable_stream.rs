use crate::stream::*;
use crate::stream::buffered_stream::*;
use crate::stream::text_stream::*;
use crate::stream::accumulator_stream::*;

pub trait AnalyzableStream : TextStream + AccumulatorStream {}

pub struct SimpleAnalyzableStream<'a> {
    pub delegate: SimpleTextStream<'a>,
    pub accumulator: String,
}

impl <'a> SimpleAnalyzableStream<'a> {
    pub fn new(
        delegate: SimpleTextStream<'a>
    ) -> SimpleAnalyzableStream<'a> {
        return SimpleAnalyzableStream::<'a> {
            delegate: delegate,
            accumulator: String::new()
        };
    }

    pub fn acquire(
        buffer_size: usize,
        buffer_indent: usize,
        backend: &'a mut (dyn Stream<Option<char>> + 'a),
    ) -> SimpleAnalyzableStream<'a> {
        SimpleAnalyzableStream::new(
            SimpleTextStream::new(
                SimpleBufferedStream::new(
                    backend,
                    buffer_size,
                    buffer_indent,
                    Some('\n')
                )
            )
        )
    }
}

impl <'a> Stream<Option<char>> for SimpleAnalyzableStream<'a> {
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

impl <'a> PeekableStream<Option<char>> for SimpleAnalyzableStream<'a> {
    fn peek(&mut self) -> Option<char> {
        return self.delegate.peek();
    }

    fn step(&mut self) {
        let it = self.peek().unwrap();
        self.accumulator.push(it);
        self.delegate.step();
    }
}

impl <'a> BufferedStream<Option<char>> for SimpleAnalyzableStream<'a> {
    fn lookahead(&self, position: usize) -> Option<char> {
        return self.delegate.lookahead(position);
    }

    fn get_buffer(&self) -> Vec<Option<char>> {
        return self.delegate.get_buffer();
    }
}

impl <'a> TextStream for SimpleAnalyzableStream<'a> {
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

impl <'a> AccumulatorStream for SimpleAnalyzableStream<'a> {
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

impl <'a> AnalyzableStream for SimpleAnalyzableStream<'a> {

}
