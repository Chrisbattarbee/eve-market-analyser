/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCorporationCorporationIdMiningObservers200Ok : 200 ok object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCorporationCorporationIdMiningObservers200Ok {
    /// last_updated string
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    /// The entity that was observing the asteroid field when it was mined. 
    #[serde(rename = "observer_id")]
    pub observer_id: i64,
    /// The category of the observing entity
    #[serde(rename = "observer_type")]
    pub observer_type: ObserverType,
}

impl GetCorporationCorporationIdMiningObservers200Ok {
    /// 200 ok object
    pub fn new(last_updated: String, observer_id: i64, observer_type: ObserverType) -> GetCorporationCorporationIdMiningObservers200Ok {
        GetCorporationCorporationIdMiningObservers200Ok {
            last_updated,
            observer_id,
            observer_type,
        }
    }
}

/// The category of the observing entity
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ObserverType {
    #[serde(rename = "structure")]
    Structure,
}

impl Default for ObserverType {
    fn default() -> ObserverType {
        Self::Structure
    }
}

