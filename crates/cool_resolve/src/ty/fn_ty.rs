use crate::{FnAbi, TyId};
use smallvec::SmallVec;
use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct FnTy {
    pub abi: FnAbi,
    pub params: SmallVec<[TyId; 2]>,
    pub is_variadic: bool,
    pub ret: TyId,
}

impl fmt::Display for FnTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.abi != FnAbi::Cool {
            write!(f, "extern \"{}\" ", self.abi)?;
        }

        write!(f, "fn(")?;

        let has_params = match self.params.as_slice() {
            [] => false,
            [param] => {
                write!(f, "{}", param)?;
                true
            }
            [first, others @ ..] => {
                write!(f, "{}", first)?;

                for other in others {
                    write!(f, ", {}", other)?;
                }

                true
            }
        };

        if self.is_variadic {
            if has_params {
                write!(f, ", ...")?;
            } else {
                write!(f, "...")?;
            }
        }

        write!(f, ") -> {}", self.ret)
    }
}