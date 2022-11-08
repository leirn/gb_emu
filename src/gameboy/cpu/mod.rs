mod cb_instructions;
pub mod flags;
mod instructions;
mod registers;

use crate::gameboy::bus::Bus;
use cb_instructions::CB_INSTRUCTION_TABLE;
use flags::Flags;
use instructions::INSTRUCTION_TABLE;
use registers::Registers;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use self::instructions::InstructionCode;
use self::registers::{RegisterNames, RegisterNames16b};

use super::cartridge::Cartridge;

const INTERRUPT_ADDRESS_VBLANK: u16 = 0x0040;
const INTERRUPT_ADDRESS_STAT: u16 = 0x0048;
const INTERRUPT_ADDRESS_TIMER: u16 = 0x0050;
const INTERRUPT_ADDRESS_SERIAL: u16 = 0x0058;
const INTERRUPT_ADDRESS_JOYPAD: u16 = 0x0060;

pub struct Cpu<'a> {
    pub registers: Registers,
    pub flags: Flags,
    pub bus: Bus<'a>,
    remaining_cycles: u32,
    total_cycles: u32,
    interruption_enabled: bool,
    future_interruption_enabled: bool,
    switch_interruption_enabled_in: u8,
    pub is_halted: bool,
}

impl fmt::Display for Cpu<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cycles:{},{}", self.total_cycles, self.registers)
    }
}

