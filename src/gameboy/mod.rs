mod bus;
mod cartridge;
mod cpu;
mod ppu;

use self::cartridge::Cartridge;
use cpu::Cpu;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct GameBoy<'a> {
    cpu: Cpu<'a>,
    sdl_context: Rc<RefCell<sdl2::Sdl>>,
}

impl GameBoy<'_> {
    pub fn new() -> GameBoy<'static> {
        let _sdl_context = Rc::new(RefCell::new(sdl2::init().unwrap()));
        GameBoy {
            cpu: Cpu::new(_sdl_context.clone()),
            sdl_context: _sdl_context,
        }
    }

    pub fn start(&mut self, rom_file: String) {
        // Loading the cartridge
        let cartridge = Cartridge::load(rom_file);
        println!("{}", cartridge);
        /*
            - Une fois que la console est allumée, le processeur commence à lire à l’adresse 0x00 (emplacement de la ROM de la Game Boy).

            - Le son et la RAM sont initialisés.

            - Le logo Nintendo est copié de la ROM de la cartouche vers la Display RAM pour être ensuite dessiné sur la partie haute de
              l’écran. S’il n’y a pas de cartouche insérée, le logo contiendra des tuiles corrompues. La même chose peut se produire si
              elle est mal insérée.

            - Le logo défile vers le bas et le fameux son po-ling est joué.

            - Le logo Nintendo du jeu est comparé à celui stocké dans la ROM de la console ; si la vérification échoue, la console se bloque.

            - Une rapide somme de contrôle est effectuée sur l’en-tête de la ROM de la cartouche pour s’assurer que celle-ci est correctement
              insérée ; si la vérification échoue, la console se bloque.

            - La ROM de la console est effacée de la mémoire (memory map).

            - Le processeur commence à exécuter le jeu.
        */

        self.cpu.start(cartridge);

        self.cpu.bus.load_boot_rom();
        self.cpu.registers.pc = 0x0000;
        //let sleep_time = std::time::Duration::from_millis(100);

        let mut running: bool = true;
        let mut paused: bool = false;

        while running {
            // 4 cpu clock tick per ppu clock tick ?
            if !paused {
                self.cpu.next();
                self.cpu.next();
                self.cpu.next();
                self.cpu.next();
                self.cpu.bus.ppu.next();
                //println!("{}", self.cpu.bus.ppu);
                //std::thread::sleep(sleep_time);
            }
            let mut event_pump = self.sdl_context.borrow_mut().event_pump().unwrap();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Q),
                        ..
                    } => running = false,
                    Event::KeyDown {
                        keycode: Some(Keycode::P),
                        ..
                    } => paused = !paused,
                    _ => (),
                }
            }
        }
    }
}
