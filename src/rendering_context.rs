use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::video::Window;
use sdl2::EventPump;

pub type Sprite = Vec<u8>;

pub const DISPLAY_WIDTH: u32 = 64;
pub const DISPLAY_HEIGHT: u32 = 32;

const SCALING: u32 = 10;

pub struct SDLRenderingContext {
    pub event_pump: EventPump,
    pub window: Window,
}

impl SDLRenderingContext {
    pub fn new() -> SDLRenderingContext {
        let _sdl = sdl2::init().unwrap();
        let video_subsystem = _sdl.video().unwrap();
        let window = video_subsystem
            .window("Chip8", DISPLAY_WIDTH * SCALING, DISPLAY_HEIGHT * SCALING)
            .build()
            .unwrap();

        let event_pump = _sdl.event_pump().unwrap();

        return SDLRenderingContext { event_pump, window };
    }

    pub fn redraw(&self, &display: &[[bool; DISPLAY_HEIGHT as usize]; DISPLAY_WIDTH as usize]) {
        let mut buffer: [u8; DISPLAY_HEIGHT as usize * DISPLAY_WIDTH as usize] =
            [0; DISPLAY_HEIGHT as usize * DISPLAY_WIDTH as usize];

        for column in 0..(DISPLAY_WIDTH as usize) {
            for row in 0..(DISPLAY_HEIGHT as usize) {
                let pixel = &mut buffer[(row * DISPLAY_WIDTH as usize) + column as usize];
                if display[column][row] {
                    *pixel = 0;
                } else {
                    *pixel = 255;
                }
            }
        }

        let mut window_surface = self.window.surface(&self.event_pump).unwrap();
        let surface = Surface::from_data(
            &mut buffer,
            DISPLAY_WIDTH,
            DISPLAY_HEIGHT,
            DISPLAY_WIDTH,
            PixelFormatEnum::RGB332,
        )
        .unwrap();

        surface
            .blit_scaled(
                Rect::new(0, 0, DISPLAY_WIDTH, DISPLAY_WIDTH),
                &mut window_surface,
                Rect::new(0, 0, DISPLAY_WIDTH * SCALING, DISPLAY_WIDTH * SCALING),
            )
            .unwrap();

        window_surface.update_window().unwrap();
    }

    pub fn clear(&mut self) {
        let mut window_surface = self.window.surface(&self.event_pump).unwrap();

        window_surface
            .fill_rect(
                Rect::new(0, 0, DISPLAY_WIDTH * SCALING, DISPLAY_HEIGHT * SCALING),
                Color::RGB(255, 255, 255),
            )
            .unwrap();

        window_surface.update_window().unwrap();
    }

    pub fn run(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Window { win_event, .. } => {
                    if win_event == WindowEvent::Close {
                        return false;
                    }
                }
                Event::Quit { .. } => return false,
                _ => {}
            }
        }
        return true;
    }
}
