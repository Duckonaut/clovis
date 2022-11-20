use crate::{engine::CharGraphic, State};
use anyhow::Result;

pub mod blobs;
pub mod waves;

pub type ShaderOutput = CharGraphic;

pub trait Shader<'s> {
    type Params<'p>
    where
        'p: 's;

    fn pixel(&self, pos: (u16, u16), params: &Self::Params<'s>) -> ShaderOutput;

    fn run<'st, 'pa>(
        &self,
        state: &'st mut State,
        params: &'pa Self::Params<'s>,
    ) -> Vec<ShaderOutput>
    where
        'st: 'pa,
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

    fn update_params(&self, _state: &State, _params: &mut Self::Params<'s>) -> Result<()> {
        Ok(())
    }
}
