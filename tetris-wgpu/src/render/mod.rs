use crate::game;

mod bg;
mod block;
mod camera;
mod constants;

fn block_color_into_data(value: &game::BlockColor) -> [f32; 3] {
    match value {
        game::BlockColor::Cyan => constants::color::FG_CYAN,
        game::BlockColor::Yellow => constants::color::FG_YELLOW,
        game::BlockColor::Green => constants::color::FG_GREEN,
        game::BlockColor::Red => constants::color::FG_RED,
        game::BlockColor::Blue => constants::color::FG_BLUE,
        game::BlockColor::Orange => constants::color::FG_ORANGE,
        game::BlockColor::Purple => constants::color::FG_PURPLE,
    }
}

pub struct RenderSystem {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: winit::window::Window,

    // renderer
    camera: camera::Renderer,
    bg: bg::Renderer,
    block: block::Renderer,
}

impl RenderSystem {
    pub async fn new_async(window: winit::window::Window) -> Self {
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

        let camera = camera::Renderer::new(&device, size);
        let bg = bg::Renderer::new(&device, &config, &camera.camera_bind_group_layout);
        let block = block::Renderer::new(&device, &config, &camera.camera_bind_group_layout);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
            camera,
            bg,
            block,
        }
    }

    pub fn render(&mut self, cx: game::GameContext) {
        let mut instances = vec![];
        for (row, items) in cx.blocks.iter().enumerate() {
            for (col, item) in items.iter().enumerate() {
                if let Some(block_color) = item.as_ref() {
                    let position = [
                        col as f32 - constants::WIDTH * 0.5,
                        row as f32 - constants::HEIGHT * 0.5,
                        0.0,
                    ];
                    let color = block_color_into_data(block_color);
                    instances.push(block::Instance { position, color });
                }
            }
        }
        if let Some(block_set) = cx.block_set.as_ref() {
            for (col, row, block_color) in block_set.content.iter() {
                let position = [
                    block_set.x as f32 + *col as f32 - constants::WIDTH * 0.5,
                    block_set.y as f32 + *row as f32 - constants::HEIGHT * 0.5,
                    0.0,
                ];
                let color = block_color_into_data(block_color);
                instances.push(block::Instance { position, color })
            }
        }
        self.block.set_instances(&self.queue, &instances);

        let output = self.surface.get_current_texture().unwrap();

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        let color = wgpu::Color {
            r: constants::color::BG_SUBTLE[0] as _,
            g: constants::color::BG_SUBTLE[1] as _,
            b: constants::color::BG_SUBTLE[2] as _,
            a: 1.0,
        };

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(color),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
            label: None,
        });

        self.bg
            .render(&mut render_pass, &self.camera.camera_bind_group);
        self.block
            .render(&mut render_pass, &self.camera.camera_bind_group);
        drop(render_pass);

        self.queue.submit([encoder.finish()]);

        output.present();
    }

    pub fn match_id(&self, id: winit::window::WindowId) -> bool {
        self.window.id() == id
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if 0 < new_size.width && 0 < new_size.height {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;

            self.surface.configure(&self.device, &self.config);

            self.camera.resize(&self.queue, new_size);
        }
    }
}
