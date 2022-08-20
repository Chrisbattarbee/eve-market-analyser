/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetFwLeaderboardsCorporationsYesterdayYesterday1 : yesterday object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFwLeaderboardsCorporationsYesterdayYesterday1 {
  /// Amount of victory points
  #[serde(rename = "amount")]
  amount: Option<i32>,
  /// corporation_id integer
  #[serde(rename = "corporation_id")]
  corporation_id: Option<i32>
}

impl GetFwLeaderboardsCorporationsYesterdayYesterday1 {
  /// yesterday object
  pub fn new() -> GetFwLeaderboardsCorporationsYesterdayYesterday1 {
    GetFwLeaderboardsCorporationsYesterdayYesterday1 {
      amount: None,
      corporation_id: None
    }
  }

  pub fn set_amount(&mut self, amount: i32) {
    self.amount = Some(amount);
  }

  pub fn with_amount(mut self, amount: i32) -> GetFwLeaderboardsCorporationsYesterdayYesterday1 {
    self.amount = Some(amount);
    self
  }

  pub fn amount(&self) -> Option<&i32> {
    self.amount.as_ref()
  }

  pub fn reset_amount(&mut self) {
    self.amount = None;
  }

  pub fn set_corporation_id(&mut self, corporation_id: i32) {
    self.corporation_id = Some(corporation_id);
  }

  pub fn with_corporation_id(mut self, corporation_id: i32) -> GetFwLeaderboardsCorporationsYesterdayYesterday1 {
    self.corporation_id = Some(corporation_id);
    self
  }

  pub fn corporation_id(&self) -> Option<&i32> {
    self.corporation_id.as_ref()
  }

  pub fn reset_corporation_id(&mut self) {
    self.corporation_id = None;
  }

}



