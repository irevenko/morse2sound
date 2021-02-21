use std::{thread::sleep, time::Duration};

use anyhow::Result;
use rodio::Source;
use structopt::StructOpt;

pub mod args;

use crate::args::{Message, Unit};

#[derive(Debug, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::AllowLeadingHyphen)]
struct App {
    input: String,
    #[structopt(long, short, default_value = "100")]
    dot_duration: u64,
    #[structopt(long, short, default_value = "500")]
    frequency: u32,
}

fn main() -> Result<()> {
    let app = App::from_args();
    run(&app)
}

fn run(app: &App) -> Result<()> {
    let (_stream, stream_handle) = rodio::OutputStream::try_default()?;
    let sink = rodio::Sink::try_new(&stream_handle)?;

    let tone = rodio::source::SineWave::new(app.frequency);
    let dot_duration = Duration::from_millis(app.dot_duration);
    let dot = tone.clone().take_duration(dot_duration);
    let dash = tone.take_duration(dot_duration * 3);

    let message = app.input.parse::<Message>()?;

    for unit in message.0 {
        match unit {
            Unit::Dot => {
                sink.append(dot.clone());
                sink.sleep_until_end();
                sleep(dot_duration);
            }
            Unit::Dash => {
                sink.append(dash.clone());
                sink.sleep_until_end();
                sleep(dot_duration)
            }
            Unit::Space => {
                sleep(dot_duration * 3);
            }
            Unit::Slash => {
                sleep(dot_duration * 3);
            }
        }
    }
    Ok(())
}