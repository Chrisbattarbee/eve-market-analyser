/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetFwLeaderboardsOk : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFwLeaderboardsOk {
  #[serde(rename = "kills")]
  kills: ::models::GetFwLeaderboardsKills,
  #[serde(rename = "victory_points")]
  victory_points: ::models::GetFwLeaderboardsVictoryPoints
}

impl GetFwLeaderboardsOk {
  /// 200 ok object
  pub fn new(kills: ::models::GetFwLeaderboardsKills, victory_points: ::models::GetFwLeaderboardsVictoryPoints) -> GetFwLeaderboardsOk {
    GetFwLeaderboardsOk {
      kills: kills,
      victory_points: victory_points
    }
  }

  pub fn set_kills(&mut self, kills: ::models::GetFwLeaderboardsKills) {
    self.kills = kills;
  }

  pub fn with_kills(mut self, kills: ::models::GetFwLeaderboardsKills) -> GetFwLeaderboardsOk {
    self.kills = kills;
    self
  }

  pub fn kills(&self) -> &::models::GetFwLeaderboardsKills {
    &self.kills
  }


  pub fn set_victory_points(&mut self, victory_points: ::models::GetFwLeaderboardsVictoryPoints) {
    self.victory_points = victory_points;
  }

  pub fn with_victory_points(mut self, victory_points: ::models::GetFwLeaderboardsVictoryPoints) -> GetFwLeaderboardsOk {
    self.victory_points = victory_points;
    self
  }

  pub fn victory_points(&self) -> &::models::GetFwLeaderboardsVictoryPoints {
    &self.victory_points
  }


}



