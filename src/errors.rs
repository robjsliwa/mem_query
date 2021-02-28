use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("MemQuery Error")]
  MQError(String),

  #[error("Invalid Operator")]
  MQInvalidOp(String),

  #[error("Invalid Value")]
  MQInvalidValue(String),

  #[error("Invalid Type")]
  MQInvalidType,

  #[error("Document Not Found")]
  MQDocumentNotFound,

  #[error("Collection Not Found")]
  MQCollectionNotFound,

  #[error(transparent)]
  IOError(#[from] std::io::Error),
}
