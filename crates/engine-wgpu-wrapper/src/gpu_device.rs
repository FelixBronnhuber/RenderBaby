use std::sync::OnceLock;
use anyhow::{Result, anyhow};

pub struct GpuDevice {
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
}

impl GpuDevice {
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

        let mut limits = wgpu::Limits::default();
        limits.max_storage_buffers_per_shader_stage = 16;

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
