use std::path::PathBuf;

use clap::{ArgAction, Parser};

use crate::utils::version;

#[derive(Parser, Debug)]
#[command(author, version = version(), about)]
pub struct Cli {
  #[arg(
    short,
    long,
    value_name = "FLOAT",
    help = "Tick rate, i.e. number of ticks per second",
    default_value_t = 0.07692307692
  )]
  pub tick_rate: f64,

  #[arg(
    short,
    long,
    value_name = "FLOAT",
    help = "Frame rate, i.e. number of frames per second",
    default_value_t = 13.0
  )]
  pub frame_rate: f64,
  #[arg(
    short,
    long,
    value_name = "DASHBOARD",
    help = "Invoked from mempool-space --dashboard",
    default_value_t = false,
    action=ArgAction::SetTrue
  )]
  pub dashboard: bool,
}
