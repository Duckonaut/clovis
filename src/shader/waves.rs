use crate::State;
use anyhow::Result;

use super::{Shader, ShaderOutput};

// Based on https://www.shadertoy.com/view/XsVSzW
pub struct WavesShader;

pub struct WavesParams {
    pub size: (u16, u16),
    pub time: f32,
    pub iterations: usize,
    pub scale: f32,
}

impl<'s> Shader<'s> for WavesShader {
    type Params<'p> = WavesParams where 'p: 's;

    fn pixel(&self, pos: (u16, u16), params: &Self::Params<'s>) -> ShaderOutput {
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

    fn update_params(&self, state: &State, params: &mut Self::Params<'s>) -> Result<()> {
        params.time = state.start.elapsed()?.as_secs_f32();

        Ok(())
    }
}
