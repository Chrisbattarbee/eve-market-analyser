/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetUniverseRaces200Ok : 200 ok object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetUniverseRaces200Ok {
    /// The alliance generally associated with this race
    #[serde(rename = "alliance_id")]
    pub alliance_id: i32,
    /// description string
    #[serde(rename = "description")]
    pub description: String,
    /// name string
    #[serde(rename = "name")]
    pub name: String,
    /// race_id integer
    #[serde(rename = "race_id")]
    pub race_id: i32,
}

impl GetUniverseRaces200Ok {
    /// 200 ok object
    pub fn new(alliance_id: i32, description: String, name: String, race_id: i32) -> GetUniverseRaces200Ok {
        GetUniverseRaces200Ok {
            alliance_id,
            description,
            name,
            race_id,
        }
    }
}


