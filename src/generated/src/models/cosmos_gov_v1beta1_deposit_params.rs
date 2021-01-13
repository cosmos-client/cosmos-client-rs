/*
 * Cosmos SDK - Legacy REST and gRPC Gateway docs
 *
 * A REST interface for state queries, legacy transactions
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosGovV1beta1DepositParams : DepositParams defines the params for deposits on governance proposals.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosGovV1beta1DepositParams {
    /// Minimum deposit for a proposal to enter voting period.
    #[serde(rename = "min_deposit", skip_serializing_if = "Option::is_none")]
    pub min_deposit: Option<Vec<crate::models::InlineResponse20027Balances>>,
    /// Maximum period for Atom holders to deposit on a proposal. Initial value: 2  months.
    #[serde(rename = "max_deposit_period", skip_serializing_if = "Option::is_none")]
    pub max_deposit_period: Option<String>,
}

impl CosmosGovV1beta1DepositParams {
    /// DepositParams defines the params for deposits on governance proposals.
    pub fn new() -> CosmosGovV1beta1DepositParams {
        CosmosGovV1beta1DepositParams {
            min_deposit: None,
            max_deposit_period: None,
        }
    }
}


