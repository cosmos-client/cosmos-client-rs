/*
 * Cosmos SDK - Legacy REST and gRPC Gateway docs
 *
 * A REST interface for state queries, legacy transactions
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosEvidenceV1beta1QueryAllEvidenceResponse : QueryAllEvidenceResponse is the response type for the Query/AllEvidence RPC method.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosEvidenceV1beta1QueryAllEvidenceResponse {
    /// evidence returns all evidences.
    #[serde(rename = "evidence", skip_serializing_if = "Option::is_none")]
    pub evidence: Option<Vec<crate::models::InlineResponseDefaultDetails>>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<crate::models::InlineResponse20027Pagination>,
}

impl CosmosEvidenceV1beta1QueryAllEvidenceResponse {
    /// QueryAllEvidenceResponse is the response type for the Query/AllEvidence RPC method.
    pub fn new() -> CosmosEvidenceV1beta1QueryAllEvidenceResponse {
        CosmosEvidenceV1beta1QueryAllEvidenceResponse {
            evidence: None,
            pagination: None,
        }
    }
}


