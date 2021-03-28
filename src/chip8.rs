use sdl2::render::Canvas;
use sdl2::video::Window;

const PROGRAM_START: u16 = 0x200;

pub struct Chip8 {
    program: Vec<u16>,
    program_counter: u16, // Points to a ROM address

    stack: [u16; 16],
    stack_pointer: u8,

    registers: [u16; 16],
    v_i: u16,
    //memory: [u8; 4096],
}

impl Chip8 {
    pub fn new(path: &str) -> Chip8 {
        return Chip8 {
            program: Chip8::load_program(path),
            program_counter: PROGRAM_START, // By convention programs start at 0x200
            stack: [0; 16],
            stack_pointer: 0,
            registers: [0; 16],
            v_i: 0,
            //memory: [0; 4096],
        };
    }

    fn advance_counter(&mut self, count: u16) {
        self.program_counter += count * 2
    }

    fn reset_counter(&mut self) {
        self.program_counter = PROGRAM_START;
    }

    pub fn do_command(&mut self, canvas: &mut Canvas<Window>) {
        let program_index = ((self.program_counter - PROGRAM_START) / 2) as usize;

        let command = self.program[program_index];

        println!("Byte: {:#04x}", command);

        if self.program_counter == (self.program.len() as u16 * 2 + PROGRAM_START) {
            self.reset_counter();
        } else {
            self.advance_counter(1);
        }

        let command_family = (command >> 12) & 0x000F;

        match command_family {
            0x0 => self.do_0_commands(command, canvas),
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
            _ => self.pass(),
        };
    }

    fn pass(&self) {
        println!("Pass")
    }

    fn do_0_commands(&mut self, command: u16, canvas: &mut Canvas<Window>) {
        if command == 0x00E0 {
            canvas.clear();
            canvas.present();
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

        if self.registers[register as usize] == (command & 0x00FF) {
            self.advance_counter(1);
        }
    }

    fn do_4_commands(&mut self, command: u16) {
        let register = (command >> 8) & 0x000F;

        if self.registers[register as usize] != (command & 0x00FF) {
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
        let value = (command & 0x00FF) as u16;

        self.registers[register] = value;
    }

    fn do_7_commands(&mut self, command: u16) {
        let register = ((command >> 8) & 0x000F) as usize;
        let value = (command & 0x00FF) as u16;

        self.registers[register] += value;
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
                self.registers[0xF] = overflowed as u16;
            }
            // SUB with borrow
            0x5 => {
                let (result, overflowed) = register1.overflowing_sub(register2);
                *register1 = result;
                self.registers[0xF] = overflowed as u16;
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
                self.registers[0xF] = overflowed as u16;
            }
            // Left shift 1 into VF
            0xE => {
                let significant_bit = *register1 & 0x8000;
                *register1 = *register1 << 1;
                self.registers[0xF] = significant_bit;
            }
            _ => self.pass(),
        }
    }

    fn do_9_commands(&mut self, command: u16) {
        let register1 = ((command >> 8) & 0x000F) as usize;
        let register2 = ((command >> 4) & 0x000F) as usize;

        if self.registers[register1] == self.registers[register2] {
            self.advance_counter(1);
        }
    }

    fn do_a_commands(&mut self, command: u16) {
        let value = (command & 0x0FFF) as u16;

        self.v_i = value;
    }

    fn do_b_commands(&mut self, command: u16) {
        self.program_counter = (command & 0x0FFF) + self.registers[0];
    }

    pub fn print(&self) {
        println!("Program Counter: {}", self.program_counter);

        for index in 0..16 {
            println!("V{}: {}", index, self.registers[index as usize]);
        }
    }

    fn load_program(path: &str) -> Vec<u16> {
        let program = std::fs::read(path).unwrap();
        return program
            .chunks_exact(2)
            .into_iter()
            .map(|a| u16::from_ne_bytes([a[1], a[0]]))
            .collect();
    }
}
