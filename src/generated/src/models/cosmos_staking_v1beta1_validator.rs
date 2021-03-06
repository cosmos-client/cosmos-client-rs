/*
 * Cosmos SDK - Legacy REST and gRPC Gateway docs
 *
 * A REST interface for state queries, legacy transactions
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosStakingV1beta1Validator : Validator defines a validator, together with the total amount of the Validator's bond shares and their exchange rate to coins. Slashing results in a decrease in the exchange rate, allowing correct calculation of future undelegations without iterating over delegators. When coins are delegated to this validator, the validator is credited with a delegation whose number of bond shares is based on the amount of coins delegated divided by the current exchange rate. Voting power can be calculated as total bonded shares multiplied by exchange rate.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosStakingV1beta1Validator {
    #[serde(rename = "operator_address", skip_serializing_if = "Option::is_none")]
    pub operator_address: Option<String>,
    #[serde(rename = "consensus_pubkey", skip_serializing_if = "Option::is_none")]
    pub consensus_pubkey: Option<crate::models::InlineResponseDefaultDetails>,
    #[serde(rename = "jailed", skip_serializing_if = "Option::is_none")]
    pub jailed: Option<bool>,
    /// BondStatus is the status of a validator.   - BOND_STATUS_UNSPECIFIED: UNSPECIFIED defines an invalid validator status.  - BOND_STATUS_UNBONDED: UNBONDED defines a validator that is not bonded.  - BOND_STATUS_UNBONDING: UNBONDING defines a validator that is unbonding.  - BOND_STATUS_BONDED: BONDED defines a validator that is bonded.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "tokens", skip_serializing_if = "Option::is_none")]
    pub tokens: Option<String>,
    #[serde(rename = "delegator_shares", skip_serializing_if = "Option::is_none")]
    pub delegator_shares: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<crate::models::InlineResponse20062Description>,
    #[serde(rename = "unbonding_height", skip_serializing_if = "Option::is_none")]
    pub unbonding_height: Option<String>,
    #[serde(rename = "unbonding_time", skip_serializing_if = "Option::is_none")]
    pub unbonding_time: Option<String>,
    #[serde(rename = "commission", skip_serializing_if = "Option::is_none")]
    pub commission: Option<crate::models::InlineResponse20062Commission>,
    #[serde(rename = "min_self_delegation", skip_serializing_if = "Option::is_none")]
    pub min_self_delegation: Option<String>,
}

impl CosmosStakingV1beta1Validator {
    /// Validator defines a validator, together with the total amount of the Validator's bond shares and their exchange rate to coins. Slashing results in a decrease in the exchange rate, allowing correct calculation of future undelegations without iterating over delegators. When coins are delegated to this validator, the validator is credited with a delegation whose number of bond shares is based on the amount of coins delegated divided by the current exchange rate. Voting power can be calculated as total bonded shares multiplied by exchange rate.
    pub fn new() -> CosmosStakingV1beta1Validator {
        CosmosStakingV1beta1Validator {
            operator_address: None,
            consensus_pubkey: None,
            jailed: None,
            status: None,
            tokens: None,
            delegator_shares: None,
            description: None,
            unbonding_height: None,
            unbonding_time: None,
            commission: None,
            min_self_delegation: None,
        }
    }
}

/// BondStatus is the status of a validator.   - BOND_STATUS_UNSPECIFIED: UNSPECIFIED defines an invalid validator status.  - BOND_STATUS_UNBONDED: UNBONDED defines a validator that is not bonded.  - BOND_STATUS_UNBONDING: UNBONDING defines a validator that is unbonding.  - BOND_STATUS_BONDED: BONDED defines a validator that is bonded.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "BOND_STATUS_UNSPECIFIED")]
    UNSPECIFIED,
    #[serde(rename = "BOND_STATUS_UNBONDED")]
    UNBONDED,
    #[serde(rename = "BOND_STATUS_UNBONDING")]
    UNBONDING,
    #[serde(rename = "BOND_STATUS_BONDED")]
    BONDED,
}

