/*
 * Cosmos SDK - Legacy REST and gRPC Gateway docs
 *
 * A REST interface for state queries, legacy transactions
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosGovV1beta1QueryDepositResponse : QueryDepositResponse is the response type for the Query/Deposit RPC method.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosGovV1beta1QueryDepositResponse {
    #[serde(rename = "deposit", skip_serializing_if = "Option::is_none")]
    pub deposit: Option<crate::models::InlineResponse20051Deposit>,
}

impl CosmosGovV1beta1QueryDepositResponse {
    /// QueryDepositResponse is the response type for the Query/Deposit RPC method.
    pub fn new() -> CosmosGovV1beta1QueryDepositResponse {
        CosmosGovV1beta1QueryDepositResponse {
            deposit: None,
        }
    }
}


