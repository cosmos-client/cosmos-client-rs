/*
 * Cosmos SDK - Legacy REST and gRPC Gateway docs
 *
 * A REST interface for state queries, legacy transactions
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IbcCoreChannelV1QueryConnectionChannelsResponse {
    /// list of channels associated with a connection.
    #[serde(rename = "channels", skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<crate::models::InlineResponse20075Channels>>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<crate::models::PaginationResponse>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<crate::models::QueryBlockHeight>,
}

impl IbcCoreChannelV1QueryConnectionChannelsResponse {
    pub fn new() -> IbcCoreChannelV1QueryConnectionChannelsResponse {
        IbcCoreChannelV1QueryConnectionChannelsResponse {
            channels: None,
            pagination: None,
            height: None,
        }
    }
}


