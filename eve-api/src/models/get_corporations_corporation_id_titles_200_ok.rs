/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCorporationsCorporationIdTitles200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdTitles200Ok {
  /// grantable_roles array
  #[serde(rename = "grantable_roles")]
  grantable_roles: Option<Vec<String>>,
  /// grantable_roles_at_base array
  #[serde(rename = "grantable_roles_at_base")]
  grantable_roles_at_base: Option<Vec<String>>,
  /// grantable_roles_at_hq array
  #[serde(rename = "grantable_roles_at_hq")]
  grantable_roles_at_hq: Option<Vec<String>>,
  /// grantable_roles_at_other array
  #[serde(rename = "grantable_roles_at_other")]
  grantable_roles_at_other: Option<Vec<String>>,
  /// name string
  #[serde(rename = "name")]
  name: Option<String>,
  /// roles array
  #[serde(rename = "roles")]
  roles: Option<Vec<String>>,
  /// roles_at_base array
  #[serde(rename = "roles_at_base")]
  roles_at_base: Option<Vec<String>>,
  /// roles_at_hq array
  #[serde(rename = "roles_at_hq")]
  roles_at_hq: Option<Vec<String>>,
  /// roles_at_other array
  #[serde(rename = "roles_at_other")]
  roles_at_other: Option<Vec<String>>,
  /// title_id integer
  #[serde(rename = "title_id")]
  title_id: Option<i32>
}

impl GetCorporationsCorporationIdTitles200Ok {
  /// 200 ok object
  pub fn new() -> GetCorporationsCorporationIdTitles200Ok {
    GetCorporationsCorporationIdTitles200Ok {
      grantable_roles: None,
      grantable_roles_at_base: None,
      grantable_roles_at_hq: None,
      grantable_roles_at_other: None,
      name: None,
      roles: None,
      roles_at_base: None,
      roles_at_hq: None,
      roles_at_other: None,
      title_id: None
    }
  }

  pub fn set_grantable_roles(&mut self, grantable_roles: Vec<String>) {
    self.grantable_roles = Some(grantable_roles);
  }

  pub fn with_grantable_roles(mut self, grantable_roles: Vec<String>) -> GetCorporationsCorporationIdTitles200Ok {
    self.grantable_roles = Some(grantable_roles);
    self
  }

  pub fn grantable_roles(&self) -> Option<&Vec<String>> {
    self.grantable_roles.as_ref()
  }

  pub fn reset_grantable_roles(&mut self) {
    self.grantable_roles = None;
  }

  pub fn set_grantable_roles_at_base(&mut self, grantable_roles_at_base: Vec<String>) {
    self.grantable_roles_at_base = Some(grantable_roles_at_base);
  }

  pub fn with_grantable_roles_at_base(mut self, grantable_roles_at_base: Vec<String>) -> GetCorporationsCorporationIdTitles200Ok {
    self.grantable_roles_at_base = Some(grantable_roles_at_base);
    self
  }

  pub fn grantable_roles_at_base(&self) -> Option<&Vec<String>> {
    self.grantable_roles_at_base.as_ref()
  }

  pub fn reset_grantable_roles_at_base(&mut self) {
    self.grantable_roles_at_base = None;
  }

  pub fn set_grantable_roles_at_hq(&mut self, grantable_roles_at_hq: Vec<String>) {
    self.grantable_roles_at_hq = Some(grantable_roles_at_hq);
  }

  pub fn with_grantable_roles_at_hq(mut self, grantable_roles_at_hq: Vec<String>) -> GetCorporationsCorporationIdTitles200Ok {
    self.grantable_roles_at_hq = Some(grantable_roles_at_hq);
    self
  }

  pub fn grantable_roles_at_hq(&self) -> Option<&Vec<String>> {
    self.grantable_roles_at_hq.as_ref()
  }

  pub fn reset_grantable_roles_at_hq(&mut self) {
    self.grantable_roles_at_hq = None;
  }

  pub fn set_grantable_roles_at_other(&mut self, grantable_roles_at_other: Vec<String>) {
    self.grantable_roles_at_other = Some(grantable_roles_at_other);
  }

  pub fn with_grantable_roles_at_other(mut self, grantable_roles_at_other: Vec<String>) -> GetCorporationsCorporationIdTitles200Ok {
    self.grantable_roles_at_other = Some(grantable_roles_at_other);
    self
  }

  pub fn grantable_roles_at_other(&self) -> Option<&Vec<String>> {
    self.grantable_roles_at_other.as_ref()
  }

  pub fn reset_grantable_roles_at_other(&mut self) {
    self.grantable_roles_at_other = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> GetCorporationsCorporationIdTitles200Ok {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_roles(&mut self, roles: Vec<String>) {
    self.roles = Some(roles);
  }

  pub fn with_roles(mut self, roles: Vec<String>) -> GetCorporationsCorporationIdTitles200Ok {
    self.roles = Some(roles);
    self
  }

  pub fn roles(&self) -> Option<&Vec<String>> {
    self.roles.as_ref()
  }

  pub fn reset_roles(&mut self) {
    self.roles = None;
  }

  pub fn set_roles_at_base(&mut self, roles_at_base: Vec<String>) {
    self.roles_at_base = Some(roles_at_base);
  }

  pub fn with_roles_at_base(mut self, roles_at_base: Vec<String>) -> GetCorporationsCorporationIdTitles200Ok {
    self.roles_at_base = Some(roles_at_base);
    self
  }

  pub fn roles_at_base(&self) -> Option<&Vec<String>> {
    self.roles_at_base.as_ref()
  }

  pub fn reset_roles_at_base(&mut self) {
    self.roles_at_base = None;
  }

  pub fn set_roles_at_hq(&mut self, roles_at_hq: Vec<String>) {
    self.roles_at_hq = Some(roles_at_hq);
  }

  pub fn with_roles_at_hq(mut self, roles_at_hq: Vec<String>) -> GetCorporationsCorporationIdTitles200Ok {
    self.roles_at_hq = Some(roles_at_hq);
    self
  }

  pub fn roles_at_hq(&self) -> Option<&Vec<String>> {
    self.roles_at_hq.as_ref()
  }

  pub fn reset_roles_at_hq(&mut self) {
    self.roles_at_hq = None;
  }

  pub fn set_roles_at_other(&mut self, roles_at_other: Vec<String>) {
    self.roles_at_other = Some(roles_at_other);
  }

  pub fn with_roles_at_other(mut self, roles_at_other: Vec<String>) -> GetCorporationsCorporationIdTitles200Ok {
    self.roles_at_other = Some(roles_at_other);
    self
  }

  pub fn roles_at_other(&self) -> Option<&Vec<String>> {
    self.roles_at_other.as_ref()
  }

  pub fn reset_roles_at_other(&mut self) {
    self.roles_at_other = None;
  }

  pub fn set_title_id(&mut self, title_id: i32) {
    self.title_id = Some(title_id);
  }

  pub fn with_title_id(mut self, title_id: i32) -> GetCorporationsCorporationIdTitles200Ok {
    self.title_id = Some(title_id);
    self
  }

  pub fn title_id(&self) -> Option<&i32> {
    self.title_id.as_ref()
  }

  pub fn reset_title_id(&mut self) {
    self.title_id = None;
  }

}



