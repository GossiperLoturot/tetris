#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    pub fn layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    pub position: [f32; 3],
}

impl Instance {
    const ATTRIBUTES: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![2 => Float32x3];

    pub fn layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

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
