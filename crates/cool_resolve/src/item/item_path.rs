use cool_lexer::symbols::{sym, Symbol};
use smallvec::SmallVec;
use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash, Default)]
pub struct ItemPathBuf(SmallVec<[Symbol; 4]>);

impl ItemPathBuf {
    #[inline]
    pub fn as_path(&self) -> ItemPath {
        ItemPath(self.0.as_slice())
    }

    #[must_use]
    pub fn pop(&self) -> Self {
        let remaining_len = self.0.len().checked_sub(1).unwrap_or(0);
        let remaining_symbols = &self.0[..remaining_len];
        Self(SmallVec::from_slice(remaining_symbols))
    }

    #[must_use]
    pub fn append(&self, symbol: Symbol) -> Self {
        let mut symbols = self.0.clone();
        symbols.push(symbol);
        Self(symbols)
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

impl FromIterator<Symbol> for ItemPathBuf {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Symbol>,
    {
        Self(SmallVec::from_iter(iter))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ItemPath<'a>(&'a [Symbol]);

impl ItemPath<'_> {
    #[must_use]
    pub fn pop(&self) -> Self {
        let remaining_len = self.0.len().checked_sub(1).unwrap_or(0);
        let remaining_symbols = &self.0[..remaining_len];
        Self(remaining_symbols)
    }

    #[inline]
    #[must_use]
    pub fn to_path_buf(&self) -> ItemPathBuf {
        ItemPathBuf(SmallVec::from_slice(self.0))
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

#[rustfmt::skip]
macro_rules! impl_path {
    ($ty:ty) => {
        impl $ty {
            #[inline]
            pub fn as_symbol_slice(&self) -> &[Symbol] {
                &self.0[..]
            }

            #[inline]
            pub fn len(&self) -> usize {
                self.0.len()
            }

            #[inline]
            pub fn first(&self) -> Symbol {
                self.0.first().copied().unwrap_or(sym::EMPTY)
            }

            #[inline]
            pub fn last(&self) -> Symbol {
                self.0.last().copied().unwrap_or(sym::EMPTY)
            }
        }

        impl fmt::Display for $ty {
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

        impl fmt::Debug for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let Some((first, others)) = self.0.split_first() else {
                    return write!(f, "\"\"")
                };

                write!(f, "\"{}", first)?;

                for other in others {
                    write!(f, ".{}", other)?;
                }

                f.write_str("\"")
            }
        }
    };
}

impl_path!(ItemPathBuf);
impl_path!(ItemPath<'_>);
