/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetUniverseTypesTypeIdDogmaEffect : dogma_effect object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetUniverseTypesTypeIdDogmaEffect {
    /// effect_id integer
    #[serde(rename = "effect_id")]
    pub effect_id: i32,
    /// is_default boolean
    #[serde(rename = "is_default")]
    pub is_default: bool,
}

impl GetUniverseTypesTypeIdDogmaEffect {
    /// dogma_effect object
    pub fn new(effect_id: i32, is_default: bool) -> GetUniverseTypesTypeIdDogmaEffect {
        GetUniverseTypesTypeIdDogmaEffect {
            effect_id,
            is_default,
        }
    }
}


