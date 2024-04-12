use std::fmt::{Debug, Display};

use crate::clickhouse::errors::ClickhouseError;

#[derive(Debug)]
pub struct DatabaseError {
    pub error: Box<dyn std::error::Error>,
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.error)
    }
}

impl std::error::Error for DatabaseError {}

pub trait MapError: Into<DatabaseError> {
    fn to_db_err(self) -> DatabaseError {
        self.into()
    }
}
