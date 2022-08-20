/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetUniversePlanetsPlanetIdOk : 200 ok object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetUniversePlanetsPlanetIdOk {
    /// name string
    #[serde(rename = "name")]
    pub name: String,
    /// planet_id integer
    #[serde(rename = "planet_id")]
    pub planet_id: i32,
    #[serde(rename = "position")]
    pub position: Box<crate::models::GetUniversePlanetsPlanetIdPosition>,
    /// The solar system this planet is in
    #[serde(rename = "system_id")]
    pub system_id: i32,
    /// type_id integer
    #[serde(rename = "type_id")]
    pub type_id: i32,
}

impl GetUniversePlanetsPlanetIdOk {
    /// 200 ok object
    pub fn new(name: String, planet_id: i32, position: crate::models::GetUniversePlanetsPlanetIdPosition, system_id: i32, type_id: i32) -> GetUniversePlanetsPlanetIdOk {
        GetUniversePlanetsPlanetIdOk {
            name,
            planet_id,
            position: Box::new(position),
            system_id,
            type_id,
        }
    }
}


