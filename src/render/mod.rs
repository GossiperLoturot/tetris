use std::sync::Arc;

use crate::{consts, game};

mod bg;
mod block;
mod camera;
mod text;

pub struct RenderSystem {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    window: Arc<winit::window::Window>,
    camera_resource: camera::Resource,
    bg_pipeline: bg::Pipeline,
    block_pipeline: block::Pipeline,
    text_pipeline: text::Pipeline,
}

impl RenderSystem {
    pub async fn new_async(window: Arc<winit::window::Window>) -> Self {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .unwrap();
        let inner_size = window.inner_size();
        let config = surface
            .get_default_config(&adapter, inner_size.width, inner_size.height)
            .unwrap();
        surface.configure(&device, &config);

        let camera_resource = camera::Resource::new(&device, config.width, config.height);
        let bg_pipeline =
            bg::Pipeline::new(&device, config.format, &camera_resource.bind_group_layout);
        let block_pipeline =
            block::Pipeline::new(&device, config.format, &camera_resource.bind_group_layout);
        let text_pipeline =
            text::Pipeline::new(&device, config.format, config.width, config.height);

        Self {
            surface,
            device,
            queue,
            config,
            window,
            camera_resource,
            bg_pipeline,
            block_pipeline,
            text_pipeline,
        }
    }

    pub fn render(&mut self, cx: game::GameContext) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.bg_pipeline.render(
            &self.device,
            &self.queue,
            &view,
            &self.camera_resource.bind_group,
        );

