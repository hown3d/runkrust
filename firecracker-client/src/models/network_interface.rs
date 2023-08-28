/*
 * Firecracker API
 *
 * RESTful public-facing API. The API is accessible through HTTP calls on specific URLs carrying JSON modeled data. The transport medium is a Unix Domain Socket.
 *
 * The version of the OpenAPI document: 1.4.1
 * Contact: compute-capsule@amazon.com
 * Generated by: https://openapi-generator.tech
 */

/// NetworkInterface : Defines a network interface.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkInterface {
    #[serde(rename = "guest_mac", skip_serializing_if = "Option::is_none")]
    pub guest_mac: Option<String>,
    /// Host level path for the guest network interface
    #[serde(rename = "host_dev_name")]
    pub host_dev_name: String,
    #[serde(rename = "iface_id")]
    pub iface_id: String,
    #[serde(rename = "rx_rate_limiter", skip_serializing_if = "Option::is_none")]
    pub rx_rate_limiter: Option<Box<crate::models::RateLimiter>>,
    #[serde(rename = "tx_rate_limiter", skip_serializing_if = "Option::is_none")]
    pub tx_rate_limiter: Option<Box<crate::models::RateLimiter>>,
}

impl NetworkInterface {
    /// Defines a network interface.
    pub fn new(host_dev_name: String, iface_id: String) -> NetworkInterface {
        NetworkInterface {
            guest_mac: None,
            host_dev_name,
            iface_id,
            rx_rate_limiter: None,
            tx_rate_limiter: None,
        }
    }
}


