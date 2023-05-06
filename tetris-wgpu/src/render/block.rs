use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
}

impl Vertex {
    const ATTRIBUTES: &[wgpu::VertexAttribute] = &wgpu::vertex_attr_array![0 => Float32x3];

    pub fn layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: Self::ATTRIBUTES,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Instance {
    const ATTRIBUTES: &[wgpu::VertexAttribute] =
        &wgpu::vertex_attr_array![1 => Float32x3, 2 => Float32x3];

    pub fn layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: Self::ATTRIBUTES,
        }
    }
}

pub struct Pipeline {
    vertex_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    num_instances: u32,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    pipeline: wgpu::RenderPipeline,
}

impl Pipeline {
    #[rustfmt::skip]
    const VERTICES: &[Vertex] = &[
        Vertex { position: [0.0, 0.0, 0.0] },
        Vertex { position: [1.0, 0.0, 0.0] },
        Vertex { position: [1.0, 1.0, 0.0] },
        Vertex { position: [0.0, 1.0, 0.0] },
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

        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: device.limits().max_buffer_size,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let num_instances = 0;

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(Self::INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = Self::INDICES.len() as u32;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(
                include_str!("../../assets/shaders/block.wgsl").into(),
            ),
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
                buffers: &[Vertex::layout(), Instance::layout()],
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
            instance_buffer,
            num_instances,
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

        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
            label: None,
        });

        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        pass.set_bind_group(0, bind_group, &[]);
        pass.draw_indexed(0..self.num_indices, 0, 0..self.num_instances);

        drop(pass);

        queue.submit([encoder.finish()]);
    }

    pub fn set_instances(&mut self, queue: &wgpu::Queue, instances: &[Instance]) {
        self.num_instances = instances.len() as _;
        let bytes = bytemuck::cast_slice(instances);
        queue.write_buffer(&self.instance_buffer, 0, bytes);
    }
}
