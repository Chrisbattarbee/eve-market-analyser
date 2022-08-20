/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetFleetsFleetIdWingsNotFound : Not found

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFleetsFleetIdWingsNotFound {
  /// Not found message
  #[serde(rename = "error")]
  error: Option<String>
}

impl GetFleetsFleetIdWingsNotFound {
  /// Not found
  pub fn new() -> GetFleetsFleetIdWingsNotFound {
    GetFleetsFleetIdWingsNotFound {
      error: None
    }
  }

  pub fn set_error(&mut self, error: String) {
    self.error = Some(error);
  }

  pub fn with_error(mut self, error: String) -> GetFleetsFleetIdWingsNotFound {
    self.error = Some(error);
    self
  }

  pub fn error(&self) -> Option<&String> {
    self.error.as_ref()
  }

  pub fn reset_error(&mut self) {
    self.error = None;
  }

}



