use std::{
    io::{stdout, Stdout},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::SystemTime,
};

use anyhow::Result;
use clap::{ColorChoice, Parser, Subcommand};
use engine::run_shader;
use shader::{blobs::{BlobsShader, BlobsParams}, waves::{WavesShader, WavesParams}};

mod engine;
mod renderer;
mod shader;

#[derive(Parser, Debug, Clone)]
#[command(name = "clovis", about = "Screensavers for your terminal", color = ColorChoice::Always)]
struct Args {
    #[arg(
        short = 'f',
        long = "fullcolor",
        help = "Run without mapping colors to the 16 standard colors."
    )]
    rgb: bool,
    #[arg(short = 'r', long = "refresh", help = "Target refresh rate")]
    refresh: Option<u32>,
    #[arg(short = 'c', long = "characters", help = "Characters")]
    characters: Option<String>,
    #[command(subcommand)]
    mode: Mode,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Mode {
    Waves {
        #[arg(short = 'i', long = "iterations", help = "Iterations of complexity")]
        iterations: Option<usize>,
        #[arg(short = 's', long = "scale", help = "Scale of surface")]
        scale: Option<f32>,
    },
    Blobs,
}

pub struct Settings {
    pub size: (u16, u16),
    pub char_map: Vec<char>,
    pub colors: [(u8, u8, u8); 16],
    pub rgb: bool,
    pub refresh: u32,
    pub mode_args: Mode,
}

pub struct State {
    pub settings: Settings,
    pub stdout: Stdout,
    pub start: SystemTime,
    pub running: Arc<AtomicBool>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let size = crossterm::terminal::size()?;

    let settings = Settings {
        size,
        char_map: if let Some(s) = args.characters {
            s.chars().collect::<Vec<char>>()
        } else {
            vec![' ', '.', '-', '/', '!', '*', '=', '#', '@']
        },
        colors: [
            (0x00, 0x00, 0x00),
            (0x80, 0x00, 0x00),
            (0x00, 0x80, 0x00),
            (0x80, 0x80, 0x00),
            (0x00, 0x00, 0x80),
            (0x80, 0x00, 0x80),
            (0x00, 0x80, 0x80),
            (0x80, 0x80, 0x80),
            (0x80, 0x80, 0x80),
            (0xFF, 0x00, 0x00),
            (0x00, 0xFF, 0x00),
            (0xFF, 0xFF, 0x00),
            (0x00, 0x00, 0xFF),
            (0xFF, 0x00, 0xFF),
            (0x00, 0xFF, 0xFF),
            (0xFF, 0xFF, 0xFF),
        ],
        rgb: args.rgb,
        refresh: args.refresh.unwrap_or(32),
        mode_args: args.mode.clone(),
    };

    let mut state = State {
        settings,
        stdout: stdout(),
        start: SystemTime::now(),
        running: Arc::new(AtomicBool::new(true)),
    };

    let r = state.running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::Relaxed);
    })?;

    match args.mode {
        Mode::Waves { .. } => waves(&mut state)?,
        Mode::Blobs => blobs(&mut state)?,
    }

    Ok(())
}

fn waves(state: &mut State) -> Result<()> {
    let params = WavesParams {
        size: state.settings.size,
        time: 0.0,
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
    };
    run_shader(state, WavesShader, params)?;
    Ok(())
}

fn blobs(state: &mut State) -> Result<()> {
    let mut count = 0;

    let params = BlobsParams {
        size: state.settings.size,
        time: 0.0,
        times_ran: &mut count,
    };

    run_shader(state, BlobsShader, params)?;

    println!("Ran {} times", count);

    Ok(())
}
