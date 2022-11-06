mod gameboy;

use gameboy::GameBoy;
extern crate argparse;
use argparse::{ArgumentParser, Store};

fn main() {
    let mut rom_file: String = String::new();
    {
        // For debugging only

        std::env::set_var("RUST_BACKTRACE", "1");

        // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Yet another GB Emulator in Rust");
        ap.refer(&mut rom_file)
            .add_argument("rom_file", Store, "File path to ROM File");
        ap.parse_args_or_exit();
    }
    let mut gb = GameBoy::new();
    gb.start(rom_file);
}
