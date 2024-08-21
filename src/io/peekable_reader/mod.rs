//! A reader that supports peeking at the next bytes without consuming them.

use std::{collections::VecDeque, io::Read};

/// A reader that supports unlimited peeking via [`peek_exact`](Self::peek_exact).
pub struct PeekableReader<R> {
    buffer: VecDeque<u8>,
    reader: R,
}

impl<R: Read> Read for PeekableReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let buffer_drain_length = buf.len().min(self.buffer.len());
        for (source_byte, destination_byte) in
            self.buffer.drain(..buffer_drain_length).zip(buf.iter_mut())
        {
            *destination_byte = source_byte;
        }
        self.reader
            .read(&mut buf[buffer_drain_length..])
            .map(|read_bytes| read_bytes + buffer_drain_length)
    }
}

impl<R> PeekableReader<R> {
    /// Create a new peekable reader wrapping the given reader.
    pub fn new(reader: R) -> Self {
        Self {
            buffer: Default::default(),
            reader,
        }
    }
}

impl<R: Read> PeekableReader<R> {
    /// Peek at the first `buf.len()` bytes of the reader.
    pub fn peek_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        self.ensure_buffer_length(buf.len())?;

        for (source_byte, destination_byte) in self.buffer.iter().copied().zip(buf.iter_mut()) {
            *destination_byte = source_byte;
        }

        Ok(())
    }

    fn ensure_buffer_length(&mut self, length: usize) -> std::io::Result<()> {
        if length <= self.buffer.len() {
            return Ok(());
        }

        let extend_offset = self.buffer.len();
        self.buffer.resize(length, 0);

        let (first_slice, second_slice) = self.buffer.as_mut_slices();
        if first_slice.len() > extend_offset {
            self.reader.read_exact(&mut first_slice[extend_offset..])?;
            self.reader.read_exact(second_slice)?;
        } else {
            self.reader
                .read_exact(&mut second_slice[extend_offset - first_slice.len()..])?;
        }

        Ok(())
    }
}
