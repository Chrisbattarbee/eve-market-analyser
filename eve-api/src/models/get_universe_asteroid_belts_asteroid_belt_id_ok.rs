/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetUniverseAsteroidBeltsAsteroidBeltIdOk : 200 ok object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetUniverseAsteroidBeltsAsteroidBeltIdOk {
    /// name string
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "position")]
    pub position: Box<crate::models::GetUniverseAsteroidBeltsAsteroidBeltIdPosition>,
    /// The solar system this asteroid belt is in
    #[serde(rename = "system_id")]
    pub system_id: i32,
}

impl GetUniverseAsteroidBeltsAsteroidBeltIdOk {
    /// 200 ok object
    pub fn new(name: String, position: crate::models::GetUniverseAsteroidBeltsAsteroidBeltIdPosition, system_id: i32) -> GetUniverseAsteroidBeltsAsteroidBeltIdOk {
        GetUniverseAsteroidBeltsAsteroidBeltIdOk {
            name,
            position: Box::new(position),
            system_id,
        }
    }
}


