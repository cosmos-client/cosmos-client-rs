/*
 * Cosmos SDK - Legacy REST and gRPC Gateway docs
 *
 * A REST interface for state queries, legacy transactions
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosTxV1beta1GetTxsEventResponseTxResponses : TxResponse defines a structure containing relevant tx data and metadata. The tags are stringified and the log is JSON decoded.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosTxV1beta1GetTxsEventResponseTxResponses {
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
    /// The transaction hash.
    #[serde(rename = "txhash", skip_serializing_if = "Option::is_none")]
    pub txhash: Option<String>,
    #[serde(rename = "codespace", skip_serializing_if = "Option::is_none")]
    pub codespace: Option<String>,
    /// Response code.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    /// Result bytes, if any.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// The output of the application's logger (raw string). May be non-deterministic.
    #[serde(rename = "raw_log", skip_serializing_if = "Option::is_none")]
    pub raw_log: Option<String>,
    /// The output of the application's logger (typed). May be non-deterministic.
    #[serde(rename = "logs", skip_serializing_if = "Option::is_none")]
    pub logs: Option<Vec<crate::models::InlineResponse20071TxResponseLogs>>,
    /// Additional information. May be non-deterministic.
    #[serde(rename = "info", skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
    /// Amount of gas requested for transaction.
    #[serde(rename = "gas_wanted", skip_serializing_if = "Option::is_none")]
    pub gas_wanted: Option<String>,
    /// Amount of gas consumed by transaction.
    #[serde(rename = "gas_used", skip_serializing_if = "Option::is_none")]
    pub gas_used: Option<String>,
    #[serde(rename = "tx", skip_serializing_if = "Option::is_none")]
    pub tx: Option<crate::models::InlineResponse20071TxResponseTx>,
    /// Time of the previous block. For heights > 1, it's the weighted median of the timestamps of the valid votes in the block.LastCommit. For height == 1, it's genesis time.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl CosmosTxV1beta1GetTxsEventResponseTxResponses {
    /// TxResponse defines a structure containing relevant tx data and metadata. The tags are stringified and the log is JSON decoded.
    pub fn new() -> CosmosTxV1beta1GetTxsEventResponseTxResponses {
        CosmosTxV1beta1GetTxsEventResponseTxResponses {
            height: None,
            txhash: None,
            codespace: None,
            code: None,
            data: None,
            raw_log: None,
            logs: None,
            info: None,
            gas_wanted: None,
            gas_used: None,
            tx: None,
            timestamp: None,
        }
    }
}


