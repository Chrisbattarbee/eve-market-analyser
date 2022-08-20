/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetUniverseStructuresStructureIdOk : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUniverseStructuresStructureIdOk {
  /// The full name of the structure
  #[serde(rename = "name")]
  name: String,
  /// The ID of the corporation who owns this particular structure
  #[serde(rename = "owner_id")]
  owner_id: i32,
  #[serde(rename = "position")]
  position: Option<::models::GetUniverseStructuresStructureIdPosition>,
  /// solar_system_id integer
  #[serde(rename = "solar_system_id")]
  solar_system_id: i32,
  /// type_id integer
  #[serde(rename = "type_id")]
  type_id: Option<i32>
}

impl GetUniverseStructuresStructureIdOk {
  /// 200 ok object
  pub fn new(name: String, owner_id: i32, solar_system_id: i32) -> GetUniverseStructuresStructureIdOk {
    GetUniverseStructuresStructureIdOk {
      name: name,
      owner_id: owner_id,
      position: None,
      solar_system_id: solar_system_id,
      type_id: None
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> GetUniverseStructuresStructureIdOk {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_owner_id(&mut self, owner_id: i32) {
    self.owner_id = owner_id;
  }

  pub fn with_owner_id(mut self, owner_id: i32) -> GetUniverseStructuresStructureIdOk {
    self.owner_id = owner_id;
    self
  }

  pub fn owner_id(&self) -> &i32 {
    &self.owner_id
  }


  pub fn set_position(&mut self, position: ::models::GetUniverseStructuresStructureIdPosition) {
    self.position = Some(position);
  }

  pub fn with_position(mut self, position: ::models::GetUniverseStructuresStructureIdPosition) -> GetUniverseStructuresStructureIdOk {
    self.position = Some(position);
    self
  }

  pub fn position(&self) -> Option<&::models::GetUniverseStructuresStructureIdPosition> {
    self.position.as_ref()
  }

  pub fn reset_position(&mut self) {
    self.position = None;
  }

  pub fn set_solar_system_id(&mut self, solar_system_id: i32) {
    self.solar_system_id = solar_system_id;
  }

  pub fn with_solar_system_id(mut self, solar_system_id: i32) -> GetUniverseStructuresStructureIdOk {
    self.solar_system_id = solar_system_id;
    self
  }

  pub fn solar_system_id(&self) -> &i32 {
    &self.solar_system_id
  }


  pub fn set_type_id(&mut self, type_id: i32) {
    self.type_id = Some(type_id);
  }

  pub fn with_type_id(mut self, type_id: i32) -> GetUniverseStructuresStructureIdOk {
    self.type_id = Some(type_id);
    self
  }

  pub fn type_id(&self) -> Option<&i32> {
    self.type_id.as_ref()
  }

  pub fn reset_type_id(&mut self) {
    self.type_id = None;
  }

}


