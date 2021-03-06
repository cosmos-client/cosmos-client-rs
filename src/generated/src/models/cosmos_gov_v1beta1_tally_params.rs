/*
 * Cosmos SDK - Legacy REST and gRPC Gateway docs
 *
 * A REST interface for state queries, legacy transactions
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosGovV1beta1TallyParams : TallyParams defines the params for tallying votes on governance proposals.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosGovV1beta1TallyParams {
    /// Minimum percentage of total stake needed to vote for a result to be  considered valid.
    #[serde(rename = "quorum", skip_serializing_if = "Option::is_none")]
    pub quorum: Option<String>,
    /// Minimum proportion of Yes votes for proposal to pass. Default value: 0.5.
    #[serde(rename = "threshold", skip_serializing_if = "Option::is_none")]
    pub threshold: Option<String>,
    /// Minimum value of Veto votes to Total votes ratio for proposal to be  vetoed. Default value: 1/3.
    #[serde(rename = "veto_threshold", skip_serializing_if = "Option::is_none")]
    pub veto_threshold: Option<String>,
}

impl CosmosGovV1beta1TallyParams {
    /// TallyParams defines the params for tallying votes on governance proposals.
    pub fn new() -> CosmosGovV1beta1TallyParams {
        CosmosGovV1beta1TallyParams {
            quorum: None,
            threshold: None,
            veto_threshold: None,
        }
    }
}


