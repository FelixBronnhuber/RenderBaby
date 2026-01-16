use std::sync::OnceLock;
use anyhow::{Result, anyhow};

/// Represents a handle to the GPU device and command queue.
///
/// This struct implements a singleton pattern using `OnceLock` to ensure that only one
/// `wgpu::Device` and `wgpu::Queue` are created for the application, even if multiple
/// engines or wrappers are instantiated.
pub struct GpuDevice {
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
}

impl GpuDevice {
    /// Acquires the GPU device and queue.
    ///
    /// If the device has not been initialized yet, it requests a high-performance adapter
    /// and creates a logical device with limits suitable for the rendering tasks
    /// (e.g., increased storage buffer limits).
    ///
    /// Subsequent calls return a clone of the existing device and queue handle.
    ///
    /// # Returns
    ///
    /// * `Ok(GpuDevice)` - A new instance containing the shared device and queue.
    /// * `Err` - If no suitable adapter is found or device creation fails.
    pub fn new() -> Result<Self> {
        static DEVICE_ONCE: OnceLock<(wgpu::Device, wgpu::Queue)> = OnceLock::new();

        if let Some((device, queue)) = DEVICE_ONCE.get() {
            return Ok(Self {
                device: device.clone(),
                queue: queue.clone(),
            });
        }

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let adapter_result =
            pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            }));

        let adapter = match adapter_result {
            Ok(adapter) => adapter,
            Err(_) => return Err(anyhow!("WGPU: no suitable GPU adapter found")),
        };

        let limits = wgpu::Limits {
            max_storage_buffers_per_shader_stage: 16,
            ..Default::default()
        };

        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("Render device"),
            required_features: wgpu::Features::empty(),
            required_limits: limits,
            ..Default::default()
        }))
        .map_err(|e| anyhow!("WGPU: failed to create device/queue: {}", e))?;

        let _ = DEVICE_ONCE.set((device.clone(), queue.clone()));

        Ok(Self { device, queue })
    }
}