        match cx {
            game::GameContext::Start(_) => {
                self.text_pipeline.render(
                    &self.device,
                    &self.queue,
                    &view,
                    &[
                        wgpu_glyph::Section::default()
                            .add_text(
                                wgpu_glyph::Text::new("TETRIS")
                                    .with_scale(consts::TEXT_SCALE * 4.0)
                                    .with_color(consts::text_color::TEXT_PRIMARY),
                            )
                            .with_screen_position((
                                self.config.width as f32 * 0.5,
                                self.config.height as f32 * 0.5 - consts::TEXT_SCALE * 4.0,
                            ))
                            .with_layout(
                                wgpu_glyph::Layout::default()
                                    .h_align(wgpu_glyph::HorizontalAlign::Center)
                                    .v_align(wgpu_glyph::VerticalAlign::Center),
                            ),
                        wgpu_glyph::Section::default()
                            .add_text(
                                wgpu_glyph::Text::new("PRESS RETURN TO PLAY")
                                    .with_scale(consts::TEXT_SCALE)
                                    .with_color(consts::text_color::TEXT_PRIMARY),
                            )
                            .with_screen_position((
                                self.config.width as f32 * 0.5,
                                self.config.height as f32 * 0.5 + consts::TEXT_SCALE,
                            ))
                            .with_layout(
                                wgpu_glyph::Layout::default()
                                    .h_align(wgpu_glyph::HorizontalAlign::Center)
                                    .v_align(wgpu_glyph::VerticalAlign::Center),
                            ),
                        wgpu_glyph::Section::default()
                            .add_text(
                                wgpu_glyph::Text::new(
                                    "ARROWS: MOVE    Z/X: ROTATE\nSPACE: HARD DROP    P: PAUSE",
                                )
                                .with_scale(consts::TEXT_SCALE * 0.75)
                                .with_color(consts::text_color::TEXT_PRIMARY),
                            )
                            .with_screen_position((
                                self.config.width as f32 * 0.5,
                                self.config.height as f32 * 0.5 + consts::TEXT_SCALE * 2.5,
                            ))
                            .with_layout(
                                wgpu_glyph::Layout::default()
                                    .h_align(wgpu_glyph::HorizontalAlign::Center)
                                    .v_align(wgpu_glyph::VerticalAlign::Center),
                            ),
                    ],
                );
            }
            game::GameContext::Playing(cx) => {
                let mut instances = vec![];

                for col in 0..consts::VIEW_WIDTH as usize {
                    let position = [
                        col as f32 - consts::VIEW_WIDTH * 0.5,
                        consts::MAX_STACK_HEIGHT as f32 - consts::VIEW_HEIGHT * 0.5,
                        0.0,
                    ];
                    let color = consts::block_color::BG_MAX_STACK;
                    instances.push(block::Instance { position, color });
                }

                for (row, items) in cx.blocks.iter().enumerate() {
                    for (col, item) in items.iter().enumerate() {
                        if let Some(block_color) = item.as_ref() {
                            let position = [
                                col as f32 - consts::VIEW_WIDTH * 0.5,
                                row as f32 - consts::VIEW_HEIGHT * 0.5,
                                0.0,
                            ];
                            let color = consts::to_rgb(block_color);
                            instances.push(block::Instance { position, color });
                        }
                    }
                }

                if let Some(active_mino) = cx.active_mino.as_ref() {
                    for (col, row) in active_mino.blocks.iter() {
                        let position = [
                            active_mino.x as f32 + *col as f32 - consts::VIEW_WIDTH * 0.5,
                            active_mino.y as f32 + *row as f32 - consts::VIEW_HEIGHT * 0.5,
                            0.0,
                        ];
                        let color = consts::to_rgb(&active_mino.template.color);
                        instances.push(block::Instance { position, color })
                    }
                }

                self.block_pipeline.set_instances(&self.queue, &instances);
                self.block_pipeline.render(
                    &self.device,
                    &self.queue,
                    &view,
                    &self.camera_resource.bind_group,
                );

                self.text_pipeline.render(
                    &self.device,
                    &self.queue,
                    &view,
                    &[
                        wgpu_glyph::Section::default()
                            .add_text(
                                wgpu_glyph::Text::new(&format!("SCORE: {}", cx.score))
                                    .with_scale(consts::TEXT_SCALE)
                                    .with_color(consts::text_color::TEXT_SECONDARY),
                            )
                            .with_screen_position((
                                self.config.width as f32 * 0.5,
                                consts::TEXT_SCALE * 0.5,
                            ))
                            .with_layout(
                                wgpu_glyph::Layout::default()
                                    .h_align(wgpu_glyph::HorizontalAlign::Center),
                            ),
                        wgpu_glyph::Section::default()
                            .add_text(
                                wgpu_glyph::Text::new(
                                    "ARROWS: MOVE    Z/X: ROTATE\nSPACE: HARD DROP    P: PAUSE",
                                )
                                .with_scale(consts::TEXT_SCALE * 0.75)
                                .with_color(consts::text_color::TEXT_SECONDARY),
                            )
                            .with_screen_position((
                                self.config.width as f32 - consts::TEXT_SCALE,
                                consts::TEXT_SCALE * 0.5,
                            ))
                            .with_layout(
                                wgpu_glyph::Layout::default()
                                    .h_align(wgpu_glyph::HorizontalAlign::Right),
                            ),
                    ],
                );

                if *cx.paused {
                    self.text_pipeline.render(
                        &self.device,
                        &self.queue,
                        &view,
                        &[wgpu_glyph::Section::default()
                            .add_text(
                                wgpu_glyph::Text::new("PAUSED")
                                    .with_scale(consts::TEXT_SCALE * 2.0)
                                    .with_color(consts::text_color::TEXT_PRIMARY),
                            )
                            .with_screen_position((
                                self.config.width as f32 * 0.5,
                                self.config.height as f32 * 0.5,
                            ))
                            .with_layout(
                                wgpu_glyph::Layout::default()
                                    .h_align(wgpu_glyph::HorizontalAlign::Center)
                                    .v_align(wgpu_glyph::VerticalAlign::Center),
                            )],
                    );
                }
            }
            game::GameContext::End(cx) => {
                let mut instances = vec![];

                for col in 0..consts::VIEW_WIDTH as usize {
                    let position = [
                        col as f32 - consts::VIEW_WIDTH * 0.5,
                        consts::MAX_STACK_HEIGHT as f32 - consts::VIEW_HEIGHT * 0.5,
                        0.0,
                    ];
                    let color = consts::block_color::BG_MAX_STACK;
                    instances.push(block::Instance { position, color });
                }

                for (row, items) in cx.blocks.iter().enumerate() {
                    for (col, item) in items.iter().enumerate() {
                        if let Some(block_color) = item.as_ref() {
                            let position = [
                                col as f32 - consts::VIEW_WIDTH * 0.5,
                                row as f32 - consts::VIEW_HEIGHT * 0.5,
                                0.0,
                            ];
                            let color = consts::to_rgb(block_color);
                            instances.push(block::Instance { position, color });
                        }
                    }
                }

                self.block_pipeline.set_instances(&self.queue, &instances);
                self.block_pipeline.render(
                    &self.device,
                    &self.queue,
                    &view,
                    &self.camera_resource.bind_group,
                );

                self.text_pipeline.render(
                    &self.device,
                    &self.queue,
                    &view,
                    &[
                        wgpu_glyph::Section::default()
                            .add_text(
                                wgpu_glyph::Text::new("GAME OVER")
                                    .with_scale(consts::TEXT_SCALE * 2.0)
                                    .with_color(consts::text_color::TEXT_PRIMARY),
                            )
                            .with_screen_position((
                                self.config.width as f32 * 0.5,
                                self.config.height as f32 * 0.5 - consts::TEXT_SCALE * 2.0,
                            ))
                            .with_layout(
                                wgpu_glyph::Layout::default()
                                    .h_align(wgpu_glyph::HorizontalAlign::Center)
                                    .v_align(wgpu_glyph::VerticalAlign::Center),
                            ),
                        wgpu_glyph::Section::default()
                            .add_text(
                                wgpu_glyph::Text::new(&format!("SCORE: {}", cx.score))
                                    .with_scale(consts::TEXT_SCALE)
                                    .with_color(consts::text_color::TEXT_PRIMARY),
                            )
                            .with_screen_position((
                                self.config.width as f32 * 0.5,
                                self.config.height as f32 * 0.5 + consts::TEXT_SCALE,
                            ))
                            .with_layout(
                                wgpu_glyph::Layout::default()
                                    .h_align(wgpu_glyph::HorizontalAlign::Center)
                                    .v_align(wgpu_glyph::VerticalAlign::Center),
                            ),
                        wgpu_glyph::Section::default()
                            .add_text(
                                wgpu_glyph::Text::new("PRESS RETURN TO RESTART")
                                    .with_scale(consts::TEXT_SCALE)
                                    .with_color(consts::text_color::TEXT_PRIMARY),
                            )
                            .with_screen_position((
                                self.config.width as f32 * 0.5,
                                self.config.height as f32 * 0.5 + consts::TEXT_SCALE * 2.5,
                            ))
                            .with_layout(
                                wgpu_glyph::Layout::default()
                                    .h_align(wgpu_glyph::HorizontalAlign::Center)
                                    .v_align(wgpu_glyph::VerticalAlign::Center),
                            ),
                    ],
                );
            }
        }

        output.present();
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if 0 < new_size.width && 0 < new_size.height {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.camera_resource
                .resize(&self.queue, self.config.width, self.config.height);
            self.text_pipeline
                .resize(self.config.width, self.config.height);
        }
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn match_id(&self, id: winit::window::WindowId) -> bool {
        self.window.id() == id
    }
}
