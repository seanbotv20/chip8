use crate::rendering_context::SDLRenderingContext;
use crate::rendering_context::Sprite;
use crate::timer::{DelayTimer, MainTimer};

use rand::prelude::random;

const PROGRAM_START: u16 = 0x200;

const DISPLAY_HEIGHT: usize = crate::rendering_context::DISPLAY_HEIGHT as usize;
const DISPLAY_WIDTH: usize = crate::rendering_context::DISPLAY_WIDTH as usize;
const MEMORY_SIZE: usize = 4096;

pub struct Chip8 {
    context: SDLRenderingContext,

    program_counter: u16, // Points to a RAM address

    stack: [u16; 16],
    stack_pointer: u8,

    registers: [u8; 16],
    v_i: u16,
    memory: [u8; MEMORY_SIZE],
    display: [[bool; DISPLAY_HEIGHT]; DISPLAY_WIDTH],

    main_timer: MainTimer,
    delay_timer: DelayTimer,
}

impl Chip8 {
    pub fn new(path: &str, context: SDLRenderingContext) -> Chip8 {
        return Chip8 {
            program_counter: PROGRAM_START,
            stack: [0; 16],
            stack_pointer: 0,
            registers: [0; 16],
            v_i: 0,
            memory: Chip8::load_memory(path),
            display: [[false; DISPLAY_HEIGHT]; DISPLAY_WIDTH],
            context: context,
            main_timer: MainTimer::new(360),
            delay_timer: DelayTimer::new(),
        };
    }

