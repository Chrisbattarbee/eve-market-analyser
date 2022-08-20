/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCorporationsCorporationIdIndustryJobs200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdIndustryJobs200Ok {
  /// Job activity ID
  #[serde(rename = "activity_id")]
  activity_id: i32,
  /// blueprint_id integer
  #[serde(rename = "blueprint_id")]
  blueprint_id: i64,
  /// Location ID of the location from which the blueprint was installed. Normally a station ID, but can also be an asset (e.g. container) or corporation facility
  #[serde(rename = "blueprint_location_id")]
  blueprint_location_id: i64,
  /// blueprint_type_id integer
  #[serde(rename = "blueprint_type_id")]
  blueprint_type_id: i32,
  /// ID of the character which completed this job
  #[serde(rename = "completed_character_id")]
  completed_character_id: Option<i32>,
  /// Date and time when this job was completed
  #[serde(rename = "completed_date")]
  completed_date: Option<String>,
  /// The sume of job installation fee and industry facility tax
  #[serde(rename = "cost")]
  cost: Option<f64>,
  /// Job duration in seconds
  #[serde(rename = "duration")]
  duration: i32,
  /// Date and time when this job finished
  #[serde(rename = "end_date")]
  end_date: String,
  /// ID of the facility where this job is running
  #[serde(rename = "facility_id")]
  facility_id: i64,
  /// ID of the character which installed this job
  #[serde(rename = "installer_id")]
  installer_id: i32,
  /// Unique job ID
  #[serde(rename = "job_id")]
  job_id: i32,
  /// Number of runs blueprint is licensed for
  #[serde(rename = "licensed_runs")]
  licensed_runs: Option<i32>,
  /// ID of the location for the industry facility
  #[serde(rename = "location_id")]
  location_id: i64,
  /// Location ID of the location to which the output of the job will be delivered. Normally a station ID, but can also be a corporation facility
  #[serde(rename = "output_location_id")]
  output_location_id: i64,
  /// Date and time when this job was paused (i.e. time when the facility where this job was installed went offline)
  #[serde(rename = "pause_date")]
  pause_date: Option<String>,
  /// Chance of success for invention
  #[serde(rename = "probability")]
  probability: Option<f32>,
  /// Type ID of product (manufactured, copied or invented)
  #[serde(rename = "product_type_id")]
  product_type_id: Option<i32>,
  /// Number of runs for a manufacturing job, or number of copies to make for a blueprint copy
  #[serde(rename = "runs")]
  runs: i32,
  /// Date and time when this job started
  #[serde(rename = "start_date")]
  start_date: String,
  /// status string
  #[serde(rename = "status")]
  status: String,
  /// Number of successful runs for this job. Equal to runs unless this is an invention job
  #[serde(rename = "successful_runs")]
  successful_runs: Option<i32>
}

impl GetCorporationsCorporationIdIndustryJobs200Ok {
  /// 200 ok object
  pub fn new(activity_id: i32, blueprint_id: i64, blueprint_location_id: i64, blueprint_type_id: i32, duration: i32, end_date: String, facility_id: i64, installer_id: i32, job_id: i32, location_id: i64, output_location_id: i64, runs: i32, start_date: String, status: String) -> GetCorporationsCorporationIdIndustryJobs200Ok {
    GetCorporationsCorporationIdIndustryJobs200Ok {
      activity_id: activity_id,
      blueprint_id: blueprint_id,
      blueprint_location_id: blueprint_location_id,
      blueprint_type_id: blueprint_type_id,
      completed_character_id: None,
      completed_date: None,
      cost: None,
      duration: duration,
      end_date: end_date,
      facility_id: facility_id,
      installer_id: installer_id,
      job_id: job_id,
      licensed_runs: None,
      location_id: location_id,
      output_location_id: output_location_id,
      pause_date: None,
      probability: None,
      product_type_id: None,
      runs: runs,
      start_date: start_date,
      status: status,
      successful_runs: None
    }
  }

  pub fn set_activity_id(&mut self, activity_id: i32) {
    self.activity_id = activity_id;
  }

  pub fn with_activity_id(mut self, activity_id: i32) -> GetCorporationsCorporationIdIndustryJobs200Ok {
    self.activity_id = activity_id;
    self
  }

  pub fn activity_id(&self) -> &i32 {
    &self.activity_id
  }


  pub fn set_blueprint_id(&mut self, blueprint_id: i64) {
    self.blueprint_id = blueprint_id;
  }

