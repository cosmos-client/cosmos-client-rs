/*
 * Cosmos SDK - Legacy REST and gRPC Gateway docs
 *
 * A REST interface for state queries, legacy transactions
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosTxV1beta1BroadcastTxRequest : BroadcastTxRequest is the request type for the Service.BroadcastTxRequest RPC method.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosTxV1beta1BroadcastTxRequest {
    /// tx_bytes is the raw transaction.
    #[serde(rename = "tx_bytes", skip_serializing_if = "Option::is_none")]
    pub tx_bytes: Option<String>,
    /// BroadcastMode specifies the broadcast mode for the TxService.Broadcast RPC method.   - BROADCAST_MODE_UNSPECIFIED: zero-value for mode ordering  - BROADCAST_MODE_BLOCK: BROADCAST_MODE_BLOCK defines a tx broadcasting mode where the client waits for the tx to be committed in a block.  - BROADCAST_MODE_SYNC: BROADCAST_MODE_SYNC defines a tx broadcasting mode where the client waits for a CheckTx execution response only.  - BROADCAST_MODE_ASYNC: BROADCAST_MODE_ASYNC defines a tx broadcasting mode where the client returns immediately.
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
}

impl CosmosTxV1beta1BroadcastTxRequest {
    /// BroadcastTxRequest is the request type for the Service.BroadcastTxRequest RPC method.
    pub fn new() -> CosmosTxV1beta1BroadcastTxRequest {
        CosmosTxV1beta1BroadcastTxRequest {
            tx_bytes: None,
            mode: None,
        }
    }
}

/// BroadcastMode specifies the broadcast mode for the TxService.Broadcast RPC method.   - BROADCAST_MODE_UNSPECIFIED: zero-value for mode ordering  - BROADCAST_MODE_BLOCK: BROADCAST_MODE_BLOCK defines a tx broadcasting mode where the client waits for the tx to be committed in a block.  - BROADCAST_MODE_SYNC: BROADCAST_MODE_SYNC defines a tx broadcasting mode where the client waits for a CheckTx execution response only.  - BROADCAST_MODE_ASYNC: BROADCAST_MODE_ASYNC defines a tx broadcasting mode where the client returns immediately.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "BROADCAST_MODE_UNSPECIFIED")]
    UNSPECIFIED,
    #[serde(rename = "BROADCAST_MODE_BLOCK")]
    BLOCK,
    #[serde(rename = "BROADCAST_MODE_SYNC")]
    SYNC,
    #[serde(rename = "BROADCAST_MODE_ASYNC")]
    _ASYNC,
}
