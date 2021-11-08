use crate::MiraklImpl;
use reqwest::blocking::{Client, Response};
pub use reqwest::{Method, blocking::RequestBuilder};
use crate::result::{MiraklError, MiraklResult};
use serde::Deserialize;
use serde_json;

pub struct MiraklClient {
  http: Client,
  endpoint: &'static str,
  token: String,
}

impl MiraklClient {
  pub fn new(i: MiraklImpl, token: &str) -> Self {
    Self::with_http_client(i, token, Client::new())
  }

  pub fn with_http_client(i: MiraklImpl, token: &str, http: Client) -> Self {
    Self {
      endpoint: i.get_endpoint(),
      token: token.to_owned(),
      http,
    }
  }

  pub fn request(&self, method: Method, path: &str) -> RequestBuilder {
    use reqwest::header::{self, HeaderValue};
    let b = self
      .http
      .request(method, &format!("{}{}", self.endpoint, path));
    b.header(header::AUTHORIZATION, {
      if let Ok(v) = HeaderValue::from_str(&self.token.clone()) {
        v
      } else {
        HeaderValue::from_static("")
      }
    })
      .header(header::CACHE_CONTROL, HeaderValue::from_static("no-cache"))
      .header(header::ACCEPT, HeaderValue::from_static("application/json"))
  }
}

pub trait MiraklResponse {
  fn get_response<T: for<'de> Deserialize<'de>>(self) -> MiraklResult<T>;
  fn no_content(self) -> MiraklResult<()>;
}

impl MiraklResponse for Response {
  fn get_response<T: for<'de> Deserialize<'de>>(self) -> MiraklResult<T> {
    let status = self.status();
    let path = self.url().to_string();
    let body = self.text()?;

    if !status.is_success() {
      return Err(MiraklError::Request {
        path,
        status,
        body,
      });
    }

    match serde_json::from_str(&body) {
      Ok(v) => Ok(v),
      Err(err) => {
        return Err(MiraklError::Deserialize {
          msg: err.to_string(),
          body,
        })
      }
    }
  }

  fn no_content(self) -> MiraklResult<()> {
    let status = self.status();
    let path = self.url().to_string();
    let body = self.text()?;

    if !status.is_success() {
      return Err(MiraklError::Request {
        path,
        status,
        body,
      });
    }
    Ok(())
  }
}
