/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Forbidden : Forbidden model

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Forbidden {
  /// Forbidden message
  #[serde(rename = "error")]
  error: String,
  /// status code received from SSO
  #[serde(rename = "sso_status")]
  sso_status: Option<i32>
}

impl Forbidden {
  /// Forbidden model
  pub fn new(error: String) -> Forbidden {
    Forbidden {
      error: error,
      sso_status: None
    }
  }

  pub fn set_error(&mut self, error: String) {
    self.error = error;
  }

  pub fn with_error(mut self, error: String) -> Forbidden {
    self.error = error;
    self
  }

  pub fn error(&self) -> &String {
    &self.error
  }


  pub fn set_sso_status(&mut self, sso_status: i32) {
    self.sso_status = Some(sso_status);
  }

  pub fn with_sso_status(mut self, sso_status: i32) -> Forbidden {
    self.sso_status = Some(sso_status);
    self
  }

  pub fn sso_status(&self) -> Option<&i32> {
    self.sso_status.as_ref()
  }

  pub fn reset_sso_status(&mut self) {
    self.sso_status = None;
  }

}



