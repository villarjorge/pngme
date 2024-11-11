use crc;
use std::convert::TryFrom;
use std::fmt;
use std::io::{BufReader, Read};

use std::string::FromUtf8Error;
use crate::chunk_type::ChunkType;

/// A validated PNG chunk. See the PNG spec for more details.
/// http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
#[derive(Debug, Clone)]
pub struct Chunk {
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
}

impl Chunk {
    /// Creates a new chunk from a Chunk type and a vector of u8
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        return Chunk {
            chunk_type: chunk_type,
            chunk_data: data
        }
    }

    /// Returns the length of the data in the chunk
    pub fn length(&self) -> u32 {
        return self.data().len().try_into().unwrap()
    }
    /// Returns a reference to the ChunkType
    pub fn chunk_type(&self) -> &ChunkType {
        return &self.chunk_type
    }
    /// The raw data contained in this chunk in bytes
    pub fn data(&self) -> &[u8] {
        return &self.chunk_data
    }
    /// Calculates the CRC for the chunk
    pub fn crc(&self) -> u32 {
        // I do not know exacty what algorithm this is but it seems to work
        const ISO: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let to_checksum: Vec<u8> = self
            .chunk_type
            .bytes()
            .iter()
            .chain(self.data())
            .copied()
            .collect();

        return ISO.checksum(&to_checksum)
    }
    /// Returns the data stored in this chunk as a `String`. This function will return an error
    /// if the stored data is not valid UTF-8.
    pub fn data_as_string(&self) -> Result<String, FromUtf8Error> {
        return String::from_utf8(self.chunk_data.to_vec())
    }
    /// Returns this chunk as a byte sequences described by the PNG spec.
    /// The following data is included in this byte sequence in order:
    /// 1. Length of the data *(4 bytes)*
    /// 2. Chunk type *(4 bytes)*
    /// 3. The data itself *(`length` bytes)*
    /// 4. The CRC of the chunk type and data *(4 bytes)*
    #[allow(dead_code)]
    pub fn as_bytes(&self) -> Vec<u8>{
        let mut v: Vec<u8> = vec!();
    
        v.extend(self.length().to_be_bytes());
        v.extend(self.chunk_type.bytes());
        v.extend(self.data());
        v.extend(self.crc().to_be_bytes());
    
        return v
    }
}

// https://doc.rust-lang.org/std/convert/trait.TryFrom.html
impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 12 {
            return Err("Given vector is too short")
        }
        // Create a reader and two buffers, one of length four for the various chunks of four bytes
        let mut reader = BufReader::new(bytes);
        let mut buffer: [u8; 4] = [0, 0, 0, 0];

        reader.read_exact(&mut buffer).unwrap();
        let data_length: u32 = u32::from_be_bytes(buffer);

        reader.read_exact(&mut buffer).unwrap();
        let chunk_type: ChunkType = ChunkType::try_from(buffer).unwrap();

        // Create a big buffer for the data of the chunk 
        // Warning: what happens when the lenght is zero? 
        let mut big_buffer = vec!(0; data_length as usize);

        reader.read_exact(&mut big_buffer).unwrap();
        let chunk_data: Vec<u8> = big_buffer.to_vec();

        reader.read_exact(&mut buffer).unwrap();
        let crc: u32 = u32::from_be_bytes(buffer);

        let possible_chunk = Chunk{chunk_type: chunk_type, chunk_data: chunk_data};
        let p: u32 = possible_chunk.crc();
        if p == crc {
            return Ok(possible_chunk)
        } else {
            Err("Given CRC does not match with computed crc")
        }
    }
}

// https://doc.rust-lang.org/std/fmt/trait.Display.html
impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}