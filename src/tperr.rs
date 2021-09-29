use thiserror::Error;

pub type TPResult<T> = std::result::Result<T, TPError>;

#[derive(Debug, Error)]
pub enum TPError {}
