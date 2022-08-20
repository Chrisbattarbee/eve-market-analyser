/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCharactersCharacterIdFleetOk : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdFleetOk {
  /// The character's current fleet ID
  #[serde(rename = "fleet_id")]
  fleet_id: i64,
  /// Member’s role in fleet
  #[serde(rename = "role")]
  role: String,
  /// ID of the squad the member is in. If not applicable, will be set to -1
  #[serde(rename = "squad_id")]
  squad_id: i64,
  /// ID of the wing the member is in. If not applicable, will be set to -1
  #[serde(rename = "wing_id")]
  wing_id: i64
}

impl GetCharactersCharacterIdFleetOk {
  /// 200 ok object
  pub fn new(fleet_id: i64, role: String, squad_id: i64, wing_id: i64) -> GetCharactersCharacterIdFleetOk {
    GetCharactersCharacterIdFleetOk {
      fleet_id: fleet_id,
      role: role,
      squad_id: squad_id,
      wing_id: wing_id
    }
  }

  pub fn set_fleet_id(&mut self, fleet_id: i64) {
    self.fleet_id = fleet_id;
  }

  pub fn with_fleet_id(mut self, fleet_id: i64) -> GetCharactersCharacterIdFleetOk {
    self.fleet_id = fleet_id;
    self
  }

  pub fn fleet_id(&self) -> &i64 {
    &self.fleet_id
  }


  pub fn set_role(&mut self, role: String) {
    self.role = role;
  }

  pub fn with_role(mut self, role: String) -> GetCharactersCharacterIdFleetOk {
    self.role = role;
    self
  }

  pub fn role(&self) -> &String {
    &self.role
  }


  pub fn set_squad_id(&mut self, squad_id: i64) {
    self.squad_id = squad_id;
  }

  pub fn with_squad_id(mut self, squad_id: i64) -> GetCharactersCharacterIdFleetOk {
    self.squad_id = squad_id;
    self
  }

  pub fn squad_id(&self) -> &i64 {
    &self.squad_id
  }


  pub fn set_wing_id(&mut self, wing_id: i64) {
    self.wing_id = wing_id;
  }

  pub fn with_wing_id(mut self, wing_id: i64) -> GetCharactersCharacterIdFleetOk {
    self.wing_id = wing_id;
    self
  }

  pub fn wing_id(&self) -> &i64 {
    &self.wing_id
  }


}