  pub fn with_blueprint_id(mut self, blueprint_id: i64) -> GetCorporationsCorporationIdIndustryJobs200Ok {
    self.blueprint_id = blueprint_id;
    self
  }

  pub fn blueprint_id(&self) -> &i64 {
    &self.blueprint_id
  }


  pub fn set_blueprint_location_id(&mut self, blueprint_location_id: i64) {
    self.blueprint_location_id = blueprint_location_id;
  }

  pub fn with_blueprint_location_id(mut self, blueprint_location_id: i64) -> GetCorporationsCorporationIdIndustryJobs200Ok {
    self.blueprint_location_id = blueprint_location_id;
    self
  }

  pub fn blueprint_location_id(&self) -> &i64 {
    &self.blueprint_location_id
  }


  pub fn set_blueprint_type_id(&mut self, blueprint_type_id: i32) {
    self.blueprint_type_id = blueprint_type_id;
  }

  pub fn with_blueprint_type_id(mut self, blueprint_type_id: i32) -> GetCorporationsCorporationIdIndustryJobs200Ok {
    self.blueprint_type_id = blueprint_type_id;
    self
  }

  pub fn blueprint_type_id(&self) -> &i32 {
    &self.blueprint_type_id
  }


  pub fn set_completed_character_id(&mut self, completed_character_id: i32) {
    self.completed_character_id = Some(completed_character_id);
  }

  pub fn with_completed_character_id(mut self, completed_character_id: i32) -> GetCorporationsCorporationIdIndustryJobs200Ok {
    self.completed_character_id = Some(completed_character_id);
    self
  }

  pub fn completed_character_id(&self) -> Option<&i32> {
    self.completed_character_id.as_ref()
  }

  pub fn reset_completed_character_id(&mut self) {
    self.completed_character_id = None;
  }

  pub fn set_completed_date(&mut self, completed_date: String) {
    self.completed_date = Some(completed_date);
  }

  pub fn with_completed_date(mut self, completed_date: String) -> GetCorporationsCorporationIdIndustryJobs200Ok {
    self.completed_date = Some(completed_date);
    self
  }

  pub fn completed_date(&self) -> Option<&String> {
    self.completed_date.as_ref()
  }

  pub fn reset_completed_date(&mut self) {
    self.completed_date = None;
  }

  pub fn set_cost(&mut self, cost: f64) {
    self.cost = Some(cost);
  }

  pub fn with_cost(mut self, cost: f64) -> GetCorporationsCorporationIdIndustryJobs200Ok {
    self.cost = Some(cost);
    self
  }

  pub fn cost(&self) -> Option<&f64> {
    self.cost.as_ref()
  }

  pub fn reset_cost(&mut self) {
    self.cost = None;
  }

  pub fn set_duration(&mut self, duration: i32) {
    self.duration = duration;
  }

  pub fn with_duration(mut self, duration: i32) -> GetCorporationsCorporationIdIndustryJobs200Ok {
    self.duration = duration;
    self
  }

  pub fn duration(&self) -> &i32 {
    &self.duration
  }


  pub fn set_end_date(&mut self, end_date: String) {
    self.end_date = end_date;
  }

  pub fn with_end_date(mut self, end_date: String) -> GetCorporationsCorporationIdIndustryJobs200Ok {
    self.end_date = end_date;
    self
  }

  pub fn end_date(&self) -> &String {
    &self.end_date
  }


  pub fn set_facility_id(&mut self, facility_id: i64) {
    self.facility_id = facility_id;
  }

  pub fn with_facility_id(mut self, facility_id: i64) -> GetCorporationsCorporationIdIndustryJobs200Ok {
    self.facility_id = facility_id;
    self
  }

  pub fn facility_id(&self) -> &i64 {
    &self.facility_id
  }


  pub fn set_installer_id(&mut self, installer_id: i32) {
    self.installer_id = installer_id;
  }

  pub fn with_installer_id(mut self, installer_id: i32) -> GetCorporationsCorporationIdIndustryJobs200Ok {
    self.installer_id = installer_id;
    self
  }

  pub fn installer_id(&self) -> &i32 {
    &self.installer_id
  }


  pub fn set_job_id(&mut self, job_id: i32) {
    self.job_id = job_id;
  }

  pub fn with_job_id(mut self, job_id: i32) -> GetCorporationsCorporationIdIndustryJobs200Ok {
    self.job_id = job_id;
    self
  }

