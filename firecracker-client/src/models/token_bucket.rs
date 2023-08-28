/*
 * Firecracker API
 *
 * RESTful public-facing API. The API is accessible through HTTP calls on specific URLs carrying JSON modeled data. The transport medium is a Unix Domain Socket.
 *
 * The version of the OpenAPI document: 1.4.1
 * Contact: compute-capsule@amazon.com
 * Generated by: https://openapi-generator.tech
 */

/// TokenBucket : Defines a token bucket with a maximum capacity (size), an initial burst size (one_time_burst) and an interval for refilling purposes (refill_time). The refill-rate is derived from size and refill_time, and it is the constant rate at which the tokens replenish. The refill process only starts happening after the initial burst budget is consumed. Consumption from the token bucket is unbounded in speed which allows for bursts bound in size by the amount of tokens available. Once the token bucket is empty, consumption speed is bound by the refill_rate.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokenBucket {
    /// The initial size of a token bucket.
    #[serde(rename = "one_time_burst", skip_serializing_if = "Option::is_none")]
    pub one_time_burst: Option<i64>,
    /// The amount of milliseconds it takes for the bucket to refill.
    #[serde(rename = "refill_time")]
    pub refill_time: i64,
    /// The total number of tokens this bucket can hold.
    #[serde(rename = "size")]
    pub size: i64,
}

impl TokenBucket {
    /// Defines a token bucket with a maximum capacity (size), an initial burst size (one_time_burst) and an interval for refilling purposes (refill_time). The refill-rate is derived from size and refill_time, and it is the constant rate at which the tokens replenish. The refill process only starts happening after the initial burst budget is consumed. Consumption from the token bucket is unbounded in speed which allows for bursts bound in size by the amount of tokens available. Once the token bucket is empty, consumption speed is bound by the refill_rate.
    pub fn new(refill_time: i64, size: i64) -> TokenBucket {
        TokenBucket {
            one_time_burst: None,
            refill_time,
            size,
        }
    }
}


