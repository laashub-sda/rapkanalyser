// MIT License
//
// Copyright (c) 2017 Guillem Nieto
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct ChunkHeader {
    offset: u64,
    header_size: u16,
    chunk_size: u32,
    chunk_type: u16,
}

impl ChunkHeader {
    pub fn new(offset: u64, header_size: u16, chunk_size: u32, chunk_type: u16) -> Self {
        Self {
            offset,
            header_size,
            chunk_size,
            chunk_type,
        }
    }

    pub fn get_offset(&self) -> u64 {
        self.offset
    }

    pub fn get_header_size(&self) -> u16 {
        self.header_size
    }

    pub fn get_data_offset(&self) -> u64 {
        self.offset + u64::from(self.header_size)
    }

    pub fn get_chunk_end(&self) -> u64 {
        self.offset + u64::from(self.chunk_size)
    }

    pub fn absolute(&self, relative: u64) -> u64 {
        let absolute = self.offset + relative;

        if absolute > self.get_chunk_end() {
            panic!("Requested a relative value out of bounds");
        }

        absolute
    }

    pub fn get_token(&self) -> u16 {
        self.chunk_type
    }
}

impl fmt::Display for ChunkHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(Token:{:X}; Start: {}; Data: {}; End {})",
            self.chunk_type,
            self.offset,
            self.get_data_offset(),
            self.get_chunk_end()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::ChunkHeader;

    #[test]
    pub fn it_returns_data_offset() {
        let chunk = ChunkHeader::new(4000, 8, 16, 0);

        assert_eq!(4008, chunk.get_data_offset());
    }

    #[test]
    pub fn it_returns_chunk_end() {
        let chunk = ChunkHeader::new(4000, 8, 16, 0);

        assert_eq!(4016, chunk.get_chunk_end());
    }

    #[test]
    #[should_panic]
    pub fn it_panics_from_relative_out_of_bound() {
        let chunk = ChunkHeader::new(4000, 8, 500, 0);
        chunk.absolute(510);
    }

    #[test]
    pub fn it_returns_absolute_offsets_from_relative_ones() {
        let chunk = ChunkHeader::new(4000, 8, 500, 0);
        let res = chunk.absolute(490);

        assert_eq!(4490, res);
    }
}
