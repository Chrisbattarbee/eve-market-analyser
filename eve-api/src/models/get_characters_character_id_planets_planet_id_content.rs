/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdPlanetsPlanetIdContent : content object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdPlanetsPlanetIdContent {
    /// amount integer
    #[serde(rename = "amount")]
    pub amount: i64,
    /// type_id integer
    #[serde(rename = "type_id")]
    pub type_id: i32,
}

impl GetCharactersCharacterIdPlanetsPlanetIdContent {
    /// content object
    pub fn new(amount: i64, type_id: i32) -> GetCharactersCharacterIdPlanetsPlanetIdContent {
        GetCharactersCharacterIdPlanetsPlanetIdContent {
            amount,
            type_id,
        }
    }
}


