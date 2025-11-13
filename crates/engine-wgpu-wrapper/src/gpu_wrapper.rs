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
    pub rc: RenderConfig,
}

impl GpuWrapper {
    ///initializes shared Config, deligated to Sub modules
    pub fn new(rc:RenderConfig) -> Self {
        let gpu = GpuDevice::new().unwrap();
        let buffers = GpuBuffers::new(&rc,&gpu.device);
        let layout = BindGroupLayout::new(&gpu.device, &buffers);
        let groups = BindGroup::new(&gpu.device,&buffers,&layout.bind_group_layout);
        Self {
            buffers,
            bind_group_layout: layout,
            bind_group: groups,
            device: gpu.device,
            queue: gpu.queue,
            rc
        }
    }

    pub fn update(&mut self, rc:RenderConfig) {
        let new_size = rc.camera.height*rc.camera.width;
        if self.get_size() !=  new_size {
            self.buffers.grow_resolution(&self.device, (new_size * 4) as u64);
            self.bind_group = BindGroup::new(&self.device, &self.buffers, self.get_bind_group_layout())
        }
        self.rc = rc;
    }
    pub fn get_size(&self) -> u32 {
        self.rc.camera.height*self.rc.camera.width
    }

    pub fn get_width(&self) -> u32 {
        self.rc.camera.width
    }

    pub fn get_height(&self) -> u32 {
        self.rc.camera.height
    }

    pub fn get_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout.bind_group_layout
    }

    pub fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group.bind_group
    }


}