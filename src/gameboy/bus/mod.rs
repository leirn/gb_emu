const BOOT_SEQUENCE_SIZE: usize = 256;
const BOOT_SEQUENCE: [u8; BOOT_SEQUENCE_SIZE] = [
    0x31, 0xfe, 0xff, 0xaf, 0x21, 0xff, 0x9f, 0x32, 0xcb, 0x7c, 0x20, 0xfb, 0x21, 0x26, 0xff, 0x0e,
    0x11, 0x3e, 0x80, 0x32, 0xe2, 0x0c, 0x3e, 0xf3, 0xe2, 0x32, 0x3e, 0x77, 0x77, 0x3e, 0xfc, 0xe0,
    0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1a, 0xcd, 0x95, 0x00, 0xcd, 0x96, 0x00, 0x13, 0x7b,
    0xfe, 0x34, 0x20, 0xf3, 0x11, 0xd8, 0x00, 0x06, 0x08, 0x1a, 0x13, 0x22, 0x23, 0x05, 0x20, 0xf9,
    0x3e, 0x19, 0xea, 0x10, 0x99, 0x21, 0x2f, 0x99, 0x0e, 0x0c, 0x3d, 0x28, 0x08, 0x32, 0x0d, 0x20,
    0xf9, 0x2e, 0x0f, 0x18, 0xf3, 0x67, 0x3e, 0x64, 0x57, 0xe0, 0x42, 0x3e, 0x91, 0xe0, 0x40, 0x04,
    0x1e, 0x02, 0x0e, 0x0c, 0xf0, 0x44, 0xfe, 0x90, 0x20, 0xfa, 0x0d, 0x20, 0xf7, 0x1d, 0x20, 0xf2,
    0x0e, 0x13, 0x24, 0x7c, 0x1e, 0x83, 0xfe, 0x62, 0x28, 0x06, 0x1e, 0xc1, 0xfe, 0x64, 0x20, 0x06,
    0x7b, 0xe2, 0x0c, 0x3e, 0x87, 0xe2, 0xf0, 0x42, 0x90, 0xe0, 0x42, 0x15, 0x20, 0xd2, 0x05, 0x20,
    0x4f, 0x16, 0x20, 0x18, 0xcb, 0x4f, 0x06, 0x04, 0xc5, 0xcb, 0x11, 0x17, 0xc1, 0xcb, 0x11, 0x17,
    0x05, 0x20, 0xf5, 0x22, 0x23, 0x22, 0x23, 0xc9, 0xce, 0xed, 0x66, 0x66, 0xcc, 0x0d, 0x00, 0x0b,
    0x03, 0x73, 0x00, 0x83, 0x00, 0x0c, 0x00, 0x0d, 0x00, 0x08, 0x11, 0x1f, 0x88, 0x89, 0x00, 0x0e,
    0xdc, 0xcc, 0x6e, 0xe6, 0xdd, 0xdd, 0xd9, 0x99, 0xbb, 0xbb, 0x67, 0x63, 0x6e, 0x0e, 0xec, 0xcc,
    0xdd, 0xdc, 0x99, 0x9f, 0xbb, 0xb9, 0x33, 0x3e, 0x3c, 0x42, 0xb9, 0xa5, 0xb9, 0xa5, 0x42, 0x3c,
    0x21, 0x04, 0x01, 0x11, 0xa8, 0x00, 0x1a, 0x13, 0xbe, 0x20, 0xfe, 0x23, 0x7d, 0xfe, 0x34, 0x20,
    0xf5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xfb, 0x86, 0x20, 0xfe, 0x3e, 0x01, 0xe0, 0x50,
];

mod controller;
mod timer;

use controller::Controller;
use timer::Timer;

use crate::gameboy::ppu::{Ppu, OAM_SIZE};

use super::cartridge::Cartridge;
use std::cell::RefCell;
use std::rc::Rc;

const RAM_SIZE: usize = 0x2000;
const HIRAM_SIZE: usize = 0x80;

pub struct Bus<'a> {
    ram: [u8; RAM_SIZE],
    pub ppu: Ppu<'a>,
    cartridge: Cartridge,
    hiram: [u8; HIRAM_SIZE],
    interrupt_enabled: u8,
    boot_rom_enabled: u8,
    pub controller: Controller,
    pub timer: Timer,
}

