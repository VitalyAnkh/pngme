use crate::chunk_type::ChunkType;
use crc;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Chunk<'a> {
    length: u32,
    chunk_type: ChunkType,
    data: &'a [u8],
    crc: u32,
}

impl Chunk<'_> {
    fn new(chunk_type: ChunkType, data: &[u8]) -> Self {
        let length = data.len() as u32;
        let crc = crc::checksum_ieee(data);
        Self {
            length,
            chunk_type,
            data,
            crc,
        }
    }
}
