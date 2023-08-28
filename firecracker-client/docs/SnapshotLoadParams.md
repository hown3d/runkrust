# SnapshotLoadParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enable_diff_snapshots** | Option<**bool**> | Enable support for incremental (diff) snapshots by tracking dirty guest pages. | [optional]
**mem_file_path** | Option<**String**> | Path to the file that contains the guest memory to be loaded. This parameter has been deprecated and is only allowed if `mem_backend` is not present. | [optional]
**mem_backend** | Option<[**crate::models::MemoryBackend**](MemoryBackend.md)> |  | [optional]
**snapshot_path** | **String** | Path to the file that contains the microVM state to be loaded. | 
**resume_vm** | Option<**bool**> | When set to true, the vm is also resumed if the snapshot load is successful. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


