extern crate sdl2;

mod chip8;

use sdl2::event::Event;
use sdl2::event::WindowEvent;

use chip8::Chip8;

fn main() {
    let _sdl = sdl2::init().unwrap();
    let video_subsystem = _sdl.video().unwrap();
    let window = video_subsystem.window("Game", 1280, 320).build().unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    let mut event_pump = _sdl.event_pump().unwrap();

    let path = std::env::args().nth(1).unwrap();

    let mut chip8 = Chip8::new(&path);

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Window { win_event, .. } => {
                    if win_event == WindowEvent::Close {
                        break 'main;
                    }
                }
                Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        chip8.do_command(&mut canvas);

        chip8.print();
    }
}
