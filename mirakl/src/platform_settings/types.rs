#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Carrier {
  pub code: String,
  pub label: String,
  pub tracking_url: String,
}