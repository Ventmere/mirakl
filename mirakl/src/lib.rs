extern crate chrono;
extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
mod helpers;
mod types;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MiraklImpl {
  BestbuyCa,
  CatchComAu,
  HudsonsBay,
}

impl MiraklImpl {
  pub(crate) fn get_endpoint(&self) -> &'static str {
    match *self {
      MiraklImpl::BestbuyCa => "https://marketplace.bestbuy.ca",
      MiraklImpl::CatchComAu => "https://marketplace.catch.com.au",
      MiraklImpl::HudsonsBay => "https://hudsonsbayus-prod.mirakl.net",
    }
  }
}

pub use self::types::*;
pub mod client;
pub mod offer;
pub mod order;
pub mod result;
