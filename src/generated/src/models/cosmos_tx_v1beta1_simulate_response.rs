/*
 * Cosmos SDK - Legacy REST and gRPC Gateway docs
 *
 * A REST interface for state queries, legacy transactions
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosTxV1beta1SimulateResponse : SimulateResponse is the response type for the Service.SimulateRPC method.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosTxV1beta1SimulateResponse {
    #[serde(rename = "gas_info", skip_serializing_if = "Option::is_none")]
    pub gas_info: Option<crate::models::InlineResponse20070GasInfo>,
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<crate::models::InlineResponse20070Result>,
}

impl CosmosTxV1beta1SimulateResponse {
    /// SimulateResponse is the response type for the Service.SimulateRPC method.
    pub fn new() -> CosmosTxV1beta1SimulateResponse {
        CosmosTxV1beta1SimulateResponse {
            gas_info: None,
            result: None,
        }
    }
}


