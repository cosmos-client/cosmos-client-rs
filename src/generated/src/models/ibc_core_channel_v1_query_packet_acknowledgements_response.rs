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
pub struct IbcCoreChannelV1QueryPacketAcknowledgementsResponse {
    #[serde(rename = "acknowledgements", skip_serializing_if = "Option::is_none")]
    pub acknowledgements: Option<Vec<crate::models::QueryPacketAcknowledgemetsResponseIsTheRequestTypeForTheQueryQueryPacketAcknowledgementsRpcMethodAcknowledgements>>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<crate::models::PaginationResponse>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<crate::models::QueryBlockHeight>,
}

impl IbcCoreChannelV1QueryPacketAcknowledgementsResponse {
    pub fn new() -> IbcCoreChannelV1QueryPacketAcknowledgementsResponse {
        IbcCoreChannelV1QueryPacketAcknowledgementsResponse {
            acknowledgements: None,
            pagination: None,
            height: None,
        }
    }
}


