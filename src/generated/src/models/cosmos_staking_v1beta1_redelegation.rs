/*
 * Cosmos SDK - Legacy REST and gRPC Gateway docs
 *
 * A REST interface for state queries, legacy transactions
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosStakingV1beta1Redelegation : Redelegation contains the list of a particular delegator's redelegating bonds from a particular source validator to a particular destination validator.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosStakingV1beta1Redelegation {
    #[serde(rename = "delegator_address", skip_serializing_if = "Option::is_none")]
    pub delegator_address: Option<String>,
    #[serde(rename = "validator_src_address", skip_serializing_if = "Option::is_none")]
    pub validator_src_address: Option<String>,
    #[serde(rename = "validator_dst_address", skip_serializing_if = "Option::is_none")]
    pub validator_dst_address: Option<String>,
    #[serde(rename = "entries", skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<crate::models::InlineResponse20060RedelegationEntries>>,
}

impl CosmosStakingV1beta1Redelegation {
    /// Redelegation contains the list of a particular delegator's redelegating bonds from a particular source validator to a particular destination validator.
    pub fn new() -> CosmosStakingV1beta1Redelegation {
        CosmosStakingV1beta1Redelegation {
            delegator_address: None,
            validator_src_address: None,
            validator_dst_address: None,
            entries: None,
        }
    }
}


