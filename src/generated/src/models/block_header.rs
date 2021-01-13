/*
 * Cosmos SDK - Legacy REST and gRPC Gateway docs
 *
 * A REST interface for state queries, legacy transactions
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockHeader {
    #[serde(rename = "chain_id", skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<String>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<f32>,
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(rename = "num_txs", skip_serializing_if = "Option::is_none")]
    pub num_txs: Option<f32>,
    #[serde(rename = "last_block_id", skip_serializing_if = "Option::is_none")]
    pub last_block_id: Option<crate::models::InlineResponse2002BlockMetaHeaderLastBlockId>,
    #[serde(rename = "total_txs", skip_serializing_if = "Option::is_none")]
    pub total_txs: Option<f32>,
    #[serde(rename = "last_commit_hash", skip_serializing_if = "Option::is_none")]
    pub last_commit_hash: Option<String>,
    #[serde(rename = "data_hash", skip_serializing_if = "Option::is_none")]
    pub data_hash: Option<String>,
    #[serde(rename = "validators_hash", skip_serializing_if = "Option::is_none")]
    pub validators_hash: Option<String>,
    #[serde(rename = "next_validators_hash", skip_serializing_if = "Option::is_none")]
    pub next_validators_hash: Option<String>,
    #[serde(rename = "consensus_hash", skip_serializing_if = "Option::is_none")]
    pub consensus_hash: Option<String>,
    #[serde(rename = "app_hash", skip_serializing_if = "Option::is_none")]
    pub app_hash: Option<String>,
    #[serde(rename = "last_results_hash", skip_serializing_if = "Option::is_none")]
    pub last_results_hash: Option<String>,
    #[serde(rename = "evidence_hash", skip_serializing_if = "Option::is_none")]
    pub evidence_hash: Option<String>,
    /// bech32 encoded address
    #[serde(rename = "proposer_address", skip_serializing_if = "Option::is_none")]
    pub proposer_address: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::models::InlineResponse2002BlockMetaHeaderVersion>,
}

impl BlockHeader {
    pub fn new() -> BlockHeader {
        BlockHeader {
            chain_id: None,
            height: None,
            time: None,
            num_txs: None,
            last_block_id: None,
            total_txs: None,
            last_commit_hash: None,
            data_hash: None,
            validators_hash: None,
            next_validators_hash: None,
            consensus_hash: None,
            app_hash: None,
            last_results_hash: None,
            evidence_hash: None,
            proposer_address: None,
            version: None,
        }
    }
}


