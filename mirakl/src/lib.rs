#[macro_use]
extern crate serde;

#[macro_use]
mod helpers;
mod types;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MiraklImpl {
  BestbuyCa,
  CatchComAu,
  HudsonsBay,
  Belk,
}

impl MiraklImpl {
  pub(crate) fn get_endpoint(&self) -> &'static str {
    match *self {
      MiraklImpl::BestbuyCa => "https://marketplace.bestbuy.ca",
      MiraklImpl::CatchComAu => "https://marketplace.catch.com.au",
      MiraklImpl::HudsonsBay => "https://hudsonsbayus-prod.mirakl.net",
      MiraklImpl::Belk => "https://belkus-prod.mirakl.net",
    }
  }
}

pub use self::types::*;
pub mod client;
pub mod offer;
pub mod order;
pub mod platform_settings;
pub mod result;
