/*
 * Cosmos SDK - Legacy REST and gRPC Gateway docs
 *
 * A REST interface for state queries, legacy transactions
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosStakingV1beta1QueryHistoricalInfoResponse : QueryHistoricalInfoResponse is response type for the Query/HistoricalInfo RPC method.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosStakingV1beta1QueryHistoricalInfoResponse {
    #[serde(rename = "hist", skip_serializing_if = "Option::is_none")]
    pub hist: Option<crate::models::InlineResponse20064Hist>,
}

impl CosmosStakingV1beta1QueryHistoricalInfoResponse {
    /// QueryHistoricalInfoResponse is response type for the Query/HistoricalInfo RPC method.
    pub fn new() -> CosmosStakingV1beta1QueryHistoricalInfoResponse {
        CosmosStakingV1beta1QueryHistoricalInfoResponse {
            hist: None,
        }
    }
}

