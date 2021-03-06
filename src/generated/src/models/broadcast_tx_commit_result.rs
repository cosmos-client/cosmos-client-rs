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
pub struct BroadcastTxCommitResult {
    #[serde(rename = "check_tx", skip_serializing_if = "Option::is_none")]
    pub check_tx: Option<crate::models::InlineResponse2006CheckTx>,
    #[serde(rename = "deliver_tx", skip_serializing_if = "Option::is_none")]
    pub deliver_tx: Option<crate::models::InlineResponse2006DeliverTx>,
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
}

impl BroadcastTxCommitResult {
    pub fn new() -> BroadcastTxCommitResult {
        BroadcastTxCommitResult {
            check_tx: None,
            deliver_tx: None,
            hash: None,
            height: None,
        }
    }
}


