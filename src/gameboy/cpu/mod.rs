mod cb_instructions;
pub mod flags;
mod instructions;
mod registers;

use crate::gameboy::bus::Bus;
use flags::Flags;
use instructions::INSTRUCTION_TABLE;
use registers::Registers;

use self::registers::RegisterNames;

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
        if self.registers.pc > 0x1000 {
            panic!("Out of boot rom : {:04x}", self.registers.pc);
        }

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
        println!("Call to {}", address);
        self.push(self.registers.pc + 3);
        self.registers.pc = address;
    }

    fn ret(&mut self) {
        let address = self.pop();
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
        println!("Push {:04x} at {:04x}", value, self.registers.sp);

        self.set_value_16_at(self.registers.sp, value);
        self.registers.dec_sp();
        self.registers.dec_sp();
    }

    fn pop(&mut self) -> u16 {
        self.registers.inc_sp();
        self.registers.inc_sp();
        let value = self.get_value_16_at(self.registers.sp);
        println!("Pop {:04x} at {:04x}", value, self.registers.sp);
        value
    }

    /// Rotates arg1 to the left with bit 7 being moved to bit 0 and also stored into the carry
    fn rlc(&mut self, register: RegisterNames) {
        let mut value = match register {
            RegisterNames::A => self.registers.a,
            RegisterNames::B => self.registers.b,
            RegisterNames::C => self.registers.c,
            RegisterNames::D => self.registers.d,
            RegisterNames::E => self.registers.e,
            RegisterNames::H => self.registers.h,
            RegisterNames::L => self.registers.l,
            RegisterNames::IndirectHL => self.get_value_at(self.registers.get_hl()),
        };
        self.flags.clear_flags();
        value = value.rotate_left(1);
        self.flags.set_zero(value);
        self.flags.carry = (value) & 0x1 == 0x1;
        match register {
            RegisterNames::A => self.registers.a = value,
            RegisterNames::B => self.registers.b = value,
            RegisterNames::C => self.registers.c = value,
            RegisterNames::D => self.registers.d = value,
            RegisterNames::E => self.registers.e = value,
            RegisterNames::H => self.registers.h = value,
            RegisterNames::L => self.registers.l = value,
            RegisterNames::IndirectHL => self.set_value_at(self.registers.get_hl(), value),
        }
    }

    /// Rotates arg1 register to the left with the carry's value put into bit 0 and bit 7 is put into the carry
    fn rl(&mut self, register: RegisterNames) {
        let mut value = match register {
            RegisterNames::A => self.registers.a,
            RegisterNames::B => self.registers.b,
            RegisterNames::C => self.registers.c,
            RegisterNames::D => self.registers.d,
            RegisterNames::E => self.registers.e,
            RegisterNames::H => self.registers.h,
            RegisterNames::L => self.registers.l,
            RegisterNames::IndirectHL => self.get_value_at(self.registers.get_hl()),
        };

        let carry = self.flags.carry;
        self.flags.clear_flags();

        self.flags.carry = value >> 7 == 0x1;
        value = ((carry as u8) << 7) | (value & 0x7f);

        value = value.rotate_left(1);
        self.flags.set_zero(value);

        match register {
            RegisterNames::A => self.registers.a = value,
            RegisterNames::B => self.registers.b = value,
            RegisterNames::C => self.registers.c = value,
            RegisterNames::D => self.registers.d = value,
            RegisterNames::E => self.registers.e = value,
            RegisterNames::H => self.registers.h = value,
            RegisterNames::L => self.registers.l = value,
            RegisterNames::IndirectHL => self.set_value_at(self.registers.get_hl(), value),
        }
    }

    /// Rotates arg1 to the right with bit 0 moved to bit 7 and also stored into the carry.
    fn rrc(&mut self, register: RegisterNames) {
        let mut value = match register {
            RegisterNames::A => self.registers.a,
            RegisterNames::B => self.registers.b,
            RegisterNames::C => self.registers.c,
            RegisterNames::D => self.registers.d,
            RegisterNames::E => self.registers.e,
            RegisterNames::H => self.registers.h,
            RegisterNames::L => self.registers.l,
            RegisterNames::IndirectHL => self.get_value_at(self.registers.get_hl()),
        };
        self.flags.clear_flags();
        self.flags.carry = (value) & 0x1 == 0x1;
        value = value.rotate_right(1);
        self.flags.set_zero(value);
        match register {
            RegisterNames::A => self.registers.a = value,
            RegisterNames::B => self.registers.b = value,
            RegisterNames::C => self.registers.c = value,
            RegisterNames::D => self.registers.d = value,
            RegisterNames::E => self.registers.e = value,
            RegisterNames::H => self.registers.h = value,
            RegisterNames::L => self.registers.l = value,
            RegisterNames::IndirectHL => self.set_value_at(self.registers.get_hl(), value),
        }
    }

    /// Rotates arg1 to the right with the carry put in bit 7 and bit 0 put into the carry.
    fn rr(&mut self, register: RegisterNames) {
        let mut value = match register {
            RegisterNames::A => self.registers.a,
            RegisterNames::B => self.registers.b,
            RegisterNames::C => self.registers.c,
            RegisterNames::D => self.registers.d,
            RegisterNames::E => self.registers.e,
            RegisterNames::H => self.registers.h,
            RegisterNames::L => self.registers.l,
            RegisterNames::IndirectHL => self.get_value_at(self.registers.get_hl()),
        };

        let carry = self.flags.carry;
        self.flags.clear_flags();

        self.flags.carry = value & 0x1 == 0x1;
        value = (carry as u8) | (value & 0xfe);

        value = value.rotate_right(1);
        self.flags.set_zero(value);

        match register {
            RegisterNames::A => self.registers.a = value,
            RegisterNames::B => self.registers.b = value,
            RegisterNames::C => self.registers.c = value,
            RegisterNames::D => self.registers.d = value,
            RegisterNames::E => self.registers.e = value,
            RegisterNames::H => self.registers.h = value,
            RegisterNames::L => self.registers.l = value,
            RegisterNames::IndirectHL => self.set_value_at(self.registers.get_hl(), value),
        }
    }

    /// Shifts arg1 register to the left with bit 7 moved to the carry flag and bit 0 reset (zeroed)
    fn sla(&mut self, register: RegisterNames) {
        let mut value = match register {
            RegisterNames::A => self.registers.a,
            RegisterNames::B => self.registers.b,
            RegisterNames::C => self.registers.c,
            RegisterNames::D => self.registers.d,
            RegisterNames::E => self.registers.e,
            RegisterNames::H => self.registers.h,
            RegisterNames::L => self.registers.l,
            RegisterNames::IndirectHL => self.get_value_at(self.registers.get_hl()),
        };
        self.flags.clear_flags();
        self.flags.carry = (value >> 7) & 0x1 == 0x1;
        value = value.wrapping_shl(1);
        self.flags.set_zero(value);
        match register {
            RegisterNames::A => self.registers.a = value,
            RegisterNames::B => self.registers.b = value,
            RegisterNames::C => self.registers.c = value,
            RegisterNames::D => self.registers.d = value,
            RegisterNames::E => self.registers.e = value,
            RegisterNames::H => self.registers.h = value,
            RegisterNames::L => self.registers.l = value,
            RegisterNames::IndirectHL => self.set_value_at(self.registers.get_hl(), value),
        }
    }

    /// Shifts arg1 register to the right with bit 0 moved to the carry flag and bit 7 retaining it's original value
    fn sra(&mut self, register: RegisterNames) {
        let mut value = match register {
            RegisterNames::A => self.registers.a,
            RegisterNames::B => self.registers.b,
            RegisterNames::C => self.registers.c,
            RegisterNames::D => self.registers.d,
            RegisterNames::E => self.registers.e,
            RegisterNames::H => self.registers.h,
            RegisterNames::L => self.registers.l,
            RegisterNames::IndirectHL => self.get_value_at(self.registers.get_hl()),
        };
        self.flags.clear_flags();
        self.flags.carry = (value) & 0x1 == 0x1;
        value = (value & 0x80) | value.wrapping_shr(1);
        self.flags.set_zero(value);
        match register {
            RegisterNames::A => self.registers.a = value,
            RegisterNames::B => self.registers.b = value,
            RegisterNames::C => self.registers.c = value,
            RegisterNames::D => self.registers.d = value,
            RegisterNames::E => self.registers.e = value,
            RegisterNames::H => self.registers.h = value,
            RegisterNames::L => self.registers.l = value,
            RegisterNames::IndirectHL => self.set_value_at(self.registers.get_hl(), value),
        }
    }

    /// Shifts arg1 register to the right with bit 0 moved to the carry flag and bit 7 zeroed
    fn srl(&mut self, register: RegisterNames) {
        let mut value = match register {
            RegisterNames::A => self.registers.a,
            RegisterNames::B => self.registers.b,
            RegisterNames::C => self.registers.c,
            RegisterNames::D => self.registers.d,
            RegisterNames::E => self.registers.e,
            RegisterNames::H => self.registers.h,
            RegisterNames::L => self.registers.l,
            RegisterNames::IndirectHL => self.get_value_at(self.registers.get_hl()),
        };
        self.flags.clear_flags();
        self.flags.carry = (value) & 0x1 == 0x1;
        value = value.wrapping_shr(1);
        self.flags.set_zero(value);
        match register {
            RegisterNames::A => self.registers.a = value,
            RegisterNames::B => self.registers.b = value,
            RegisterNames::C => self.registers.c = value,
            RegisterNames::D => self.registers.d = value,
            RegisterNames::E => self.registers.e = value,
            RegisterNames::H => self.registers.h = value,
            RegisterNames::L => self.registers.l = value,
            RegisterNames::IndirectHL => self.set_value_at(self.registers.get_hl(), value),
        }
    }

    /// Reset bit *bit_index* in *register register*
    fn res(&mut self, register: RegisterNames, bit_index: u8) {
        let mask = !(1_u8 << bit_index);
        match register {
            RegisterNames::A => self.registers.a &= mask,
            RegisterNames::B => self.registers.b &= mask,
            RegisterNames::C => self.registers.c &= mask,
            RegisterNames::D => self.registers.d &= mask,
            RegisterNames::E => self.registers.e &= mask,
            RegisterNames::H => self.registers.h &= mask,
            RegisterNames::L => self.registers.l &= mask,
            RegisterNames::IndirectHL => {
                let hl = self.registers.get_hl();
                let value = self.get_value_at(hl) & mask;
                self.set_value_at(hl, value);
            }
        }
    }

    /// Set bit *bit_index* in *register register*
    fn set(&mut self, register: RegisterNames, bit_index: u8) {
        let mask = (1_u8 << bit_index);
        match register {
            RegisterNames::A => self.registers.a |= mask,
            RegisterNames::B => self.registers.b |= mask,
            RegisterNames::C => self.registers.c |= mask,
            RegisterNames::D => self.registers.d |= mask,
            RegisterNames::E => self.registers.e |= mask,
            RegisterNames::H => self.registers.h |= mask,
            RegisterNames::L => self.registers.l |= mask,
            RegisterNames::IndirectHL => {
                let hl = self.registers.get_hl();
                let value = self.get_value_at(hl) | mask;
                self.set_value_at(hl, value);
            }
        }
    }

    fn swap(&mut self, register: RegisterNames) {
        let swap_internal = |value: u8| ((value & 0x0f) << 4) | ((value & 0xf0) >> 4);

        match register {
            RegisterNames::A => self.registers.a = swap_internal(self.registers.a),
            RegisterNames::B => self.registers.b = swap_internal(self.registers.b),
            RegisterNames::C => self.registers.c = swap_internal(self.registers.c),
            RegisterNames::D => self.registers.d = swap_internal(self.registers.d),
            RegisterNames::E => self.registers.e = swap_internal(self.registers.e),
            RegisterNames::H => self.registers.h = swap_internal(self.registers.h),
            RegisterNames::L => self.registers.l = swap_internal(self.registers.l),
            RegisterNames::IndirectHL => {
                let hl = self.registers.get_hl();
                let value = self.get_value_at(hl);
                let value = swap_internal(value);
                self.set_value_at(hl, value);
            }
        }
    }
}
