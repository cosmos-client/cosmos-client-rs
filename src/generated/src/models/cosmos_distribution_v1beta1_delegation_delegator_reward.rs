/*
 * Cosmos SDK - Legacy REST and gRPC Gateway docs
 *
 * A REST interface for state queries, legacy transactions
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosDistributionV1beta1DelegationDelegatorReward : DelegationDelegatorReward represents the properties of a delegator's delegation reward.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosDistributionV1beta1DelegationDelegatorReward {
    #[serde(rename = "validator_address", skip_serializing_if = "Option::is_none")]
    pub validator_address: Option<String>,
    #[serde(rename = "reward", skip_serializing_if = "Option::is_none")]
    pub reward: Option<Vec<crate::models::InlineResponse20037Pool>>,
}

impl CosmosDistributionV1beta1DelegationDelegatorReward {
    /// DelegationDelegatorReward represents the properties of a delegator's delegation reward.
    pub fn new() -> CosmosDistributionV1beta1DelegationDelegatorReward {
        CosmosDistributionV1beta1DelegationDelegatorReward {
            validator_address: None,
            reward: None,
        }
    }
}


