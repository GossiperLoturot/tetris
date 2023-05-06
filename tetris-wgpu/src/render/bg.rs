use crate::render::constants;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    const ATTRIBUTES: &[wgpu::VertexAttribute] =
        &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    pub fn layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: Self::ATTRIBUTES,
        }
    }
}

pub struct Pipeline {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    pipeline: wgpu::RenderPipeline,
}

impl Pipeline {
    #[rustfmt::skip]
    const VERTICES: &[Vertex] = &[
        Vertex { position: [-constants::WIDTH * 0.5, -constants::HEIGHT * 0.5, 0.0], color: constants::color::BG_DEFAULT },
        Vertex { position: [ constants::WIDTH * 0.5, -constants::HEIGHT * 0.5, 0.0], color: constants::color::BG_DEFAULT },
        Vertex { position: [ constants::WIDTH * 0.5,  constants::HEIGHT * 0.5, 0.0], color: constants::color::BG_DEFAULT },
        Vertex { position: [-constants::WIDTH * 0.5,  constants::HEIGHT * 0.5, 0.0], color: constants::color::BG_DEFAULT },
    ];

    const INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];

    pub fn new(
        device: &wgpu::Device,
        target_format: wgpu::TextureFormat,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(Self::VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(Self::INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = Self::INDICES.len() as u32;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("../../assets/shaders/bg.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[bind_group_layout],
                push_constant_ranges: &[],
            });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: target_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            vertex_buffer,
            index_buffer,
            num_indices,
            pipeline,
        }
    }

    pub fn render(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        target: &wgpu::TextureView,
        bind_group: &wgpu::BindGroup,
    ) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        let color = wgpu::Color {
            r: constants::color::BG_SUBTLE[0] as _,
            g: constants::color::BG_SUBTLE[1] as _,
            b: constants::color::BG_SUBTLE[2] as _,
            a: 1.0,
        };

        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(color),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
            label: None,
        });

        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        pass.set_bind_group(0, bind_group, &[]);
        pass.draw_indexed(0..self.num_indices, 0, 0..1);

        drop(pass);

        queue.submit([encoder.finish()]);
    }
}
