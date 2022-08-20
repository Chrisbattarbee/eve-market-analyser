/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCharactersCharacterIdLocationOk : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdLocationOk {
  /// solar_system_id integer
  #[serde(rename = "solar_system_id")]
  solar_system_id: i32,
  /// station_id integer
  #[serde(rename = "station_id")]
  station_id: Option<i32>,
  /// structure_id integer
  #[serde(rename = "structure_id")]
  structure_id: Option<i64>
}

impl GetCharactersCharacterIdLocationOk {
  /// 200 ok object
  pub fn new(solar_system_id: i32) -> GetCharactersCharacterIdLocationOk {
    GetCharactersCharacterIdLocationOk {
      solar_system_id: solar_system_id,
      station_id: None,
      structure_id: None
    }
  }

  pub fn set_solar_system_id(&mut self, solar_system_id: i32) {
    self.solar_system_id = solar_system_id;
  }

  pub fn with_solar_system_id(mut self, solar_system_id: i32) -> GetCharactersCharacterIdLocationOk {
    self.solar_system_id = solar_system_id;
    self
  }

  pub fn solar_system_id(&self) -> &i32 {
    &self.solar_system_id
  }


  pub fn set_station_id(&mut self, station_id: i32) {
    self.station_id = Some(station_id);
  }

  pub fn with_station_id(mut self, station_id: i32) -> GetCharactersCharacterIdLocationOk {
    self.station_id = Some(station_id);
    self
  }

  pub fn station_id(&self) -> Option<&i32> {
    self.station_id.as_ref()
  }

  pub fn reset_station_id(&mut self) {
    self.station_id = None;
  }

  pub fn set_structure_id(&mut self, structure_id: i64) {
    self.structure_id = Some(structure_id);
  }

  pub fn with_structure_id(mut self, structure_id: i64) -> GetCharactersCharacterIdLocationOk {
    self.structure_id = Some(structure_id);
    self
  }

  pub fn structure_id(&self) -> Option<&i64> {
    self.structure_id.as_ref()
  }

  pub fn reset_structure_id(&mut self) {
    self.structure_id = None;
  }

}



