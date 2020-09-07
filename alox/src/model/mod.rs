pub mod user;

use noria::{
    DataType
};

pub trait Model: Sized {
    fn from_row(row: &[DataType]) -> Result<Self, ()>;
    fn into_row(self) -> Vec<DataType>;
}