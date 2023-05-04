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
        let remaining_len = self.0.len().saturating_sub(1);
        let remaining_symbols = &self.0[..remaining_len];
        Self(SmallVec::from_slice(remaining_symbols))
    }

    #[must_use]
    pub fn try_pop(&self) -> Option<Self> {
        let remaining_len = self.0.len().checked_sub(1)?;
        let remaining_symbols = &self.0[..remaining_len];
        Some(Self(SmallVec::from_slice(remaining_symbols)))
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
        let remaining_len = self.0.len().saturating_sub(1);
        let remaining_symbols = &self.0[..remaining_len];
        Self(remaining_symbols)
    }

    #[must_use]
    pub fn try_pop(&self) -> Option<Self> {
        let remaining_len = self.0.len().checked_sub(1)?;
        let remaining_symbols = &self.0[..remaining_len];
        Some(Self(remaining_symbols))
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
            pub fn starts_with_self_or_super(&self) -> bool {
                let first = self.first();
                first == sym::KW_SELF || first == sym::KW_SUPER
            }
            
            #[inline]
            pub fn starts_with_crate(&self) -> bool {
                self.first() == sym::KW_CRATE
            }
            
            #[inline]
            pub fn starts_with<'a, P>(&self, path: P) -> bool
            where
                P: Into<ItemPath<'a>>,
            {
                self.0.starts_with(path.into().0)   
            }

            #[inline]
            pub fn as_symbol_slice(&self) -> &[Symbol] {
                &self.0[..]
            }

            #[inline]
            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
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
