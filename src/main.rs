mod chip8;
mod rendering_context;
mod timer;

use chip8::Chip8;
use rendering_context::SDLRenderingContext;

fn main() {
    let path = std::env::args().nth(1).unwrap();

    let mut chip8 = Chip8::new(&path, SDLRenderingContext::new());

    chip8.run()
}
