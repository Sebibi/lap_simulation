#[cfg(feature = "ffmpeg")]
mod common;

#[cfg(feature = "ffmpeg")]
#[path = "simulation/open_loop.rs"]
mod open_loop;

#[path = "simulation/base_simulation.rs"]
mod base_simulation;
