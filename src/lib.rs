//Basic struct to represent the Chip-8 interpreter structure
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
pub const WIDTH : u32 = 32;
pub const HEIGHT : u32 = 64;
pub const SCALE : u32 = 20;

const DISPLAY_LENGTH : usize = (WIDTH * HEIGHT) as usize;


pub struct Chip {
    mem : [u8; 4096],
    registers: [u8; 16],
    index: u16,
    pc: u16, 
    stack: [u16; 16],
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    keys : [Keycode; 16],
    fonts : [u32; 80],
    display:  [Rect; DISPLAY_LENGTH as usize]
}

impl Chip {
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
            display: [Rect::new(0, 0, SCALE - 1, SCALE - 1); DISPLAY_LENGTH],
        }
    }
    pub fn init(mut self) {

    }

    pub fn render(& mut self) -> &[Rect] {
        for idx in 0..DISPLAY_LENGTH {
            let (mut x_coord, mut y_coord) : (i32, i32) =((idx as i32 % WIDTH as i32), (idx as i32 / WIDTH as i32));
            x_coord *= SCALE as i32;
            y_coord *= SCALE as i32;
            self.display[idx].set_x(x_coord);
            self.display[idx].set_y(y_coord);

        }
        &self.display
    }
}
