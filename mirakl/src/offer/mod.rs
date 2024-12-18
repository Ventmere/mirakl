use std::io::Read;

use crate::client::*;
use crate::result::MiraklResult;

mod types;

pub use self::types::*;

#[derive(Serialize)]
pub enum OfferSort {
  #[serde(rename = "totalPrice")]
  TotalPrice,
  #[serde(rename = "price")]
  Price,
  #[serde(rename = "productTitle")]
  ProductTitle,
}

use crate::types::{Pagination, Sort};

pub type ListOffersSort = Sort<OfferSort>;

#[derive(Default, Serialize, Clone)]
pub struct ListOffersParams {
  pub offer_state_codes: Option<Vec<String>>,
  pub sku: Option<String>,
  pub product_id: Option<String>,
  pub favorite: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ListOffersResponse {
  pub offers: Vec<Offer>,
  pub total_count: i32,
}

pub trait OfferApi {
  fn list_offers(
    &self,
    params: &ListOffersParams,
    sort: Option<ListOffersSort>,
    page: Option<Pagination>,
  ) -> MiraklResult<ListOffersResponse>;

  fn import_offers<R: Read + Send + 'static>(
    &self,
    mode: ImportMode,
    file_name: &str,
    r: R,
    mime: &str,
  ) -> MiraklResult<ImportTracking>;

  fn get_offers_import_info(&self, id: i64) -> MiraklResult<ImportInformation>;

  fn get_offers_import_error_report(&self, id: i64) -> MiraklResult<Vec<u8>>;
}

impl OfferApi for MiraklClient {
  fn list_offers(
    &self,
    params: &ListOffersParams,
    sort: Option<ListOffersSort>,
    page: Option<Pagination>,
  ) -> MiraklResult<ListOffersResponse> {
    let mut req = self.request(Method::GET, "/api/offers");

    req = req.query(&params);

    if let Some(sort) = sort {
      req = req.query(&sort);
    }

    if let Some(page) = page {
      req = req.query(&page);
    }
    req.send()?.get_response()
  }

  fn import_offers<R: Read + Send + 'static>(
    &self,
    mode: ImportMode,
    file_name: &str,
    r: R,
    mime: &str,
  ) -> MiraklResult<ImportTracking> {
    use reqwest::blocking::multipart::{Form, Part};

    let form = Form::new()
      .text("import_mode", mode.as_str())
      .part("file", {
        Part::reader(r).file_name(file_name.to_string()).mime_str(mime)?
      });
    let res = self
      .request(Method::POST, "/api/offers/imports")
      .multipart(form)
      .send()?;

    res.get_response()
  }

  fn get_offers_import_info(&self, id: i64) -> MiraklResult<ImportInformation> {
    let res = self
      .request(Method::GET, &format!("/api/offers/imports/{}", id))
      .send()?;

    res.get_response()
  }

  fn get_offers_import_error_report(&self, id: i64) -> MiraklResult<Vec<u8>> {
    let mut data = vec![];
    let mut res = self
      .request(Method::GET, &format!("/api/offers/imports/{}", id))
      .send_status_checked()?;

    res.copy_to(&mut data)?;

    Ok(data)
  }
}
