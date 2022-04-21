use macroquad::prelude::*;
use std::usize;

pub struct GfxPipeline<const SIZE: usize> {
    pipeline: [Material; SIZE],
    pub buffer: RenderTarget,
    pub camera: Camera2D,
}

impl<const SIZE: usize> GfxPipeline<SIZE> {
    pub fn new(width: f32, height: f32, pipeline: &[Material; SIZE]) -> Self {
        let buffer = render_target(width as u32, height as u32);
        let camera = Camera2D {
            render_target: Some(buffer),
            ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, width, height))
        };
        Self {
            pipeline: *pipeline,
            buffer,
            camera,
        }
    }

    pub fn pipe(&self, draw: &dyn Fn() -> ()) {
        if SIZE == 0 {
            clear_background(TRANSPARENT);
            draw();
            return;
        }

        set_camera(&self.camera);
        clear_background(TRANSPARENT);
        draw();

        if SIZE > 1 {
            for i in 0..(SIZE - 1) {
                gl_use_material(self.pipeline[i]);
                self.draw_buffer();
            }
            gl_use_material(self.pipeline[SIZE - 1]);
        } else {
            gl_use_material(self.pipeline[0]);
        }

        set_default_camera();
        self.draw_buffer();
        gl_use_default_material();
    }

    pub fn draw_buffer(&self) {
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
    }

    pub fn update_dimensions(&mut self, width: f32, height: f32) {
        self.buffer = render_target(width as u32, height as u32);
        self.camera = Camera2D {
            render_target: Some(self.buffer),
            ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, width, height))
        };
    }
}

pub const TRANSPARENT: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 0.0,
};
