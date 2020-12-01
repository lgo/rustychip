extern crate getopts;
extern crate sdl2;
extern crate time;

fn main() {}

// Note about emulation speed based on
// https://github.com/AfBu/haxe-CHIP-8-emulator/wiki/(Super)CHIP-8-Secrets#speed-of-emulation.
//
// My first attempt was to create thread with loop inside. Stepping one
// instruction by cycle and then sleeping few milliseconds. However this
// approach show itself at not-very-clever since sleep instruction does
// not always gives you accurate enough result and based of target
// platform you can easily reach it's precision limit. So it's much
// better to run our loop at much lower speed (100Hz in my case),
// calculating time between two loop cycles, then based on target
// frequency calculate number of operations that should be performed and
// perform them at once. This will give you much more precise control
// over emulation speed.
