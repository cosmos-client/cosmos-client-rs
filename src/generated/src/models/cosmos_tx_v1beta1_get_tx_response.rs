/*
 * Cosmos SDK - Legacy REST and gRPC Gateway docs
 *
 * A REST interface for state queries, legacy transactions
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosTxV1beta1GetTxResponse : GetTxResponse is the response type for the Service.GetTx method.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosTxV1beta1GetTxResponse {
    #[serde(rename = "tx", skip_serializing_if = "Option::is_none")]
    pub tx: Option<crate::models::CosmosTxV1beta1Tx>,
    #[serde(rename = "tx_response", skip_serializing_if = "Option::is_none")]
    pub tx_response: Option<crate::models::InlineResponse20071TxResponse>,
}

impl CosmosTxV1beta1GetTxResponse {
    /// GetTxResponse is the response type for the Service.GetTx method.
    pub fn new() -> CosmosTxV1beta1GetTxResponse {
        CosmosTxV1beta1GetTxResponse {
            tx: None,
            tx_response: None,
        }
    }
}


