use crate::{
    constraint_builder::{Query, ToExpr},
    util::Field,
};
use strum_macros::EnumIter;

pub const N_RW_TABLE_TAG_BYTES: usize = 4;

/// Tag to identify the operation type in a RwTable row
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum RwTableTag {
    Start = 1,
    Memory,
    Stack,
    Global,
    Table,
}

impl Into<usize> for RwTableTag {
    fn into(self) -> usize {
        self as usize
    }
}

impl ToExpr for RwTableTag {
    fn expr<F: Field>(&self) -> Query<F> {
        Query::Constant(F::from(*self as u64))
    }
}