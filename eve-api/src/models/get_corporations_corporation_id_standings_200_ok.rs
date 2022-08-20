/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCorporationsCorporationIdStandings200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdStandings200Ok {
  /// from_id integer
  #[serde(rename = "from_id")]
  from_id: i32,
  /// from_type string
  #[serde(rename = "from_type")]
  from_type: String,
  /// standing number
  #[serde(rename = "standing")]
  standing: f32
}

impl GetCorporationsCorporationIdStandings200Ok {
  /// 200 ok object
  pub fn new(from_id: i32, from_type: String, standing: f32) -> GetCorporationsCorporationIdStandings200Ok {
    GetCorporationsCorporationIdStandings200Ok {
      from_id: from_id,
      from_type: from_type,
      standing: standing
    }
  }

  pub fn set_from_id(&mut self, from_id: i32) {
    self.from_id = from_id;
  }

  pub fn with_from_id(mut self, from_id: i32) -> GetCorporationsCorporationIdStandings200Ok {
    self.from_id = from_id;
    self
  }

  pub fn from_id(&self) -> &i32 {
    &self.from_id
  }


  pub fn set_from_type(&mut self, from_type: String) {
    self.from_type = from_type;
  }

  pub fn with_from_type(mut self, from_type: String) -> GetCorporationsCorporationIdStandings200Ok {
    self.from_type = from_type;
    self
  }

  pub fn from_type(&self) -> &String {
    &self.from_type
  }


  pub fn set_standing(&mut self, standing: f32) {
    self.standing = standing;
  }

  pub fn with_standing(mut self, standing: f32) -> GetCorporationsCorporationIdStandings200Ok {
    self.standing = standing;
    self
  }

  pub fn standing(&self) -> &f32 {
    &self.standing
  }


}



