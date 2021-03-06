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
pub struct CosmosBaseTendermintV1beta1Module {
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "sum", skip_serializing_if = "Option::is_none")]
    pub sum: Option<String>,
}

impl CosmosBaseTendermintV1beta1Module {
    pub fn new() -> CosmosBaseTendermintV1beta1Module {
        CosmosBaseTendermintV1beta1Module {
            path: None,
            version: None,
            sum: None,
        }
    }
}


