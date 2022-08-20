/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCharactersCharacterIdMining200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdMining200Ok {
  /// date string
  #[serde(rename = "date")]
  date: String,
  /// quantity integer
  #[serde(rename = "quantity")]
  quantity: i64,
  /// solar_system_id integer
  #[serde(rename = "solar_system_id")]
  solar_system_id: i32,
  /// type_id integer
  #[serde(rename = "type_id")]
  type_id: i32
}

impl GetCharactersCharacterIdMining200Ok {
  /// 200 ok object
  pub fn new(date: String, quantity: i64, solar_system_id: i32, type_id: i32) -> GetCharactersCharacterIdMining200Ok {
    GetCharactersCharacterIdMining200Ok {
      date: date,
      quantity: quantity,
      solar_system_id: solar_system_id,
      type_id: type_id
    }
  }

  pub fn set_date(&mut self, date: String) {
    self.date = date;
  }

  pub fn with_date(mut self, date: String) -> GetCharactersCharacterIdMining200Ok {
    self.date = date;
    self
  }

  pub fn date(&self) -> &String {
    &self.date
  }


  pub fn set_quantity(&mut self, quantity: i64) {
    self.quantity = quantity;
  }

  pub fn with_quantity(mut self, quantity: i64) -> GetCharactersCharacterIdMining200Ok {
    self.quantity = quantity;
    self
  }

  pub fn quantity(&self) -> &i64 {
    &self.quantity
  }


  pub fn set_solar_system_id(&mut self, solar_system_id: i32) {
    self.solar_system_id = solar_system_id;
  }

  pub fn with_solar_system_id(mut self, solar_system_id: i32) -> GetCharactersCharacterIdMining200Ok {
    self.solar_system_id = solar_system_id;
    self
  }

  pub fn solar_system_id(&self) -> &i32 {
    &self.solar_system_id
  }


  pub fn set_type_id(&mut self, type_id: i32) {
    self.type_id = type_id;
  }

  pub fn with_type_id(mut self, type_id: i32) -> GetCharactersCharacterIdMining200Ok {
    self.type_id = type_id;
    self
  }

  pub fn type_id(&self) -> &i32 {
    &self.type_id
  }


}


