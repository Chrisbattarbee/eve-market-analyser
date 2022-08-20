/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetUniverseMoonsMoonIdOk : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUniverseMoonsMoonIdOk {
  /// moon_id integer
  #[serde(rename = "moon_id")]
  moon_id: i32,
  /// name string
  #[serde(rename = "name")]
  name: String,
  #[serde(rename = "position")]
  position: ::models::GetUniverseMoonsMoonIdPosition,
  /// The solar system this moon is in
  #[serde(rename = "system_id")]
  system_id: i32
}

impl GetUniverseMoonsMoonIdOk {
  /// 200 ok object
  pub fn new(moon_id: i32, name: String, position: ::models::GetUniverseMoonsMoonIdPosition, system_id: i32) -> GetUniverseMoonsMoonIdOk {
    GetUniverseMoonsMoonIdOk {
      moon_id: moon_id,
      name: name,
      position: position,
      system_id: system_id
    }
  }

  pub fn set_moon_id(&mut self, moon_id: i32) {
    self.moon_id = moon_id;
  }

  pub fn with_moon_id(mut self, moon_id: i32) -> GetUniverseMoonsMoonIdOk {
    self.moon_id = moon_id;
    self
  }

  pub fn moon_id(&self) -> &i32 {
    &self.moon_id
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> GetUniverseMoonsMoonIdOk {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_position(&mut self, position: ::models::GetUniverseMoonsMoonIdPosition) {
    self.position = position;
  }

  pub fn with_position(mut self, position: ::models::GetUniverseMoonsMoonIdPosition) -> GetUniverseMoonsMoonIdOk {
    self.position = position;
    self
  }

  pub fn position(&self) -> &::models::GetUniverseMoonsMoonIdPosition {
    &self.position
  }


  pub fn set_system_id(&mut self, system_id: i32) {
    self.system_id = system_id;
  }

  pub fn with_system_id(mut self, system_id: i32) -> GetUniverseMoonsMoonIdOk {
    self.system_id = system_id;
    self
  }

  pub fn system_id(&self) -> &i32 {
    &self.system_id
  }


}


