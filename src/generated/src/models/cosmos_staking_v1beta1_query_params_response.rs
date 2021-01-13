/*
 * Cosmos SDK - Legacy REST and gRPC Gateway docs
 *
 * A REST interface for state queries, legacy transactions
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosStakingV1beta1QueryParamsResponse : QueryParamsResponse is response type for the Query/Params RPC method.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosStakingV1beta1QueryParamsResponse {
    #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
    pub params: Option<crate::models::InlineResponse20065Params>,
}

impl CosmosStakingV1beta1QueryParamsResponse {
    /// QueryParamsResponse is response type for the Query/Params RPC method.
    pub fn new() -> CosmosStakingV1beta1QueryParamsResponse {
        CosmosStakingV1beta1QueryParamsResponse {
            params: None,
        }
    }
}

