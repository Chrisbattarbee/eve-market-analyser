/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetKillmailsKillmailIdKillmailHashItemsItem : item object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetKillmailsKillmailIdKillmailHashItemsItem {
  /// flag integer
  #[serde(rename = "flag")]
  flag: i32,
  /// item_type_id integer
  #[serde(rename = "item_type_id")]
  item_type_id: i32,
  /// quantity_destroyed integer
  #[serde(rename = "quantity_destroyed")]
  quantity_destroyed: Option<i64>,
  /// quantity_dropped integer
  #[serde(rename = "quantity_dropped")]
  quantity_dropped: Option<i64>,
  /// singleton integer
  #[serde(rename = "singleton")]
  singleton: i32
}

impl GetKillmailsKillmailIdKillmailHashItemsItem {
  /// item object
  pub fn new(flag: i32, item_type_id: i32, singleton: i32) -> GetKillmailsKillmailIdKillmailHashItemsItem {
    GetKillmailsKillmailIdKillmailHashItemsItem {
      flag: flag,
      item_type_id: item_type_id,
      quantity_destroyed: None,
      quantity_dropped: None,
      singleton: singleton
    }
  }

  pub fn set_flag(&mut self, flag: i32) {
    self.flag = flag;
  }

  pub fn with_flag(mut self, flag: i32) -> GetKillmailsKillmailIdKillmailHashItemsItem {
    self.flag = flag;
    self
  }

  pub fn flag(&self) -> &i32 {
    &self.flag
  }


  pub fn set_item_type_id(&mut self, item_type_id: i32) {
    self.item_type_id = item_type_id;
  }

  pub fn with_item_type_id(mut self, item_type_id: i32) -> GetKillmailsKillmailIdKillmailHashItemsItem {
    self.item_type_id = item_type_id;
    self
  }

  pub fn item_type_id(&self) -> &i32 {
    &self.item_type_id
  }


  pub fn set_quantity_destroyed(&mut self, quantity_destroyed: i64) {
    self.quantity_destroyed = Some(quantity_destroyed);
  }

  pub fn with_quantity_destroyed(mut self, quantity_destroyed: i64) -> GetKillmailsKillmailIdKillmailHashItemsItem {
    self.quantity_destroyed = Some(quantity_destroyed);
    self
  }

  pub fn quantity_destroyed(&self) -> Option<&i64> {
    self.quantity_destroyed.as_ref()
  }

  pub fn reset_quantity_destroyed(&mut self) {
    self.quantity_destroyed = None;
  }

  pub fn set_quantity_dropped(&mut self, quantity_dropped: i64) {
    self.quantity_dropped = Some(quantity_dropped);
  }

  pub fn with_quantity_dropped(mut self, quantity_dropped: i64) -> GetKillmailsKillmailIdKillmailHashItemsItem {
    self.quantity_dropped = Some(quantity_dropped);
    self
  }

  pub fn quantity_dropped(&self) -> Option<&i64> {
    self.quantity_dropped.as_ref()
  }

  pub fn reset_quantity_dropped(&mut self) {
    self.quantity_dropped = None;
  }

  pub fn set_singleton(&mut self, singleton: i32) {
    self.singleton = singleton;
  }

  pub fn with_singleton(mut self, singleton: i32) -> GetKillmailsKillmailIdKillmailHashItemsItem {
    self.singleton = singleton;
    self
  }

  pub fn singleton(&self) -> &i32 {
    &self.singleton
  }


}


