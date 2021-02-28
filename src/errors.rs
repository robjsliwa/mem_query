use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("MemQuery Error")]
  MQError(String),

  #[error("Document Not Found")]
  MQDocumentNotFound,

  #[error("Collection Not Found")]
  MQCollectionNotFound,

  #[error(transparent)]
  IOError(#[from] std::io::Error),
}
