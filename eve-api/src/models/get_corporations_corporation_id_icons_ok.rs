/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCorporationsCorporationIdIconsOk : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdIconsOk {
  /// px128x128 string
  #[serde(rename = "px128x128")]
  px128x128: Option<String>,
  /// px256x256 string
  #[serde(rename = "px256x256")]
  px256x256: Option<String>,
  /// px64x64 string
  #[serde(rename = "px64x64")]
  px64x64: Option<String>
}

impl GetCorporationsCorporationIdIconsOk {
  /// 200 ok object
  pub fn new() -> GetCorporationsCorporationIdIconsOk {
    GetCorporationsCorporationIdIconsOk {
      px128x128: None,
      px256x256: None,
      px64x64: None
    }
  }

  pub fn set_px128x128(&mut self, px128x128: String) {
    self.px128x128 = Some(px128x128);
  }

  pub fn with_px128x128(mut self, px128x128: String) -> GetCorporationsCorporationIdIconsOk {
    self.px128x128 = Some(px128x128);
    self
  }

  pub fn px128x128(&self) -> Option<&String> {
    self.px128x128.as_ref()
  }

  pub fn reset_px128x128(&mut self) {
    self.px128x128 = None;
  }

  pub fn set_px256x256(&mut self, px256x256: String) {
    self.px256x256 = Some(px256x256);
  }

  pub fn with_px256x256(mut self, px256x256: String) -> GetCorporationsCorporationIdIconsOk {
    self.px256x256 = Some(px256x256);
    self
  }

  pub fn px256x256(&self) -> Option<&String> {
    self.px256x256.as_ref()
  }

  pub fn reset_px256x256(&mut self) {
    self.px256x256 = None;
  }

  pub fn set_px64x64(&mut self, px64x64: String) {
    self.px64x64 = Some(px64x64);
  }

  pub fn with_px64x64(mut self, px64x64: String) -> GetCorporationsCorporationIdIconsOk {
    self.px64x64 = Some(px64x64);
    self
  }

  pub fn px64x64(&self) -> Option<&String> {
    self.px64x64.as_ref()
  }

  pub fn reset_px64x64(&mut self) {
    self.px64x64 = None;
  }

}



