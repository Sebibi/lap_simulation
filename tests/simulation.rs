#[cfg(feature = "ffmpeg")]
mod common;

#[cfg(feature = "ffmpeg")]
#[path = "simulation/open_loop.rs"]
mod open_loop;
