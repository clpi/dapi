use sqlx::error::DatabaseError;
use std::{fmt, io};

pub type DbResult<R> = Result<R, DbError>;

#[derive(Debug)]
pub enum DbError {
    Connection(sqlx::Error),
    TableNotFound(sqlx::Error),
    RowNotFound(sqlx::Error),
    Database(Box<dyn DatabaseError>),
    Other,
}

/*
impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbError::Connection(e) => f.write_fmt("Connection failed: {}", e.into()),
            DbError::TableNotFound(e) => f.write_fmt("No table: {}", e.into()),
            DbError::RowNotFound(e) => f.write_fmt("No entry: {}", e.into()),
            DbError::Database(e) => f.write_fmt("DB error: {}", e.into()),
            _ => { f.write_str("Other error") }
        }
    }
}

impl std::error::Error for DbError {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self {
            DbError::Database(e) => Some(e),
            _ => None,
        }
    }
}

impl From<sqlx::Error> for DbError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Self::RowNotFound(sqlx::Error::RowNotFound),
            _ => Self::Database(err.as_database_error()),
        }
    }
}
*/
