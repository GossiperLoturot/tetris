fn main() {
    pollster::block_on(start());
}

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: winit::window::Window,
    render_pipeline: wgpu::RenderPipeline,
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

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
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
        render_pass.draw(0..3, 0..1);
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

    fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        if 0 < size.width && 0 < size.height {
            self.size = size;
            self.config.width = size.width;
            self.config.height = size.height;
            self.surface.configure(&self.device, &self.config);
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
            window_id: id,
            ref event,
        } if state.match_id(id) => match event {
            WindowEvent::CloseRequested => {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }
            WindowEvent::Resized(size) => {
                state.resize(*size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                state.resize(**new_inner_size)
            }
            _ => {}
        },
        Event::RedrawRequested(id) if state.match_id(id) => {
            state.render();
        }
        Event::RedrawEventsCleared => {
            state.request_redraw();
        }
        _ => {}
    });
}
