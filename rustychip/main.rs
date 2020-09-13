extern crate sdl2;
extern crate getopts;
extern crate time;

use getopts::Options;

use sdl2::event::Event;

use std::env;
use std::time::Duration;
use std::thread;

use cpu::Cpu;

mod cpu;
mod opcode;
mod util;
mod display;
mod keypad;
mod loader;

static MS_TO_NS: u64 = 1000000;

fn parse_arguments() -> String {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("d", "debug", "print debug information");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("d") {
        unsafe {
            util::DEBUG_MODE = true;
        }
    }
    if !matches.free.is_empty() {
        return matches.free[0].clone();
    } else {
        return String::from("pong");
    }
}

fn main() {
    let game_name: String = parse_arguments();
    let sdl_context = sdl2::init().unwrap();

    let mut chip = Cpu::new(sdl_context);
    loader::load_game(&mut chip, game_name);
    // sdl::init(&[sdl::InitFlag::Video,
    //             sdl::InitFlag::Audio,
    //             sdl::InitFlag::Timer]);

    let event_pump = sdl_context.event_pump().unwrap();

    'main: loop {
        let start_cycle = time::precise_time_ns();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::KeyDown { keycode, .. } => chip.keypad.press(keycode, true),
                Event::KeyUp { keycode, .. } => chip.keypad.press(keycode, false),
                _ => {}
            }
        }

        chip.emulate_cycle();
        // chip.display.draw_screen();

        let cycle_time = time::precise_time_ns() - start_cycle;
        let wait_time = 500 * MS_TO_NS - cycle_time;
        if wait_time > 0 {
            thread::sleep(Duration::from_millis(wait_time / MS_TO_NS))
        }
    }
}
