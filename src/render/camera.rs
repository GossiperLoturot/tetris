use crate::render::constants;
use wgpu::util::DeviceExt;

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RawCamera {
    pub view_proj: [[f32; 4]; 4],
}

#[derive(Debug)]
pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Vector3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub w_range: f32,
    pub h_range: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl Camera {
    pub fn build(&self) -> RawCamera {
        let view = cgmath::Matrix4::look_to_rh(self.eye, self.target, self.up);
        let proj = cgmath::ortho(
            -self.w_range * 0.5,
            self.w_range * 0.5,
            -self.h_range * 0.5,
            self.h_range * 0.5,
            self.z_near,
            self.z_far,
        );
        let view_proj = OPENGL_TO_WGPU_MATRIX * proj * view;

        RawCamera {
            view_proj: view_proj.into(),
        }
    }
}

pub struct Resource {
    camera: Camera,
    buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl Resource {
    pub fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
        let (clipping_width, clipping_height) = Self::get_contain_clipping(
            constants::WIDTH,
            constants::HEIGHT,
            width as _,
            height as _,
        );
        let camera = Camera {
            eye: cgmath::point3(0.0, 0.0, 10.0),
            target: -cgmath::Vector3::unit_z(),
            up: cgmath::Vector3::unit_y(),
            w_range: clipping_width,
            h_range: clipping_height,
            z_near: 0.001,
            z_far: 1000.0,
        };

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[camera.build()]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: None,
        });

        Self {
            camera,
            buffer,
            bind_group_layout,
            bind_group,
        }
    }

    pub fn resize(&mut self, queue: &wgpu::Queue, width: u32, height: u32) {
        let (clipping_width, clipping_height) = Self::get_contain_clipping(
            constants::WIDTH,
            constants::HEIGHT,
            width as _,
            height as _,
        );
        self.camera.w_range = clipping_width;
        self.camera.h_range = clipping_height;
        queue.write_buffer(
            &self.buffer,
            0,
            bytemuck::cast_slice(&[self.camera.build()]),
        );
    }

    fn get_contain_clipping(
        target_width: f32,
        target_height: f32,
        width: f32,
        height: f32,
    ) -> (f32, f32) {
        let aspect = width / height;
        (
            target_width.max(target_height * aspect),
            target_height.max(target_width / aspect),
        )
    }
}
