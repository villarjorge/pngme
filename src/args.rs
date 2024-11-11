use std::path::PathBuf;

use crate::chunk_type::ChunkType;

pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

/// Encodes a secret message on the Png
pub struct EncodeArgs {
    /// Path to the Jpg to modify
    file_path: PathBuf,
    /// Type of the chunk to modify
    chunk_type: ChunkType,
    /// Message to write
    message: String,
    /// Optional: File path to output to
    output_file: PathBuf
}
/// Decodes a message from a Png
pub struct DecodeArgs {
    /// Path to the Png to decode from
    file_path: PathBuf,
    /// Type of the chunk to doecode from
    chunk_type: ChunkType
}
/// Removes a chunk from a Png
pub struct RemoveArgs {
    /// Path to the Png to remove from
    file_path: PathBuf,
    /// Chunk type of the chunk to be removed
    chunk_type: ChunkType
}
/// Prints a Png
pub struct PrintArgs {
    /// Path to the Png to print
    file_path: PathBuf
}