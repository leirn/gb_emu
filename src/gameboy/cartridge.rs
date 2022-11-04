use std::fs::File;
use std::io::BufReader;
use std::io::Read;

const ROM_BANK_SIZE: usize = 0x4000;
const RAM_BANK_SIZE: usize = 0x4000;

const TITLE_START_ADDRESS: usize = 0x0134;
const TITLE_END_ADDRESS: usize = 0x0143;

const TYPE_ADDRESS: usize = 0x0147;
const ROM_SIZE_ADDRESS: usize = 0x0148;
const RAM_SIZE_ADDRESS: usize = 0x0149;

pub enum CartrigeModels {
    RomOnly,
    MBC1,
    MBC1Ram,
    MBC1RamBattery,
    MBC2,
    MBC2Battery,
    RomRam,
    RomRamBattery,
}

pub enum RomSize {
    KBytes32NoBanking,
    KBytes64Bank4,
    KBytes128Bank8,
    KBytes256Bank16,
    KBytes512Bank32,
    MBytes1Bank64,  // only 63 banks used by MBC1
    MBytes2Bank128, // only 127 banks used by MBC1
    MBytes4Bank256,
    MBytes8Bank512,
    MBytes1_1Bank72,
    MBytes1_2Bank80,
    MBytes1_5Bank96,
}

pub enum RamSize {
    None,
    KBytes2,
    Kbytes8,
    KBytes32Banks4,
    KBytes64Banks8,
    KBytes128Banks16,
}

pub struct Cartridge {
    pub title: String,
    model: CartrigeModels,
    rom_size: RomSize,
    ram_size: RamSize,
    bank_0: [u8; ROM_BANK_SIZE],
    bank_n: Vec<[u8; ROM_BANK_SIZE]>,
    ram_n: Vec<[u8; RAM_BANK_SIZE]>,
    active_bank: usize,
    active_ram: usize,
}

impl Cartridge {
    pub fn new() -> Cartridge {
        Cartridge {
            title: String::new(),
            model: CartrigeModels::RomOnly,
            ram_size: RamSize::None,
            rom_size: RomSize::KBytes32NoBanking,
            active_bank: 0,
            active_ram: 0,

            bank_0: [(); ROM_BANK_SIZE].map(|_| 0),
            bank_n: vec![],
            ram_n: vec![],
        }
    }

