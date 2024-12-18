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
  SnappyGiftsUs,
  BedBathAndBeyondUs,
  MacysUs,
  Worten,
  RueDuCommerce,
  Bunnings,
  Mathis,
  MediaMarktSaturn,
  GE,
  Kohlsus,
  GACD,
  KrogerPreprod,
  Kroger,
  TSC,
  Rein,
  UFA,
  Carrefour,
  Boulanger,
  PCComponentes,
  Adeo,
  JBGroup,
  BigBang,
}

impl MiraklImpl {
  pub(crate) fn get_endpoint(&self) -> &'static str {
    match *self {
      MiraklImpl::BestbuyCa => "https://marketplace.bestbuy.ca",
      MiraklImpl::CatchComAu => "https://marketplace.catch.com.au",
      MiraklImpl::HudsonsBay => "https://hudsonsbayus-prod.mirakl.net",
      MiraklImpl::Belk => "https://belkus-prod.mirakl.net",
      MiraklImpl::SnappyGiftsUs => "https://snappygiftsus-prod.mirakl.net",
      MiraklImpl::BedBathAndBeyondUs => "https://bedbathandbeyondus-prod.mirakl.net",
      MiraklImpl::MacysUs => "https://macysus-prod.mirakl.net",
      MiraklImpl::Worten => "https://marketplace.worten.pt",
      MiraklImpl::RueDuCommerce => "https://mirakl-web.groupe-rueducommerce.fr",
      MiraklImpl::Bunnings => "https://marketlink.bunnings.com.au",
      MiraklImpl::Mathis => "https://mathisbrothersus-prod.mirakl.net",
      MiraklImpl::MediaMarktSaturn => "https://mediamarktsaturn.mirakl.net",
      MiraklImpl::GE => "https://generalelectricus-prod.mirakl.net",
      MiraklImpl::Kohlsus => "https://kohlsus-prod.mirakl.net",
      MiraklImpl::GACD => "https://tdmp.mirakl.net",
      MiraklImpl::KrogerPreprod => "https://kroger-preprod.mirakl.net",
      MiraklImpl::Kroger => "https://kroger-prod.mirakl.net",
      MiraklImpl::TSC => "https://tsc.mirakl.net",
      MiraklImpl::Rein => "https://rein-prod.mirakl.net",
      MiraklImpl::UFA => "https://unitedfarmersca-prod.mirakl.net",
      MiraklImpl::Carrefour => "https://carrefoures-prod.mirakl.net",
      MiraklImpl::Boulanger => "https://merchant.boulanger.com",
      MiraklImpl::PCComponentes => "https://pccomponentes-prod.mirakl.net",
      MiraklImpl::Adeo => "https://adeo-marketplace.mirakl.net",
      MiraklImpl::JBGroup => "https://marketplace-jbgroup.mirakl.net",
      MiraklImpl::BigBang => "https://bigbangmarketplace.mirakl.net",
    }
  }
}

pub use self::types::*;
pub mod client;
pub mod offer;
pub mod order;
pub mod platform_settings;
pub mod result;
