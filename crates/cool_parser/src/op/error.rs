use derive_more::{Display, Error};

#[derive(Clone, Error, Debug, Display)]
#[display("Invalid operator")]
pub struct InvalidOp;
