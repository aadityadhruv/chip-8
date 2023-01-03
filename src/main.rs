extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use chippy::Chip;
use std::env;


pub fn main() {
    //new chip struct
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 { panic!("Incorrect format; please pass a single rom file as a parameter"); }
    let mut chip = Chip::new();
    chip.init();
    chip.read_rom(args[1].as_str());


    //SDL initalizationa and window creation
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("CHIP-8", chippy::WIDTH * chippy::SCALE, chippy::HEIGHT * chippy::SCALE)
        .position_centered()
        .build()
        .unwrap();

    //Canvas to interact with
    let mut canvas = window.into_canvas().build().unwrap();

    //Keyboard input handler
    let mut event_pump = sdl_context.event_pump().unwrap();
    //Main loop called 'running', checks for keyboard input and renders the display grid
    'running: loop {
        chip.clear_input();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode : Some(key), .. } => {
                    chip.feed_input(key);
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        //Draw black bg
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        //clear the screen with black bg
        canvas.clear();
        //choose white color 
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        //render all the rectangles as white pixels on the canvas
        chip.decrement_delay_timer();
        chip.fetch();
        chip.execute();
        chip.render(&mut canvas);
        //display canvas
        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
