use crate::stream::*;
use crate::stream::buffered_stream::*;
use crate::stream::text_stream::*;
use crate::stream::accumulator_stream::*;

trait AnalyzableStream : TextStream + AccumulatorStream {}

pub struct SimpleAnalyzableStream {
    pub delegate: SimpleTextStream,
    pub accumulator: String,
}

impl SimpleAnalyzableStream {
    pub fn new(
        delegate: SimpleTextStream
    ) -> SimpleAnalyzableStream {
        return SimpleAnalyzableStream {
            delegate: delegate,
            accumulator: String::new()
        };
    }
}

impl Stream<Option<char>> for SimpleAnalyzableStream {
    fn get_end_value(&self) -> Option<char> {
        return self.delegate.get_end_value();
    }

    fn peek(&mut self) -> Option<char> {
        return self.delegate.peek();
    }

    fn step(&mut self) {
        let it = self.peek().unwrap();
        self.accumulator.push(it);
        self.delegate.step();
    }

    fn get_offset(&self) -> usize {
        return self.delegate.get_offset();
    }
}

impl BufferedStream<Option<char>> for SimpleAnalyzableStream {
    fn lookahead(&self, position: usize) -> Option<char> {
        return self.delegate.lookahead(position);
    }

    fn get_buffer(&self) -> Vec<Option<char>> {
        return self.delegate.get_buffer();
    }
}

impl TextStream for SimpleAnalyzableStream {
    fn get_text(&self) -> String {
        return self.delegate.get_text();
    }

    fn match_text(&self, next: &str) -> usize {
        return self.delegate.match_text(next);
    }
}

impl AccumulatorStream for SimpleAnalyzableStream {
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
