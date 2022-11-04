mod cb_instructions;
pub mod flags;
mod instructions;
mod registers;

use crate::gameboy::bus::Bus;
use flags::Flags;
use instructions::INSTRUCTION_TABLE;
use registers::Registers;

use super::cartridge::Cartridge;

pub struct Cpu {
    pub registers: Registers,
    pub flags: Flags,
    pub bus: Bus,
    remaining_cycles: u32,
    total_cycles: u32,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: Registers::default(),
            flags: Flags::default(),
            bus: Bus::new(),
            remaining_cycles: 0,
            total_cycles: 0,
        }
    }

    pub fn start(&mut self, cartridge: Cartridge) {}

    pub fn next(&mut self) {
        if self.remaining_cycles > 0 {
            self.remaining_cycles -= 1;
            return;
        }
        let opcode = self.bus.read_rom(self.registers.pc) as usize;

        let instruction_result = (&INSTRUCTION_TABLE[opcode].operation)(self);

        self.remaining_cycles = INSTRUCTION_TABLE[opcode].cycles[instruction_result];
        self.total_cycles += self.remaining_cycles;
        self.remaining_cycles -= 1; // Do not count current cycle twice
        self.registers.pc += &INSTRUCTION_TABLE[opcode].length;
    }
}
