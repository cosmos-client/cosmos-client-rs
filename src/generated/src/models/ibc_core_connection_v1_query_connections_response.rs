/*
 * Cosmos SDK - Legacy REST and gRPC Gateway docs
 *
 * A REST interface for state queries, legacy transactions
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IbcCoreConnectionV1QueryConnectionsResponse : QueryConnectionsResponse is the response type for the Query/Connections RPC method.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IbcCoreConnectionV1QueryConnectionsResponse {
    /// list of stored connections of the chain.
    #[serde(rename = "connections", skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<crate::models::InlineResponse20080Connections>>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<crate::models::PaginationResponse>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<crate::models::QueryBlockHeight>,
}

impl IbcCoreConnectionV1QueryConnectionsResponse {
    /// QueryConnectionsResponse is the response type for the Query/Connections RPC method.
    pub fn new() -> IbcCoreConnectionV1QueryConnectionsResponse {
        IbcCoreConnectionV1QueryConnectionsResponse {
            connections: None,
            pagination: None,
            height: None,
        }
    }
}


