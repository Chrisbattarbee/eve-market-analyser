/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdMailRecipient : recipient object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdMailRecipient {
    /// recipient_id integer
    #[serde(rename = "recipient_id")]
    pub recipient_id: i32,
    /// recipient_type string
    #[serde(rename = "recipient_type")]
    pub recipient_type: RecipientType,
}

impl GetCharactersCharacterIdMailRecipient {
    /// recipient object
    pub fn new(recipient_id: i32, recipient_type: RecipientType) -> GetCharactersCharacterIdMailRecipient {
        GetCharactersCharacterIdMailRecipient {
            recipient_id,
            recipient_type,
        }
    }
}

/// recipient_type string
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RecipientType {
    #[serde(rename = "alliance")]
    Alliance,
    #[serde(rename = "character")]
    Character,
    #[serde(rename = "corporation")]
    Corporation,
    #[serde(rename = "mailing_list")]
    MailingList,
}

impl Default for RecipientType {
    fn default() -> RecipientType {
        Self::Alliance
    }
}

