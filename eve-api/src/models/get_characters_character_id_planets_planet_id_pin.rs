/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCharactersCharacterIdPlanetsPlanetIdPin : pin object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdPlanetsPlanetIdPin {
  /// contents array
  #[serde(rename = "contents")]
  contents: Option<Vec<::models::GetCharactersCharacterIdPlanetsPlanetIdContent>>,
  /// expiry_time string
  #[serde(rename = "expiry_time")]
  expiry_time: Option<String>,
  #[serde(rename = "extractor_details")]
  extractor_details: Option<::models::GetCharactersCharacterIdPlanetsPlanetIdExtractorDetails>,
  #[serde(rename = "factory_details")]
  factory_details: Option<::models::GetCharactersCharacterIdPlanetsPlanetIdFactoryDetails>,
  /// install_time string
  #[serde(rename = "install_time")]
  install_time: Option<String>,
  /// last_cycle_start string
  #[serde(rename = "last_cycle_start")]
  last_cycle_start: Option<String>,
  /// latitude number
  #[serde(rename = "latitude")]
  latitude: f32,
  /// longitude number
  #[serde(rename = "longitude")]
  longitude: f32,
  /// pin_id integer
  #[serde(rename = "pin_id")]
  pin_id: i64,
  /// schematic_id integer
  #[serde(rename = "schematic_id")]
  schematic_id: Option<i32>,
  /// type_id integer
  #[serde(rename = "type_id")]
  type_id: i32
}

impl GetCharactersCharacterIdPlanetsPlanetIdPin {
  /// pin object
  pub fn new(latitude: f32, longitude: f32, pin_id: i64, type_id: i32) -> GetCharactersCharacterIdPlanetsPlanetIdPin {
    GetCharactersCharacterIdPlanetsPlanetIdPin {
      contents: None,
      expiry_time: None,
      extractor_details: None,
      factory_details: None,
      install_time: None,
      last_cycle_start: None,
      latitude: latitude,
      longitude: longitude,
      pin_id: pin_id,
      schematic_id: None,
      type_id: type_id
    }
  }

  pub fn set_contents(&mut self, contents: Vec<::models::GetCharactersCharacterIdPlanetsPlanetIdContent>) {
    self.contents = Some(contents);
  }

  pub fn with_contents(mut self, contents: Vec<::models::GetCharactersCharacterIdPlanetsPlanetIdContent>) -> GetCharactersCharacterIdPlanetsPlanetIdPin {
    self.contents = Some(contents);
    self
  }

  pub fn contents(&self) -> Option<&Vec<::models::GetCharactersCharacterIdPlanetsPlanetIdContent>> {
    self.contents.as_ref()
  }

  pub fn reset_contents(&mut self) {
    self.contents = None;
  }

  pub fn set_expiry_time(&mut self, expiry_time: String) {
    self.expiry_time = Some(expiry_time);
  }

  pub fn with_expiry_time(mut self, expiry_time: String) -> GetCharactersCharacterIdPlanetsPlanetIdPin {
    self.expiry_time = Some(expiry_time);
    self
  }

  pub fn expiry_time(&self) -> Option<&String> {
    self.expiry_time.as_ref()
  }

  pub fn reset_expiry_time(&mut self) {
    self.expiry_time = None;
  }

  pub fn set_extractor_details(&mut self, extractor_details: ::models::GetCharactersCharacterIdPlanetsPlanetIdExtractorDetails) {
    self.extractor_details = Some(extractor_details);
  }

  pub fn with_extractor_details(mut self, extractor_details: ::models::GetCharactersCharacterIdPlanetsPlanetIdExtractorDetails) -> GetCharactersCharacterIdPlanetsPlanetIdPin {
    self.extractor_details = Some(extractor_details);
    self
  }

  pub fn extractor_details(&self) -> Option<&::models::GetCharactersCharacterIdPlanetsPlanetIdExtractorDetails> {
    self.extractor_details.as_ref()
  }

