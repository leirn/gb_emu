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
    running: bool,
    paused: bool,
}

impl GameBoy<'_> {
    pub fn new() -> GameBoy<'static> {
        let _sdl_context = Rc::new(RefCell::new(sdl2::init().unwrap()));
        GameBoy {
            cpu: Cpu::new(_sdl_context.clone()),
            sdl_context: _sdl_context,
            running: true,
            paused: false,
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
        let sleep_time = std::time::Duration::from_millis(1000);

        while self.running {
            // 4 cpu clock tick per ppu clock tick ?
            if !self.paused {
                if !self.cpu.is_halted {
                    self.cpu.bus.timer.tick();
                    self.cpu.next();
                    self.cpu.bus.timer.tick();
                    self.cpu.next();
                    self.cpu.bus.timer.tick();
                    self.cpu.next();
                    self.cpu.bus.timer.tick();
                    self.cpu.next();
                }
                self.cpu.bus.ppu.next();
                //println!("{}", self.cpu.bus.ppu);
                std::thread::sleep(sleep_time);
            }

            self.event_handler();
        }
    }

    fn event_handler(&mut self) {
        let mut event_pump = self.sdl_context.borrow_mut().event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => self.running = false,
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => self.paused = !self.paused,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::LCtrl),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Backspace),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => {
                    let keycode = match event {
                        Event::KeyDown { keycode, .. } => keycode,
                        _ => panic!(""),
                    }
                    .unwrap();
                    self.handle_controller_event_down(keycode);
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::LCtrl),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Backspace),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => {
                    let keycode = match event {
                        Event::KeyUp { keycode, .. } => keycode,
                        _ => panic!(""),
                    }
                    .unwrap();
                    self.handle_controller_event_up(keycode);
                }
                _ => (),
            }
        }
    }

    fn handle_controller_event_down(&mut self, keycode: Keycode) {
        match keycode {
            Keycode::Up => self.cpu.bus.controller.press_up(),
            Keycode::Down => self.cpu.bus.controller.press_down(),
            Keycode::Left => self.cpu.bus.controller.press_left(),
            Keycode::Right => self.cpu.bus.controller.press_right(),
            Keycode::LCtrl => self.cpu.bus.controller.press_a(),
            Keycode::Space => self.cpu.bus.controller.press_b(),
            Keycode::Return => self.cpu.bus.controller.press_start(),
            Keycode::Backspace => self.cpu.bus.controller.press_select(),
            _ => (),
        }
    }

    fn handle_controller_event_up(&mut self, keycode: Keycode) {
        self.cpu.is_halted = false;
        match keycode {
            Keycode::Up => self.cpu.bus.controller.unpress_up(),
            Keycode::Down => self.cpu.bus.controller.unpress_down(),
            Keycode::Left => self.cpu.bus.controller.unpress_left(),
            Keycode::Right => self.cpu.bus.controller.unpress_right(),
            Keycode::LCtrl => self.cpu.bus.controller.unpress_a(),
            Keycode::Space => self.cpu.bus.controller.unpress_b(),
            Keycode::Return => self.cpu.bus.controller.unpress_start(),
            Keycode::Backspace => self.cpu.bus.controller.unpress_select(),
            _ => (),
        }
    }
}
