mod models;

use wgpu::util::DeviceExt;

fn main() {
    pollster::block_on(start());
}

const WIDTH: f32 = 10.0;
const HEIGHT: f32 = 20.0;

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: winit::window::Window,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    num_indices: u32,
    index_buffer: wgpu::Buffer,
    camera: models::Camera,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    instances: Vec<models::Instance>,
    instance_buffer: wgpu::Buffer,
}

impl State {
    async fn new_async(window: winit::window::Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            width: size.width,
            height: size.height,
            format: surface_capabilities.formats[0],
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        let vertices = [
            models::Vertex {
                position: [0.0, 0.0, 0.0],
                color: [0.0, 0.0, 0.0],
            },
            models::Vertex {
                position: [1.0, 0.0, 0.0],
                color: [1.0, 0.0, 0.0],
            },
            models::Vertex {
                position: [1.0, 1.0, 0.0],
                color: [1.0, 1.0, 0.0],
            },
            models::Vertex {
                position: [0.0, 1.0, 0.0],
                color: [0.0, 1.0, 0.0],
            },
        ];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let vertex_buffer_layout = models::Vertex::layout();

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];

        let num_indices = indices.len() as u32;

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let instances = vec![
            models::Instance {
                position: [0.0, 0.0, 0.0],
            },
            models::Instance {
                position: [1.0, 0.0, 0.0],
            },
            models::Instance {
                position: [1.0, 1.0, 0.0],
            },
            models::Instance {
                position: [0.0, 1.0, 0.0],
            },
        ];

        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&instances),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let instance_buffer_layout = models::Instance::layout();

        let clipping = models::Camera::get_contain_clipping((WIDTH, HEIGHT), size);
        let camera = models::Camera {
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

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&camera_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[vertex_buffer_layout, instance_buffer_layout],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
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
            surface,
            device,
            queue,
            config,
            size,
            window,
            render_pipeline,
            vertex_buffer,
            num_indices,
            index_buffer,
            camera,
            camera_buffer,
            camera_bind_group,
            instances,
            instance_buffer,
        }
    }

    fn render(&mut self) {
        let output = self.surface.get_current_texture().unwrap();

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
            label: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..self.instances.len() as _);
        drop(render_pass);

        self.queue.submit([encoder.finish()]);

        output.present();
    }

    fn match_id(&self, id: winit::window::WindowId) -> bool {
        self.window.id() == id
    }

    fn request_redraw(&self) {
        self.window.request_redraw();
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if 0 < new_size.width && 0 < new_size.height {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            let clipping = models::Camera::get_contain_clipping((WIDTH, HEIGHT), new_size);
            self.camera.w_range = clipping.0;
            self.camera.h_range = clipping.1;
            self.queue.write_buffer(
                &self.camera_buffer,
                0,
                bytemuck::cast_slice(&[self.camera.build()]),
            );
        }
    }
}

async fn start() {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .build(&event_loop)
        .unwrap();

    let mut state = State::new_async(window).await;

    use winit::event::Event;
    use winit::event::WindowEvent;
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            window_id,
            ref event,
        } if state.match_id(window_id) => match event {
            WindowEvent::CloseRequested => {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }
            WindowEvent::Resized(new_size) => {
                state.resize(*new_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                state.resize(**new_inner_size)
            }
            _ => {}
        },
        Event::RedrawRequested(window_id) if state.match_id(window_id) => {
            state.render();
        }
        Event::RedrawEventsCleared => {
            state.request_redraw();
        }
        _ => {}
    });
}
