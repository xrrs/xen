# Xen

[![Join the chat at https://gitter.im/xr-rs/xen](https://badges.gitter.im/xr-rs/xen.svg)](https://gitter.im/xr-rs/xen?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)


[WIP] Xen is a multi-threaded, cross-platform, xR (VR/AR/MR) Engine for Rust.

## Components

- `xen-core`: Core libraries and Components of `xen`
- `xen-pbr`: Multi-threaded Stereoscopic VR Renderer built with `GFX-HAL` and `Vulkan`
- `xen-loop`: Message-bus based game loop for `xen`
- `xen-input`: Input code for `xen`, event based with support for xR controllers, HMDS, Keyboards, and Mice.
- `spatial`: Spatial and 3D audio for `xen`
- `openxr`: Rust bindings to the OpenXR SDK (Coming Soon!)
