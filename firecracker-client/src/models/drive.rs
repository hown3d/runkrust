/*
 * Firecracker API
 *
 * RESTful public-facing API. The API is accessible through HTTP calls on specific URLs carrying JSON modeled data. The transport medium is a Unix Domain Socket.
 *
 * The version of the OpenAPI document: 1.4.1
 * Contact: compute-capsule@amazon.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Drive {
    #[serde(rename = "drive_id")]
    pub drive_id: String,
    /// Represents the caching strategy for the block device.
    #[serde(rename = "cache_type", skip_serializing_if = "Option::is_none")]
    pub cache_type: Option<CacheType>,
    #[serde(rename = "is_read_only")]
    pub is_read_only: bool,
    #[serde(rename = "is_root_device")]
    pub is_root_device: bool,
    /// Represents the unique id of the boot partition of this device. It is optional and it will be taken into account only if the is_root_device field is true.
    #[serde(rename = "partuuid", skip_serializing_if = "Option::is_none")]
    pub partuuid: Option<String>,
    /// Host level path for the guest drive
    #[serde(rename = "path_on_host")]
    pub path_on_host: String,
    #[serde(rename = "rate_limiter", skip_serializing_if = "Option::is_none")]
    pub rate_limiter: Option<Box<crate::models::RateLimiter>>,
    /// Type of the IO engine used by the device. \"Async\" is supported on host kernels newer than 5.10.51.
    #[serde(rename = "io_engine", skip_serializing_if = "Option::is_none")]
    pub io_engine: Option<IoEngine>,
}

impl Drive {
    pub fn new(drive_id: String, is_read_only: bool, is_root_device: bool, path_on_host: String) -> Drive {
        Drive {
            drive_id,
            cache_type: None,
            is_read_only,
            is_root_device,
            partuuid: None,
            path_on_host,
            rate_limiter: None,
            io_engine: None,
        }
    }
}

/// Represents the caching strategy for the block device.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CacheType {
    #[serde(rename = "Unsafe")]
    Unsafe,
    #[serde(rename = "Writeback")]
    Writeback,
}

impl Default for CacheType {
    fn default() -> CacheType {
        Self::Unsafe
    }
}
/// Type of the IO engine used by the device. \"Async\" is supported on host kernels newer than 5.10.51.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IoEngine {
    #[serde(rename = "Sync")]
    Sync,
    #[serde(rename = "Async")]
    Async,
}

impl Default for IoEngine {
    fn default() -> IoEngine {
        Self::Sync
    }
}

