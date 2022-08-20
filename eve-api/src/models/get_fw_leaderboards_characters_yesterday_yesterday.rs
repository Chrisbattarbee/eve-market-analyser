/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetFwLeaderboardsCharactersYesterdayYesterday : yesterday object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFwLeaderboardsCharactersYesterdayYesterday {
  /// Amount of kills
  #[serde(rename = "amount")]
  amount: Option<i32>,
  /// character_id integer
  #[serde(rename = "character_id")]
  character_id: Option<i32>
}

impl GetFwLeaderboardsCharactersYesterdayYesterday {
  /// yesterday object
  pub fn new() -> GetFwLeaderboardsCharactersYesterdayYesterday {
    GetFwLeaderboardsCharactersYesterdayYesterday {
      amount: None,
      character_id: None
    }
  }

  pub fn set_amount(&mut self, amount: i32) {
    self.amount = Some(amount);
  }

  pub fn with_amount(mut self, amount: i32) -> GetFwLeaderboardsCharactersYesterdayYesterday {
    self.amount = Some(amount);
    self
  }

  pub fn amount(&self) -> Option<&i32> {
    self.amount.as_ref()
  }

  pub fn reset_amount(&mut self) {
    self.amount = None;
  }

  pub fn set_character_id(&mut self, character_id: i32) {
    self.character_id = Some(character_id);
  }

  pub fn with_character_id(mut self, character_id: i32) -> GetFwLeaderboardsCharactersYesterdayYesterday {
    self.character_id = Some(character_id);
    self
  }

  pub fn character_id(&self) -> Option<&i32> {
    self.character_id.as_ref()
  }

  pub fn reset_character_id(&mut self) {
    self.character_id = None;
  }

}