    pub fn run(&mut self) {
        self.main_timer.reset();

        'main: loop {
            self.delay_timer.update();

            if self.context.run() == false {
                break 'main;
            }

            if self.do_command() == false {
                break 'main;
            }
            self.print();

            self.main_timer.wait_for_next_tick();
        }
    }

    fn advance_counter(&mut self, count: u16) {
        self.program_counter += count * 2
    }

    fn get_command(&mut self) -> Result<u16, &str> {
        if self.program_counter == (MEMORY_SIZE - 2) as u16 {
            return Err("Program ran to end of memory");
        } else {
            let command_byte1 = self.memory[self.program_counter as usize];
            let command_byte2 = self.memory[self.program_counter as usize + 1];

            self.advance_counter(1);

            return Ok(u16::from_be_bytes([command_byte1, command_byte2]));
        }
    }

    pub fn do_command(&mut self) -> bool {
        let command_result = self.get_command();

        match command_result {
            Ok(command) => {
                println!("Byte: {:#04x}", command);
                let command_family = (command >> 12) & 0x000F;

                match command_family {
                    0x0 => self.do_0_commands(command),
                    0x1 => self.do_1_commands(command),
                    0x2 => self.do_2_commands(command),
                    0x3 => self.do_3_commands(command),
                    0x4 => self.do_4_commands(command),
                    0x5 => self.do_5_commands(command),
                    0x6 => self.do_6_commands(command),
                    0x7 => self.do_7_commands(command),
                    0x8 => self.do_8_commands(command),
                    0x9 => self.do_9_commands(command),
                    0xA => self.do_a_commands(command),
                    0xB => self.do_b_commands(command),
                    0xC => self.do_c_commands(command),
                    0xD => self.do_d_commands(command),
                    0xE => self.do_e_commands(command),
                    0xF => self.do_f_commands(command),
                    _ => self.pass(),
                };

                return true;
            }
            Err(message) => {
                println!("{}", message);
                return false;
            }
        };
    }

    fn pass(&self) {
        println!("Pass")
    }

    fn do_0_commands(&mut self, command: u16) {
        if command == 0x00E0 {
            self.context.clear();
        } else if command == 0x00EE {
            self.program_counter = self.stack[self.stack_pointer as usize];
            self.stack_pointer -= 1;
        }
    }

    fn do_1_commands(&mut self, command: u16) {
        self.program_counter = command & 0x0FFF;
    }

    fn do_2_commands(&mut self, command: u16) {
        self.stack_pointer += 1;
        self.stack[self.stack_pointer as usize] = self.program_counter;
        self.program_counter = command & 0x0FFF;
    }

    fn do_3_commands(&mut self, command: u16) {
        let register = (command >> 8) & 0x000F;

        if self.registers[register as usize] == (command & 0x00FF) as u8 {
            self.advance_counter(1);
        }
    }

    fn do_4_commands(&mut self, command: u16) {
        let register = (command >> 8) & 0x000F;

        if self.registers[register as usize] != (command & 0x00FF) as u8 {
            self.advance_counter(1);
        }
    }

    fn do_5_commands(&mut self, command: u16) {
        let register1 = ((command >> 8) & 0x000F) as usize;
        let register2 = ((command >> 4) & 0x000F) as usize;

        if self.registers[register1] == self.registers[register2] {
            self.advance_counter(1);
        }
    }

    fn do_6_commands(&mut self, command: u16) {
        let register = ((command >> 8) & 0x000F) as usize;
        let value = (command & 0x00FF) as u8;

        self.registers[register] = value;
    }

    fn do_7_commands(&mut self, command: u16) {
        let register = ((command >> 8) & 0x000F) as usize;
        let value = (command & 0x00FF) as u8;

        self.registers[register] = self.registers[register].wrapping_add(value);
    }

    fn do_8_commands(&mut self, command: u16) {
        let register_index1 = ((command >> 8) & 0x000F) as usize;
        let register_index2 = ((command >> 4) & 0x000F) as usize;

        let register2 = self.registers[register_index2];
        let register1 = &mut self.registers[register_index1];

        let operation = command & 0x000F;

        match operation {
            // LD
            0x0 => *register1 = register2,
            // OR
            0x1 => *register1 |= register2,
            // AND
            0x2 => *register1 &= register2,
            // XOR
            0x3 => *register1 ^= register2,
            // ADD with carry
            0x4 => {
                let (result, overflowed) = register1.overflowing_add(register2);
                *register1 = result;
                self.registers[0xF] = overflowed as u8;
            }
            // SUB with borrow
            0x5 => {
                let (result, overflowed) = register1.overflowing_sub(register2);
                *register1 = result;
                self.registers[0xF] = overflowed as u8;
            }
            // Right shift 1 into VF
            0x6 => {
                let significant_bit = *register1 & 0x0001;
                *register1 = *register1 >> 1;
                self.registers[0xF] = significant_bit;
            }
            // Reverse SUB with borrow
            0x7 => {
                let (result, overflowed) = register2.overflowing_sub(*register1);
                *register1 = result;
                self.registers[0xF] = overflowed as u8;
            }
            // Left shift 1 into VF
            0xE => {
                let significant_bit = *register1 & 0x80;
                *register1 = *register1 << 1;
                self.registers[0xF] = significant_bit;
            }
            _ => self.pass(),
        }
    }

    fn do_9_commands(&mut self, command: u16) {
        let register1_index = ((command >> 8) & 0x000F) as usize;
        let register2_index = ((command >> 4) & 0x000F) as usize;

        if self.registers[register1_index] != self.registers[register2_index] {
            self.advance_counter(1);
        }
    }

    fn do_a_commands(&mut self, command: u16) {
        let value = (command & 0x0FFF) as u16;

        self.v_i = value;
    }

    fn do_b_commands(&mut self, command: u16) {
        self.program_counter = (command & 0x0FFF) + self.registers[0] as u16;
    }

    fn do_c_commands(&mut self, command: u16) {
        let register_index = ((command >> 8) & 0x000F) as usize;
        let value = command as u8;
        self.registers[register_index] = value & random::<u8>();
    }

    fn do_d_commands(&mut self, command: u16) {
        let x = ((command >> 8) & 0x000F) as usize;
        let y = ((command >> 4) & 0x000F) as usize;

        let sprite_location = self.v_i as usize;
        let sprite_length = (command & 0x000F) as usize;

        let sprite = self.sprite_from_memory(sprite_location, sprite_length);

        self.apply_sprite(
            self.registers[x] as usize,
            self.registers[y] as usize,
            sprite,
        );
    }

    fn do_e_commands(&mut self, command: u16) {
        let operation = command & 0x00FF;

        let register_index = ((command >> 8) & 0x000F) as usize;

        let key = self.registers[register_index];

        match operation {
            0x9E => {
                if self.context.is_key_pressed(key) {
                    self.advance_counter(1);
                }
            }
            0xA1 => {
                if !self.context.is_key_pressed(key) {
                    self.advance_counter(1)
                }
            }
            _ => self.pass(),
        }
    }

    fn do_f_commands(&mut self, command: u16) {
        let register_index = (command >> 8) & 0x000F;
        let register = &mut self.registers[register_index as usize];
        let operation = command & 0x00FF;

        match operation {
            0x07 => {
                *register = self.delay_timer.get();
            }
            0x0A => self.pass(), // wait for key press
            0x15 => self.delay_timer.set(*register),
            0x18 => self.pass(), // sound timer
            0x1E => self.v_i = self.v_i.wrapping_add(*register as u16),
            // Only use the last nibble of the register
            0x29 => self.v_i = ((*register & 0x000F) * 5) as u16,
            0x33 => {
                let hundreds_digit = *register / 100;
                let tens_digit = (*register - (hundreds_digit * 100)) / 10;
                let ones_digit = *register - (hundreds_digit * 100) - (tens_digit * 10);

                self.memory[self.v_i as usize] = hundreds_digit;
                self.memory[(self.v_i + 1) as usize % MEMORY_SIZE] = tens_digit;
                self.memory[(self.v_i + 2) as usize % MEMORY_SIZE] = ones_digit;
            }
            0x55 => {
                for register_i in 0..(register_index + 1) {
                    self.memory[(self.v_i + register_i) as usize % MEMORY_SIZE] =
                        self.registers[register_i as usize]
                }
            }
            0x65 => {
                for register_i in 0..(register_index + 1) {
                    self.registers[register_i as usize] =
                        self.memory[(self.v_i + register_i) as usize % MEMORY_SIZE]
                }
            }
            _ => self.pass(),
        }
    }

    fn sprite_from_memory(&mut self, location: usize, length: usize) -> Sprite {
        let start = location;
        let end = start + length;

        return self.memory[start..end].to_vec();
    }

    fn apply_sprite(&mut self, x: usize, y: usize, sprite: Sprite) {
        for row in 0..sprite.len() {
            for bit in 0..8 as usize {
                let adjusted_x = (x + bit) % DISPLAY_WIDTH;
                let adjusted_y = (y + row) % DISPLAY_HEIGHT;
                self.display[adjusted_x][adjusted_y] ^= (sprite[row] & (0x1 << (7 - bit))) != 0
            }
        }

        self.context.redraw(&self.display)
    }

    pub fn print(&self) {
        println!("Program Counter: {:x}", self.program_counter);
    }

    fn load_memory(path: &str) -> [u8; MEMORY_SIZE] {
        println! {"Loading program {}", path}
        let mut memory: [u8; MEMORY_SIZE] = [0; MEMORY_SIZE];

        const TEXT_SPRITES: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80,
            0xF0, 0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0,
            0x10, 0xF0, 0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90,
            0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0, 0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0,
            0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0,
            0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
        ];

        for i in 0..80 {
            memory[i] = TEXT_SPRITES[i]
        }

        let program = std::fs::read(path).unwrap();

        for i in 0..(program.len()) {
            memory[i + PROGRAM_START as usize] = program[i];
        }

        return memory;
    }
}
