use std::fmt::Display;

use level_zero_sys::ze_result_t;

pub type Result<T> = std::result::Result<T, LevelZeroError>;

#[derive(Debug)]
pub struct LevelZeroError(ze_result_t);

impl Display for LevelZeroError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:#?}", self.0)
    }
}

impl From<ze_result_t> for LevelZeroError {
    fn from(value: ze_result_t) -> Self {
        Self(value)
    }
}
