/*
 * Cosmos SDK - Legacy REST and gRPC Gateway docs
 *
 * A REST interface for state queries, legacy transactions
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IbcCoreConnectionV1IdentifiedConnection : IdentifiedConnection defines a connection with additional connection identifier field.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IbcCoreConnectionV1IdentifiedConnection {
    /// connection identifier.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// client associated with this connection.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "versions", skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<crate::models::InlineResponse20080Versions>>,
    /// current state of the connection end.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    #[serde(rename = "counterparty", skip_serializing_if = "Option::is_none")]
    pub counterparty: Option<crate::models::InlineResponse20080Counterparty>,
    /// delay period associated with this connection.
    #[serde(rename = "delay_period", skip_serializing_if = "Option::is_none")]
    pub delay_period: Option<String>,
}

impl IbcCoreConnectionV1IdentifiedConnection {
    /// IdentifiedConnection defines a connection with additional connection identifier field.
    pub fn new() -> IbcCoreConnectionV1IdentifiedConnection {
        IbcCoreConnectionV1IdentifiedConnection {
            id: None,
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

