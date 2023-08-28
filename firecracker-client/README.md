# Rust API client for firecracker-client

RESTful public-facing API. The API is accessible through HTTP calls on specific URLs carrying JSON modeled data. The transport medium is a Unix Domain Socket.


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.4.1
- Package version: 1.4.1
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `firecracker-client` and add the following to `Cargo.toml` under `[dependencies]`:

```
firecracker-client = { path = "./firecracker-client" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*DefaultApi* | [**create_snapshot**](docs/DefaultApi.md#create_snapshot) | **PUT** /snapshot/create | Creates a full or diff snapshot. Post-boot only.
*DefaultApi* | [**create_sync_action**](docs/DefaultApi.md#create_sync_action) | **PUT** /actions | Creates a synchronous action.
*DefaultApi* | [**describe_balloon_config**](docs/DefaultApi.md#describe_balloon_config) | **GET** /balloon | Returns the current balloon device configuration.
*DefaultApi* | [**describe_balloon_stats**](docs/DefaultApi.md#describe_balloon_stats) | **GET** /balloon/statistics | Returns the latest balloon device statistics, only if enabled pre-boot.
*DefaultApi* | [**describe_instance**](docs/DefaultApi.md#describe_instance) | **GET** / | Returns general information about an instance.
*DefaultApi* | [**get_export_vm_config**](docs/DefaultApi.md#get_export_vm_config) | **GET** /vm/config | Gets the full VM configuration.
*DefaultApi* | [**get_firecracker_version**](docs/DefaultApi.md#get_firecracker_version) | **GET** /version | Gets the Firecracker version.
*DefaultApi* | [**get_machine_configuration**](docs/DefaultApi.md#get_machine_configuration) | **GET** /machine-config | Gets the machine configuration of the VM.
*DefaultApi* | [**get_mmds**](docs/DefaultApi.md#get_mmds) | **GET** /mmds | Get the MMDS data store.
*DefaultApi* | [**load_snapshot**](docs/DefaultApi.md#load_snapshot) | **PUT** /snapshot/load | Loads a snapshot. Pre-boot only.
*DefaultApi* | [**patch_balloon**](docs/DefaultApi.md#patch_balloon) | **PATCH** /balloon | Updates a balloon device.
*DefaultApi* | [**patch_balloon_stats_interval**](docs/DefaultApi.md#patch_balloon_stats_interval) | **PATCH** /balloon/statistics | Updates a balloon device statistics polling interval.
*DefaultApi* | [**patch_guest_drive_by_id**](docs/DefaultApi.md#patch_guest_drive_by_id) | **PATCH** /drives/{drive_id} | Updates the properties of a drive. Post-boot only.
*DefaultApi* | [**patch_guest_network_interface_by_id**](docs/DefaultApi.md#patch_guest_network_interface_by_id) | **PATCH** /network-interfaces/{iface_id} | Updates the rate limiters applied to a network interface. Post-boot only.
*DefaultApi* | [**patch_machine_configuration**](docs/DefaultApi.md#patch_machine_configuration) | **PATCH** /machine-config | Partially updates the Machine Configuration of the VM. Pre-boot only.
*DefaultApi* | [**patch_mmds**](docs/DefaultApi.md#patch_mmds) | **PATCH** /mmds | Updates the MMDS data store.
*DefaultApi* | [**patch_vm**](docs/DefaultApi.md#patch_vm) | **PATCH** /vm | Updates the microVM state.
*DefaultApi* | [**put_balloon**](docs/DefaultApi.md#put_balloon) | **PUT** /balloon | Creates or updates a balloon device.
*DefaultApi* | [**put_cpu_configuration**](docs/DefaultApi.md#put_cpu_configuration) | **PUT** /cpu-config | Configures CPU features flags for the vCPUs of the guest VM. Pre-boot only.
*DefaultApi* | [**put_entropy_device**](docs/DefaultApi.md#put_entropy_device) | **PUT** /entropy | Creates an entropy device. Pre-boot only.
*DefaultApi* | [**put_guest_boot_source**](docs/DefaultApi.md#put_guest_boot_source) | **PUT** /boot-source | Creates or updates the boot source. Pre-boot only.
*DefaultApi* | [**put_guest_drive_by_id**](docs/DefaultApi.md#put_guest_drive_by_id) | **PUT** /drives/{drive_id} | Creates or updates a drive. Pre-boot only.
*DefaultApi* | [**put_guest_network_interface_by_id**](docs/DefaultApi.md#put_guest_network_interface_by_id) | **PUT** /network-interfaces/{iface_id} | Creates a network interface. Pre-boot only.
*DefaultApi* | [**put_guest_vsock**](docs/DefaultApi.md#put_guest_vsock) | **PUT** /vsock | Creates/updates a vsock device. Pre-boot only.
*DefaultApi* | [**put_logger**](docs/DefaultApi.md#put_logger) | **PUT** /logger | Initializes the logger by specifying a named pipe or a file for the logs output.
*DefaultApi* | [**put_machine_configuration**](docs/DefaultApi.md#put_machine_configuration) | **PUT** /machine-config | Updates the Machine Configuration of the VM. Pre-boot only.
*DefaultApi* | [**put_metrics**](docs/DefaultApi.md#put_metrics) | **PUT** /metrics | Initializes the metrics system by specifying a named pipe or a file for the metrics output.
*DefaultApi* | [**put_mmds**](docs/DefaultApi.md#put_mmds) | **PUT** /mmds | Creates a MMDS (Microvm Metadata Service) data store.
*DefaultApi* | [**put_mmds_config**](docs/DefaultApi.md#put_mmds_config) | **PUT** /mmds/config | Set MMDS configuration. Pre-boot only.


## Documentation For Models

 - [Balloon](docs/Balloon.md)
 - [BalloonStats](docs/BalloonStats.md)
 - [BalloonStatsUpdate](docs/BalloonStatsUpdate.md)
 - [BalloonUpdate](docs/BalloonUpdate.md)
 - [BootSource](docs/BootSource.md)
 - [CpuTemplate](docs/CpuTemplate.md)
 - [Drive](docs/Drive.md)
 - [EntropyDevice](docs/EntropyDevice.md)
 - [Error](docs/Error.md)
 - [FirecrackerVersion](docs/FirecrackerVersion.md)
 - [FullVmConfiguration](docs/FullVmConfiguration.md)
 - [InstanceActionInfo](docs/InstanceActionInfo.md)
 - [InstanceInfo](docs/InstanceInfo.md)
 - [Logger](docs/Logger.md)
 - [MachineConfiguration](docs/MachineConfiguration.md)
 - [MemoryBackend](docs/MemoryBackend.md)
 - [Metrics](docs/Metrics.md)
 - [MmdsConfig](docs/MmdsConfig.md)
 - [NetworkInterface](docs/NetworkInterface.md)
 - [PartialDrive](docs/PartialDrive.md)
 - [PartialNetworkInterface](docs/PartialNetworkInterface.md)
 - [RateLimiter](docs/RateLimiter.md)
 - [SnapshotCreateParams](docs/SnapshotCreateParams.md)
 - [SnapshotLoadParams](docs/SnapshotLoadParams.md)
 - [TokenBucket](docs/TokenBucket.md)
 - [Vm](docs/Vm.md)
 - [Vsock](docs/Vsock.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

compute-capsule@amazon.com
