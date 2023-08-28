/*
 * Firecracker API
 *
 * RESTful public-facing API. The API is accessible through HTTP calls on specific URLs carrying JSON modeled data. The transport medium is a Unix Domain Socket.
 *
 * The version of the OpenAPI document: 1.4.1
 * Contact: compute-capsule@amazon.com
 * Generated by: https://openapi-generator.tech
 */

/// SnapshotLoadParams : Defines the configuration used for handling snapshot resume. Exactly one of the two `mem_*` fields must be present in the body of the request.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotLoadParams {
    /// Enable support for incremental (diff) snapshots by tracking dirty guest pages.
    #[serde(rename = "enable_diff_snapshots", skip_serializing_if = "Option::is_none")]
    pub enable_diff_snapshots: Option<bool>,
    /// Path to the file that contains the guest memory to be loaded. This parameter has been deprecated and is only allowed if `mem_backend` is not present.
    #[serde(rename = "mem_file_path", skip_serializing_if = "Option::is_none")]
    pub mem_file_path: Option<String>,
    #[serde(rename = "mem_backend", skip_serializing_if = "Option::is_none")]
    pub mem_backend: Option<Box<crate::models::MemoryBackend>>,
    /// Path to the file that contains the microVM state to be loaded.
    #[serde(rename = "snapshot_path")]
    pub snapshot_path: String,
    /// When set to true, the vm is also resumed if the snapshot load is successful.
    #[serde(rename = "resume_vm", skip_serializing_if = "Option::is_none")]
    pub resume_vm: Option<bool>,
}

impl SnapshotLoadParams {
    /// Defines the configuration used for handling snapshot resume. Exactly one of the two `mem_*` fields must be present in the body of the request.
    pub fn new(snapshot_path: String) -> SnapshotLoadParams {
        SnapshotLoadParams {
            enable_diff_snapshots: None,
            mem_file_path: None,
            mem_backend: None,
            snapshot_path,
            resume_vm: None,
        }
    }
}


