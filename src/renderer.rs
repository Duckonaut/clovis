use anyhow::Result;
use crossterm::style::Color;
use crossterm::{queue, style};
use std::io::Write;
use std::iter::Iterator;

use crate::Settings;
use crate::{shader::ShaderOutput, State};

pub fn render(state: &mut State, shader_output: Vec<ShaderOutput>) -> Result<()> {
    let mut last = None;
    let mut slice = [0u8; 4];
    for so in shader_output {
        let render = if state.settings.rgb {
            so.render(&state.settings)
        } else {
            so.render_quantized(&state.settings)
        };

        if render.style().foreground_color != last {
            last = render.style().foreground_color;

            queue!(
                state.stdout,
                style::SetForegroundColor(last.unwrap_or(Color::Black))
            )?;
        }

        state
            .stdout
            .write_all(render.content().encode_utf8(&mut slice).as_bytes())?;
    }
    Ok(())
}

pub fn quantize_color(settings: &Settings, color: Color) -> Color {
    match color {
        Color::Rgb { r, g, b } => color_map(&settings.colors, (r, g, b)),
        _ => color,
    }
}

fn color_map(colors: &[(u8, u8, u8); 16], color: (u8, u8, u8)) -> Color {
    let i = colors
        .iter()
        .enumerate()
        .min_by_key(|(_, c)| {
            (c.0 as i32 - color.0 as i32).pow(2)
                + (c.1 as i32 - color.1 as i32).pow(2)
                + (c.2 as i32 - color.2 as i32).pow(2)
        })
        .unwrap()
        .0;

    Color::parse_ansi(format!("5;{}", i).as_str()).unwrap()
}
