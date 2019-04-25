use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderDocument {
  pub date_uploaded: DateTime<Utc>,
  pub file_name: String,
  pub file_size: i64,
  pub id: i64,
  pub order_id: String,
  #[serde(rename = "type")]
  pub type_: String,
}
