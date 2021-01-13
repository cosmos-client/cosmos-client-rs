/*
 * Cosmos SDK - Legacy REST and gRPC Gateway docs
 *
 * A REST interface for state queries, legacy transactions
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosAuthV1beta1QueryAccountResponse : QueryAccountResponse is the response type for the Query/Account RPC method.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosAuthV1beta1QueryAccountResponse {
    #[serde(rename = "account", skip_serializing_if = "Option::is_none")]
    pub account: Option<crate::models::InlineResponse20025Account>,
}

impl CosmosAuthV1beta1QueryAccountResponse {
    /// QueryAccountResponse is the response type for the Query/Account RPC method.
    pub fn new() -> CosmosAuthV1beta1QueryAccountResponse {
        CosmosAuthV1beta1QueryAccountResponse {
            account: None,
        }
    }
}


