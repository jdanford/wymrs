use std::time::{Duration, Instant};

use sdl2::{
    event::Event,
    rect::Point,
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
    Sdl,
};

use crate::{
    config::{
        CLEAR_COLOR, PIXEL_FORMAT, STEP_TIME, TILE_SIZE, WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH,
    },
    NewWorldParams, Result, World,
};

pub struct App {
    width: u16,
    #[allow(dead_code)]
    height: u16,
    display_ratio: u32,
    tile_size: u32,
    sdl_context: Sdl,
    canvas: Canvas<Window>,
    #[allow(dead_code)]
    texture_creator: TextureCreator<WindowContext>,
    texture: Texture,
    pixel_data: Vec<u8>,
    world: World,
    last_step_time: Option<Instant>,
    elapsed_time: Duration,
}

impl App {
    pub fn new() -> Result<App> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        sdl2::hint::set("SDL_HINT_RENDER_SCALE_QUALITY", "nearest");

        let window = video_subsystem
            .window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .allow_highdpi()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window
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

        let pitch = PIXEL_FORMAT.byte_size_of_pixels(width.into());
        let byte_size = PIXEL_FORMAT.byte_size_from_pitch_and_height(pitch, height.into());
        let pixel_data = vec![0u8; byte_size];

        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
            .create_texture_streaming(PIXEL_FORMAT, width.into(), height.into())
            .map_err(|e| e.to_string())?;

        let world = World::new(&NewWorldParams { width, height });

        Ok(App {
            width,
            height,
            display_ratio,
            tile_size,
            sdl_context,
            canvas,
            texture_creator,
            texture,
            pixel_data,
            world,
            last_step_time: None,
            elapsed_time: Duration::ZERO,
        })
    }

    #[must_use]
    fn pixel_pitch(&self) -> usize {
        PIXEL_FORMAT.byte_size_of_pixels(self.width.into())
    }

    #[allow(clippy::cast_possible_wrap)]
    #[allow(clippy::cast_sign_loss)]
    fn world_coordinate_from_screen(&self, x: i32) -> i32 {
        x * self.display_ratio as i32 / self.tile_size as i32
    }

    fn render(&mut self) -> Result<()> {
        self.world.render(&mut self.pixel_data[..]);
        self.texture
            .update(None, &self.pixel_data[..], self.pixel_pitch())
            .map_err(|e| e.to_string())?;

        self.canvas.set_draw_color(CLEAR_COLOR);
        self.canvas.clear();
        self.canvas.copy(&self.texture, None, None)?;
        self.canvas.present();
        Ok(())
    }

    fn step(&mut self) -> Result<()> {
        let needs_update = if let Some(last_step_time) = self.last_step_time {
            let elapsed_time = last_step_time.elapsed();
            self.elapsed_time += elapsed_time;
            elapsed_time >= STEP_TIME
        } else {
            true
        };

        if needs_update {
            self.last_step_time = Some(Instant::now());
            self.world.step()?;
            self.render()?;
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        let mut event_pump = self.sdl_context.event_pump()?;

        self.render()?;

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::MouseButtonUp { x, y, .. } => {
                        let world_x = self.world_coordinate_from_screen(x);
                        let world_y = self.world_coordinate_from_screen(y);
                        let position = Point::new(world_x, world_y);
                        self.world.create_wyrm(position)?;
                    }
                    _ => {}
                }
            }

            self.step()?;
        }

        Ok(())
    }
}
