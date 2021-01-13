/*
 * Cosmos SDK - Legacy REST and gRPC Gateway docs
 *
 * A REST interface for state queries, legacy transactions
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosStakingV1beta1DelegationResponse : DelegationResponse is equivalent to Delegation except that it contains a balance in addition to shares which is more suitable for client responses.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosStakingV1beta1DelegationResponse {
    #[serde(rename = "delegation", skip_serializing_if = "Option::is_none")]
    pub delegation: Option<crate::models::InlineResponse20059Delegation>,
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none")]
    pub balance: Option<crate::models::InlineResponse20027Balances>,
}

impl CosmosStakingV1beta1DelegationResponse {
    /// DelegationResponse is equivalent to Delegation except that it contains a balance in addition to shares which is more suitable for client responses.
    pub fn new() -> CosmosStakingV1beta1DelegationResponse {
        CosmosStakingV1beta1DelegationResponse {
            delegation: None,
            balance: None,
        }
    }
}

