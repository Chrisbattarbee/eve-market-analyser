/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetUniverseGraphicsGraphicIdOk : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUniverseGraphicsGraphicIdOk {
  /// collision_file string
  #[serde(rename = "collision_file")]
  collision_file: Option<String>,
  /// graphic_file string
  #[serde(rename = "graphic_file")]
  graphic_file: Option<String>,
  /// graphic_id integer
  #[serde(rename = "graphic_id")]
  graphic_id: i32,
  /// icon_folder string
  #[serde(rename = "icon_folder")]
  icon_folder: Option<String>,
  /// sof_dna string
  #[serde(rename = "sof_dna")]
  sof_dna: Option<String>,
  /// sof_fation_name string
  #[serde(rename = "sof_fation_name")]
  sof_fation_name: Option<String>,
  /// sof_hull_name string
  #[serde(rename = "sof_hull_name")]
  sof_hull_name: Option<String>,
  /// sof_race_name string
  #[serde(rename = "sof_race_name")]
  sof_race_name: Option<String>
}

impl GetUniverseGraphicsGraphicIdOk {
  /// 200 ok object
  pub fn new(graphic_id: i32) -> GetUniverseGraphicsGraphicIdOk {
    GetUniverseGraphicsGraphicIdOk {
      collision_file: None,
      graphic_file: None,
      graphic_id: graphic_id,
      icon_folder: None,
      sof_dna: None,
      sof_fation_name: None,
      sof_hull_name: None,
      sof_race_name: None
    }
  }

  pub fn set_collision_file(&mut self, collision_file: String) {
    self.collision_file = Some(collision_file);
  }

  pub fn with_collision_file(mut self, collision_file: String) -> GetUniverseGraphicsGraphicIdOk {
    self.collision_file = Some(collision_file);
    self
  }

  pub fn collision_file(&self) -> Option<&String> {
    self.collision_file.as_ref()
  }

  pub fn reset_collision_file(&mut self) {
    self.collision_file = None;
  }

  pub fn set_graphic_file(&mut self, graphic_file: String) {
    self.graphic_file = Some(graphic_file);
  }

  pub fn with_graphic_file(mut self, graphic_file: String) -> GetUniverseGraphicsGraphicIdOk {
    self.graphic_file = Some(graphic_file);
    self
  }

  pub fn graphic_file(&self) -> Option<&String> {
    self.graphic_file.as_ref()
  }

  pub fn reset_graphic_file(&mut self) {
    self.graphic_file = None;
  }

  pub fn set_graphic_id(&mut self, graphic_id: i32) {
    self.graphic_id = graphic_id;
  }

  pub fn with_graphic_id(mut self, graphic_id: i32) -> GetUniverseGraphicsGraphicIdOk {
    self.graphic_id = graphic_id;
    self
  }

  pub fn graphic_id(&self) -> &i32 {
    &self.graphic_id
  }


  pub fn set_icon_folder(&mut self, icon_folder: String) {
    self.icon_folder = Some(icon_folder);
  }

  pub fn with_icon_folder(mut self, icon_folder: String) -> GetUniverseGraphicsGraphicIdOk {
    self.icon_folder = Some(icon_folder);
    self
  }

  pub fn icon_folder(&self) -> Option<&String> {
    self.icon_folder.as_ref()
  }

  pub fn reset_icon_folder(&mut self) {
    self.icon_folder = None;
  }

  pub fn set_sof_dna(&mut self, sof_dna: String) {
    self.sof_dna = Some(sof_dna);
  }

  pub fn with_sof_dna(mut self, sof_dna: String) -> GetUniverseGraphicsGraphicIdOk {
    self.sof_dna = Some(sof_dna);
    self
  }

  pub fn sof_dna(&self) -> Option<&String> {
    self.sof_dna.as_ref()
  }

  pub fn reset_sof_dna(&mut self) {
    self.sof_dna = None;
  }

  pub fn set_sof_fation_name(&mut self, sof_fation_name: String) {
    self.sof_fation_name = Some(sof_fation_name);
  }

  pub fn with_sof_fation_name(mut self, sof_fation_name: String) -> GetUniverseGraphicsGraphicIdOk {
    self.sof_fation_name = Some(sof_fation_name);
    self
  }

  pub fn sof_fation_name(&self) -> Option<&String> {
    self.sof_fation_name.as_ref()
  }

  pub fn reset_sof_fation_name(&mut self) {
    self.sof_fation_name = None;
  }

  pub fn set_sof_hull_name(&mut self, sof_hull_name: String) {
    self.sof_hull_name = Some(sof_hull_name);
  }

  pub fn with_sof_hull_name(mut self, sof_hull_name: String) -> GetUniverseGraphicsGraphicIdOk {
    self.sof_hull_name = Some(sof_hull_name);
    self
  }

  pub fn sof_hull_name(&self) -> Option<&String> {
    self.sof_hull_name.as_ref()
  }

  pub fn reset_sof_hull_name(&mut self) {
    self.sof_hull_name = None;
  }

  pub fn set_sof_race_name(&mut self, sof_race_name: String) {
    self.sof_race_name = Some(sof_race_name);
  }

  pub fn with_sof_race_name(mut self, sof_race_name: String) -> GetUniverseGraphicsGraphicIdOk {
    self.sof_race_name = Some(sof_race_name);
    self
  }

  pub fn sof_race_name(&self) -> Option<&String> {
    self.sof_race_name.as_ref()
  }

  pub fn reset_sof_race_name(&mut self) {
    self.sof_race_name = None;
  }

}



