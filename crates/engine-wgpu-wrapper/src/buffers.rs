use std::collections::HashMap;
use std::process::Output;
use wgpu::{Buffer, Device};
use wgpu::util::DeviceExt;
use engine_config::RenderConfig;

pub struct GpuBuffers {
    pub(crate) spheres: Buffer,
    pub(crate) camera: Buffer,
    pub(crate) output: Buffer,
    pub(crate) staging: Buffer,
}

impl GpuBuffers {
    pub fn new(rc:RenderConfig, device: &Device) -> Self {
        let cam = rc.camera.clone();
                let size = (rc.camera.width * rc.camera.height * 4) as u64;
        
                let dimensions_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Dimensions Buffer"),
                    contents: bytemuck::bytes_of(&cam),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });
        
                let spheres_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Spheres Buffer"),
                    contents: bytemuck::cast_slice(&rc.spheres),
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                });
        
                let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("Output Buffer"),
                    size: size,
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
                    mapped_at_creation: false,
                });
        
                let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("Staging Buffer"),
                    size: size,
                    usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
        
        Self{
            spheres: spheres_buffer,
            camera: dimensions_buffer,
            output: output_buffer,
            staging: staging_buffer
        }
    }
    
    pub fn grow(&mut self, buffer: Buffer) {
        
    }
}