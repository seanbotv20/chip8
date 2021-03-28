extern crate sdl2;

use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::pixels::Color;

fn main() {
    let _sdl = sdl2::init().unwrap();
    let video_subsystem = _sdl.video().unwrap();
    let window = video_subsystem.window("Game", 64, 32).build().unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = _sdl.event_pump().unwrap();

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
    }
}
