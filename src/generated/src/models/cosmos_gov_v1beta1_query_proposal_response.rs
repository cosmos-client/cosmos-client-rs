/*
 * Cosmos SDK - Legacy REST and gRPC Gateway docs
 *
 * A REST interface for state queries, legacy transactions
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosGovV1beta1QueryProposalResponse : QueryProposalResponse is the response type for the Query/Proposal RPC method.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosGovV1beta1QueryProposalResponse {
    #[serde(rename = "proposal", skip_serializing_if = "Option::is_none")]
    pub proposal: Option<crate::models::InlineResponse20048Proposals>,
}

impl CosmosGovV1beta1QueryProposalResponse {
    /// QueryProposalResponse is the response type for the Query/Proposal RPC method.
    pub fn new() -> CosmosGovV1beta1QueryProposalResponse {
        CosmosGovV1beta1QueryProposalResponse {
            proposal: None,
        }
    }
}


