/*
 * Cosmos SDK - Legacy REST and gRPC Gateway docs
 *
 * A REST interface for state queries, legacy transactions
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosDistributionV1beta1QueryParamsResponse : QueryParamsResponse is the response type for the Query/Params RPC method.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosDistributionV1beta1QueryParamsResponse {
    #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
    pub params: Option<crate::models::InlineResponse20042Params>,
}

impl CosmosDistributionV1beta1QueryParamsResponse {
    /// QueryParamsResponse is the response type for the Query/Params RPC method.
    pub fn new() -> CosmosDistributionV1beta1QueryParamsResponse {
        CosmosDistributionV1beta1QueryParamsResponse {
            params: None,
        }
    }
}


