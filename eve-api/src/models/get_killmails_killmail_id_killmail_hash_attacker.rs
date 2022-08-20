/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetKillmailsKillmailIdKillmailHashAttacker : attacker object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetKillmailsKillmailIdKillmailHashAttacker {
    /// alliance_id integer
    #[serde(rename = "alliance_id", skip_serializing_if = "Option::is_none")]
    pub alliance_id: Option<i32>,
    /// character_id integer
    #[serde(rename = "character_id", skip_serializing_if = "Option::is_none")]
    pub character_id: Option<i32>,
    /// corporation_id integer
    #[serde(rename = "corporation_id", skip_serializing_if = "Option::is_none")]
    pub corporation_id: Option<i32>,
    /// damage_done integer
    #[serde(rename = "damage_done")]
    pub damage_done: i32,
    /// faction_id integer
    #[serde(rename = "faction_id", skip_serializing_if = "Option::is_none")]
    pub faction_id: Option<i32>,
    /// Was the attacker the one to achieve the final blow 
    #[serde(rename = "final_blow")]
    pub final_blow: bool,
    /// Security status for the attacker 
    #[serde(rename = "security_status")]
    pub security_status: f32,
    /// What ship was the attacker flying 
    #[serde(rename = "ship_type_id", skip_serializing_if = "Option::is_none")]
    pub ship_type_id: Option<i32>,
    /// What weapon was used by the attacker for the kill 
    #[serde(rename = "weapon_type_id", skip_serializing_if = "Option::is_none")]
    pub weapon_type_id: Option<i32>,
}

impl GetKillmailsKillmailIdKillmailHashAttacker {
    /// attacker object
    pub fn new(damage_done: i32, final_blow: bool, security_status: f32) -> GetKillmailsKillmailIdKillmailHashAttacker {
        GetKillmailsKillmailIdKillmailHashAttacker {
            alliance_id: None,
            character_id: None,
            corporation_id: None,
            damage_done,
            faction_id: None,
            final_blow,
            security_status,
            ship_type_id: None,
            weapon_type_id: None,
        }
    }
}


