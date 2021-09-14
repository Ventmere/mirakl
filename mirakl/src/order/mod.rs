use chrono::{DateTime, Utc};
use client::*;
use result::MiraklResult;

pub mod document;
mod types;

pub use self::types::*;

#[derive(Debug, Clone, Copy, Serialize)]
pub enum OrderSort {
  #[serde(rename = "dateCreated")]
  DateCreated,
}

use types::{Pagination, Sort};

pub type ListOrdersSort = Sort<OrderSort>;

#[derive(Debug, Default, Serialize, Clone)]
pub struct ListOrdersParams {
  pub order_ids: Option<Vec<String>>,
  pub order_state_codes: Option<Vec<OrderState>>,
  pub channel_codes: Option<Vec<String>>,
  pub start_date: Option<DateTime<Utc>>,
  pub end_date: Option<DateTime<Utc>>,
  pub start_update_date: Option<DateTime<Utc>>,
  pub end_update_date: Option<DateTime<Utc>>,
  pub paginate: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListOrdersResponse {
  pub orders: Vec<Order>,
  pub total_count: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrderAccept {
  pub order_lines: Vec<OrderAcceptLine>,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrderAcceptLine {
  pub accepted: bool,
  pub id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrderTracking {
  pub carrier_code: Option<String>,
  pub carrier_name: Option<String>,
  pub carrier_url: Option<String>,
  pub tracking_number: Option<String>,
}

pub trait OrderApi {
  fn list_orders(
    &self,
    params: &ListOrdersParams,
    sort: Option<ListOrdersSort>,
    page: Option<Pagination>,
  ) -> MiraklResult<ListOrdersResponse>;

  fn accept(&self, order_id: &str, accept: &OrderAccept) -> MiraklResult<()>;

  fn update_tracking(&self, order_id: &str, tracking: &OrderTracking) -> MiraklResult<()>;

  /// Update the shipment of the order which is in "SHIPPING"
  /// state to "SHIPPED" state
  fn ship(&self, order_id: &str) -> MiraklResult<()>;
}

impl OrderApi for MiraklClient {
  fn list_orders(
    &self,
    params: &ListOrdersParams,
    sort: Option<ListOrdersSort>,
    page: Option<Pagination>,
  ) -> MiraklResult<ListOrdersResponse> {
    let mut req = self.request(Method::Get, "/api/orders");

    let mut params = params.clone();

    if let Some(order_ids) = params.order_ids.take() {
      req.query(&[("order_ids", order_ids.join(","))]);
    }

    if let Some(order_state_codes) = params.order_state_codes.take() {
      req.query(&[(
        "order_state_codes",
        order_state_codes
          .iter()
          .map(ToString::to_string)
          .collect::<Vec<_>>()
          .join(","),
      )]);
    }

    req.query(&params);

    if let Some(sort) = sort {
      req.query(&sort);
    }

    if let Some(page) = page {
      req.query(&page);
    }
    req.send()?.get_response()
  }

  fn accept(&self, order_id: &str, accept: &OrderAccept) -> MiraklResult<()> {
    self
      .request(Method::Put, &format!("/api/orders/{}/accept", order_id))
      .json(accept)
      .send()?
      .no_content()
  }

  fn update_tracking(&self, order_id: &str, tracking: &OrderTracking) -> MiraklResult<()> {
    self
      .request(Method::Put, &format!("/api/orders/{}/tracking", order_id))
      .json(tracking)
      .send()?
      .no_content()
  }

  fn ship(&self, order_id: &str) -> MiraklResult<()> {
    self
      .request(Method::Put, &format!("/api/orders/{}/ship", order_id))
      .json(&serde_json::Value::Null) // workaround for Error 411 (Length Required)
      .send()?
      .no_content()
  }
}
