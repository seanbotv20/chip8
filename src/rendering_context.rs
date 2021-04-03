use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

pub struct SDLRenderingContext {
    pub event_pump: EventPump,
    pub canvas: Canvas<Window>,
}

impl SDLRenderingContext {
    pub fn new() -> SDLRenderingContext {
        let _sdl = sdl2::init().unwrap();
        let video_subsystem = _sdl.video().unwrap();
        let window = video_subsystem.window("Game", 1280, 320).build().unwrap();

        let canvas = window.into_canvas().present_vsync().build().unwrap();

        let event_pump = _sdl.event_pump().unwrap();

        return SDLRenderingContext {
            event_pump: event_pump,
            canvas: canvas,
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
