#![allow(non_upper_case_globals)]
use macroquad::{
    miniquad::{BlendFactor, BlendState, BlendValue, Equation},
    prelude::*,
};

use crate::postprocessing::GfxShader;

lazy_static! {
    pub static ref DEFAULT_SHADER: GfxShader = GfxShader::new(0.0, 0.0, *DefaultFrag, false);
    pub static ref DefaultFrag: Material = load_material(
        &VERTEX_SHADER,
        "#version 100
         precision lowp float;
         varying vec4 color;
         void main() {
             gl_FragColor = color;
         }
        ",
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
    pub static ref ChromaticAberration: Material = load_material(
        &VERTEX_SHADER,
        &CHROMATIC_ABERRATION_FRAG,
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
    pub static ref GaussianBlur: Material = load_material(
        &VERTEX_SHADER,
        &GAUSSIAN_BLUR_FRAG,
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
    pub static ref Bloom: Material = load_material(
        &VERTEX_SHADER,
        &BLOOM_FRAG,
        MaterialParams {
            pipeline_params: PipelineParams {
                color_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::Value(BlendValue::DestinationAlpha),
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                )),
                ..Default::default()
            },
            ..Default::default()
        },
    )
    .unwrap();
}

const CHROMATIC_ABERRATION_FRAG: &str = "
    #version 100
    precision lowp float;
    varying vec4 color;
    varying vec2 uv;
    uniform sampler2D Texture;

    void main() {
        // CHROMATIC ABERRATION SETTINGS {{{
        float aberrationAmount = 0.05; // (Default 0.05)
        // CHROMATIC ABERRATION SETTINGS }}}

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

const GAUSSIAN_BLUR_FRAG: &str = "
    #version 100
    precision lowp float;
    varying vec4 color;
    varying vec2 uv;
    uniform sampler2D Texture;
    
    const float Pi = 6.28318530718;
    void main() {
        vec4 Color = texture2D(Texture, uv);

        // GAUSSIAN BLUR SETTINGS {{{
        const float Directions = 16.0; // BLUR DIRECTIONS (Default 16.0 - More is better but slower)
        const float Quality = 10.0; // BLUR QUALITY (Default 4.0 - More is better but slower)
        const float Size = 2.0; // BLUR SIZE (Radius) (Default 10.0 - More is better but slower)
        // GAUSSIAN BLUR SETTINGS }}}
    
        vec2 Radius = Size / vec2(1000.0, 1000.0);

        for( float d=0.0; d<Pi; d+=Pi/Directions)
        {
            for(float i=1.0/Quality; i<=1.0; i+=1.0/Quality)
            {
                Color += texture2D(Texture, uv+vec2(cos(d),sin(d))*Radius*i);		
            }
        }
        
        Color /= Quality * Directions - 15.0;
        gl_FragColor = Color ; //* 1.2;
    }
";

const BLOOM_FRAG: &str = "
    #version 100
    precision lowp float;
    varying vec4 color;
    varying vec2 uv;
    uniform sampler2D Texture;
    
    const float Pi = 6.28318530718;
    void main() {
        vec4 Color = texture2D(Texture, uv);

        // GAUSSIAN BLUR SETTINGS {{{
        const float Directions = 16.0; // BLUR DIRECTIONS (Default 16.0 - More is better but slower)
        const float Quality = 10.0; // BLUR QUALITY (Default 4.0 - More is better but slower)
        const float Size = 10.0; // BLUR SIZE (Radius) (Default 10.0 - More is better but slower)
        // GAUSSIAN BLUR SETTINGS }}}
    
        vec2 Radius = Size / vec2(1000.0, 1000.0);

        for( float d=0.0; d<Pi; d+=Pi/Directions)
        {
            for(float i=1.0/Quality; i<=1.0; i+=1.0/Quality)
            {
                Color += texture2D(Texture, uv+vec2(cos(d),sin(d))*Radius*i);		
            }
        }
        
        Color /= Quality * Directions - 15.0;
        gl_FragColor = Color ; //* 1.2;
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
