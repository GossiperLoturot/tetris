use super::constants;
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
    pub fn get_contain_clipping(
        target_clipping: (f32, f32),
        window_size: winit::dpi::PhysicalSize<u32>,
    ) -> (f32, f32) {
        let aspect = window_size.width as f32 / window_size.height as f32;
        (
            target_clipping.0.max(target_clipping.1 * aspect),
            target_clipping.1.max(target_clipping.0 / aspect),
        )
    }

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

pub struct Renderer {
    camera: Camera,
    camera_buffer: wgpu::Buffer,
    pub camera_bind_group_layout: wgpu::BindGroupLayout,
    pub camera_bind_group: wgpu::BindGroup,
}

impl Renderer {
    pub fn new(device: &wgpu::Device, size: winit::dpi::PhysicalSize<u32>) -> Self {
        let clipping = Camera::get_contain_clipping((constants::WIDTH, constants::HEIGHT), size);
        let camera = Camera {
            eye: cgmath::point3(0.0, 0.0, 10.0),
            target: -cgmath::Vector3::unit_z(),
            up: cgmath::Vector3::unit_y(),
            w_range: clipping.0,
            h_range: clipping.1,
            z_near: 0.001,
            z_far: 1000.0,
        };

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[camera.build()]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: None,
        });

        Self {
            camera,
            camera_buffer,
            camera_bind_group_layout,
            camera_bind_group,
        }
    }

    pub fn resize(&mut self, queue: &wgpu::Queue, new_size: winit::dpi::PhysicalSize<u32>) {
        let clipping =
            Camera::get_contain_clipping((constants::WIDTH, constants::HEIGHT), new_size);
        self.camera.w_range = clipping.0;
        self.camera.h_range = clipping.1;
        queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera.build()]),
        );
    }
}
