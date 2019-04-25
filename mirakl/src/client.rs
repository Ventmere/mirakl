use r#impl::MiraklImpl;
use reqwest::{Client, Response};
pub use reqwest::{Method, RequestBuilder};
use result::{MiraklError, MiraklResult};
use serde::Deserialize;
use serde_json;

pub struct MiraklClient {
  http: Client,
  endpoint: &'static str,
  token: String,
}

impl MiraklClient {
  pub fn new<I: MiraklImpl>(i: I, token: &str) -> Self {
    Self::with_http_client(i, token, Client::new())
  }

  pub fn with_http_client<I: MiraklImpl>(_i: I, token: &str, http: Client) -> Self {
    Self {
      endpoint: I::ENDPOINT,
      token: token.to_owned(),
      http,
    }
  }

  pub fn request(&self, method: Method, path: &str) -> RequestBuilder {
    use reqwest::{
      header::{qitem, Accept, Authorization, CacheControl, CacheDirective},
      mime,
    };
    let mut b = self
      .http
      .request(method, &format!("{}{}", self.endpoint, path));
    b.header(Authorization(self.token.clone()))
      .header(CacheControl(vec![CacheDirective::NoCache]))
      .header(Accept(vec![qitem(mime::APPLICATION_JSON)]));
    b
  }
}

pub trait MiraklResponse {
  fn get_response<T: for<'de> Deserialize<'de>>(&mut self) -> MiraklResult<T>;
  fn no_content(&mut self) -> MiraklResult<()>;
}

impl MiraklResponse for Response {
  fn get_response<T: for<'de> Deserialize<'de>>(&mut self) -> MiraklResult<T> {
    let body = self.text()?;

    if !self.status().is_success() {
      return Err(MiraklError::Request {
        path: self.url().to_string(),
        status: self.status(),
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

  fn no_content(&mut self) -> MiraklResult<()> {
    let body = self.text()?;

    if !self.status().is_success() {
      return Err(MiraklError::Request {
        path: self.url().to_string(),
        status: self.status(),
        body,
      });
    }
    Ok(())
  }
}
