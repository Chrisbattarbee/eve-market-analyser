/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetFwLeaderboardsCharactersKills : Top 100 rankings of pilots by number of kills from yesterday, last week and in total

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFwLeaderboardsCharactersKills {
  /// Top 100 ranking of pilots active in faction warfare by total kills. A pilot is considered \"active\" if they have participated in faction warfare in the past 14 days
  #[serde(rename = "active_total")]
  active_total: Vec<::models::GetFwLeaderboardsCharactersActiveTotalActiveTotal>,
  /// Top 100 ranking of pilots by kills in the past week
  #[serde(rename = "last_week")]
  last_week: Vec<::models::GetFwLeaderboardsCharactersLastWeekLastWeek>,
  /// Top 100 ranking of pilots by kills in the past day
  #[serde(rename = "yesterday")]
  yesterday: Vec<::models::GetFwLeaderboardsCharactersYesterdayYesterday>
}

impl GetFwLeaderboardsCharactersKills {
  /// Top 100 rankings of pilots by number of kills from yesterday, last week and in total
  pub fn new(active_total: Vec<::models::GetFwLeaderboardsCharactersActiveTotalActiveTotal>, last_week: Vec<::models::GetFwLeaderboardsCharactersLastWeekLastWeek>, yesterday: Vec<::models::GetFwLeaderboardsCharactersYesterdayYesterday>) -> GetFwLeaderboardsCharactersKills {
    GetFwLeaderboardsCharactersKills {
      active_total: active_total,
      last_week: last_week,
      yesterday: yesterday
    }
  }

  pub fn set_active_total(&mut self, active_total: Vec<::models::GetFwLeaderboardsCharactersActiveTotalActiveTotal>) {
    self.active_total = active_total;
  }

  pub fn with_active_total(mut self, active_total: Vec<::models::GetFwLeaderboardsCharactersActiveTotalActiveTotal>) -> GetFwLeaderboardsCharactersKills {
    self.active_total = active_total;
    self
  }

  pub fn active_total(&self) -> &Vec<::models::GetFwLeaderboardsCharactersActiveTotalActiveTotal> {
    &self.active_total
  }


  pub fn set_last_week(&mut self, last_week: Vec<::models::GetFwLeaderboardsCharactersLastWeekLastWeek>) {
    self.last_week = last_week;
  }

  pub fn with_last_week(mut self, last_week: Vec<::models::GetFwLeaderboardsCharactersLastWeekLastWeek>) -> GetFwLeaderboardsCharactersKills {
    self.last_week = last_week;
    self
  }

  pub fn last_week(&self) -> &Vec<::models::GetFwLeaderboardsCharactersLastWeekLastWeek> {
    &self.last_week
  }


  pub fn set_yesterday(&mut self, yesterday: Vec<::models::GetFwLeaderboardsCharactersYesterdayYesterday>) {
    self.yesterday = yesterday;
  }

  pub fn with_yesterday(mut self, yesterday: Vec<::models::GetFwLeaderboardsCharactersYesterdayYesterday>) -> GetFwLeaderboardsCharactersKills {
    self.yesterday = yesterday;
    self
  }

  pub fn yesterday(&self) -> &Vec<::models::GetFwLeaderboardsCharactersYesterdayYesterday> {
    &self.yesterday
  }


}



