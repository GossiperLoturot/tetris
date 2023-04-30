use crate::game;

mod bg;
mod block;
mod camera;
mod constants;

pub struct RenderSystem {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: winit::window::Window,
    staging_belt: wgpu::util::StagingBelt,

    // text
    glyph_blush: wgpu_glyph::GlyphBrush<(), wgpu_glyph::ab_glyph::FontArc>,

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

        let staging_belt = wgpu::util::StagingBelt::new(1024);

        let font = wgpu_glyph::ab_glyph::FontArc::try_from_slice(include_bytes!(
            "../../assets/fonts/Roboto-Regular.ttf"
        ))
        .unwrap();

        let glyph_blush = wgpu_glyph::GlyphBrushBuilder::using_font(font)
            .build(&device, surface_capabilities.formats[0]);

        let camera = camera::Renderer::new(&device, size);
        let bg = bg::Renderer::new(&device, &config, &camera.bind_group_layout);
        let block = block::Renderer::new(&device, &config, &camera.bind_group_layout);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
            staging_belt,
            glyph_blush,
            camera,
            bg,
            block,
        }
    }

    pub fn render(&mut self, cx: game::GameContext) {
        let output = self.surface.get_current_texture().unwrap();

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        self.render_common(&view, &mut encoder);

        match cx {
            game::GameContext::Start(cx) => self.render_cx_start(&view, &mut encoder, cx),
            game::GameContext::Playing(cx) => self.render_cx_playing(&view, &mut encoder, cx),
            game::GameContext::End(cx) => self.render_cx_end(&view, &mut encoder, cx),
        }

        self.staging_belt.finish();
        self.queue.submit([encoder.finish()]);

        output.present();
        self.staging_belt.recall();
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

    fn render_common(&mut self, view: &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder) {
        let color = wgpu::Color {
            r: constants::color::BG_SUBTLE[0] as _,
            g: constants::color::BG_SUBTLE[1] as _,
            b: constants::color::BG_SUBTLE[2] as _,
            a: 1.0,
        };

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(color),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
            label: None,
        });

        self.bg.render(&mut render_pass, &self.camera.bind_group);
    }

    fn render_cx_start(
        &mut self,
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        _cx: game::start::GameContext,
    ) {
        self.glyph_blush.queue(
            wgpu_glyph::Section::default()
                .add_text(
                    wgpu_glyph::Text::new("TETRIS")
                        .with_scale(constants::TEXT_SCALE * 4.0)
                        .with_color(constants::color::TEXT),
                )
                .with_screen_position((
                    self.size.width as f32 * 0.5,
                    self.size.height as f32 * 0.5 - constants::TEXT_SCALE * 4.0,
                ))
                .with_layout(
                    wgpu_glyph::Layout::default()
                        .h_align(wgpu_glyph::HorizontalAlign::Center)
                        .v_align(wgpu_glyph::VerticalAlign::Center),
                ),
        );

        self.glyph_blush.queue(
            wgpu_glyph::Section::default()
                .add_text(
                    wgpu_glyph::Text::new("PRESS RETURN TO PLAY")
                        .with_scale(constants::TEXT_SCALE)
                        .with_color(constants::color::TEXT),
                )
                .with_screen_position((
                    self.size.width as f32 * 0.5,
                    self.size.height as f32 * 0.5 + constants::TEXT_SCALE,
                ))
                .with_layout(
                    wgpu_glyph::Layout::default()
                        .h_align(wgpu_glyph::HorizontalAlign::Center)
                        .v_align(wgpu_glyph::VerticalAlign::Center),
                ),
        );

        self.glyph_blush
            .draw_queued(
                &self.device,
                &mut self.staging_belt,
                encoder,
                view,
                self.size.width,
                self.size.height,
            )
            .unwrap();
    }

    fn render_cx_playing(
        &mut self,
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        cx: game::playing::GameContext,
    ) {
        fn color_to_data(value: &game::playing::BlockColor) -> [f32; 3] {
            match value {
                game::playing::BlockColor::Cyan => constants::color::FG_CYAN,
                game::playing::BlockColor::Yellow => constants::color::FG_YELLOW,
                game::playing::BlockColor::Green => constants::color::FG_GREEN,
                game::playing::BlockColor::Red => constants::color::FG_RED,
                game::playing::BlockColor::Blue => constants::color::FG_BLUE,
                game::playing::BlockColor::Orange => constants::color::FG_ORANGE,
                game::playing::BlockColor::Purple => constants::color::FG_PURPLE,
            }
        }

        let mut instances = vec![];

        for (row, items) in cx.blocks.iter().enumerate() {
            for (col, item) in items.iter().enumerate() {
                if let Some(block_color) = item.as_ref() {
                    let position = [
                        col as f32 - constants::WIDTH * 0.5,
                        row as f32 - constants::HEIGHT * 0.5,
                        0.0,
                    ];
                    let color = color_to_data(block_color);
                    instances.push(block::Instance { position, color });
                }
            }
        }

        if let Some(block_set) = cx.block_set.as_ref() {
            for (col, row) in block_set.content.iter() {
                let position = [
                    block_set.x as f32 + *col as f32 - constants::WIDTH * 0.5,
                    block_set.y as f32 + *row as f32 - constants::HEIGHT * 0.5,
                    0.0,
                ];
                let color = color_to_data(&block_set.color);
                instances.push(block::Instance { position, color })
            }
        }

        self.block
            .set_instances(&self.device, encoder, &mut self.staging_belt, &instances);

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
            label: None,
        });

        self.block.render(&mut render_pass, &self.camera.bind_group);

        drop(render_pass);

        self.glyph_blush.queue(
            wgpu_glyph::Section::default()
                .add_text(
                    wgpu_glyph::Text::new(&format!("SCORE: {}", cx.score))
                        .with_scale(constants::TEXT_SCALE)
                        .with_color(constants::color::TEXT_PLAYING),
                )
                .with_screen_position((self.size.width as f32 * 0.5, constants::TEXT_SCALE * 0.5))
                .with_layout(
                    wgpu_glyph::Layout::default().h_align(wgpu_glyph::HorizontalAlign::Center),
                ),
        );

        self.glyph_blush
            .draw_queued(
                &self.device,
                &mut self.staging_belt,
                encoder,
                view,
                self.size.width,
                self.size.height,
            )
            .unwrap();
    }

    fn render_cx_end(
        &mut self,
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        cx: game::end::GameContext,
    ) {
        self.glyph_blush.queue(
            wgpu_glyph::Section::default()
                .add_text(
                    wgpu_glyph::Text::new("GAME OVER")
                        .with_scale(constants::TEXT_SCALE * 2.0)
                        .with_color(constants::color::TEXT),
                )
                .with_screen_position((
                    self.size.width as f32 * 0.5,
                    self.size.height as f32 * 0.5 - constants::TEXT_SCALE * 2.0,
                ))
                .with_layout(
                    wgpu_glyph::Layout::default()
                        .h_align(wgpu_glyph::HorizontalAlign::Center)
                        .v_align(wgpu_glyph::VerticalAlign::Center),
                ),
        );

        self.glyph_blush.queue(
            wgpu_glyph::Section::default()
                .add_text(
                    wgpu_glyph::Text::new(&format!("SCORE: {}", cx.score))
                        .with_scale(constants::TEXT_SCALE)
                        .with_color(constants::color::TEXT),
                )
                .with_screen_position((
                    self.size.width as f32 * 0.5,
                    self.size.height as f32 * 0.5 + constants::TEXT_SCALE,
                ))
                .with_layout(
                    wgpu_glyph::Layout::default()
                        .h_align(wgpu_glyph::HorizontalAlign::Center)
                        .v_align(wgpu_glyph::VerticalAlign::Center),
                ),
        );

        self.glyph_blush
            .draw_queued(
                &self.device,
                &mut self.staging_belt,
                encoder,
                view,
                self.size.width,
                self.size.height,
            )
            .unwrap();
    }
}
