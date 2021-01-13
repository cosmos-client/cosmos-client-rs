/*
 * Cosmos SDK - Legacy REST and gRPC Gateway docs
 *
 * A REST interface for state queries, legacy transactions
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CosmosUpgradeV1beta1QueryCurrentPlanResponse : QueryCurrentPlanResponse is the response type for the Query/CurrentPlan RPC method.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosUpgradeV1beta1QueryCurrentPlanResponse {
    #[serde(rename = "plan", skip_serializing_if = "Option::is_none")]
    pub plan: Option<crate::models::InlineResponse20073Plan>,
}

impl CosmosUpgradeV1beta1QueryCurrentPlanResponse {
    /// QueryCurrentPlanResponse is the response type for the Query/CurrentPlan RPC method.
    pub fn new() -> CosmosUpgradeV1beta1QueryCurrentPlanResponse {
        CosmosUpgradeV1beta1QueryCurrentPlanResponse {
            plan: None,
        }
    }
}