impl Cpu<'_> {
    pub fn new(sdl_context: Rc<RefCell<sdl2::Sdl>>) -> Cpu<'static> {
        Cpu {
            registers: Registers::default(),
            flags: Flags::default(),
            bus: Bus::new(sdl_context),
            remaining_cycles: 0,
            total_cycles: 0,
            interruption_enabled: true,
            future_interruption_enabled: true,
            switch_interruption_enabled_in: 0,
            is_halted: false,
        }
    }

    pub fn start(&mut self, cartridge: Cartridge) {
        self.bus.load_cartridge(cartridge);
    }

    pub fn next(&mut self) {
        if self.registers.pc > 0x1000 {
            panic!("Out of boot rom : {:04x}", self.registers.pc);
        }

        if self.switch_interruption_enabled_in > 0 {
            self.switch_interruption_enabled_in -= 1;
            if self.switch_interruption_enabled_in == 0 {
                self.interruption_enabled = self.future_interruption_enabled;
            }
        }

        if self.remaining_cycles > 0 {
            self.remaining_cycles -= 1;
            return;
        }

        self.print_status();
        let opcode = self.get_immediate() as usize;

        let instruction_result = (&INSTRUCTION_TABLE[opcode].operation)(self);

        self.remaining_cycles = INSTRUCTION_TABLE[opcode].cycles[instruction_result];
        self.total_cycles += self.remaining_cycles;
        self.remaining_cycles -= 1; // Do not count current cycle twice
    }

    /// Get 8 bit immediate value on PC and increment PC of 1
    fn get_immediate(&mut self) -> u8 {
        let value = self.bus.read_8(self.registers.pc);
        self.registers.pc += 1;
        value
    }

    /// Get 16 bit immediate value on PC and increment PC of 2
    fn get_immediate_16(&mut self) -> u16 {
        let value = self.bus.read_16(self.registers.pc);
        self.registers.pc += 2;
        value
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

    fn reti(&mut self) {
        self.ret();
        self.interruption_enabled = true;
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

    fn add_16b_register_to_hl(&mut self, register: RegisterNames16b) {
        let mut value = match register {
            RegisterNames16b::BC => self.registers.get_bc(),
            RegisterNames16b::DE => self.registers.get_de(),
            RegisterNames16b::HL => self.registers.get_hl(),
            RegisterNames16b::SP => self.registers.sp,
        };
        self.flags.negative = false;
        let old_value = self.registers.get_hl();
        self.flags.set_half_carry_16(old_value, value);
        (value, self.flags.carry) = old_value.overflowing_add(value);
        self.registers.set_hl(value);
    }

    fn add_to_sp(&mut self, value: u8) {
        self.flags.clear_flags();
        let old_value = self.registers.sp;
        (self.registers.sp, self.flags.carry) = self.registers.sp.overflowing_add(value as u16);
        self.flags.set_half_carry_16(old_value, value as u16);
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
        let value = self.get_value_16_at(self.registers.sp);
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
        let mask = 1_u8 << bit_index;
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

        self.flags.clear_flags();

        let value = match register {
            RegisterNames::A => {
                self.registers.a = swap_internal(self.registers.a);
                self.registers.a
            }
            RegisterNames::B => {
                self.registers.b = swap_internal(self.registers.b);
                self.registers.b
            }
            RegisterNames::C => {
                self.registers.c = swap_internal(self.registers.c);
                self.registers.c
            }
            RegisterNames::D => {
                self.registers.d = swap_internal(self.registers.d);
                self.registers.d
            }
            RegisterNames::E => {
                self.registers.e = swap_internal(self.registers.e);
                self.registers.e
            }
            RegisterNames::H => {
                self.registers.h = swap_internal(self.registers.h);
                self.registers.h
            }
            RegisterNames::L => {
                self.registers.l = swap_internal(self.registers.l);
                self.registers.l
            }
            RegisterNames::IndirectHL => {
                let hl = self.registers.get_hl();
                let value = self.get_value_at(hl);
                let value = swap_internal(value);
                self.set_value_at(hl, value);
                value
            }
        };
        self.flags.set_zero(value);
    }

    fn bit(&mut self, register: RegisterNames, bit_index: u8) {
        self.flags.negative = false;
        self.flags.half_carry = true;

        let value = match register {
            RegisterNames::A => self.registers.a,
            RegisterNames::B => self.registers.b,
            RegisterNames::C => self.registers.c,
            RegisterNames::D => self.registers.d,
            RegisterNames::E => self.registers.e,
            RegisterNames::H => self.registers.h,
            RegisterNames::L => self.registers.l,
            RegisterNames::IndirectHL => {
                let hl = self.registers.get_hl();
                self.get_value_at(hl)
            }
        };

        self.flags.zero = (value >> bit_index) & 0x1 == 0x0;
    }

    fn inc(&mut self, register: RegisterNames) {
        self.flags.negative = false;

        let (old_value, new_value) = match register {
            RegisterNames::A => {
                let old = self.registers.a;
                self.registers.a += 1;
                (old, self.registers.a)
            }
            RegisterNames::B => {
                let old = self.registers.b;
                self.registers.b += 1;
                (old, self.registers.b)
            }
            RegisterNames::C => {
                let old = self.registers.c;
                self.registers.c += 1;
                (old, self.registers.c)
            }
            RegisterNames::D => {
                let old = self.registers.d;
                self.registers.d += 1;
                (old, self.registers.d)
            }
            RegisterNames::E => {
                let old = self.registers.e;
                self.registers.e += 1;
                (old, self.registers.e)
            }
            RegisterNames::H => {
                let old = self.registers.h;
                self.registers.h += 1;
                (old, self.registers.h)
            }
            RegisterNames::L => {
                let old = self.registers.l;
                self.registers.l += 1;
                (old, self.registers.l)
            }
            RegisterNames::IndirectHL => {
                let hl = self.registers.get_hl();
                let old = self.get_value_at(hl);
                let value = old + 1;
                self.set_value_at(hl, value);
                (old, value)
            }
        };
        self.flags.set_half_carry_8(old_value, new_value);
        self.flags.set_zero(new_value);
    }

    fn dec(&mut self, register: RegisterNames) {
        self.flags.negative = false;

        let (old_value, new_value) = match register {
            RegisterNames::A => {
                let old = self.registers.a;
                self.registers.a -= 1;
                (old, self.registers.a)
            }
            RegisterNames::B => {
                let old = self.registers.b;
                self.registers.b -= 1;
                (old, self.registers.b)
            }
            RegisterNames::C => {
                let old = self.registers.c;
                self.registers.c -= 1;
                (old, self.registers.c)
            }
            RegisterNames::D => {
                let old = self.registers.d;
                self.registers.d -= 1;
                (old, self.registers.d)
            }
            RegisterNames::E => {
                let old = self.registers.e;
                self.registers.e -= 1;
                (old, self.registers.e)
            }
            RegisterNames::H => {
                let old = self.registers.h;
                self.registers.h -= 1;
                (old, self.registers.h)
            }
            RegisterNames::L => {
                let old = self.registers.l;
                self.registers.l -= 1;
                (old, self.registers.l)
            }
            RegisterNames::IndirectHL => {
                let hl = self.registers.get_hl();
                let old = self.get_value_at(hl);
                let value = old - 1;
                self.set_value_at(hl, value);
                (old, value)
            }
        };
        self.flags.set_half_carry_8(old_value, new_value); // TODO - half borrow ?
        self.flags.set_zero(new_value);
    }

    /// ei activate interrupt, but only in 3 cycles
    fn ei(&mut self) {
        self.future_interruption_enabled = true;
        self.switch_interruption_enabled_in = 3;
    }

    /// di deactivate interrupt, but only in 3 cycles
    fn di(&mut self) {
        self.future_interruption_enabled = true;
        self.switch_interruption_enabled_in = 3;
    }

    fn rst(&mut self, n: u16) {
        self.push(self.registers.pc);
        self.registers.pc = n;
    }

    fn cpl(&mut self) {
        self.flags.clear_flags();
        self.registers.a = !self.registers.a;
        self.flags.negative = true;
        self.flags.half_carry = true;
    }

    fn halt(&mut self) {
        self.is_halted = true;
    }

    fn print_status(&mut self) {
        let opcode = self.bus.read_8(self.registers.pc) as usize;

        let mut instruction = &INSTRUCTION_TABLE[opcode];
        if instruction.name == InstructionCode::PREFIX {
            let opcode = self.bus.read_8(self.registers.pc + 1) as usize;

            instruction = &CB_INSTRUCTION_TABLE[opcode];
        }

        if instruction.length == 1 {
            println!(
                "{:04x}  {:02x}        {:indent$}    {}   Flags:{:08b}",
                self.registers.pc,
                opcode,
                instruction,
                self,
                self.flags.get_flags(),
                indent = 30
            );
        } else if instruction.length == 2 {
            let imm1 = self.get_value_at(self.registers.pc + 1);
            println!(
                "{:04x}  {:02x} {:02x}     {:indent$}    {}   Flags:{:08b}",
                self.registers.pc,
                opcode,
                imm1,
                instruction,
                self,
                self.flags.get_flags(),
                indent = 30
            );
        } else if instruction.length == 3 {
            let imm1 = self.get_value_at(self.registers.pc + 1);
            let imm2 = self.get_value_at(self.registers.pc + 2);
            println!(
                "{:04x}  {:02x} {:02x} {:02x}  {:indent$}    {}   Flags:{:08b}",
                self.registers.pc,
                opcode,
                imm1,
                imm2,
                instruction,
                self,
                self.flags.get_flags(),
                indent = 30
            );
        }
    }
}
