mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use std::env; // https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html


// https://jrdngr.github.io/pngme_book/chapter_4.html#chapter-4-command-line-argumentss





fn main() -> Result<()> {
    todo!()
}
