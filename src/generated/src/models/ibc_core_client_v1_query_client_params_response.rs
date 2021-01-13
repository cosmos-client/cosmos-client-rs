/*
 * Cosmos SDK - Legacy REST and gRPC Gateway docs
 *
 * A REST interface for state queries, legacy transactions
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IbcCoreClientV1QueryClientParamsResponse : QueryClientParamsResponse is the response type for the Query/ClientParams RPC method.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IbcCoreClientV1QueryClientParamsResponse {
    #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
    pub params: Option<crate::models::InlineResponse20077Params>,
}

impl IbcCoreClientV1QueryClientParamsResponse {
    /// QueryClientParamsResponse is the response type for the Query/ClientParams RPC method.
    pub fn new() -> IbcCoreClientV1QueryClientParamsResponse {
        IbcCoreClientV1QueryClientParamsResponse {
            params: None,
        }
    }
}


