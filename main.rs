mod chip8;
use chip8::Chip8 as Chip8;

fn main () {
    let chip8 = Chip8();

    // set up render system and register input callbacks
    setupGraphics();
    setupInput();

    // initialize chip8 system and load game into memory
    chip8.initialize();
    chip8.loadGame("pong");

    // emulation loop
    loop {
        // emulate one cycle
        chip8.emulateCycle();

        // if draw flag is set, update the screen
        if (chip8.drawFlag) {
            drawGraphics();
        }

        // store key press state (press and release)
        chip8.setKeys();
    }
}

fn setupGraphics () {

}

fn setupInput () {

}

fn drawGraphics () {

}
