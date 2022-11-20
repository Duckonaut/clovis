use std::{
    io::Write,
    thread::sleep,
    time::{Duration, SystemTime},
};

use crate::{
    renderer::{quantize_color, render},
    shader::Shader,
    Settings, State,
};
use anyhow::Result;
use crossterm::{
    cursor, execute,
    style::{self, Color, Stylize},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

#[derive(Clone, Copy, PartialEq)]
pub struct CharGraphic {
    pub value: f32,
    pub color: Color,
}

impl CharGraphic {
    pub fn render(&self, settings: &Settings) -> style::StyledContent<char> {
        settings.char_map[(self.value * settings.char_map.len() as f32)
            .clamp(0., settings.char_map.len() as f32 - 1.0) as usize
            as usize]
            .with(self.color)
    }

    pub fn render_quantized(&self, settings: &Settings) -> style::StyledContent<char> {
        settings.char_map[(self.value * settings.char_map.len() as f32)
            .clamp(0., settings.char_map.len() as f32 - 1.0) as usize]
            .with(quantize_color(settings, self.color))
    }
}

impl Default for CharGraphic {
    fn default() -> Self {
        Self {
            value: Default::default(),
            color: Color::Black,
        }
    }
}

pub fn run_shader<'s, S>(state: &mut State, shader: S, params: S::Params<'s>) -> Result<()>
where
    S: Shader<'s>,
{
    let interval = 1.0 / state.settings.refresh as f32;

    execute!(
        state.stdout,
        EnterAlternateScreen,
        Clear(ClearType::All),
        cursor::SetCursorShape(cursor::CursorShape::UnderScore),
        cursor::MoveTo(0, 0)
    )?;

    let mut params = params;

    loop {
        let loop_start = SystemTime::now();

        loop_run(state, &shader, &mut params)?;

        if !state.running.load(std::sync::atomic::Ordering::Relaxed) {
            break;
        }
        sleep(Duration::from_secs_f32(
            (interval - loop_start.elapsed()?.as_secs_f32()).clamp(0., interval),
        ));
    }

    execute!(state.stdout, LeaveAlternateScreen)?;
    Ok(())
}

pub fn loop_run<'s, S>(
    state: &mut State,
    shader: &S,
    params: &mut S::Params<'s>,
) -> Result<()>
where
    S: Shader<'s>,
{
    shader.update_params(state, params)?;

    execute!(state.stdout, cursor::MoveTo(0, 0))?;

    let shader_output = shader.run(state, params);
    render(state, shader_output)?;
    state.stdout.flush()?;

    Ok(())
}
