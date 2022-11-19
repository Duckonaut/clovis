use crate::{Mode, State};
use anyhow::Result;
use crossterm::style::Color;

use super::{Shader, ShaderOutput};

// Based on https://www.shadertoy.com/view/XsVSzW
pub struct BlobsShader;

pub struct BlobsParams {
    size: (u16, u16),
    time: f32,
}
const K: f32 = 20.0;

fn center(time: f32, border: (f32, f32), offset: (f32, f32)) -> (f32, f32) {
    (time.cos(), time.sin())
}

fn circle(
    params: &BlobsParams,
    coord: (f32, f32),
    r: f32,
    col: (f32, f32, f32),
    offset: (f32, f32),
    vel: (f32, f32),
) -> f32 {
    let c = center(params.time, offset, vel);
    let d_squared = (coord.0 - c.0) * (coord.0 - c.0) + (coord.1 - c.1) * (coord.1 - c.1);
    (K * r) / d_squared
}

fn gradient(shade: f32) -> (f32, f32, f32) {
    (shade, shade, shade)
}

impl Shader for BlobsShader {
    type Params = BlobsParams;

    fn pixel(&self, pos: (u16, u16), params: &Self::Params) -> ShaderOutput {
        let normalized = (
            pos.0 as f32 / params.size.0 as f32,
            pos.1 as f32 / params.size.1 as f32,
        );

        let coord = normalized;
        let mut field = 0.0;

        field += circle(
            params,
            coord,
            0.03,
            (0.7, 0.2, 0.8),
            (0.6, 0.6),
            (0.30, 0.70),
        );
        field += circle(
            params,
            coord,
            0.05,
            (0.7, 0.9, 0.6),
            (0.1, 0.1),
            (0.02, 0.20),
        );
        field += circle(
            params,
            coord,
            0.07,
            (0.3, 0.4, 0.1),
            (0.1, 0.1),
            (0.10, 0.04),
        );
        field += circle(
            params,
            coord,
            0.10,
            (0.2, 0.5, 0.1),
            (0.3, 0.3),
            (0.10, 0.20),
        );
        field += circle(
            params,
            coord,
            0.20,
            (0.1, 0.3, 0.7),
            (0.2, 0.2),
            (0.40, 0.25),
        );
        field += circle(
            params,
            coord,
            0.30,
            (0.9, 0.4, 0.2),
            (0.0, 0.0),
            (0.15, 0.20),
        );

        let shade = (field / 256.0).clamp(0.0, 1.0);

        let color = gradient(shade);

        ShaderOutput {
            value: color.0,
            color: crossterm::style::Color::Rgb {
                /*
                r: (color.0 * 255.0) as u8,
                g: (color.1 * 255.0) as u8,
                b: (color.2 * 255.0) as u8,
                */
                r: 255,
                g: 255,
                b: 255,
            },
        }
    }

    fn get_params(&self, state: &State) -> Result<Self::Params> {
        Ok(BlobsParams {
            size: state.settings.size,
            time: state.start.elapsed()?.as_secs_f32(),
        })
    }
}
