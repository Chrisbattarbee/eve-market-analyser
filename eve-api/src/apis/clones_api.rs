/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.12
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`get_characters_character_id_clones`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCharactersCharacterIdClonesError {
    Status400(crate::models::BadRequest),
    Status401(crate::models::Unauthorized),
    Status403(crate::models::Forbidden),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_characters_character_id_implants`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCharactersCharacterIdImplantsError {
    Status400(crate::models::BadRequest),
    Status401(crate::models::Unauthorized),
    Status403(crate::models::Forbidden),
    Status420(crate::models::ErrorLimited),
    Status500(crate::models::InternalServerError),
    Status503(crate::models::ServiceUnavailable),
    Status504(crate::models::GatewayTimeout),
    UnknownValue(serde_json::Value),
}


/// A list of the character's clones  --- Alternate route: `/dev/characters/{character_id}/clones/`  Alternate route: `/v3/characters/{character_id}/clones/`  Alternate route: `/v4/characters/{character_id}/clones/`  --- This route is cached for up to 120 seconds
pub async fn get_characters_character_id_clones(configuration: &configuration::Configuration, character_id: i32, datasource: Option<&str>, if_none_match: Option<&str>, token: Option<&str>) -> Result<crate::models::GetCharactersCharacterIdClonesOk, Error<GetCharactersCharacterIdClonesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/characters/{character_id}/clones/", local_var_configuration.base_path, character_id=character_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder = local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = token {
        local_var_req_builder = local_var_req_builder.query(&[("token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder = local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetCharactersCharacterIdClonesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Return implants on the active clone of a character  --- Alternate route: `/dev/characters/{character_id}/implants/`  Alternate route: `/legacy/characters/{character_id}/implants/`  Alternate route: `/v1/characters/{character_id}/implants/`  Alternate route: `/v2/characters/{character_id}/implants/`  --- This route is cached for up to 120 seconds
pub async fn get_characters_character_id_implants(configuration: &configuration::Configuration, character_id: i32, datasource: Option<&str>, if_none_match: Option<&str>, token: Option<&str>) -> Result<Vec<i32>, Error<GetCharactersCharacterIdImplantsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/characters/{character_id}/implants/", local_var_configuration.base_path, character_id=character_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = datasource {
        local_var_req_builder = local_var_req_builder.query(&[("datasource", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = token {
        local_var_req_builder = local_var_req_builder.query(&[("token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_none_match {
        local_var_req_builder = local_var_req_builder.header("If-None-Match", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetCharactersCharacterIdImplantsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

