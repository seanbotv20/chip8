use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Scancode;
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

    // Uses octo keyboard standard
    // Chip-8 Key  Keyboard
    // ----------  ---------
    //   1 2 3 C    1 2 3 4
    //   4 5 6 D    q w e r
    //   7 8 9 E    a s d f
    //   A 0 B F    z x c v
    pub fn is_key_pressed(&mut self, key: u8) -> bool {
        let scancode = match key {
            0x0 => Some(Scancode::X),
            0x1 => Some(Scancode::Num1),
            0x2 => Some(Scancode::Num2),
            0x3 => Some(Scancode::Num3),
            0x4 => Some(Scancode::Q),
            0x5 => Some(Scancode::W),
            0x6 => Some(Scancode::E),
            0x7 => Some(Scancode::A),
            0x8 => Some(Scancode::S),
            0x9 => Some(Scancode::D),
            0xA => Some(Scancode::Z),
            0xB => Some(Scancode::C),
            0xC => Some(Scancode::Num4),
            0xD => Some(Scancode::R),
            0xE => Some(Scancode::F),
            0xF => Some(Scancode::V),
            _ => None,
        };

        return match scancode {
            Some(code) => self.event_pump.keyboard_state().is_scancode_pressed(code),
            None => false,
        };
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
