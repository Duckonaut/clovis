use crate::{engine::CharGraphic, State};
use anyhow::Result;

pub mod blobs;
pub mod waves;

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
