/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetLoyaltyStoresCorporationIdOffers200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetLoyaltyStoresCorporationIdOffers200Ok {
  /// Analysis kredit cost
  #[serde(rename = "ak_cost")]
  ak_cost: Option<i32>,
  /// isk_cost integer
  #[serde(rename = "isk_cost")]
  isk_cost: i64,
  /// lp_cost integer
  #[serde(rename = "lp_cost")]
  lp_cost: i32,
  /// offer_id integer
  #[serde(rename = "offer_id")]
  offer_id: i32,
  /// quantity integer
  #[serde(rename = "quantity")]
  quantity: i32,
  /// required_items array
  #[serde(rename = "required_items")]
  required_items: Vec<::models::GetLoyaltyStoresCorporationIdOffersRequiredItem>,
  /// type_id integer
  #[serde(rename = "type_id")]
  type_id: i32
}

impl GetLoyaltyStoresCorporationIdOffers200Ok {
  /// 200 ok object
  pub fn new(isk_cost: i64, lp_cost: i32, offer_id: i32, quantity: i32, required_items: Vec<::models::GetLoyaltyStoresCorporationIdOffersRequiredItem>, type_id: i32) -> GetLoyaltyStoresCorporationIdOffers200Ok {
    GetLoyaltyStoresCorporationIdOffers200Ok {
      ak_cost: None,
      isk_cost: isk_cost,
      lp_cost: lp_cost,
      offer_id: offer_id,
      quantity: quantity,
      required_items: required_items,
      type_id: type_id
    }
  }

  pub fn set_ak_cost(&mut self, ak_cost: i32) {
    self.ak_cost = Some(ak_cost);
  }

  pub fn with_ak_cost(mut self, ak_cost: i32) -> GetLoyaltyStoresCorporationIdOffers200Ok {
    self.ak_cost = Some(ak_cost);
    self
  }

  pub fn ak_cost(&self) -> Option<&i32> {
    self.ak_cost.as_ref()
  }

  pub fn reset_ak_cost(&mut self) {
    self.ak_cost = None;
  }

  pub fn set_isk_cost(&mut self, isk_cost: i64) {
    self.isk_cost = isk_cost;
  }

  pub fn with_isk_cost(mut self, isk_cost: i64) -> GetLoyaltyStoresCorporationIdOffers200Ok {
    self.isk_cost = isk_cost;
    self
  }

  pub fn isk_cost(&self) -> &i64 {
    &self.isk_cost
  }


  pub fn set_lp_cost(&mut self, lp_cost: i32) {
    self.lp_cost = lp_cost;
  }

  pub fn with_lp_cost(mut self, lp_cost: i32) -> GetLoyaltyStoresCorporationIdOffers200Ok {
    self.lp_cost = lp_cost;
    self
  }

  pub fn lp_cost(&self) -> &i32 {
    &self.lp_cost
  }


  pub fn set_offer_id(&mut self, offer_id: i32) {
    self.offer_id = offer_id;
  }

  pub fn with_offer_id(mut self, offer_id: i32) -> GetLoyaltyStoresCorporationIdOffers200Ok {
    self.offer_id = offer_id;
    self
  }

  pub fn offer_id(&self) -> &i32 {
    &self.offer_id
  }


  pub fn set_quantity(&mut self, quantity: i32) {
    self.quantity = quantity;
  }

  pub fn with_quantity(mut self, quantity: i32) -> GetLoyaltyStoresCorporationIdOffers200Ok {
    self.quantity = quantity;
    self
  }

  pub fn quantity(&self) -> &i32 {
    &self.quantity
  }


  pub fn set_required_items(&mut self, required_items: Vec<::models::GetLoyaltyStoresCorporationIdOffersRequiredItem>) {
    self.required_items = required_items;
  }

  pub fn with_required_items(mut self, required_items: Vec<::models::GetLoyaltyStoresCorporationIdOffersRequiredItem>) -> GetLoyaltyStoresCorporationIdOffers200Ok {
    self.required_items = required_items;
    self
  }

  pub fn required_items(&self) -> &Vec<::models::GetLoyaltyStoresCorporationIdOffersRequiredItem> {
    &self.required_items
  }


  pub fn set_type_id(&mut self, type_id: i32) {
    self.type_id = type_id;
  }

  pub fn with_type_id(mut self, type_id: i32) -> GetLoyaltyStoresCorporationIdOffers200Ok {
    self.type_id = type_id;
    self
  }

  pub fn type_id(&self) -> &i32 {
    &self.type_id
  }


}



