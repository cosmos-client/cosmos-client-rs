/*
 * Cosmos SDK - Legacy REST and gRPC Gateway docs
 *
 * A REST interface for state queries, legacy transactions
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosSlashingV1beta1ValidatorSigningInfo : ValidatorSigningInfo defines a validator's signing info for monitoring their liveness activity.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosSlashingV1beta1ValidatorSigningInfo {
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "start_height", skip_serializing_if = "Option::is_none")]
    pub start_height: Option<String>,
    #[serde(rename = "index_offset", skip_serializing_if = "Option::is_none")]
    pub index_offset: Option<String>,
    #[serde(rename = "jailed_until", skip_serializing_if = "Option::is_none")]
    pub jailed_until: Option<String>,
    #[serde(rename = "tombstoned", skip_serializing_if = "Option::is_none")]
    pub tombstoned: Option<bool>,
    #[serde(rename = "missed_blocks_counter", skip_serializing_if = "Option::is_none")]
    pub missed_blocks_counter: Option<String>,
}

impl CosmosSlashingV1beta1ValidatorSigningInfo {
    /// ValidatorSigningInfo defines a validator's signing info for monitoring their liveness activity.
    pub fn new() -> CosmosSlashingV1beta1ValidatorSigningInfo {
        CosmosSlashingV1beta1ValidatorSigningInfo {
            address: None,
            start_height: None,
            index_offset: None,
            jailed_until: None,
            tombstoned: None,
            missed_blocks_counter: None,
        }
    }
}

