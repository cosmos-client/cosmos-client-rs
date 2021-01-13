/*
 * Cosmos SDK - Legacy REST and gRPC Gateway docs
 *
 * A REST interface for state queries, legacy transactions
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosDistributionV1beta1QueryDelegatorWithdrawAddressResponse : QueryDelegatorWithdrawAddressResponse is the response type for the Query/DelegatorWithdrawAddress RPC method.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosDistributionV1beta1QueryDelegatorWithdrawAddressResponse {
    /// withdraw_address defines the delegator address to query for.
    #[serde(rename = "withdraw_address", skip_serializing_if = "Option::is_none")]
    pub withdraw_address: Option<String>,
}

impl CosmosDistributionV1beta1QueryDelegatorWithdrawAddressResponse {
    /// QueryDelegatorWithdrawAddressResponse is the response type for the Query/DelegatorWithdrawAddress RPC method.
    pub fn new() -> CosmosDistributionV1beta1QueryDelegatorWithdrawAddressResponse {
        CosmosDistributionV1beta1QueryDelegatorWithdrawAddressResponse {
            withdraw_address: None,
        }
    }
}


