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
pub struct CosmosSlashingV1beta1QuerySigningInfoResponse {
    #[serde(rename = "val_signing_info", skip_serializing_if = "Option::is_none")]
    pub val_signing_info: Option<crate::models::ValSigningInfoIsTheSigningInfoOfRequestedValConsAddress>,
}

impl CosmosSlashingV1beta1QuerySigningInfoResponse {
    pub fn new() -> CosmosSlashingV1beta1QuerySigningInfoResponse {
        CosmosSlashingV1beta1QuerySigningInfoResponse {
            val_signing_info: None,
        }
    }
}


