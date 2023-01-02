use sdl2::keyboard::Keycode;
use std::fs::File;
use std::io::Read;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

pub const WIDTH : u32 = 64;
pub const HEIGHT : u32 = 32;
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
    v: u8, 
    x: u8,
    y: u8,
    n: u8,
    nn: u8, 
    nnn: u16,
}

impl Chip {
    //return a new Chip, with memory 0-initialized
    pub fn new() -> Self {
        Chip {
            mem : [0; 4096],
            registers: [0; 16],
            index: 0,
            pc: 0x200,
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
            v: 0,
            x: 0,
            y: 0,
            n: 0,
            nn: 0, 
            nnn: 0,
        }
    }
    //initialize memory with all the starting values
    pub fn init(&mut self) {
        for i in 0..80 {
            self.mem[0x50 + i] = self.fonts[i];
        }
        println!("Init Chip: Loaded fonts!");
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
        println!("Loading ROM in memory!");
    }

    pub fn fetch(&mut self) {
        self.instr = ((self.mem[self.pc as usize] as u16) << 8) | self.mem[(self.pc + 1) as usize] as u16;
        self.pc += 2;
        println!("Fetching next instruction: {:#06X}", self.instr);
    }
    
    pub fn execute(&mut self) {
        self.v = ((self.instr & 0xF000) >> 12) as u8;
        self.x = ((self.instr & 0x0F00) >> 8) as u8;
        self.y = ((self.instr & 0x00F0) >> 4) as u8;
        self.n = (self.instr & 0x000F) as u8;
        self.nn = (self.instr & 0x00FF) as u8;
        self.nnn = self.instr & 0x0FFF;

        match self.instr {

            0x00E0 => { self.CLS_00E0() },
            0x1000..=0x1FFF => { self.JMP_1NNN() },
            0x6000..=0x6FFF => { self.LD_6XKK() },
            0x7000..=0x7FFF => { self.ADD_7XKK() },
            0xA000..=0xAFFF => { self.LD_ANNN() },
            0xD000..=0xDFFF => { self.DRW_DXYN() },
            _ => { println!("Doing nothing!"); }
        }

    }

    //Clear screen
    fn cls_00e0(&mut self) {
        self.display = [0; DISPLAY_LENGTH]; 
    }

    //Jump
    fn jmp_1nnn(&mut self) {
        self.pc = self.nnn;
    }

    //set register vx
    fn ld_6xkk(&mut self) {
        self.registers[self.x as usize] = self.nn as u8;
    }

    //add value to register vx
    fn add_7xkk(&mut self) {
        self.registers[self.x as usize] +=  self.nn as u8;
    }

    //set index register
    fn ld_annn(&mut self) {
        self.index = self.nnn;
    }

    //draw logic 
    fn drw_dxyn(&mut self) {
        let x = self.registers[self.x as usize] as u32 % WIDTH; //x-coord
        let y = self.registers[self.y as usize] as u32 % HEIGHT; //y-coord

        //TODO: Cover up for end of bounds
        //for every row 
        for i in 0..self.n  {
            //for every bit (column)
            for j in 0..8 {
                //get idx in display
                let idx = (((y + i as u32) * WIDTH) + x+j) as usize;
                //XOR the bit
                self.display[idx] ^= self.mem[(self.index + i as u16) as usize] as u16 >> (7 - j) & 1; //7 - j for reverse bit shifting
            }
        }
    }

    //render the current grid of pixels accounting for scale
    pub fn render(&mut self, canvas : &mut sdl2::render::WindowCanvas) {
        for idx in 0..DISPLAY_LENGTH {
            let (mut x_coord, mut y_coord) : (i32, i32) =((idx as i32 % WIDTH as i32), (idx as i32 / WIDTH as i32)); //get x and y coord
            //Change scale to specified one
            x_coord *= SCALE as i32;
            y_coord *= SCALE as i32;
            //Draw rectangle as pixel, scale - 1 so border are seen
            let rect = Rect::new(x_coord, y_coord, SCALE - 1, SCALE - 1);
            //Choose color of bit
            let color = match self.display[idx] {
                    0 => Color::RGB(0, 0, 0),
                    _ => Color::RGB(255, 255, 255),
            };
            //Draw into buffer
            canvas.set_draw_color(color);
            canvas.fill_rect(rect).unwrap();
        }
    }
}
