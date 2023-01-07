use std::str::Chars;

pub struct SourceCharIter<'a> {
    chars: Chars<'a>,
    offset: usize,
}

impl Iterator for SourceCharIter<'_> {
    type Item = (u32, char);

    fn next(&mut self) -> Option<Self::Item> {
        let char = self.chars.next()?;

        let offset = self.offset;
        self.offset += char.len_utf8();

        Some((offset as u32, char))
    }
}

impl<'a> From<&'a str> for SourceCharIter<'a> {
    fn from(source: &'a str) -> Self {
        Self {
            chars: source.chars(),
            offset: 0,
        }
    }
}
