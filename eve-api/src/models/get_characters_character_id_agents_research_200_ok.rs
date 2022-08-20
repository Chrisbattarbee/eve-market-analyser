/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCharactersCharacterIdAgentsResearch200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdAgentsResearch200Ok {
  /// agent_id integer
  #[serde(rename = "agent_id")]
  agent_id: i32,
  /// points_per_day number
  #[serde(rename = "points_per_day")]
  points_per_day: f32,
  /// remainder_points number
  #[serde(rename = "remainder_points")]
  remainder_points: f32,
  /// skill_type_id integer
  #[serde(rename = "skill_type_id")]
  skill_type_id: i32,
  /// started_at string
  #[serde(rename = "started_at")]
  started_at: String
}

impl GetCharactersCharacterIdAgentsResearch200Ok {
  /// 200 ok object
  pub fn new(agent_id: i32, points_per_day: f32, remainder_points: f32, skill_type_id: i32, started_at: String) -> GetCharactersCharacterIdAgentsResearch200Ok {
    GetCharactersCharacterIdAgentsResearch200Ok {
      agent_id: agent_id,
      points_per_day: points_per_day,
      remainder_points: remainder_points,
      skill_type_id: skill_type_id,
      started_at: started_at
    }
  }

  pub fn set_agent_id(&mut self, agent_id: i32) {
    self.agent_id = agent_id;
  }

  pub fn with_agent_id(mut self, agent_id: i32) -> GetCharactersCharacterIdAgentsResearch200Ok {
    self.agent_id = agent_id;
    self
  }

  pub fn agent_id(&self) -> &i32 {
    &self.agent_id
  }


  pub fn set_points_per_day(&mut self, points_per_day: f32) {
    self.points_per_day = points_per_day;
  }

  pub fn with_points_per_day(mut self, points_per_day: f32) -> GetCharactersCharacterIdAgentsResearch200Ok {
    self.points_per_day = points_per_day;
    self
  }

  pub fn points_per_day(&self) -> &f32 {
    &self.points_per_day
  }


  pub fn set_remainder_points(&mut self, remainder_points: f32) {
    self.remainder_points = remainder_points;
  }

  pub fn with_remainder_points(mut self, remainder_points: f32) -> GetCharactersCharacterIdAgentsResearch200Ok {
    self.remainder_points = remainder_points;
    self
  }

  pub fn remainder_points(&self) -> &f32 {
    &self.remainder_points
  }


  pub fn set_skill_type_id(&mut self, skill_type_id: i32) {
    self.skill_type_id = skill_type_id;
  }

  pub fn with_skill_type_id(mut self, skill_type_id: i32) -> GetCharactersCharacterIdAgentsResearch200Ok {
    self.skill_type_id = skill_type_id;
    self
  }

  pub fn skill_type_id(&self) -> &i32 {
    &self.skill_type_id
  }


  pub fn set_started_at(&mut self, started_at: String) {
    self.started_at = started_at;
  }

  pub fn with_started_at(mut self, started_at: String) -> GetCharactersCharacterIdAgentsResearch200Ok {
    self.started_at = started_at;
    self
  }

  pub fn started_at(&self) -> &String {
    &self.started_at
  }


}



