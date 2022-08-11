use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChunkType(u8, u8, u8, u8);

impl ChunkType {
    pub fn new(a: u8, b: u8, c: u8, d: u8) -> ChunkType {
        ChunkType(a, b, c, d)
    }
    pub fn bytes(&self) -> [u8; 4] {
        [self.0, self.1, self.2, self.3]
    }
    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }
    pub fn is_critical(&self) -> bool {
        (self.0 & 0x20) == 0
    }
    pub fn is_public(&self) -> bool {
        (self.1 & 0x20) == 0
    }
    pub fn is_reserved_bit_valid(&self) -> bool {
        (self.2 & 0x20) == 0
    }
    pub fn is_safe_to_copy(&self) -> bool {
        (self.3 & 0x20) == 0x20
    }
}

impl TryFrom<&[u8]> for ChunkType {
    type Error = std::io::Error;
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() != 4 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "ChunkType must be 4 bytes long",
            ));
        }
        Ok(ChunkType(bytes[0], bytes[1], bytes[2], bytes[3]))
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = std::io::Error;
    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        // TODO: check validity of the bytes
        Ok(ChunkType(bytes[0], bytes[1], bytes[2], bytes[3]))
    }
}

impl FromStr for ChunkType {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        for byte in bytes {
            if !byte.is_ascii_alphabetic() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "byte must be ascii characters!",
                ));
            }
        }
        ChunkType::try_from(bytes)
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}{}{}{}",
            self.0 as char, self.1 as char, self.2 as char, self.3 as char
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
