use std::path::Path;

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
    pub model: CartrigeModels,
    pub rom_size: RomSize,
    pub ram_size: RamSize,
}

impl Cartridge {
    pub fn load(cartridge: String) -> Cartridge {
        let header_space = [0_u8; 0x150];

        Cartridge {
            title: String::new(),
            model: CartrigeModels::RomOnly,
            ram_size: RamSize::None,
            rom_size: RomSize::KBytes32NoBanking,
        }
    }
}