impl Bus<'_> {
    pub fn new(sdl_context: Rc<RefCell<sdl2::Sdl>>) -> Bus<'static> {
        Bus {
            ppu: Ppu::new(sdl_context),
            ram: [(); RAM_SIZE].map(|_| 0),
            cartridge: Cartridge::new(),
            hiram: [0; HIRAM_SIZE],
            interrupt_enabled: 0,
            boot_rom_enabled: 1,
            controller: Controller::new(),
            timer: Timer::default(),
        }
    }

    pub fn load_cartridge(&mut self, cartridge: Cartridge) {
        self.cartridge = cartridge;
    }

    pub fn load_boot_rom(&mut self) {
        for i in 0..BOOT_SEQUENCE_SIZE {
            self.ram[i] = BOOT_SEQUENCE[i];
        }
    }

    /// Mapping:
    /// 0000-3fff : bank0
    /// 4000-7fff : bankn
    /// 8000-9fff : vram
    /// a000-bfff : cartridge ram
    /// c000-dfff : working ram
    /// e000-fdff : working ram mirror
    /// fe00-fe9f : OAM
    /// fea0-feff : prohibited, unusable
    /// ff00-ff7f : I/O
    /// ff80-fffe : hiram
    /// ffff-ffff : interrupt enable register
    pub fn read_8(&mut self, address: u16) -> u8 {
        match address {
            0x0000..=0x3fff => {
                if self.boot_rom_enabled > 0 && address < 0x100 {
                    BOOT_SEQUENCE[address as usize]
                } else {
                    self.cartridge.read_bank0(address as usize)
                }
            }
            0x4000..=0x7fff => self.cartridge.read_active_bank((address - 0x4000) as usize),
            0x8000..=0x9fff => self.ppu.read_vram((address - 0x8000) as usize),
            0xa000..=0xbfff => self.cartridge.read_active_ram((address - 0xa000) as usize),
            0xc000..=0xdfff => self.ram[(address - 0xc000) as usize],
            0xe000..=0xfdff => self.ram[(address - 0xe000) as usize],
            0xfe00..=0xfe9f => self.ppu.read_oam((address - 0xfe00) as usize),
            0xff00 => self.controller.get_controller_status(), // joypad
            0xff01..=0xff02 => 0,                              // serial transfer
            0xff04 => self.timer.get_div(),                    // time and divider
            0xff05 => self.timer.get_tima(),                   // time and divider
            0xff06 => self.timer.get_tma(),                    // time and divider
            0xff07 => self.timer.get_tac(),                    // time and divider
            0xff10..=0xff26 => 0,                              // audio
            0xff30..=0xff3f => 0,                              // wave pattern
            0xff40..=0xff4b => self.ppu.read_registers(address), // lcd
            0xff50 => self.boot_rom_enabled,
            0xff80..=0xfffe => self.hiram[(address - 0xff80) as usize],
            0xffff => self.interrupt_enabled,
            _ => 0,
        }
    }

    pub fn write_8(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x3fff => self.cartridge.write_bank0(address as usize, value),
            0x4000..=0x7fff => self
                .cartridge
                .write_active_bank((address - 0x4000) as usize, value),
            0x8000..=0x9fff => self.ppu.write_vram((address - 0x8000) as usize, value),
            0xa000..=0xbfff => self
                .cartridge
                .write_active_ram((address - 0xa000) as usize, value),
            0xc000..=0xdfff => self.ram[(address - 0xc000) as usize] = value,
            0xe000..=0xfdff => self.ram[(address - 0xe000) as usize] = value,
            0xfe00..=0xfe9f => self.ppu.write_oam((address - 0xfe00) as usize, value),
            0xff00 => self.controller.set_controller_status(value), // joypad
            0xff01..=0xff02 => (),                                  // serial transfer
            0xff04 => self.timer.set_div(value),                    // time and divider
            0xff05 => self.timer.set_tima(value),                   // time and divider
            0xff06 => self.timer.set_tma(value),                    // time and divider
            0xff07 => self.timer.set_tac(value),                    // time and divider
            0xff0f => (),                                           // request interrupt
            0xff10..=0xff26 => (),                                  // audio
            0xff30..=0xff3f => (),                                  // wave pattern
            0xff40..=0xff45 => self.ppu.write_registers(address, value), // lcd
            0xff46 => {
                let mut data = [0_u8; OAM_SIZE];
                for i in 0..0x100 {
                    data[i] = self.read_8((address << 8) + i as u16);
                }
                self.ppu.set_oam(data)
            } // Start DMA transfer
            0xff47..=0xff4b => self.ppu.write_registers(address, value), // lcd
            0xff50 => self.boot_rom_enabled = value,
            0xff80..=0xfffe => self.hiram[(address - 0xff80) as usize] = value,
            0xffff => self.interrupt_enabled = value,
            _ => (), // Handle most read only and should not happen cases
        }
    }

    pub fn read_16(&mut self, address: u16) -> u16 {
        (self.read_8(address + 1) as u16) << 8 | (self.read_8(address) as u16)
    }

    pub fn write_16(&mut self, address: u16, value: u16) {
        self.write_8(address + 1, (value >> 8) as u8);
        self.write_8(address, (value & 0xff) as u8);
    }

    pub fn is_vblank_interrup_enabled(&mut self) -> bool {
        self.interrupt_enabled & 1 == 1
    }

    pub fn is_stat_interrup_enabled(&mut self) -> bool {
        (self.interrupt_enabled >> 1) & 1 == 1
    }

    pub fn is_time_interrup_enabled(&mut self) -> bool {
        (self.interrupt_enabled >> 2) & 1 == 1
    }

    pub fn is_serial_interrup_enabled(&mut self) -> bool {
        (self.interrupt_enabled >> 3) & 1 == 1
    }

    pub fn is_joypad_interrup_enabled(&mut self) -> bool {
        (self.interrupt_enabled >> 4) & 1 == 1
    }
}