  pub fn job_id(&self) -> &i32 {
    &self.job_id
  }


  pub fn set_licensed_runs(&mut self, licensed_runs: i32) {
    self.licensed_runs = Some(licensed_runs);
  }

  pub fn with_licensed_runs(mut self, licensed_runs: i32) -> GetCorporationsCorporationIdIndustryJobs200Ok {
    self.licensed_runs = Some(licensed_runs);
    self
  }

  pub fn licensed_runs(&self) -> Option<&i32> {
    self.licensed_runs.as_ref()
  }

  pub fn reset_licensed_runs(&mut self) {
    self.licensed_runs = None;
  }

  pub fn set_location_id(&mut self, location_id: i64) {
    self.location_id = location_id;
  }

  pub fn with_location_id(mut self, location_id: i64) -> GetCorporationsCorporationIdIndustryJobs200Ok {
    self.location_id = location_id;
    self
  }

  pub fn location_id(&self) -> &i64 {
    &self.location_id
  }


  pub fn set_output_location_id(&mut self, output_location_id: i64) {
    self.output_location_id = output_location_id;
  }

  pub fn with_output_location_id(mut self, output_location_id: i64) -> GetCorporationsCorporationIdIndustryJobs200Ok {
    self.output_location_id = output_location_id;
    self
  }

  pub fn output_location_id(&self) -> &i64 {
    &self.output_location_id
  }


  pub fn set_pause_date(&mut self, pause_date: String) {
    self.pause_date = Some(pause_date);
  }

  pub fn with_pause_date(mut self, pause_date: String) -> GetCorporationsCorporationIdIndustryJobs200Ok {
    self.pause_date = Some(pause_date);
    self
  }

  pub fn pause_date(&self) -> Option<&String> {
    self.pause_date.as_ref()
  }

  pub fn reset_pause_date(&mut self) {
    self.pause_date = None;
  }

  pub fn set_probability(&mut self, probability: f32) {
    self.probability = Some(probability);
  }

  pub fn with_probability(mut self, probability: f32) -> GetCorporationsCorporationIdIndustryJobs200Ok {
    self.probability = Some(probability);
    self
  }

  pub fn probability(&self) -> Option<&f32> {
    self.probability.as_ref()
  }

  pub fn reset_probability(&mut self) {
    self.probability = None;
  }

  pub fn set_product_type_id(&mut self, product_type_id: i32) {
    self.product_type_id = Some(product_type_id);
  }

  pub fn with_product_type_id(mut self, product_type_id: i32) -> GetCorporationsCorporationIdIndustryJobs200Ok {
    self.product_type_id = Some(product_type_id);
    self
  }

  pub fn product_type_id(&self) -> Option<&i32> {
    self.product_type_id.as_ref()
  }

  pub fn reset_product_type_id(&mut self) {
    self.product_type_id = None;
  }

  pub fn set_runs(&mut self, runs: i32) {
    self.runs = runs;
  }

  pub fn with_runs(mut self, runs: i32) -> GetCorporationsCorporationIdIndustryJobs200Ok {
    self.runs = runs;
    self
  }

  pub fn runs(&self) -> &i32 {
    &self.runs
  }


  pub fn set_start_date(&mut self, start_date: String) {
    self.start_date = start_date;
  }

  pub fn with_start_date(mut self, start_date: String) -> GetCorporationsCorporationIdIndustryJobs200Ok {
    self.start_date = start_date;
    self
  }

  pub fn start_date(&self) -> &String {
    &self.start_date
  }


  pub fn set_status(&mut self, status: String) {
    self.status = status;
  }

  pub fn with_status(mut self, status: String) -> GetCorporationsCorporationIdIndustryJobs200Ok {
    self.status = status;
    self
  }

  pub fn status(&self) -> &String {
    &self.status
  }


  pub fn set_successful_runs(&mut self, successful_runs: i32) {
    self.successful_runs = Some(successful_runs);
  }

  pub fn with_successful_runs(mut self, successful_runs: i32) -> GetCorporationsCorporationIdIndustryJobs200Ok {
    self.successful_runs = Some(successful_runs);
    self
  }

  pub fn successful_runs(&self) -> Option<&i32> {
    self.successful_runs.as_ref()
  }

  pub fn reset_successful_runs(&mut self) {
    self.successful_runs = None;
  }

}



