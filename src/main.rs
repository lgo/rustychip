mod rustychip;

use rustychip::cpu::Cpu as RustyChip;

fn main () {
    let chip = RustyChip::new();

    // set up render system and register input callbacks
    setupGraphics();
    setupInput();

    // initialize chip8 system and load game into memory
    chip.loadGame("pong");

    // emulation loop
    loop {
        // emulate one cycle
        chip.emulateCycle();

        // if draw flag is set, update the screen
        if (chip.drawFlag) {
            drawGraphics(chip);
        }

        // store key press state (press and release)
        chip.setKeys();
    }
}

fn setupGraphics () {

}

fn setupInput () {

}

fn drawGraphics (Cpu: RustyChip) {

}
