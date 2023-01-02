use sdl2::keyboard::Keycode;
use rand::prelude::*; 
use std::fs::File;
use std::io::Read;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

pub const WIDTH : u32 = 65;
pub const HEIGHT : u32 = 33;
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
    sp: usize, //stack pointer
    delay_timer: u8, //delay timer 
    sound_timer: u8, //sound timer
    keys : [Keycode; 16], //mapping keys to chip-8 input keys
    keys_pressed : i8, //mapping keys to chip-8 input keys
    fonts : [u8; 80], //all the 16 chars that can be rendered 16 * 5
    display:  [u16; DISPLAY_LENGTH as usize], //the display array 
    instr: u16,                                //current exec instr
    v: u8, 
    x: usize,
    y: usize,
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
            keys_pressed : -1,
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

    pub fn clear_input(&mut self) {
        self.keys_pressed = -1;
    }
    pub fn feed_input(&mut self, key : Keycode) {
        let idx = self.keys.into_iter().position(|r| r == key);
        let m_idx = match idx {
            Some(v) => v as i8,
            None =>  -1 as i8,
        };
        self.keys_pressed = m_idx;
    }

    pub fn fetch(&mut self) {
        self.instr = ((self.mem[self.pc as usize] as u16) << 8) | self.mem[(self.pc + 1) as usize] as u16;
        self.pc += 2;
        //println!("Fetching next instruction: {:#06X}", self.instr);
    }
    
    pub fn execute(&mut self) {
        self.v = ((self.instr & 0xF000) >> 12) as u8;
        self.x = ((self.instr & 0x0F00) >> 8) as usize;
        self.y = ((self.instr & 0x00F0) >> 4) as usize;
        self.n = (self.instr & 0x000F) as u8;
        self.nn = (self.instr & 0x00FF) as u8;
        self.nnn = self.instr & 0x0FFF;

        match self.instr {

            0x00e0 => { self.cls_00e0() },
            0x00ee => { self.ret_00ee() },
            0x1000..=0x1fff => { self.jmp_1nnn() },
            0x2000..=0x2fff => { self.call_2nnn() },
            0x3000..=0x3fff => { self.se_3xkk() }
            0x4000..=0x4fff => { self.sne_4xkk() }
            0x5000..=0x5ff0 => { self.se_5xy0() }
            0x6000..=0x6fff => { self.ld_6xkk() },
            0x7000..=0x7fff => { self.add_7xkk() },
            0x8000..=0x8ffe => { self.match_8xxk() }
            0x9000..=0x9ff0 => { self.sne_9xy0() }
            0xa000..=0xafff => { self.ld_annn() },
            0xb000..=0xbfff => { self.jp_bnnn() }
            0xc000..=0xcfff => { self.rnd_cxnn() }
            0xd000..=0xdfff => { self.drw_dxyn() },
            0xe000..=0xefa1 => { self.match_exxk() }
            0xf000..=0xff65 => { self.match_fxxk() }
            _ => { println!("Doing nothing!"); }
        }

    }