  pub fn reset_extractor_details(&mut self) {
    self.extractor_details = None;
  }

  pub fn set_factory_details(&mut self, factory_details: ::models::GetCharactersCharacterIdPlanetsPlanetIdFactoryDetails) {
    self.factory_details = Some(factory_details);
  }

  pub fn with_factory_details(mut self, factory_details: ::models::GetCharactersCharacterIdPlanetsPlanetIdFactoryDetails) -> GetCharactersCharacterIdPlanetsPlanetIdPin {
    self.factory_details = Some(factory_details);
    self
  }

  pub fn factory_details(&self) -> Option<&::models::GetCharactersCharacterIdPlanetsPlanetIdFactoryDetails> {
    self.factory_details.as_ref()
  }

  pub fn reset_factory_details(&mut self) {
    self.factory_details = None;
  }

  pub fn set_install_time(&mut self, install_time: String) {
    self.install_time = Some(install_time);
  }

  pub fn with_install_time(mut self, install_time: String) -> GetCharactersCharacterIdPlanetsPlanetIdPin {
    self.install_time = Some(install_time);
    self
  }

  pub fn install_time(&self) -> Option<&String> {
    self.install_time.as_ref()
  }

  pub fn reset_install_time(&mut self) {
    self.install_time = None;
  }

  pub fn set_last_cycle_start(&mut self, last_cycle_start: String) {
    self.last_cycle_start = Some(last_cycle_start);
  }

  pub fn with_last_cycle_start(mut self, last_cycle_start: String) -> GetCharactersCharacterIdPlanetsPlanetIdPin {
    self.last_cycle_start = Some(last_cycle_start);
    self
  }

  pub fn last_cycle_start(&self) -> Option<&String> {
    self.last_cycle_start.as_ref()
  }

  pub fn reset_last_cycle_start(&mut self) {
    self.last_cycle_start = None;
  }

  pub fn set_latitude(&mut self, latitude: f32) {
    self.latitude = latitude;
  }

  pub fn with_latitude(mut self, latitude: f32) -> GetCharactersCharacterIdPlanetsPlanetIdPin {
    self.latitude = latitude;
    self
  }

  pub fn latitude(&self) -> &f32 {
    &self.latitude
  }


  pub fn set_longitude(&mut self, longitude: f32) {
    self.longitude = longitude;
  }

  pub fn with_longitude(mut self, longitude: f32) -> GetCharactersCharacterIdPlanetsPlanetIdPin {
    self.longitude = longitude;
    self
  }

  pub fn longitude(&self) -> &f32 {
    &self.longitude
  }


  pub fn set_pin_id(&mut self, pin_id: i64) {
    self.pin_id = pin_id;
  }

  pub fn with_pin_id(mut self, pin_id: i64) -> GetCharactersCharacterIdPlanetsPlanetIdPin {
    self.pin_id = pin_id;
    self
  }

  pub fn pin_id(&self) -> &i64 {
    &self.pin_id
  }


  pub fn set_schematic_id(&mut self, schematic_id: i32) {
    self.schematic_id = Some(schematic_id);
  }

  pub fn with_schematic_id(mut self, schematic_id: i32) -> GetCharactersCharacterIdPlanetsPlanetIdPin {
    self.schematic_id = Some(schematic_id);
    self
  }

  pub fn schematic_id(&self) -> Option<&i32> {
    self.schematic_id.as_ref()
  }

  pub fn reset_schematic_id(&mut self) {
    self.schematic_id = None;
  }

  pub fn set_type_id(&mut self, type_id: i32) {
    self.type_id = type_id;
  }

  pub fn with_type_id(mut self, type_id: i32) -> GetCharactersCharacterIdPlanetsPlanetIdPin {
    self.type_id = type_id;
    self
  }

  pub fn type_id(&self) -> &i32 {
    &self.type_id
  }


}



