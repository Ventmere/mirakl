use crate::client::*;
use crate::result::MiraklResult;
use std::io::prelude::*;

mod types;

pub use self::types::*;

#[derive(Debug, Default, Serialize, Clone)]
pub struct ListOrderDocumentsParams {
  pub order_ids: Option<Vec<String>>,
  pub shop_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListOrderDocumentsResponse {
  pub order_documents: Vec<OrderDocument>,
  pub total_count: i32,
}

#[derive(Debug, Default, Serialize, Clone)]
pub struct DownloadOrderDocumentsParams {
  pub order_ids: Option<Vec<String>>,
  pub document_ids: Option<Vec<i64>>,
  pub document_codes: Option<Vec<String>>,
  pub shop_id: Option<i64>,
}

pub trait OrderDocumentApi {
  fn list_order_documents(
    &self,
    params: &ListOrderDocumentsParams,
  ) -> MiraklResult<ListOrderDocumentsResponse>;

  fn download_order_documents<W: Write>(
    &self,
    params: &DownloadOrderDocumentsParams,
    w: W,
  ) -> MiraklResult<u64>;
}

impl OrderDocumentApi for MiraklClient {
  fn list_order_documents(
    &self,
    params: &ListOrderDocumentsParams,
  ) -> MiraklResult<ListOrderDocumentsResponse> {
    let mut req = self.request(Method::GET, "/api/orders/documents");

    let mut params = params.clone();

    if let Some(order_ids) = params.order_ids.take() {
      req = req.query(&[("order_ids", order_ids.join(","))]);
    }

    if let Some(shop_id) = params.shop_id.take() {
      req = req.query(&[("shop_id", shop_id)]);
    }

    req = req.query(&params);

    req.send()?.get_response()
  }

  fn download_order_documents<W: Write>(
    &self,
    params: &DownloadOrderDocumentsParams,
    mut w: W,
  ) -> MiraklResult<u64> {
    let mut req = self.request(Method::GET, "/api/orders/documents/download");
    let mut params = params.clone();

    if let Some(order_ids) = params.order_ids.take() {
      req = req.query(&[("order_ids", order_ids.join(","))]);
    }

    if let Some(document_ids) = params.document_ids.take() {
      req = req.query(&[(
        "document_ids",
        document_ids
          .into_iter()
          .map(|v| v.to_string())
          .collect::<Vec<String>>()
          .join(","),
      )]);
    }

    if let Some(document_codes) = params.document_codes.take() {
      req = req.query(&[("document_codes", document_codes.join(","))]);
    }

    if let Some(shop_id) = params.shop_id.take() {
      req = req.query(&[("shop_id", shop_id)]);
    }

    req = req.query(&params);

    let mut res = req.send()?;

    ::std::io::copy(&mut res, &mut w).map_err(Into::into)
  }
}
