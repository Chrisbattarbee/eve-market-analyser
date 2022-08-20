/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCharactersCharacterIdBookmarksFolders200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdBookmarksFolders200Ok {
  /// folder_id integer
  #[serde(rename = "folder_id")]
  folder_id: i32,
  /// name string
  #[serde(rename = "name")]
  name: String
}

impl GetCharactersCharacterIdBookmarksFolders200Ok {
  /// 200 ok object
  pub fn new(folder_id: i32, name: String) -> GetCharactersCharacterIdBookmarksFolders200Ok {
    GetCharactersCharacterIdBookmarksFolders200Ok {
      folder_id: folder_id,
      name: name
    }
  }

  pub fn set_folder_id(&mut self, folder_id: i32) {
    self.folder_id = folder_id;
  }

  pub fn with_folder_id(mut self, folder_id: i32) -> GetCharactersCharacterIdBookmarksFolders200Ok {
    self.folder_id = folder_id;
    self
  }

  pub fn folder_id(&self) -> &i32 {
    &self.folder_id
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> GetCharactersCharacterIdBookmarksFolders200Ok {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


}



