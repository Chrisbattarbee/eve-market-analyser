/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdFittings200Ok : 200 ok object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdFittings200Ok {
    /// description string
    #[serde(rename = "description")]
    pub description: String,
    /// fitting_id integer
    #[serde(rename = "fitting_id")]
    pub fitting_id: i32,
    /// items array
    #[serde(rename = "items")]
    pub items: Vec<crate::models::GetCharactersCharacterIdFittingsItem>,
    /// name string
    #[serde(rename = "name")]
    pub name: String,
    /// ship_type_id integer
    #[serde(rename = "ship_type_id")]
    pub ship_type_id: i32,
}

impl GetCharactersCharacterIdFittings200Ok {
    /// 200 ok object
    pub fn new(description: String, fitting_id: i32, items: Vec<crate::models::GetCharactersCharacterIdFittingsItem>, name: String, ship_type_id: i32) -> GetCharactersCharacterIdFittings200Ok {
        GetCharactersCharacterIdFittings200Ok {
            description,
            fitting_id,
            items,
            name,
            ship_type_id,
        }
    }
}


