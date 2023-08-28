# Drive

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**drive_id** | **String** |  | 
**cache_type** | Option<**String**> | Represents the caching strategy for the block device. | [optional][default to Unsafe]
**is_read_only** | **bool** |  | 
**is_root_device** | **bool** |  | 
**partuuid** | Option<**String**> | Represents the unique id of the boot partition of this device. It is optional and it will be taken into account only if the is_root_device field is true. | [optional]
**path_on_host** | **String** | Host level path for the guest drive | 
**rate_limiter** | Option<[**crate::models::RateLimiter**](RateLimiter.md)> |  | [optional]
**io_engine** | Option<**String**> | Type of the IO engine used by the device. \"Async\" is supported on host kernels newer than 5.10.51. | [optional][default to Sync]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


