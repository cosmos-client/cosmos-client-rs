/*
 * Cosmos SDK - Legacy REST and gRPC Gateway docs
 *
 * A REST interface for state queries, legacy transactions
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosBaseAbciV1beta1AbciMessageLog : ABCIMessageLog defines a structure containing an indexed tx ABCI message log.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosBaseAbciV1beta1AbciMessageLog {
    #[serde(rename = "msg_index", skip_serializing_if = "Option::is_none")]
    pub msg_index: Option<i64>,
    #[serde(rename = "log", skip_serializing_if = "Option::is_none")]
    pub log: Option<String>,
    /// Events contains a slice of Event objects that were emitted during some execution.
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<crate::models::InlineResponse20071TxResponseEvents>>,
}

impl CosmosBaseAbciV1beta1AbciMessageLog {
    /// ABCIMessageLog defines a structure containing an indexed tx ABCI message log.
    pub fn new() -> CosmosBaseAbciV1beta1AbciMessageLog {
        CosmosBaseAbciV1beta1AbciMessageLog {
            msg_index: None,
            log: None,
            events: None,
        }
    }
}

