/*
 * Cosmos SDK - Legacy REST and gRPC Gateway docs
 *
 * A REST interface for state queries, legacy transactions
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IbcCoreConnectionV1QueryConnectionResponse : QueryConnectionResponse is the response type for the Query/Connection RPC method. Besides the connection end, it includes a proof and the height from which the proof was retrieved.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IbcCoreConnectionV1QueryConnectionResponse {
    #[serde(rename = "connection", skip_serializing_if = "Option::is_none")]
    pub connection: Option<crate::models::ConnectionAssociatedWithTheRequestIdentifier>,
    #[serde(rename = "proof", skip_serializing_if = "Option::is_none")]
    pub proof: Option<String>,
    #[serde(rename = "proof_height", skip_serializing_if = "Option::is_none")]
    pub proof_height: Option<crate::models::HeightAtWhichTheProofWasRetrieved>,
}

impl IbcCoreConnectionV1QueryConnectionResponse {
    /// QueryConnectionResponse is the response type for the Query/Connection RPC method. Besides the connection end, it includes a proof and the height from which the proof was retrieved.
    pub fn new() -> IbcCoreConnectionV1QueryConnectionResponse {
        IbcCoreConnectionV1QueryConnectionResponse {
            connection: None,
            proof: None,
            proof_height: None,
        }
    }
}


