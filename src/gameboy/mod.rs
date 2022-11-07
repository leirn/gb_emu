mod bus;
mod cartridge;
mod cpu;
mod ppu;

use cpu::Cpu;

use self::cartridge::Cartridge;

pub struct GameBoy {
    cpu: Cpu,
}

impl GameBoy {
    pub fn new() -> GameBoy {
        GameBoy { cpu: Cpu::new() }
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
        let sleep_time = std::time::Duration::from_millis(10);
        loop {
            self.cpu.next();
            self.cpu.bus.ppu.next();
            println!("{}", self.cpu.bus.ppu);
            std::thread::sleep(sleep_time);
        }
    }
}
