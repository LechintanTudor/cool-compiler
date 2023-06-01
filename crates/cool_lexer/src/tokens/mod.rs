pub mod tk {
    pub use crate::consts::tk::*;
    pub use crate::tokens::punctuation::tk::*;
}

mod literal;
mod punctuation;
mod token;
mod token_kind;

pub use self::literal::*;
pub use self::punctuation::*;
pub use self::token::*;
pub use self::token_kind::*;
