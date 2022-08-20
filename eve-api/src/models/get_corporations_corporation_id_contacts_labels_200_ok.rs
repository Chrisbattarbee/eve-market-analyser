/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCorporationsCorporationIdContactsLabels200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdContactsLabels200Ok {
  /// label_id integer
  #[serde(rename = "label_id")]
  label_id: i64,
  /// label_name string
  #[serde(rename = "label_name")]
  label_name: String
}

impl GetCorporationsCorporationIdContactsLabels200Ok {
  /// 200 ok object
  pub fn new(label_id: i64, label_name: String) -> GetCorporationsCorporationIdContactsLabels200Ok {
    GetCorporationsCorporationIdContactsLabels200Ok {
      label_id: label_id,
      label_name: label_name
    }
  }

  pub fn set_label_id(&mut self, label_id: i64) {
    self.label_id = label_id;
  }

  pub fn with_label_id(mut self, label_id: i64) -> GetCorporationsCorporationIdContactsLabels200Ok {
    self.label_id = label_id;
    self
  }

  pub fn label_id(&self) -> &i64 {
    &self.label_id
  }


  pub fn set_label_name(&mut self, label_name: String) {
    self.label_name = label_name;
  }

  pub fn with_label_name(mut self, label_name: String) -> GetCorporationsCorporationIdContactsLabels200Ok {
    self.label_name = label_name;
    self
  }

  pub fn label_name(&self) -> &String {
    &self.label_name
  }


}



