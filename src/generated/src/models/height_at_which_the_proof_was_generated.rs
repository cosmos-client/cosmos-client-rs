/*
 * Cosmos SDK - Legacy REST and gRPC Gateway docs
 *
 * A REST interface for state queries, legacy transactions
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// HeightAtWhichTheProofWasGenerated : Normally the RevisionHeight is incremented at each height while keeping RevisionNumber the same. However some consensus algorithms may choose to reset the height in certain conditions e.g. hard forks, state-machine breaking changes In these cases, the RevisionNumber is incremented so that height continues to be monitonically increasing even as the RevisionHeight gets reset



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HeightAtWhichTheProofWasGenerated {
    #[serde(rename = "revision_number", skip_serializing_if = "Option::is_none")]
    pub revision_number: Option<String>,
    #[serde(rename = "revision_height", skip_serializing_if = "Option::is_none")]
    pub revision_height: Option<String>,
}

impl HeightAtWhichTheProofWasGenerated {
    /// Normally the RevisionHeight is incremented at each height while keeping RevisionNumber the same. However some consensus algorithms may choose to reset the height in certain conditions e.g. hard forks, state-machine breaking changes In these cases, the RevisionNumber is incremented so that height continues to be monitonically increasing even as the RevisionHeight gets reset
    pub fn new() -> HeightAtWhichTheProofWasGenerated {
        HeightAtWhichTheProofWasGenerated {
            revision_number: None,
            revision_height: None,
        }
    }
}


