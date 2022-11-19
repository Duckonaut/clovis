use crate::{engine::CharGraphic, Mode, State};
use anyhow::Result;

pub type ShaderOutput = CharGraphic;

pub trait Shader {
    type Params;

    fn pixel(&self, pos: (u16, u16), params: &Self::Params) -> ShaderOutput;

    fn run<'s, 'p>(&self, state: &'s mut State, params: &'p Self::Params) -> Vec<ShaderOutput>
    where
        's: 'p,
    {
        let mut out: Vec<ShaderOutput> = {
            let mut v = Vec::new();
            v.resize(
                (state.settings.size.0 * state.settings.size.1) as usize,
                ShaderOutput::default(),
            );
            v
        };

        for y in 0..state.settings.size.1 {
            for x in 0..state.settings.size.0 {
                out[(y * state.settings.size.0 + x) as usize] = self.pixel((x, y), params);
            }
        }

        out
    }

    fn get_params(&self, state: &State) -> Result<Self::Params>;
}

// Based on https://www.shadertoy.com/view/XsVSzW
pub struct WavesShader;

pub struct WavesParams {
    size: (u16, u16),
    time: f32,
    iterations: usize,
    scale: f32,
}

impl Shader for WavesShader {
    type Params = WavesParams;

    fn pixel(&self, pos: (u16, u16), params: &Self::Params) -> ShaderOutput {
        let mut normalized = (
            pos.0 as f32 / params.size.0 as f32 * params.scale,
            pos.1 as f32 / params.size.1 as f32 * params.scale,
        );
        let mut i0: f32 = 1.0;
        let mut i1: f32 = 1.0;
        let mut i2: f32 = 1.0;
        let mut i4: f32 = 0.0;

        for _ in 0..params.iterations {
            let mut r = (
                (normalized.1 * i0 - i4 + params.time / i1).cos() / i2,
                (normalized.0 * i0 - i4 + params.time / i1).sin() / i2,
            );
            r.0 += -r.1 * 0.3;
            r.1 += r.0 * 0.3;

            normalized.0 += r.0;
            normalized.1 += r.1;

            i0 *= 1.93;
            i1 *= 1.15;
            i2 *= 1.7;
            i4 += 0.05 + 0.1 * params.time * i1;
        }

        ShaderOutput {
            value: (1.0 - (normalized.0 * std::f32::consts::PI).cos()) / 2.,
            color: crossterm::style::Color::Rgb {
                r: (((normalized.0 - params.time).sin() * 0.5 + 0.5) * 255.0) as u8,
                g: (((normalized.1 + params.time).sin() * 0.5 + 0.5) * 255.0) as u8,
                b: ((((normalized.0 + normalized.1 + (params.time * 0.5).sin()) * 0.5).sin() * 0.5
                    + 0.5)
                    * 255.0) as u8,
            },
        }
    }

    fn get_params(&self, state: &State) -> Result<Self::Params> {
        Ok(WavesParams {
            size: state.settings.size,
            time: state.start.elapsed()?.as_secs_f32(),
            iterations: if let Mode::Waves { iterations, .. } = state.settings.mode_args {
                iterations.unwrap_or(5)
            } else {
                panic!(
                    "Bad mode in settings for shader {}",
                    stringify!(WavesShader)
                )
            },
            scale: if let Mode::Waves { scale, .. } = state.settings.mode_args {
                scale.unwrap_or(10.0)
            } else {
                panic!(
                    "Bad mode in settings for shader {}",
                    stringify!(WavesShader)
                )
            },
        })
    }
}
