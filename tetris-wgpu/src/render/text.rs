pub struct Pipeline {
    staging_belt: wgpu::util::StagingBelt,
    glyph_blush: wgpu_glyph::GlyphBrush<()>,
    target_width: u32,
    target_height: u32,
}

impl Pipeline {
    pub fn new(
        device: &wgpu::Device,
        target_format: wgpu::TextureFormat,
        target_width: u32,
        target_height: u32,
    ) -> Self {
        let staging_belt = wgpu::util::StagingBelt::new(1024);

        let font = wgpu_glyph::ab_glyph::FontArc::try_from_slice(include_bytes!(
            "../../assets/fonts/Roboto-Regular.ttf"
        ))
        .unwrap();

        let glyph_blush =
            wgpu_glyph::GlyphBrushBuilder::using_font(font).build(&device, target_format);

        Self {
            staging_belt,
            glyph_blush,
            target_width,
            target_height,
        }
    }

    pub fn render(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        target: &wgpu::TextureView,
        sections: &[wgpu_glyph::Section],
    ) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        for section in sections.into_iter() {
            self.glyph_blush.queue(section);
        }

        self.glyph_blush
            .draw_queued(
                device,
                &mut self.staging_belt,
                &mut encoder,
                target,
                self.target_width,
                self.target_height,
            )
            .unwrap();

        self.staging_belt.finish();
        queue.submit([encoder.finish()]);
        self.staging_belt.recall();
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.target_width = width;
        self.target_height = height;
    }
}
