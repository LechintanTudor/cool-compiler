use crate::lexer::LineOffsets;
use std::io::BufRead;

#[derive(Clone, Debug)]
pub struct LexedSourceFile {
    pub source: String,
    pub line_offsets: LineOffsets,
}

impl LexedSourceFile {
    pub fn from_reader<R>(reader: &mut R) -> LexedSourceFile
    where
        R: BufRead,
    {
        let mut source = String::new();
        let mut line_offsets = LineOffsets::default();

        loop {
            match reader.read_line(&mut source) {
                Ok(0) => break,
                Ok(_) => line_offsets.add(source.len() as u32),
                Err(_) => todo!("handle error"),
            }
        }

        Self {
            source,
            line_offsets,
        }
    }
}
