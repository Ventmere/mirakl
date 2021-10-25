use crate::client::*;
use crate::result::MiraklResult;

mod types;

pub use self::types::*;

#[derive(Serialize, Deserialize)]
pub struct ListCarriersResponse {
  pub carriers: Vec<Carrier>,
}

pub trait PlatformSettingsApi {
  fn list_all_carriers(
    &self,
  ) -> MiraklResult<ListCarriersResponse>;
}

impl PlatformSettingsApi for MiraklClient {
  fn list_all_carriers(
    &self,
  ) -> MiraklResult<ListCarriersResponse> {
    let req = self.request(Method::GET, "/api/shipping/carriers");
    req.send()?.get_response()
  }
}
