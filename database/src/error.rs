#[derive(Debug)]
pub enum DatabaseError {
    ConnectionError(String),
}

pub type Result<T> = std::result::Result<T, DatabaseError>;
