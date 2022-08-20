/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetSovereigntyCampaigns200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetSovereigntyCampaigns200Ok {
  /// Score for all attacking parties, only present in Defense Events. 
  #[serde(rename = "attackers_score")]
  attackers_score: Option<f32>,
  /// Unique ID for this campaign.
  #[serde(rename = "campaign_id")]
  campaign_id: i32,
  /// The constellation in which the campaign will take place. 
  #[serde(rename = "constellation_id")]
  constellation_id: i32,
  /// Defending alliance, only present in Defense Events 
  #[serde(rename = "defender_id")]
  defender_id: Option<i32>,
  /// Score for the defending alliance, only present in Defense Events. 
  #[serde(rename = "defender_score")]
  defender_score: Option<f32>,
  /// Type of event this campaign is for. tcu_defense, ihub_defense and station_defense are referred to as \"Defense Events\", station_freeport as \"Freeport Events\". 
  #[serde(rename = "event_type")]
  event_type: String,
  /// Alliance participating and their respective scores, only present in Freeport Events. 
  #[serde(rename = "participants")]
  participants: Option<Vec<::models::GetSovereigntyCampaignsParticipant>>,
  /// The solar system the structure is located in. 
  #[serde(rename = "solar_system_id")]
  solar_system_id: i32,
  /// Time the event is scheduled to start. 
  #[serde(rename = "start_time")]
  start_time: String,
  /// The structure item ID that is related to this campaign. 
  #[serde(rename = "structure_id")]
  structure_id: i64
}

impl GetSovereigntyCampaigns200Ok {
  /// 200 ok object
  pub fn new(campaign_id: i32, constellation_id: i32, event_type: String, solar_system_id: i32, start_time: String, structure_id: i64) -> GetSovereigntyCampaigns200Ok {
    GetSovereigntyCampaigns200Ok {
      attackers_score: None,
      campaign_id: campaign_id,
      constellation_id: constellation_id,
      defender_id: None,
      defender_score: None,
      event_type: event_type,
      participants: None,
      solar_system_id: solar_system_id,
      start_time: start_time,
      structure_id: structure_id
    }
  }

  pub fn set_attackers_score(&mut self, attackers_score: f32) {
    self.attackers_score = Some(attackers_score);
  }

  pub fn with_attackers_score(mut self, attackers_score: f32) -> GetSovereigntyCampaigns200Ok {
    self.attackers_score = Some(attackers_score);
    self
  }

  pub fn attackers_score(&self) -> Option<&f32> {
    self.attackers_score.as_ref()
  }

  pub fn reset_attackers_score(&mut self) {
    self.attackers_score = None;
  }

  pub fn set_campaign_id(&mut self, campaign_id: i32) {
    self.campaign_id = campaign_id;
  }

  pub fn with_campaign_id(mut self, campaign_id: i32) -> GetSovereigntyCampaigns200Ok {
    self.campaign_id = campaign_id;
    self
  }

  pub fn campaign_id(&self) -> &i32 {
    &self.campaign_id
  }


  pub fn set_constellation_id(&mut self, constellation_id: i32) {
    self.constellation_id = constellation_id;
  }

  pub fn with_constellation_id(mut self, constellation_id: i32) -> GetSovereigntyCampaigns200Ok {
    self.constellation_id = constellation_id;
    self
  }

  pub fn constellation_id(&self) -> &i32 {
    &self.constellation_id
  }


  pub fn set_defender_id(&mut self, defender_id: i32) {
    self.defender_id = Some(defender_id);
  }

  pub fn with_defender_id(mut self, defender_id: i32) -> GetSovereigntyCampaigns200Ok {
    self.defender_id = Some(defender_id);
    self
  }

  pub fn defender_id(&self) -> Option<&i32> {
    self.defender_id.as_ref()
  }

  pub fn reset_defender_id(&mut self) {
    self.defender_id = None;
  }

  pub fn set_defender_score(&mut self, defender_score: f32) {
    self.defender_score = Some(defender_score);
  }

  pub fn with_defender_score(mut self, defender_score: f32) -> GetSovereigntyCampaigns200Ok {
    self.defender_score = Some(defender_score);
    self
  }

  pub fn defender_score(&self) -> Option<&f32> {
    self.defender_score.as_ref()
  }

  pub fn reset_defender_score(&mut self) {
    self.defender_score = None;
  }

  pub fn set_event_type(&mut self, event_type: String) {
    self.event_type = event_type;
  }

  pub fn with_event_type(mut self, event_type: String) -> GetSovereigntyCampaigns200Ok {
    self.event_type = event_type;
    self
  }

  pub fn event_type(&self) -> &String {
    &self.event_type
  }


  pub fn set_participants(&mut self, participants: Vec<::models::GetSovereigntyCampaignsParticipant>) {
    self.participants = Some(participants);
  }

  pub fn with_participants(mut self, participants: Vec<::models::GetSovereigntyCampaignsParticipant>) -> GetSovereigntyCampaigns200Ok {
    self.participants = Some(participants);
    self
  }

  pub fn participants(&self) -> Option<&Vec<::models::GetSovereigntyCampaignsParticipant>> {
    self.participants.as_ref()
  }

  pub fn reset_participants(&mut self) {
    self.participants = None;
  }

  pub fn set_solar_system_id(&mut self, solar_system_id: i32) {
    self.solar_system_id = solar_system_id;
  }

  pub fn with_solar_system_id(mut self, solar_system_id: i32) -> GetSovereigntyCampaigns200Ok {
    self.solar_system_id = solar_system_id;
    self
  }

  pub fn solar_system_id(&self) -> &i32 {
    &self.solar_system_id
  }


  pub fn set_start_time(&mut self, start_time: String) {
    self.start_time = start_time;
  }

  pub fn with_start_time(mut self, start_time: String) -> GetSovereigntyCampaigns200Ok {
    self.start_time = start_time;
    self
  }

  pub fn start_time(&self) -> &String {
    &self.start_time
  }


  pub fn set_structure_id(&mut self, structure_id: i64) {
    self.structure_id = structure_id;
  }

  pub fn with_structure_id(mut self, structure_id: i64) -> GetSovereigntyCampaigns200Ok {
    self.structure_id = structure_id;
    self
  }

  pub fn structure_id(&self) -> &i64 {
    &self.structure_id
  }


}