    pub fn load(cartridge: String) -> Cartridge {
        let mut buffer = vec![];

        let file = File::open(cartridge).unwrap();
        let mut buf_reader = BufReader::new(file);
        buf_reader
            .by_ref()
            .take(ROM_BANK_SIZE as u64)
            .read_to_end(&mut buffer)
            .expect("File too short, check your file for error");

        let mut c = Cartridge::new();

        let s = buffer[TITLE_START_ADDRESS..=TITLE_END_ADDRESS].to_vec();
        c.title = String::from_utf8_lossy(&s).to_string();

        println!("{}", c.title);

        for i in 0..ROM_BANK_SIZE {
            c.bank_0[i] = buffer[i];
        }

        c.ram_size = match buffer[RAM_SIZE_ADDRESS] {
            0x00 => RamSize::None,
            0x01 => RamSize::KBytes2,
            0x02 => RamSize::Kbytes8,
            0x03 => RamSize::KBytes32Banks4,
            0x04 => RamSize::KBytes64Banks8,
            0x05 => RamSize::KBytes128Banks16,
            _ => panic!("Unsupported RAM Size type"),
        };

        c.rom_size = match buffer[ROM_SIZE_ADDRESS] {
            0x00 => RomSize::KBytes32NoBanking,
            0x01 => RomSize::KBytes64Bank4,
            0x02 => RomSize::KBytes128Bank8,
            0x03 => RomSize::KBytes256Bank16,
            0x04 => RomSize::KBytes512Bank32,
            0x05 => RomSize::MBytes1Bank64,
            0x06 => RomSize::MBytes2Bank128,
            0x07 => RomSize::MBytes4Bank256,
            0x08 => RomSize::MBytes8Bank512,
            0x52 => RomSize::MBytes1_1Bank72,
            0x53 => RomSize::MBytes1_2Bank80,
            0x54 => RomSize::MBytes1_5Bank96,
            _ => panic!("Unsupported ROM Size type"),
        };

        c.model = match buffer[TYPE_ADDRESS] {
            0x00 => CartrigeModels::RomOnly,
            0x01 => CartrigeModels::MBC1,
            0x02 => CartrigeModels::MBC1Ram,
            0x03 => CartrigeModels::MBC1RamBattery,
            0x05 => CartrigeModels::MBC2,
            0x06 => CartrigeModels::MBC2Battery,
            _ => panic!("Unsupported Cartridge type"),
        };

        match c.rom_size {
            RomSize::KBytes32NoBanking => {
                let mut bank = [(); ROM_BANK_SIZE].map(|_| 0);

                buf_reader
                    .by_ref()
                    .take(ROM_BANK_SIZE as u64)
                    .read_to_end(&mut buffer)
                    .expect("File too short, check your file for error");

                for i in 0..ROM_BANK_SIZE {
                    bank[i] = buffer[i];
                }
                c.bank_n.push(bank.clone());
            }
            RomSize::KBytes64Bank4 => {
                let mut bank = [(); ROM_BANK_SIZE].map(|_| 0);
                for _ in 0..=2 {
                    buf_reader
                        .by_ref()
                        .take(ROM_BANK_SIZE as u64)
                        .read_to_end(&mut buffer)
                        .expect("File too short, check your file for error");

                    for i in 0..ROM_BANK_SIZE {
                        bank[i] = buffer[i];
                    }
                    c.bank_n.push(bank.clone());
                }
            }
            RomSize::KBytes128Bank8 => {
                let mut bank = [(); ROM_BANK_SIZE].map(|_| 0);
                for _ in 0..=6 {
                    buf_reader
                        .by_ref()
                        .take(ROM_BANK_SIZE as u64)
                        .read_to_end(&mut buffer)
                        .expect("File too short, check your file for error");

                    for i in 0..ROM_BANK_SIZE {
                        bank[i] = buffer[i];
                    }
                    c.bank_n.push(bank.clone());
                }
            }
            RomSize::KBytes256Bank16 => {
                let mut bank = [(); ROM_BANK_SIZE].map(|_| 0);
                for _ in 0..=14 {
                    buf_reader
                        .by_ref()
                        .take(ROM_BANK_SIZE as u64)
                        .read_to_end(&mut buffer)
                        .expect("File too short, check your file for error");

                    for i in 0..ROM_BANK_SIZE {
                        bank[i] = buffer[i];
                    }
                    c.bank_n.push(bank.clone());
                }
            }
            RomSize::KBytes512Bank32 => {
                let mut bank = [(); ROM_BANK_SIZE].map(|_| 0);
                for _ in 0..=30 {
                    buf_reader
                        .by_ref()
                        .take(ROM_BANK_SIZE as u64)
                        .read_to_end(&mut buffer)
                        .expect("File too short, check your file for error");

                    for i in 0..ROM_BANK_SIZE {
                        bank[i] = buffer[i];
                    }
                    c.bank_n.push(bank.clone());
                }
            }
            RomSize::MBytes1Bank64 => {
                let mut bank = [(); ROM_BANK_SIZE].map(|_| 0);
                for _ in 0..=62 {
                    buf_reader
                        .by_ref()
                        .take(ROM_BANK_SIZE as u64)
                        .read_to_end(&mut buffer)
                        .expect("File too short, check your file for error");

                    for i in 0..ROM_BANK_SIZE {
                        bank[i] = buffer[i];
                    }
                    c.bank_n.push(bank.clone());
                }
            }
            RomSize::MBytes2Bank128 => {
                let mut bank = [(); ROM_BANK_SIZE].map(|_| 0);
                for _ in 0..=126 {
                    buf_reader
                        .by_ref()
                        .take(ROM_BANK_SIZE as u64)
                        .read_to_end(&mut buffer)
                        .expect("File too short, check your file for error");

                    for i in 0..ROM_BANK_SIZE {
                        bank[i] = buffer[i];
                    }
                    c.bank_n.push(bank.clone());
                }
            }
            RomSize::MBytes4Bank256 => {
                let mut bank = [(); ROM_BANK_SIZE].map(|_| 0);
                for _ in 0..=254 {
                    buf_reader
                        .by_ref()
                        .take(ROM_BANK_SIZE as u64)
                        .read_to_end(&mut buffer)
                        .expect("File too short, check your file for error");

                    for i in 0..ROM_BANK_SIZE {
                        bank[i] = buffer[i];
                    }
                    c.bank_n.push(bank.clone());
                }
            }
            RomSize::MBytes8Bank512 => {
                let mut bank = [(); ROM_BANK_SIZE].map(|_| 0);
                for _ in 0..=510 {
                    buf_reader
                        .by_ref()
                        .take(ROM_BANK_SIZE as u64)
                        .read_to_end(&mut buffer)
                        .expect("File too short, check your file for error");

                    for i in 0..ROM_BANK_SIZE {
                        bank[i] = buffer[i];
                    }
                    c.bank_n.push(bank.clone());
                }
            }
            _ => panic!("Unsupported rom size"),
        }

        c
    }

    pub fn read_bank0(&self, address: usize) -> u8 {
        self.bank_0[address]
    }
    pub fn write_bank0(&mut self, address: usize, value: u8) {
        // Switch banks
    }

    pub fn read_active_bank(&self, address: usize) -> u8 {
        self.bank_n[self.active_bank - 1][address]
    }
    pub fn write_active_bank(&mut self, address: usize, value: u8) {
        // ?
    }

    pub fn read_active_ram(&self, address: usize) -> u8 {
        self.ram_n[self.active_ram][address]
    }

    pub fn write_active_ram(&mut self, address: usize, value: u8) {
        self.ram_n[self.active_ram][address] = value;
    }
}
