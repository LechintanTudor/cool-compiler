use cool_lexer::symbols::Symbol;
use smallvec::SmallVec;
use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ItemPathBuf(SmallVec<[Symbol; 4]>);

impl ItemPathBuf {
    #[inline]
    pub fn as_symbol_slice(&self) -> &[Symbol] {
        self.0.as_slice()
    }

    #[inline]
    pub fn as_path(&self) -> ItemPath {
        ItemPath(self.0.as_slice())
    }

    #[must_use]
    pub fn append(&self, symbol: Symbol) -> Self {
        let mut symbols = self.0.clone();
        symbols.push(symbol);
        Self(symbols)
    }
}

impl fmt::Display for ItemPathBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Some((first, others)) = self.0.split_first() else {
            return Ok(());
        };

        write!(f, "{}", first)?;

        for other in others {
            write!(f, ".{}", other)?;
        }

        Ok(())
    }
}

impl From<Symbol> for ItemPathBuf {
    #[inline]
    fn from(symbol: Symbol) -> Self {
        Self(SmallVec::from_slice(&[symbol]))
    }
}

impl From<&[Symbol]> for ItemPathBuf {
    #[inline]
    fn from(symbols: &[Symbol]) -> Self {
        Self(SmallVec::from_slice(symbols))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ItemPath<'a>(&'a [Symbol]);

impl ItemPath<'_> {
    #[inline]
    pub fn as_symbol_slice(&self) -> &[Symbol] {
        self.0
    }

    #[inline]
    #[must_use]
    pub fn to_path_buf(&self) -> ItemPathBuf {
        ItemPathBuf(SmallVec::from_slice(self.0))
    }
}

impl fmt::Display for ItemPath<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Some((first, others)) = self.0.split_first() else {
            return Ok(());
        };

        write!(f, "{}", first)?;

        for other in others {
            write!(f, ".{}", other)?;
        }

        Ok(())
    }
}

impl<'a> From<&'a [Symbol]> for ItemPath<'a> {
    #[inline]
    fn from(symbols: &'a [Symbol]) -> Self {
        Self(symbols)
    }
}

impl<'a> From<&'a ItemPathBuf> for ItemPath<'a> {
    #[inline]
    fn from(path: &'a ItemPathBuf) -> Self {
        Self(&path.0)
    }
}
