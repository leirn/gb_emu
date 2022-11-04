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

    pub fn start(&mut self, cartridge: Cartridge) {
        self.bus.load_cartridge(cartridge);
    }

    pub fn next(&mut self) {
        if self.remaining_cycles > 0 {
            self.remaining_cycles -= 1;
            return;
        }
        let opcode = self.bus.read_8(self.registers.pc) as usize;

        println!(
            "{:04x}  {:02x}  {}",
            self.registers.pc, opcode, INSTRUCTION_TABLE[opcode].name
        );

        let instruction_result = (&INSTRUCTION_TABLE[opcode].operation)(self);

        self.remaining_cycles = INSTRUCTION_TABLE[opcode].cycles[instruction_result];
        self.total_cycles += self.remaining_cycles;
        self.remaining_cycles -= 1; // Do not count current cycle twice
        self.registers.pc += &INSTRUCTION_TABLE[opcode].length;
    }

    /// Get 8 bit immediate value on PC + 1
    fn get_immediate(&mut self) -> u8 {
        self.bus.read_8(self.registers.pc + 1)
    }

    /// Get 16 bit immediate value on PC + 1
    fn get_immediate_16(&mut self) -> u16 {
        self.bus.read_16(self.registers.pc + 1)
    }

    fn get_value_at(&mut self, address: u16) -> u8 {
        self.bus.read_8(address)
    }

    fn get_value_16_at(&mut self, address: u16) -> u16 {
        self.bus.read_16(address)
    }

    fn get_value_at_hl(&mut self) -> u8 {
        self.bus.read_8(self.registers.get_hl())
    }

    fn set_value_at(&mut self, address: u16, value: u8) {
        self.bus.write_8(address, value);
    }

    fn set_value_at_hl(&mut self, value: u8) {
        self.bus.write_8(self.registers.get_hl(), value);
    }

    fn set_value_16_at(&mut self, address: u16, value: u16) {
        self.bus.write_16(address, value);
    }

    fn get_af(&self) -> u16 {
        (self.registers.a as u16) << 8 | self.flags.get_flags() as u16
    }

    fn set_af(&mut self, value: u16) {
        self.registers.a = (value >> 8) as u8;
        self.flags.set_flags((value & 0xff) as u8);
    }

    fn call(&mut self, address: u16) {
        self.push(self.registers.pc + 3);
        self.registers.pc = address;
    }

    fn xor(&mut self, value: u8) {
        self.registers.a ^= value;
        self.flags.clear_flags();
        self.flags.set_zero(self.registers.a)
    }

    fn or(&mut self, value: u8) {
        self.registers.a |= value;
        self.flags.clear_flags();
        self.flags.set_zero(self.registers.a)
    }

    fn and(&mut self, value: u8) {
        self.registers.a &= value;
        self.flags.clear_flags();
        self.flags.half_carry = true;
        self.flags.set_zero(self.registers.a)
    }

    fn add(&mut self, value: u8) {
        self.flags.clear_flags();
        self.flags.set_half_carry_8(self.registers.a, value);
        (self.registers.a, self.flags.carry) = self.registers.a.overflowing_add(value);
        self.flags.set_zero(self.registers.a);
    }

    // TODO : check flags
    fn sub(&mut self, value: u8) {
        self.flags.negative = true;
        self.flags.set_half_carry_8(self.registers.a, value);
        (self.registers.a, self.flags.carry) = self.registers.a.overflowing_sub(value);
        self.flags.set_zero(self.registers.a);
    }

    fn adc(&mut self, value: u8) {
        self.flags.set_half_carry_8(self.registers.a, value);
        let adc = (value as u16) + (self.registers.a as u16) + (self.flags.carry as u16);
        self.flags.carry = ((adc >> 8) & 1) != 0;
        self.registers.a = (0xff & adc) as u8;
        self.flags.set_zero(self.registers.a);
        self.flags.negative = false;
    }

    // TODO : check flags
    fn sbc(&mut self, value: u8) {
        self.adc(255 - value);
        self.flags.negative = true;
    }

    fn cp(&mut self, value: u8) {
        self.flags.zero = self.registers.a == value;
        self.flags.negative = true;
        self.flags.carry = self.registers.a < value;
        // TODO : handle half carry with sub and sbc
    }

    fn push(&mut self, value: u16) {
        self.set_value_16_at(self.registers.sp, value);
        self.registers.dec_sp();
        self.registers.dec_sp();
    }

    fn pop(&mut self) -> u16 {
        self.registers.inc_sp();
        self.registers.inc_sp();
        self.get_value_16_at(self.registers.sp)
    }
}
