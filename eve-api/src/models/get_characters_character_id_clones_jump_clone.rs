/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdClonesJumpClone : jump_clone object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdClonesJumpClone {
    /// implants array
    #[serde(rename = "implants")]
    pub implants: Vec<i32>,
    /// jump_clone_id integer
    #[serde(rename = "jump_clone_id")]
    pub jump_clone_id: i32,
    /// location_id integer
    #[serde(rename = "location_id")]
    pub location_id: i64,
    /// location_type string
    #[serde(rename = "location_type")]
    pub location_type: LocationType,
    /// name string
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl GetCharactersCharacterIdClonesJumpClone {
    /// jump_clone object
    pub fn new(implants: Vec<i32>, jump_clone_id: i32, location_id: i64, location_type: LocationType) -> GetCharactersCharacterIdClonesJumpClone {
        GetCharactersCharacterIdClonesJumpClone {
            implants,
            jump_clone_id,
            location_id,
            location_type,
            name: None,
        }
    }
}

/// location_type string
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LocationType {
    #[serde(rename = "station")]
    Station,
    #[serde(rename = "structure")]
    Structure,
}

impl Default for LocationType {
    fn default() -> LocationType {
        Self::Station
    }
}

