/*
 * Firecracker API
 *
 * RESTful public-facing API. The API is accessible through HTTP calls on specific URLs carrying JSON modeled data. The transport medium is a Unix Domain Socket.
 *
 * The version of the OpenAPI document: 1.4.1
 * Contact: compute-capsule@amazon.com
 * Generated by: https://openapi-generator.tech
 */

/// PartialNetworkInterface : Defines a partial network interface structure, used to update the rate limiters for that interface, after microvm start.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartialNetworkInterface {
    #[serde(rename = "iface_id")]
    pub iface_id: String,
    #[serde(rename = "rx_rate_limiter", skip_serializing_if = "Option::is_none")]
    pub rx_rate_limiter: Option<Box<crate::models::RateLimiter>>,
    #[serde(rename = "tx_rate_limiter", skip_serializing_if = "Option::is_none")]
    pub tx_rate_limiter: Option<Box<crate::models::RateLimiter>>,
}

impl PartialNetworkInterface {
    /// Defines a partial network interface structure, used to update the rate limiters for that interface, after microvm start.
    pub fn new(iface_id: String) -> PartialNetworkInterface {
        PartialNetworkInterface {
            iface_id,
            rx_rate_limiter: None,
            tx_rate_limiter: None,
        }
    }
}


