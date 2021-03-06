/*
 * Cosmos SDK - Legacy REST and gRPC Gateway docs
 *
 * A REST interface for state queries, legacy transactions
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ConnectionAssociatedWithTheRequestIdentifier : ConnectionEnd defines a stateful object on a chain connected to another separate one. NOTE: there must only be 2 defined ConnectionEnds to establish a connection between two chains.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectionAssociatedWithTheRequestIdentifier {
    /// client associated with this connection.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// IBC version which can be utilised to determine encodings or protocols for channels or packets utilising this connection.
    #[serde(rename = "versions", skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<crate::models::InlineResponse20080Versions>>,
    /// current state of the connection end.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    #[serde(rename = "counterparty", skip_serializing_if = "Option::is_none")]
    pub counterparty: Option<crate::models::InlineResponse20080Counterparty>,
    /// delay period that must pass before a consensus state can be used for packet-verification NOTE: delay period logic is only implemented by some clients.
    #[serde(rename = "delay_period", skip_serializing_if = "Option::is_none")]
    pub delay_period: Option<String>,
}

impl ConnectionAssociatedWithTheRequestIdentifier {
    /// ConnectionEnd defines a stateful object on a chain connected to another separate one. NOTE: there must only be 2 defined ConnectionEnds to establish a connection between two chains.
    pub fn new() -> ConnectionAssociatedWithTheRequestIdentifier {
        ConnectionAssociatedWithTheRequestIdentifier {
            client_id: None,
            versions: None,
            state: None,
            counterparty: None,
            delay_period: None,
        }
    }
}

/// current state of the connection end.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "STATE_UNINITIALIZED_UNSPECIFIED")]
    UNINITIALIZEDUNSPECIFIED,
    #[serde(rename = "STATE_INIT")]
    INIT,
    #[serde(rename = "STATE_TRYOPEN")]
    TRYOPEN,
    #[serde(rename = "STATE_OPEN")]
    OPEN,
}

