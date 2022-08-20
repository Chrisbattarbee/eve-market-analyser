/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetDogmaDynamicItemsTypeIdItemIdNotFound : Not found



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetDogmaDynamicItemsTypeIdItemIdNotFound {
    /// Not found message
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl GetDogmaDynamicItemsTypeIdItemIdNotFound {
    /// Not found
    pub fn new() -> GetDogmaDynamicItemsTypeIdItemIdNotFound {
        GetDogmaDynamicItemsTypeIdItemIdNotFound {
            error: None,
        }
    }
}


