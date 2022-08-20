/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCharactersCharacterIdMailLabelsLabel : label object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdMailLabelsLabel {
  /// color string
  #[serde(rename = "color")]
  color: Option<String>,
  /// label_id integer
  #[serde(rename = "label_id")]
  label_id: Option<i32>,
  /// name string
  #[serde(rename = "name")]
  name: Option<String>,
  /// unread_count integer
  #[serde(rename = "unread_count")]
  unread_count: Option<i32>
}

impl GetCharactersCharacterIdMailLabelsLabel {
  /// label object
  pub fn new() -> GetCharactersCharacterIdMailLabelsLabel {
    GetCharactersCharacterIdMailLabelsLabel {
      color: None,
      label_id: None,
      name: None,
      unread_count: None
    }
  }

  pub fn set_color(&mut self, color: String) {
    self.color = Some(color);
  }

  pub fn with_color(mut self, color: String) -> GetCharactersCharacterIdMailLabelsLabel {
    self.color = Some(color);
    self
  }

  pub fn color(&self) -> Option<&String> {
    self.color.as_ref()
  }

  pub fn reset_color(&mut self) {
    self.color = None;
  }

  pub fn set_label_id(&mut self, label_id: i32) {
    self.label_id = Some(label_id);
  }

  pub fn with_label_id(mut self, label_id: i32) -> GetCharactersCharacterIdMailLabelsLabel {
    self.label_id = Some(label_id);
    self
  }

  pub fn label_id(&self) -> Option<&i32> {
    self.label_id.as_ref()
  }

  pub fn reset_label_id(&mut self) {
    self.label_id = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> GetCharactersCharacterIdMailLabelsLabel {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_unread_count(&mut self, unread_count: i32) {
    self.unread_count = Some(unread_count);
  }

  pub fn with_unread_count(mut self, unread_count: i32) -> GetCharactersCharacterIdMailLabelsLabel {
    self.unread_count = Some(unread_count);
    self
  }

  pub fn unread_count(&self) -> Option<&i32> {
    self.unread_count.as_ref()
  }

  pub fn reset_unread_count(&mut self) {
    self.unread_count = None;
  }

}