//            0x8000..=0x8ff0 => { self.ld_8xy0() }
//            0x8001..=0x8ff1 => { self.or_8xy1() }
//            0x8002..=0x8ff2 => { self.and_8xy2() }
//            0x8003..=0x8ff3 => { self.xor_8xy3() }
//            0x8004..=0x8ff4 => { self.add_8xy4() }
//            0x8005..=0x8ff5 => { self.sub_8xy5() }
//            0x8006..=0x8ff6 => { self.shr_8xy6() }
//            0x8007..=0x8ff7 => { self.sub_8xy7() }
//            0x800e..=0x8ffe => { self.shl_8xye() }

    fn match_fxxk(&mut self) {
        match self.instr << 8 >> 8 {
            0x0a => { self.ld_fx0a() }
            0x07 => { self.ld_fx07() }
            0x15 => { self.ld_fx15() }
            0x18 => { self.ld_fx18() }
            0x1e => { self.add_fx1e() }
            0x29 => { self.ld_fx29() }
            0x33 => { self.ld_fx33() }
            0x55 => { self.ld_fx55() }
            0x65 => { self.ld_fx65() }
            _ => {}
        }
    }

    fn match_exxk(&mut self) {
        match self.instr << 8 >> 8 {
            0x9e => { self.skp_ex9e() }
            0xa1 => { self.sknp_exa1() }
            _ => {}
        }

    }
    fn match_8xxk(&mut self) {
        match self.instr << 12 >> 12 {
            0x0 => { self.ld_8xy0() }
            0x1 => { self.or_8xy1() }
            0x2 => { self.and_8xy2() }
            0x3 => { self.xor_8xy3() }
            0x4 => { self.add_8xy4() }
            0x5 => { self.sub_8xy5() }
            0x6 => { self.shr_8xy6() }
            0x7 => { self.sub_8xy7() }
            0xe => { self.shl_8xye() }
            _ => {}
        }
    }

    fn ret_00ee(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }
    fn call_2nnn(&mut self) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = self.nnn;
    }
    fn se_3xkk(&mut self) {
        if self.registers[self.x] == self.nn {
            self.pc += 2;
        }
    }
    fn sne_4xkk(&mut self) {
        if self.registers[self.x] != self.nn {
            self.pc += 2;
        }
    }
    fn se_5xy0(&mut self) {
            if self.registers[self.x] == self.registers[self.y] {
                self.pc += 2;
            }
    }
    fn sne_9xy0(&mut self) {
            if self.registers[self.x] != self.registers[self.y] {
                self.pc += 2;
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
        self.registers[self.x] = self.nn as u8;
    }
    //add value to register vx
    fn add_7xkk(&mut self) {
        self.registers[self.x] = self.registers[self.x].wrapping_add(self.nn as u8);
    }
    fn ld_8xy0(&mut self) {
        self.registers[self.x] = self.registers[self.y];
    }
    fn or_8xy1(&mut self) {
        self.registers[self.x] = self.registers[self.x] | self.registers[self.y];
    }
    fn and_8xy2(&mut self) {
        self.registers[self.x] = self.registers[self.x] & self.registers[self.y];
    }
    fn xor_8xy3(&mut self) {
        self.registers[self.x] = self.registers[self.x] ^ self.registers[self.y];
    }
    fn add_8xy4(&mut self) {
        println!("Checking add");
        self.registers[self.x] = self.registers[self.x].wrapping_add(self.registers[self.y]);
        match self.registers[self.x].checked_add(self.registers[self.y]) {
            Some(_) => { self.registers[0xF] = 0; }
            None => { self.registers[0xF] = 1; }
        }
    }
    fn sub_8xy5(&mut self) {
        println!("Checking sub1");
        self.registers[self.x] = self.registers[self.x].wrapping_sub(self.registers[self.y]);
        match self.registers[self.x].checked_sub(self.registers[self.y]) {
            Some(_) => { self.registers[0xF] = 1; }
            None => { self.registers[0xF] = 0; }
        }
    }
    fn sub_8xy7(&mut self) {
        println!("Checking sub2");
        self.registers[self.x] = self.registers[self.y].wrapping_sub(self.registers[self.x]);
        match self.registers[self.y].checked_sub(self.registers[self.x]) {
            Some(_) => { self.registers[0xF] = 1; }
            None => { self.registers[0xF] = 0; }
        }
    }
    fn shr_8xy6(&mut self) {
        self.registers[self.x] = self.registers[self.y];
        self.registers[0xF] = self.registers[self.x] << 7 & 1;
        self.registers[self.x] >>= 1;
    }
    fn shl_8xye(&mut self) {
        self.registers[self.x] = self.registers[self.y];
        self.registers[0xF] = self.registers[self.x] >> 7 & 1;
        self.registers[self.x] <<= 1;
    }
    //set index register
    fn ld_annn(&mut self) {
        self.index = self.nnn;
    }
    fn jp_bnnn(&mut self) {
        self.pc = self.nnn + self.registers[0x0] as u16;
    }
    fn rnd_cxnn(&mut self) {
        let x : u8 = rand::thread_rng().gen();
        self.registers[self.x] = x | self.n;
    } 
    //draw logic 
    fn drw_dxyn(&mut self) {
        let x = self.registers[self.x] as u32 % WIDTH; //x-coord
        let y = self.registers[self.y] as u32 % HEIGHT; //y-coord
        //for every row 
        for i in 0..self.n  {
            //for every bit (column)
            for j in 0..8 {
                //get idx in display
                if (y + i as u32) < HEIGHT && (x + j) < WIDTH {
                let idx = (((y + i as u32) * WIDTH) + x+j) as usize;
                //XOR the bit
                self.display[idx] ^= self.mem[(self.index + i as u16) as usize] as u16 >> (7 - j) & 1; //7 - j for reverse bit shifting
                }
            }
        }
    }
    fn skp_ex9e(&mut self) {
        println!("Same key");
        if self.registers[self.x] as i8 == self.keys_pressed {
            self.pc += 2;
        }
    }
    fn sknp_exa1(&mut self) {
        if self.registers[self.x] as i8 != self.keys_pressed {
            self.pc += 2;
        }
    }
    pub fn decrement_delay_timer(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }
    fn ld_fx07(&mut self) {
        self.registers[self.x] = self.delay_timer;
    }
    fn ld_fx15(&mut self) {
        self.delay_timer = self.registers[self.x];
    }
    fn ld_fx18(&mut self) {
        self.sound_timer = self.registers[self.x];
    }
    fn add_fx1e(&mut self) {
        //TODO: Maybe check for overflow
        self.index += self.registers[self.x] as u16;
    }
    fn ld_fx0a(&mut self) {
        if self.keys_pressed == -1 {
            self.pc -= 2;
        }
        else {
            self.registers[self.x] = self.keys_pressed as u8;
        }
    }
    fn ld_fx29(&mut self) {
        self.index = 0x50 + self.registers[self.x] as u16 * 5;
    }
    fn ld_fx33(&mut self) {
        let r = self.registers[self.x] as u8;
        self.mem[self.index as usize] = r / 100;
        self.mem[self.index as usize + 1] = r % 100 / 10;
        self.mem[self.index as usize + 2] = r % 10;
    }
    fn ld_fx55(&mut self) {
        for i in 0..=self.x  {
            self.mem[self.index as usize + i] = self.registers[i];
        }
    }
    fn ld_fx65(&mut self) {
        for i in 0..=self.x {
             self.registers[i] = self.mem[self.index as usize + i];
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
            let rect = Rect::new(x_coord, y_coord, SCALE, SCALE);
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
