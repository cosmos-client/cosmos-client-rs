/*
 * Cosmos SDK - Legacy REST and gRPC Gateway docs
 *
 * A REST interface for state queries, legacy transactions
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IbcApplicationsTransferV1DenomTrace : DenomTrace contains the base denomination for ICS20 fungible tokens and the source tracing information path.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IbcApplicationsTransferV1DenomTrace {
    /// path defines the chain of port/channel identifiers used for tracing the source of the fungible token.
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// base denomination of the relayed fungible token.
    #[serde(rename = "base_denom", skip_serializing_if = "Option::is_none")]
    pub base_denom: Option<String>,
}

impl IbcApplicationsTransferV1DenomTrace {
    /// DenomTrace contains the base denomination for ICS20 fungible tokens and the source tracing information path.
    pub fn new() -> IbcApplicationsTransferV1DenomTrace {
        IbcApplicationsTransferV1DenomTrace {
            path: None,
            base_denom: None,
        }
    }
}


