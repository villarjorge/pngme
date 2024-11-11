use std::str::FromStr;
use std::fmt;

/// A validated PNG chunk type. See the PNG spec for more details.
/// http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    data: [u8; 4]
}

impl ChunkType {
    /// Returns the raw bytes contained in this chunk
    pub fn bytes(&self) -> [u8; 4] {
        self.data
    }

    /// Returns the property state of the first byte as described in the PNG spec
    pub fn is_critical(&self) -> bool {
        // Given by 5th bit of first byte. 
        // 0 (uppercase) = critical, 1 (lowercase) = ancillary.
        let byte: u8 = self.data[0];
        // Shift to the left 4 times then and with 00...001
        if (byte >> 5) & 1 == 0 {
            return true
        }
        return false
    }

    /// Returns the property state of the second byte as described in the PNG spec
    pub fn is_public(&self) -> bool {
        // Given by bit 5 of second byte
        // 0 (uppercase) = public, 1 (lowercase) = private.
        let byte: u8 = self.data[1];
        // Shift to the left 4 times then and with 00...001
        if (byte >> 5) & 1 == 0 {
            return true
        }
        return false
    }

    /// Returns the property state of the third byte as described in the PNG spec
    pub fn is_reserved_bit_valid(&self) -> bool {
        // Given by bit 5 of third byte
        // Must be 0 (uppercase) in files conforming to this version of PNG.
        let byte: u8 = self.data[2];
        // Shift to the left 4 times then and with 00...001
        if (byte >> 5) & 1 == 0 {
            return true
        }
        return false
    }

    /// Returns the property state of the fourth byte as described in the PNG spec
    pub fn is_safe_to_copy(&self) -> bool {
        // Given by bit 5 of fourth byte
        // 0 (uppercase) = unsafe to copy, 1 (lowercase) = safe to copy.
        let byte: u8 = self.data[3];
        // Shift to the left 4 times then and with 00...001
        if (byte >> 5) & 1 == 1 {
            return true
        }
        return false
    }

    /// Returns true if the reserved byte is valid and all four bytes are represented by the characters A-Z or a-z.
    /// Note that this chunk type should always be valid as it is validated during construction.
    pub fn is_valid(&self) -> bool {
        if self.is_reserved_bit_valid() == false {
            return false
        }

        for i in 0..=3 {
            let b = self.data[i];
            // If it is not in the ranges, then it is not valid
            if !ChunkType::is_valid_byte(b) {
                return false
            }
        }
        return true
    }

    /// Valid bytes are represented by the characters A-Z or a-z (or 65-90 and 97-122 decimal)
    pub fn is_valid_byte(byte: u8) -> bool {
        // Also can be done with u8::is_ascii_uppercase and u8::is_ascii_lowercase
        return (65 <= byte && byte <= 90) | (97 <= byte && byte <= 122)
    }
}

// https://doc.rust-lang.org/std/convert/trait.TryFrom.html
impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(arr: [u8; 4]) -> Result<Self, Self::Error> {
        let possible_chunk: ChunkType = ChunkType{data: arr};
        if possible_chunk.is_valid() {
            return Ok(possible_chunk)
        }
        Err("Invalid chunk when converting from array (Chunk bytes not within upper and lowercase ASCII letters or third byte not uppercase)") 
    }
}

// https://doc.rust-lang.org/std/str/trait.FromStr.html
impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err("The chunk lenght is not 4 bytes")
        }
        // When doing it from string, the state of the third byte is ignored
        let mut arr: [u8; 4] = [0, 0, 0, 0];
        for (i, b) in s.bytes().enumerate() {
            if ChunkType::is_valid_byte(b) {
                arr[i] = b;
            } else {
                return Err("Invalid chunk when converting from string literal (Chunk bytes not within upper and lowercase ASCII letters)");
            }
        }
        return Ok(ChunkType{data: arr})
    }
}

// https://doc.rust-lang.org/std/fmt/trait.Display.html
impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [b1, b2, b3, b4] = self.data;
        //write!(f, "[{}, {}, {}, {}]", b1, b2, b3, b4)
        // Is there a better way to do this?
        write!(f, "{}{}{}{}", b1 as char, b2 as char, b3 as char, b4 as char)
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