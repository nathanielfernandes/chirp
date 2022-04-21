use macroquad::{
    miniquad::{BlendFactor, BlendState, BlendValue, Equation},
    prelude::*,
};

pub struct PostProcessing {
    pub shader: Material,
    pub buffer: RenderTarget,
    pub camera: Camera2D,
}

impl PostProcessing {
    pub fn new(width: f32, height: f32) -> Self {
        let shader = load_material(
            &VERTEX_SHADER,
            &CHROMATIC_ABERRATION,
            MaterialParams {
                pipeline_params: PipelineParams {
                    color_blend: Some(BlendState::new(
                        Equation::Add,
                        BlendFactor::One,
                        BlendFactor::Value(BlendValue::SourceColor),
                    )),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .unwrap();

        let buffer = render_target(width as u32, height as u32);
        let camera = Camera2D {
            render_target: Some(buffer),
            ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, width, height))
        };

        PostProcessing {
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
        self.draw();
    }
}

pub const TRANSPARENT: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 0.0,
};

pub const CHROMATIC_ABERRATION: &str = "
    #version 100
    precision lowp float;
    varying vec4 color;
    varying vec2 uv;
    uniform sampler2D Texture;

    void main() {
        float aberrationAmount = 0.2;
        vec2 distFromCenter = uv - 0.5;
        vec2 aberrated = aberrationAmount * pow(distFromCenter, vec2(3.0, 3.0));

        gl_FragColor = vec4(
            texture2D(Texture, uv - aberrated).r,
            texture2D(Texture, uv).g,
            texture2D(Texture, uv + aberrated).b,
            1.0
        );
    }
";

pub const VERTEX_SHADER: &str = "
    #version 100
    attribute vec3 position;
    attribute vec2 texcoord;
    attribute vec4 color0;
    varying lowp vec2 uv;
    varying lowp vec4 color;
    uniform mat4 Model;
    uniform mat4 Projection;
    void main() {
        gl_Position = Projection * Model * vec4(position, 1);
        color = color0 / 255.0;
        uv = texcoord;
    }
";
