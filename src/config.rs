use std::time::Duration;

use sdl2::pixels::{Color, PixelFormatEnum};

pub const WINDOW_TITLE: &str = "wyrms";
pub const WINDOW_WIDTH: u32 = 1024;
pub const WINDOW_HEIGHT: u32 = 768;

pub const TILE_SIZE: u32 = 8;
pub const SPAWN_INTERVAL: usize = 32;

pub const FPS: u64 = 16;
pub const STEP_TIME: Duration = Duration::from_micros(1_000_000 / FPS);

pub const CLEAR_COLOR: Color = Color::BLACK;
pub const PIXEL_FORMAT: PixelFormatEnum = PixelFormatEnum::RGB24;
