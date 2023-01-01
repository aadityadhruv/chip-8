use sdl2::keyboard::Keycode;
use std::fs::File;
use std::io::Read;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

pub const WIDTH : u32 = 32;
pub const HEIGHT : u32 = 64;
pub const SCALE : u32 = 20;

//length for the array of display "pixels" where pixels are the rectangles being rendered
const DISPLAY_LENGTH : usize = (WIDTH * HEIGHT) as usize;


//Basic struct to represent the Chip-8 interpreter structure
pub struct Chip {
    mem : [u8; 4096], //4kb of memory
    registers: [u8; 16], //16 8-bit registers
    index: u16, //16-bit index pointer
    pc: u16, //Program counter
    stack: [u16; 16], //16 level stack
    sp: u8, //stack pointer
    delay_timer: u8, //delay timer 
    sound_timer: u8, //sound timer
    keys : [Keycode; 16], //mapping keys to chip-8 input keys
    fonts : [u8; 80], //all the 16 chars that can be rendered 16 * 5
    display:  [u16; DISPLAY_LENGTH as usize], //the display array 
    instr: u16,                                //current exec instr
}

impl Chip {
    //return a new Chip, with memory 0-initialized
    pub fn new() -> Self {
        Chip {
            mem : [0; 4096],
            registers: [0; 16],
            index: 0,
            pc: 0,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keys: [Keycode::Num1, Keycode::Num2, Keycode::Num3, Keycode::Num4, Keycode::Q, Keycode::W, Keycode::E, Keycode::R, Keycode::A, Keycode::S, Keycode::D, Keycode::F, Keycode::Z, Keycode::X, Keycode::C, Keycode::V],
            fonts: [
                0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
                0x20, 0x60, 0x20, 0x20, 0x70, // 1
                0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
                0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
                0x90, 0x90, 0xF0, 0x10, 0x10, // 4
                0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
                0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
                0xF0, 0x10, 0x20, 0x40, 0x40, // 7
                0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
                0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
                0xF0, 0x90, 0xF0, 0x90, 0x90, // A
                0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
                0xF0, 0x80, 0x80, 0x80, 0xF0, // C
                0xE0, 0x90, 0x90, 0x90, 0xE0, // D
                0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
                0xF0, 0x80, 0xF0, 0x80, 0x80  // F
            ],
            display: [0; DISPLAY_LENGTH],
            instr: 0x0000,
        }
    }
    //initialize memory with all the starting values
    pub fn init(&mut self) {
        for i in 0..80 {
            self.mem[0x50 + i] = self.fonts[i];
        }
    }

    pub fn read_rom(&mut self, rom : &str) {
        let mut buf = Vec::new();
        let mut rom = File::open(rom).unwrap_or_else(|_err| panic!("Valid ROM needed"));

        rom.read_to_end(&mut buf).unwrap_or_else(|_err| panic!("Error reading ROM"));

        let mut start = 0x200;
        for i in buf {
            self.mem[start] = i;
            start += 1;
        }

    }

    pub fn fetch(&mut self) {
        self.instr = ((self.mem[self.pc as usize] as u16) << 8) | self.mem[(self.pc + 1) as usize] as u16;
        self.pc += 2;
    }
    
    pub fn execute(&mut self) {
        let v = self.instr & 0xF000;
        let x = self.instr & 0x0F00;
        let y = self.instr & 0x00F0;
        let n = self.instr & 0x000F;

        match self.instr {

            0x00E0 => {},
            0x1 => {},
            0x00E0 => {},
            0x00E0 => {},
            0x00E0 => {},
            _ => {}
        }

    }

    pub fn CLS_00E0(&mut self) {
        
    }

    //render the current grid of pixels accounting for scale
    pub fn render(&mut self, canvas : &mut sdl2::render::WindowCanvas) {
        for idx in 0..DISPLAY_LENGTH {
            let (mut x_coord, mut y_coord) : (i32, i32) =((idx as i32 % WIDTH as i32), (idx as i32 / WIDTH as i32));
            x_coord *= SCALE as i32;
            y_coord *= SCALE as i32;
            let rect = Rect::new(x_coord, y_coord, SCALE - 1, SCALE - 1);
            let color = match self.display[idx] {
                    0 => Color::RGB(0, 0, 0),
                    1 => Color::RGB(255, 255, 255),
                    _ => Color::RGB(0, 0, 0)
            };
            canvas.set_draw_color(color);
            canvas.fill_rect(rect);
        }
    }
}
