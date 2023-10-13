use std::{time::Duration, time::Instant};

use sdl2::{
    event::Event,
    pixels::{Color, PixelFormatEnum},
};
use wymrs::{NewWorldParams, Point, Result, World};

const WINDOW_TITLE: &str = "wyrms";
const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 768;

const TILE_SIZE: u32 = 8;
const SPAWN_INTERVAL: usize = 32;

const FPS: u64 = 16;
const STEP_TIME: Duration = Duration::from_micros(1_000_000 / FPS);

pub fn main() -> Result<()> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    sdl2::hint::set("SDL_HINT_RENDER_SCALE_QUALITY", "0");

    let window = video_subsystem
        .window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .allow_highdpi()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let (window_width, window_height) = canvas.window().drawable_size();
    let display_ratio = window_width / WINDOW_WIDTH;

    let tile_size = TILE_SIZE * display_ratio;
    let width = u16::try_from(window_width / tile_size).map_err(|e| e.to_string())?;
    let height = u16::try_from(window_height / tile_size).map_err(|e| e.to_string())?;

    let pixel_format = PixelFormatEnum::RGB24;
    let pitch = pixel_format.byte_size_of_pixels(width.into());
    let byte_size = pixel_format.byte_size_from_pitch_and_height(pitch, height.into());
    let mut pixel_data = vec![0u8; byte_size];

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, width.into(), height.into())
        .map_err(|e| e.to_string())?;

    let mut world = World::new(&NewWorldParams {
        width,
        height,
        spawn_interval: SPAWN_INTERVAL,
        pixel_format,
    });

    let mut render = |world: &mut World| -> Result<()> {
        world.render(&mut pixel_data[..]);
        texture
            .update(None, &pixel_data[..], pitch)
            .map_err(|e| e.to_string())?;

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.copy(&texture, None, None)?;
        canvas.present();
        Ok(())
    };

    let world_coordinate_from_screen = |x: i32| -> Result<u16> {
        u16::try_from(x as u32 * display_ratio / tile_size).map_err(|e| e.to_string())
    };

    render(&mut world)?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut last_step_time: Option<Instant> = None;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::MouseButtonUp { x, y, .. } => {
                    let world_x = world_coordinate_from_screen(x)?;
                    let world_y = world_coordinate_from_screen(y)?;
                    let position = Point::new(world_x, world_y);
                    world.create_wyrm(position)?;
                }
                _ => {}
            }
        }

        let elapsed_time = last_step_time.map_or(Duration::MAX, |time| time.elapsed());
        if elapsed_time >= STEP_TIME {
            last_step_time = Some(Instant::now());
            world.step()?;
            render(&mut world)?;
        }
    }

    Ok(())
}
