use buffers::GpuBuffers;
use bind_group::{BindGroupLayout, BindGroup};
use engine_config::RenderConfig;
use crate::{buffers, GpuDevice};
use crate::bind_group;


pub struct GpuWrapper {
    pub buffers: GpuBuffers,
    pub bind_group: BindGroup,
    pub bind_group_layout: BindGroupLayout,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl GpuWrapper {
    ///initializes shared Config, deligated to Sub modules
    pub fn new(rc:RenderConfig) -> Self {
        let gpu = GpuDevice::new()?;
        let buffers = GpuBuffers::new(rc,&gpu.device);
        let layout = BindGroupLayout::new(&gpu.device, &buffers);
        let groups = BindGroup::new(&gpu.device,&buffers,&layout.bind_group_layout);
        Self {
            buffers,
            bind_group_layout: layout,
            bind_group: groups,
            device: gpu.device,
            queue: gpu.queue
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue) {
        self.buffers.grow(self.buffers.camera)
    }


}