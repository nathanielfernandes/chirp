use macroquad::prelude::*;

pub struct GfxPipeline {
    pipeline: Vec<GfxShader>,
}

impl GfxPipeline {
    pub fn new(shaders: Vec<GfxShader>) -> Self {
        GfxPipeline { pipeline: shaders }
    }

    pub fn pipe(&self, draw: &dyn Fn() -> ()) {
        let primary = &self.pipeline[0];
        primary.apply(draw);

        let n = self.pipeline.len();
        if n > 1 {
            (1..n).for_each(|i| self.pipeline[i].apply(&|| self.pipeline[i - 1].draw()));
            self.pipeline[n - 1].draw();
        } else {
            primary.draw();
        }
    }

    pub fn update_dimensions(&mut self, width: f32, height: f32) {
        self.pipeline
            .iter_mut()
            .for_each(|s| s.update_dimensions(width, height))
    }
}

pub struct GfxShader {
    pub shader: Material,
    pub buffer: RenderTarget,
    pub camera: Camera2D,
}

impl GfxShader {
    pub fn new(width: f32, height: f32, shader: Material) -> Self {
        let buffer = render_target(width as u32, height as u32);
        let camera = Camera2D {
            render_target: Some(buffer),
            ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, width, height))
        };

        GfxShader {
            shader,
            buffer,
            camera,
        }
    }

    pub fn update_dimensions(&mut self, width: f32, height: f32) {
        self.buffer = render_target(width as u32, height as u32);
        self.camera = Camera2D {
            render_target: Some(self.buffer),
            ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, width, height))
        };
    }

    pub fn open(&self) {
        set_camera(&self.camera);
        clear_background(TRANSPARENT);
    }

    pub fn close(&self) {
        set_default_camera();
    }

    pub fn draw(&self) {
        gl_use_material(self.shader);
        draw_texture_ex(
            self.buffer.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                flip_y: true,
                ..Default::default()
            },
        );
        gl_use_default_material();
    }

    pub fn apply(&self, draw: &dyn Fn() -> ()) {
        self.open();
        draw();
        self.close();
    }
}

pub const TRANSPARENT: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 0.0,
};
