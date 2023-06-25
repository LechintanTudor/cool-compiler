mod primitive_ty_data;

pub use self::primitive_ty_data::*;
use crate::{resolve_fields_size_align, DefineErrorKind, Field, TyId};
use cool_lexer::Symbol;
use derive_more::From;
use rustc_hash::FxHashSet;
use std::sync::{Arc, Mutex};

#[derive(Clone, From, Debug)]
pub enum TyDef {
    Undefined,
    Basic(BasicTyDef),
    Aggregate(AggregateTyDef),
    Deferred(Arc<Mutex<Option<TyDef>>>),
}

impl TyDef {
    pub fn aggregate<F>(field_iter: F) -> Result<Self, DefineErrorKind>
    where
        F: IntoIterator<Item = (Symbol, TyId)>,
    {
        let mut fields = Vec::<Field>::new();
        let mut used_symbols = FxHashSet::<Symbol>::default();

        for (symbol, ty_id) in field_iter {
            if !used_symbols.insert(symbol) {
                return Err(DefineErrorKind::StructHasDuplicatedField { field: symbol });
            }

            fields.push(Field {
                offset: 0,
                symbol,
                ty_id,
            });
        }

        let (size, align) =
            resolve_fields_size_align(&mut fields).ok_or(DefineErrorKind::TypeCannotBeDefined)?;

        Ok(Self::from(AggregateTyDef {
            size,
            align,
            fields: fields.into(),
        }))
    }

    #[inline]
    pub fn deferred() -> Self {
        Self::Deferred(Default::default())
    }

    #[inline]
    #[must_use]
    pub fn is_defined(&self) -> bool {
        self.try_get_size_align().is_some()
    }

    #[must_use]
    pub fn try_get_size_align(&self) -> Option<(u64, u64)> {
        let (size, align) = match self {
            Self::Undefined => return None,
            Self::Basic(def) => (def.size, def.align),
            Self::Aggregate(def) => (def.size, def.align),
            Self::Deferred(def) => {
                let def = def.lock().unwrap();

                match def.as_ref()? {
                    Self::Basic(def) => (def.size, def.align),
                    Self::Aggregate(def) => (def.size, def.align),
                    _ => return None,
                }
            }
        };

        Some((size, align))
    }

    #[inline]
    #[must_use]
    pub fn get_size_align(&self) -> (u64, u64) {
        self.try_get_size_align().unwrap()
    }

    #[inline]
    #[must_use]
    pub fn get_size(&self) -> u64 {
        let (size, _) = self.get_size_align();
        size
    }

    #[inline]
    #[must_use]
    pub fn get_align(&self) -> u64 {
        let (_, align) = self.get_size_align();
        align
    }

    #[inline]
    #[must_use]
    pub fn is_zero_sized(&self) -> bool {
        self.get_size() == 0
    }

    pub fn get_aggregate_fields(&self) -> Option<Arc<[Field]>> {
        let fields = match self {
            Self::Aggregate(def) => def.fields.clone(),
            Self::Deferred(def) => {
                let def = def.lock().unwrap();

                match def.as_ref()? {
                    Self::Aggregate(def) => def.fields.clone(),
                    _ => return None,
                }
            }
            _ => return None,
        };

        Some(fields)
    }

    pub fn get_aggregate_field(&self, symbol: Symbol) -> Option<Field> {
        self.get_aggregate_fields()?
            .iter()
            .find(|field| field.symbol == symbol)
            .cloned()
    }
}

#[derive(Clone, Debug)]
pub struct BasicTyDef {
    pub size: u64,
    pub align: u64,
}

#[derive(Clone, Debug)]
pub struct AggregateTyDef {
    pub size: u64,
    pub align: u64,
    pub fields: Arc<[Field]>,
}
