use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MiraklError {
  #[error("request error: path = '{path}', status = '{status}', body = '{body}'")]
  Request {
    path: String,
    status: StatusCode,
    body: String,
  },

  #[error("deserialize body error: msg = '{msg}', body = '{body}'")]
  Deserialize { msg: String, body: String },

  #[error("http error: {0}")]
  Http(#[from] ::reqwest::Error),

  #[error("json error: {0}")]
  Json(#[from] serde_json::Error),

  #[error("io error: {0}")]
  Io(#[from] std::io::Error),
}

impl MiraklError {
  pub fn should_try_again(&self) -> bool {
    match *self {
      MiraklError::Request { status, .. } => {
        let code = status.as_u16();
        // 429 Too Many Requests
        code == 429 || code == 500 || code == 503
      }
      _ => false,
    }
  }
}

pub type MiraklResult<T> = ::std::result::Result<T, MiraklError>;