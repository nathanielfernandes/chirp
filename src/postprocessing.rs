use macroquad::prelude::*;
use std::usize;

use crate::shaders::DEFAULT_SHADER;

pub struct GfxPipeline<const SIZE: usize> {
    pipeline: [GfxShader; SIZE],
}

impl<const SIZE: usize> GfxPipeline<SIZE> {
    pub fn new(width: f32, height: f32, pipeline: &[Material; SIZE]) -> Self {
        let mut new_pipeline = [*DEFAULT_SHADER; SIZE];
        for (i, &shader) in pipeline.iter().enumerate() {
            new_pipeline[i] = GfxShader::new(width, height, shader)
        }
        Self {
            pipeline: new_pipeline,
        }
    }

    pub fn pipe(&self, draw: &dyn Fn() -> ()) {
        if SIZE == 0 {
            clear_background(TRANSPARENT);
            draw();
            return;
        }

        let primary = &self.pipeline[0];
        primary.apply(draw);

        if SIZE > 1 {
            (1..SIZE).for_each(|i| self.pipeline[i].apply(&|| self.pipeline[i - 1].draw()));
            self.pipeline[SIZE - 1].draw();
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

#[derive(Clone, Copy)]
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
