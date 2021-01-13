/*
 * Cosmos SDK - Legacy REST and gRPC Gateway docs
 *
 * A REST interface for state queries, legacy transactions
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IbcCoreChannelV1Order : - ORDER_NONE_UNSPECIFIED: zero-value for channel ordering  - ORDER_UNORDERED: packets can be delivered in any order, which may differ from the order in which they were sent.  - ORDER_ORDERED: packets are delivered exactly in the order which they were sent

/// - ORDER_NONE_UNSPECIFIED: zero-value for channel ordering  - ORDER_UNORDERED: packets can be delivered in any order, which may differ from the order in which they were sent.  - ORDER_ORDERED: packets are delivered exactly in the order which they were sent
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IbcCoreChannelV1Order {
    #[serde(rename = "ORDER_NONE_UNSPECIFIED")]
    NONEUNSPECIFIED,
    #[serde(rename = "ORDER_UNORDERED")]
    UNORDERED,
    #[serde(rename = "ORDER_ORDERED")]
    ORDERED,

}




