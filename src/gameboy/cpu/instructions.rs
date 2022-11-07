use super::cb_instructions::CB_INSTRUCTION_TABLE;
use super::registers::RegisterNames;
use crate::gameboy::cpu::Cpu;
use std::fmt;

pub struct Instruction {
    pub opcode: u8,
    pub length: u16,
    pub cycles: [u32; 2],
    pub name: InstructionCode,
    pub operation: fn(cpu: &mut Cpu) -> usize,
}

#[derive(PartialEq, Debug)]
pub enum InstructionCode {
    NOP,
    LD,
    INC,
    DEC,
    RLCA,
    ADD,
    RRCA,
    STOP,
    RLA,
    JR,
    RRA,
    DAA,
    CPL,
    SCF,
    CCF,
    HALT,
    ADC,
    SUB,
    SBC,
    AND,
    XOR,
    OR,
    CP,
    RET,
    POP,
    JP,
    CALL,
    PUSH,
    RST,
    PREFIX,
    ILLEGAL_D3,
    RETI,
    ILLEGAL_DB,
    ILLEGAL_DD,
    LDH,
    ILLEGAL_E3,
    ILLEGAL_E4,
    ILLEGAL_EB,
    ILLEGAL_EC,
    ILLEGAL_ED,
    DI,
    ILLEGAL_F4,
    EI,
    ILLEGAL_FC,
    ILLEGAL_FD,
    RLC,
    RRC,
    RL,
    RR,
    SLA,
    SRA,
    SWAP,
    SRL,
    BIT,
    RES,
    SET,
}

impl fmt::Display for InstructionCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub const INSTRUCTION_TABLE: [Instruction; 0x100] = [
    Instruction {
        opcode: 0x00,
        name: InstructionCode::NOP,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x01,
        name: InstructionCode::LD,
        length: 3,
        cycles: [12, 12],
        operation: |cpu| {
            let value = cpu.get_immediate_16();
            cpu.registers.set_bc(value);
            0
        },
    },
    Instruction {
        opcode: 0x02,
        name: InstructionCode::LD,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set_value_at(cpu.registers.get_bc(), cpu.registers.a);
            0
        },
    },
    Instruction {
        opcode: 0x03,
        name: InstructionCode::INC,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.registers.inc_bc();
            0
        },
    },
    Instruction {
        opcode: 0x04,
        name: InstructionCode::INC,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x05,
        name: InstructionCode::DEC,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x06,
        name: InstructionCode::LD,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.registers.b = cpu.get_immediate();
            0
        },
    },
    Instruction {
        opcode: 0x07,
        name: InstructionCode::RLCA,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.rlc(RegisterNames::A);
            cpu.flags.zero = false;
            0
        },
    },
    Instruction {
        opcode: 0x08,
        name: InstructionCode::LD,
        length: 3,
        cycles: [20, 20],
        operation: |cpu| {
            let address = cpu.get_immediate_16();
            cpu.set_value_16_at(address, cpu.registers.sp);
            0
        },
    },
    Instruction {
        opcode: 0x09,
        name: InstructionCode::ADD,
        length: 1,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x0A,
        name: InstructionCode::LD,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.registers.a = cpu.get_value_at(cpu.registers.get_bc());
            0
        },
    },
    Instruction {
        opcode: 0x0B,
        name: InstructionCode::DEC,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.registers.dec_bc();
            0
        },
    },
    Instruction {
        opcode: 0x0C,
        name: InstructionCode::INC,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x0D,
        name: InstructionCode::DEC,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x0E,
        name: InstructionCode::LD,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.registers.c = cpu.get_immediate();
            0
        },
    },
    Instruction {
        opcode: 0x0F,
        name: InstructionCode::RRCA,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.rrc(RegisterNames::A);
            cpu.flags.zero = false;
            0
        },
    },
    Instruction {
        opcode: 0x10,
        name: InstructionCode::STOP,
        length: 2,
        cycles: [4, 4],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x11,
        name: InstructionCode::LD,
        length: 3,
        cycles: [12, 12],
        operation: |cpu| {
            let value = cpu.get_immediate_16();
            cpu.registers.set_de(value);
            0
        },
    },
    Instruction {
        opcode: 0x12,
        name: InstructionCode::LD,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set_value_at(cpu.registers.get_de(), cpu.registers.a);
            0
        },
    },
    Instruction {
        opcode: 0x13,
        name: InstructionCode::INC,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.registers.inc_de();
            0
        },
    },
    Instruction {
        opcode: 0x14,
        name: InstructionCode::INC,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x15,
        name: InstructionCode::DEC,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x16,
        name: InstructionCode::LD,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.registers.d = cpu.get_immediate();
            0
        },
    },
    Instruction {
        opcode: 0x17,
        name: InstructionCode::RLA,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.rl(RegisterNames::A);
            cpu.flags.zero = false;
            0
        },
    },
    Instruction {
        opcode: 0x18,
        name: InstructionCode::JR,
        length: 2,
        cycles: [12, 12],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x19,
        name: InstructionCode::ADD,
        length: 1,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x1A,
        name: InstructionCode::LD,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.registers.a = cpu.get_value_at(cpu.registers.get_de());
            0
        },
    },
    Instruction {
        opcode: 0x1B,
        name: InstructionCode::DEC,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.registers.dec_de();
            0
        },
    },
    Instruction {
        opcode: 0x1C,
        name: InstructionCode::INC,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x1D,
        name: InstructionCode::DEC,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x1E,
        name: InstructionCode::LD,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.registers.d = cpu.get_immediate();
            0
        },
    },
    Instruction {
        opcode: 0x1F,
        name: InstructionCode::RRA,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.rr(RegisterNames::A);
            cpu.flags.zero = false;
            0
        },
    },
    Instruction {
        opcode: 0x20,
        name: InstructionCode::JR,
        length: 2,
        cycles: [12, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x21,
        name: InstructionCode::LD,
        length: 3,
        cycles: [12, 12],
        operation: |cpu| {
            let value = cpu.get_immediate_16();
            cpu.registers.set_hl(value);
            0
        },
    },
    Instruction {
        opcode: 0x22,
        name: InstructionCode::LD,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set_value_at_hl(cpu.registers.a);
            cpu.registers.inc_hl();
            0
        },
    },
    Instruction {
        opcode: 0x23,
        name: InstructionCode::INC,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.registers.inc_hl();
            0
        },
    },
    Instruction {
        opcode: 0x24,
        name: InstructionCode::INC,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x25,
        name: InstructionCode::DEC,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x26,
        name: InstructionCode::LD,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.registers.h = cpu.get_immediate();
            0
        },
    },
    Instruction {
        opcode: 0x27,
        name: InstructionCode::DAA,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x28,
        name: InstructionCode::JR,
        length: 2,
        cycles: [12, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x29,
        name: InstructionCode::ADD,
        length: 1,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x2A,
        name: InstructionCode::LD,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.registers.a = cpu.get_value_at_hl();
            cpu.registers.inc_hl();
            0
        },
    },
    Instruction {
        opcode: 0x2B,
        name: InstructionCode::DEC,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.registers.dec_hl();
            0
        },
    },
    Instruction {
        opcode: 0x2C,
        name: InstructionCode::INC,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x2D,
        name: InstructionCode::DEC,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x2E,
        name: InstructionCode::LD,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.registers.l = cpu.get_immediate();
            0
        },
    },
    Instruction {
        opcode: 0x2F,
        name: InstructionCode::CPL,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x30,
        name: InstructionCode::JR,
        length: 2,
        cycles: [12, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x31,
        name: InstructionCode::LD,
        length: 3,
        cycles: [12, 12],
        operation: |cpu| {
            cpu.registers.sp = cpu.get_immediate_16();
            0
        },
    },
    Instruction {
        opcode: 0x32,
        name: InstructionCode::LD,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set_value_at_hl(cpu.registers.a);
            cpu.registers.dec_hl();
            0
        },
    },
    Instruction {
        opcode: 0x33,
        name: InstructionCode::INC,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.registers.inc_sp();
            0
        },
    },
    Instruction {
        opcode: 0x34,
        name: InstructionCode::INC,
        length: 1,
        cycles: [12, 12],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x35,
        name: InstructionCode::DEC,
        length: 1,
        cycles: [12, 12],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x36,
        name: InstructionCode::LD,
        length: 2,
        cycles: [12, 12],
        operation: |cpu| {
            let value = cpu.get_immediate();
            cpu.set_value_at_hl(value);
            0
        },
    },
    Instruction {
        opcode: 0x37,
        name: InstructionCode::SCF,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x38,
        name: InstructionCode::JR,
        length: 2,
        cycles: [12, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x39,
        name: InstructionCode::ADD,
        length: 1,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x3A,
        name: InstructionCode::LD,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.registers.a = cpu.get_value_at_hl();
            cpu.registers.dec_hl();
            0
        },
    },
    Instruction {
        opcode: 0x3B,
        name: InstructionCode::DEC,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.registers.dec_sp();
            0
        },
    },
    Instruction {
        opcode: 0x3C,
        name: InstructionCode::INC,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x3D,
        name: InstructionCode::DEC,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x3E,
        name: InstructionCode::LD,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.registers.a = cpu.get_immediate();
            0
        },
    },
    Instruction {
        opcode: 0x3F,
        name: InstructionCode::CCF,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x40,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.b = cpu.registers.b;
            0
        },
    },
    Instruction {
        opcode: 0x41,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.b = cpu.registers.c;
            0
        },
    },
    Instruction {
        opcode: 0x42,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.b = cpu.registers.d;
            0
        },
    },
    Instruction {
        opcode: 0x43,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.b = cpu.registers.e;
            0
        },
    },
    Instruction {
        opcode: 0x44,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.b = cpu.registers.h;
            0
        },
    },
    Instruction {
        opcode: 0x45,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.b = cpu.registers.l;
            0
        },
    },
    Instruction {
        opcode: 0x46,
        name: InstructionCode::LD,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.registers.b = cpu.get_value_at_hl();
            0
        },
    },
    Instruction {
        opcode: 0x47,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.b = cpu.registers.a;
            0
        },
    },
    Instruction {
        opcode: 0x48,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.c = cpu.registers.b;
            0
        },
    },
    Instruction {
        opcode: 0x49,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.c = cpu.registers.c;
            0
        },
    },
    Instruction {
        opcode: 0x4A,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.c = cpu.registers.d;
            0
        },
    },
    Instruction {
        opcode: 0x4B,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.c = cpu.registers.e;
            0
        },
    },
    Instruction {
        opcode: 0x4C,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.c = cpu.registers.h;
            0
        },
    },
    Instruction {
        opcode: 0x4D,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.c = cpu.registers.l;
            0
        },
    },
    Instruction {
        opcode: 0x4E,
        name: InstructionCode::LD,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.registers.c = cpu.get_value_at_hl();
            0
        },
    },
    Instruction {
        opcode: 0x4F,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.c = cpu.registers.a;
            0
        },
    },
    Instruction {
        opcode: 0x50,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.d = cpu.registers.b;
            0
        },
    },
    Instruction {
        opcode: 0x51,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.d = cpu.registers.c;
            0
        },
    },
    Instruction {
        opcode: 0x52,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.d = cpu.registers.d;
            0
        },
    },
    Instruction {
        opcode: 0x53,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.d = cpu.registers.e;
            0
        },
    },
    Instruction {
        opcode: 0x54,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.d = cpu.registers.h;
            0
        },
    },
    Instruction {
        opcode: 0x55,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.d = cpu.registers.l;
            0
        },
    },
    Instruction {
        opcode: 0x56,
        name: InstructionCode::LD,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.registers.d = cpu.get_value_at_hl();
            0
        },
    },
    Instruction {
        opcode: 0x57,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.d = cpu.registers.a;
            0
        },
    },
    Instruction {
        opcode: 0x58,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.e = cpu.registers.b;
            0
        },
    },
    Instruction {
        opcode: 0x59,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.e = cpu.registers.c;
            0
        },
    },
    Instruction {
        opcode: 0x5A,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.e = cpu.registers.d;
            0
        },
    },
    Instruction {
        opcode: 0x5B,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.e = cpu.registers.e;
            0
        },
    },
    Instruction {
        opcode: 0x5C,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.e = cpu.registers.h;
            0
        },
    },
    Instruction {
        opcode: 0x5D,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.e = cpu.registers.l;
            0
        },
    },
    Instruction {
        opcode: 0x5E,
        name: InstructionCode::LD,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.registers.e = cpu.get_value_at_hl();
            0
        },
    },
    Instruction {
        opcode: 0x5F,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.e = cpu.registers.a;
            0
        },
    },
    Instruction {
        opcode: 0x60,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.h = cpu.registers.b;
            0
        },
    },
    Instruction {
        opcode: 0x61,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.h = cpu.registers.c;
            0
        },
    },
    Instruction {
        opcode: 0x62,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.h = cpu.registers.d;
            0
        },
    },
    Instruction {
        opcode: 0x63,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.h = cpu.registers.e;
            0
        },
    },
    Instruction {
        opcode: 0x64,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.h = cpu.registers.h;
            0
        },
    },
    Instruction {
        opcode: 0x65,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.h = cpu.registers.l;
            0
        },
    },
    Instruction {
        opcode: 0x66,
        name: InstructionCode::LD,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.registers.h = cpu.get_value_at_hl();
            0
        },
    },
    Instruction {
        opcode: 0x67,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.h = cpu.registers.a;
            0
        },
    },
    Instruction {
        opcode: 0x68,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.l = cpu.registers.b;
            0
        },
    },
    Instruction {
        opcode: 0x69,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.l = cpu.registers.c;
            0
        },
    },
    Instruction {
        opcode: 0x6A,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.l = cpu.registers.d;
            0
        },
    },
    Instruction {
        opcode: 0x6B,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.l = cpu.registers.e;
            0
        },
    },
    Instruction {
        opcode: 0x6C,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.l = cpu.registers.h;
            0
        },
    },
    Instruction {
        opcode: 0x6D,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.l = cpu.registers.l;
            0
        },
    },
    Instruction {
        opcode: 0x6E,
        name: InstructionCode::LD,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.registers.l = cpu.get_value_at_hl();
            0
        },
    },
    Instruction {
        opcode: 0x6F,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.l = cpu.registers.a;
            0
        },
    },
    Instruction {
        opcode: 0x70,
        name: InstructionCode::LD,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set_value_at_hl(cpu.registers.b);
            0
        },
    },
    Instruction {
        opcode: 0x71,
        name: InstructionCode::LD,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set_value_at_hl(cpu.registers.c);
            0
        },
    },
    Instruction {
        opcode: 0x72,
        name: InstructionCode::LD,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set_value_at_hl(cpu.registers.d);
            0
        },
    },
    Instruction {
        opcode: 0x73,
        name: InstructionCode::LD,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set_value_at_hl(cpu.registers.e);
            0
        },
    },
    Instruction {
        opcode: 0x74,
        name: InstructionCode::LD,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set_value_at_hl(cpu.registers.h);
            0
        },
    },
    Instruction {
        opcode: 0x75,
        name: InstructionCode::LD,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set_value_at_hl(cpu.registers.l);
            0
        },
    },
    Instruction {
        opcode: 0x76,
        name: InstructionCode::HALT,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x77,
        name: InstructionCode::LD,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set_value_at_hl(cpu.registers.a);
            0
        },
    },
    Instruction {
        opcode: 0x78,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.a = cpu.registers.b;
            0
        },
    },
    Instruction {
        opcode: 0x79,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.a = cpu.registers.c;
            0
        },
    },
    Instruction {
        opcode: 0x7A,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.a = cpu.registers.d;
            0
        },
    },
    Instruction {
        opcode: 0x7B,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.a = cpu.registers.e;
            0
        },
    },
    Instruction {
        opcode: 0x7C,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.a = cpu.registers.h;
            0
        },
    },
    Instruction {
        opcode: 0x7D,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.a = cpu.registers.l;
            0
        },
    },
    Instruction {
        opcode: 0x7E,
        name: InstructionCode::LD,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.registers.a = cpu.get_value_at_hl();
            0
        },
    },
    Instruction {
        opcode: 0x7F,
        name: InstructionCode::LD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.a = cpu.registers.a;
            0
        },
    },
    Instruction {
        opcode: 0x80,
        name: InstructionCode::ADD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.add(cpu.registers.b);
            0
        },
    },
    Instruction {
        opcode: 0x81,
        name: InstructionCode::ADD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.add(cpu.registers.c);
            0
        },
    },
    Instruction {
        opcode: 0x82,
        name: InstructionCode::ADD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.add(cpu.registers.d);
            0
        },
    },
    Instruction {
        opcode: 0x83,
        name: InstructionCode::ADD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.add(cpu.registers.e);
            0
        },
    },
    Instruction {
        opcode: 0x84,
        name: InstructionCode::ADD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.add(cpu.registers.h);
            0
        },
    },
    Instruction {
        opcode: 0x85,
        name: InstructionCode::ADD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.add(cpu.registers.l);
            0
        },
    },
    Instruction {
        opcode: 0x86,
        name: InstructionCode::ADD,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            let value = cpu.get_value_at_hl();
            cpu.add(value);
            0
        },
    },
    Instruction {
        opcode: 0x87,
        name: InstructionCode::ADD,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.add(cpu.registers.a);
            0
        },
    },
    Instruction {
        opcode: 0x88,
        name: InstructionCode::ADC,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.adc(cpu.registers.b);
            0
        },
    },
    Instruction {
        opcode: 0x89,
        name: InstructionCode::ADC,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.adc(cpu.registers.c);
            0
        },
    },
    Instruction {
        opcode: 0x8A,
        name: InstructionCode::ADC,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.adc(cpu.registers.d);
            0
        },
    },
    Instruction {
        opcode: 0x8B,
        name: InstructionCode::ADC,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.adc(cpu.registers.e);
            0
        },
    },
    Instruction {
        opcode: 0x8C,
        name: InstructionCode::ADC,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.adc(cpu.registers.h);
            0
        },
    },
    Instruction {
        opcode: 0x8D,
        name: InstructionCode::ADC,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.adc(cpu.registers.l);
            0
        },
    },
    Instruction {
        opcode: 0x8E,
        name: InstructionCode::ADC,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            let value = cpu.get_value_at_hl();
            cpu.adc(value);
            0
        },
    },
    Instruction {
        opcode: 0x8F,
        name: InstructionCode::ADC,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.adc(cpu.registers.a);
            0
        },
    },
    Instruction {
        opcode: 0x90,
        name: InstructionCode::SUB,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.sub(cpu.registers.b);
            0
        },
    },
    Instruction {
        opcode: 0x91,
        name: InstructionCode::SUB,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.sub(cpu.registers.c);
            0
        },
    },
    Instruction {
        opcode: 0x92,
        name: InstructionCode::SUB,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.sub(cpu.registers.d);
            0
        },
    },
    Instruction {
        opcode: 0x93,
        name: InstructionCode::SUB,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.sub(cpu.registers.e);
            0
        },
    },
    Instruction {
        opcode: 0x94,
        name: InstructionCode::SUB,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.sub(cpu.registers.h);
            0
        },
    },
    Instruction {
        opcode: 0x95,
        name: InstructionCode::SUB,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.sub(cpu.registers.l);
            0
        },
    },
    Instruction {
        opcode: 0x96,
        name: InstructionCode::SUB,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            let value = cpu.get_value_at_hl();
            cpu.sub(value);
            0
        },
    },
    Instruction {
        opcode: 0x97,
        name: InstructionCode::SUB,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.sub(cpu.registers.a);
            0
        },
    },
    Instruction {
        opcode: 0x98,
        name: InstructionCode::SBC,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.sbc(cpu.registers.b);
            0
        },
    },
    Instruction {
        opcode: 0x99,
        name: InstructionCode::SBC,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.sbc(cpu.registers.c);
            0
        },
    },
    Instruction {
        opcode: 0x9A,
        name: InstructionCode::SBC,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.sbc(cpu.registers.d);
            0
        },
    },
    Instruction {
        opcode: 0x9B,
        name: InstructionCode::SBC,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.sbc(cpu.registers.e);
            0
        },
    },
    Instruction {
        opcode: 0x9C,
        name: InstructionCode::SBC,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.sbc(cpu.registers.h);
            0
        },
    },
    Instruction {
        opcode: 0x9D,
        name: InstructionCode::SBC,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.sbc(cpu.registers.l);
            0
        },
    },
    Instruction {
        opcode: 0x9E,
        name: InstructionCode::SBC,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            let value = cpu.get_value_at_hl();
            cpu.sbc(value);
            0
        },
    },
    Instruction {
        opcode: 0x9F,
        name: InstructionCode::SBC,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.sbc(cpu.registers.a);
            0
        },
    },
    Instruction {
        opcode: 0xA0,
        name: InstructionCode::AND,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.xor(cpu.registers.b);
            0
        },
    },
    Instruction {
        opcode: 0xA1,
        name: InstructionCode::AND,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.xor(cpu.registers.c);
            0
        },
    },
    Instruction {
        opcode: 0xA2,
        name: InstructionCode::AND,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.xor(cpu.registers.d);
            0
        },
    },
    Instruction {
        opcode: 0xA3,
        name: InstructionCode::AND,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.xor(cpu.registers.e);
            0
        },
    },
    Instruction {
        opcode: 0xA4,
        name: InstructionCode::AND,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.xor(cpu.registers.h);
            0
        },
    },
    Instruction {
        opcode: 0xA5,
        name: InstructionCode::AND,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.xor(cpu.registers.l);
            0
        },
    },
    Instruction {
        opcode: 0xA6,
        name: InstructionCode::AND,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            let value = cpu.get_value_at_hl();
            cpu.and(value);
            0
        },
    },
    Instruction {
        opcode: 0xA7,
        name: InstructionCode::AND,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.xor(cpu.registers.a);
            0
        },
    },
    Instruction {
        opcode: 0xA8,
        name: InstructionCode::XOR,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.xor(cpu.registers.b);
            0
        },
    },
    Instruction {
        opcode: 0xA9,
        name: InstructionCode::XOR,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.xor(cpu.registers.c);
            0
        },
    },
    Instruction {
        opcode: 0xAA,
        name: InstructionCode::XOR,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.xor(cpu.registers.d);
            0
        },
    },
    Instruction {
        opcode: 0xAB,
        name: InstructionCode::XOR,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.xor(cpu.registers.e);
            0
        },
    },
    Instruction {
        opcode: 0xAC,
        name: InstructionCode::XOR,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.xor(cpu.registers.h);
            0
        },
    },
    Instruction {
        opcode: 0xAD,
        name: InstructionCode::XOR,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.xor(cpu.registers.l);
            0
        },
    },
    Instruction {
        opcode: 0xAE,
        name: InstructionCode::XOR,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            let value = cpu.get_value_at_hl();
            cpu.xor(value);
            0
        },
    },
    Instruction {
        opcode: 0xAF,
        name: InstructionCode::XOR,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.xor(cpu.registers.a);
            0
        },
    },
    Instruction {
        opcode: 0xB0,
        name: InstructionCode::OR,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.or(cpu.registers.b);
            0
        },
    },
    Instruction {
        opcode: 0xB1,
        name: InstructionCode::OR,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.or(cpu.registers.c);
            0
        },
    },
    Instruction {
        opcode: 0xB2,
        name: InstructionCode::OR,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.or(cpu.registers.d);
            0
        },
    },
    Instruction {
        opcode: 0xB3,
        name: InstructionCode::OR,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.or(cpu.registers.e);
            0
        },
    },
    Instruction {
        opcode: 0xB4,
        name: InstructionCode::OR,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.or(cpu.registers.h);
            0
        },
    },
    Instruction {
        opcode: 0xB5,
        name: InstructionCode::OR,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.or(cpu.registers.l);
            0
        },
    },
    Instruction {
        opcode: 0xB6,
        name: InstructionCode::OR,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            let value = cpu.get_value_at_hl();
            cpu.or(value);
            0
        },
    },
    Instruction {
        opcode: 0xB7,
        name: InstructionCode::OR,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.or(cpu.registers.a);
            0
        },
    },
    Instruction {
        opcode: 0xB8,
        name: InstructionCode::CP,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.cp(cpu.registers.b);
            0
        },
    },
    Instruction {
        opcode: 0xB9,
        name: InstructionCode::CP,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.cp(cpu.registers.c);
            0
        },
    },
    Instruction {
        opcode: 0xBA,
        name: InstructionCode::CP,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.cp(cpu.registers.d);
            0
        },
    },
    Instruction {
        opcode: 0xBB,
        name: InstructionCode::CP,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.cp(cpu.registers.e);
            0
        },
    },
    Instruction {
        opcode: 0xBC,
        name: InstructionCode::CP,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.cp(cpu.registers.h);
            0
        },
    },
    Instruction {
        opcode: 0xBD,
        name: InstructionCode::CP,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.cp(cpu.registers.l);
            0
        },
    },
    Instruction {
        opcode: 0xBE,
        name: InstructionCode::CP,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            let value = cpu.get_value_at_hl();
            cpu.cp(value);
            0
        },
    },
    Instruction {
        opcode: 0xBF,
        name: InstructionCode::CP,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.cp(cpu.registers.a);
            0
        },
    },
    Instruction {
        opcode: 0xC0,
        name: InstructionCode::RET,
        length: 1,
        cycles: [20, 8],
        operation: |cpu| {
            if !cpu.flags.zero {
                cpu.ret();
                0
            } else {
                1
            }
        },
    },
    Instruction {
        opcode: 0xC1,
        name: InstructionCode::POP,
        length: 1,
        cycles: [12, 12],
        operation: |cpu| {
            let value = cpu.pop();
            cpu.registers.set_bc(value);
            0
        },
    },
    Instruction {
        opcode: 0xC2,
        name: InstructionCode::JP,
        length: 3,
        cycles: [16, 12],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0xC3,
        name: InstructionCode::JP,
        length: 3,
        cycles: [16, 16],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0xC4,
        name: InstructionCode::CALL,
        length: 3,
        cycles: [24, 12],
        operation: |cpu| match cpu.flags.zero {
            false => {
                let address = cpu.get_immediate_16();
                cpu.call(address);
                0
            }
            true => 1,
        },
    },
    Instruction {
        opcode: 0xC5,
        name: InstructionCode::PUSH,
        length: 1,
        cycles: [16, 16],
        operation: |cpu| {
            cpu.push(cpu.registers.get_bc());
            0
        },
    },
    Instruction {
        opcode: 0xC6,
        name: InstructionCode::ADD,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            let value = cpu.get_immediate();
            cpu.add(value);
            0
        },
    },
    Instruction {
        opcode: 0xC7,
        name: InstructionCode::RST,
        length: 1,
        cycles: [16, 16],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0xC8,
        name: InstructionCode::RET,
        length: 1,
        cycles: [20, 8],
        operation: |cpu| {
            if cpu.flags.zero {
                cpu.ret();
                0
            } else {
                1
            }
        },
    },
    Instruction {
        opcode: 0xC9,
        name: InstructionCode::RET,
        length: 1,
        cycles: [16, 16],
        operation: |cpu| {
            cpu.ret();
            0
        },
    },
    Instruction {
        opcode: 0xCA,
        name: InstructionCode::JP,
        length: 3,
        cycles: [16, 12],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0xCB,
        name: InstructionCode::PREFIX,
        length: 1,
        cycles: [4, 4],
        operation: |cpu| {
            cpu.registers.pc += 1;
            let opcode = cpu.bus.read_8(cpu.registers.pc) as usize;

            let instruction_result = (&CB_INSTRUCTION_TABLE[opcode].operation)(cpu);

            instruction_result
        },
    },
    Instruction {
        opcode: 0xCC,
        name: InstructionCode::CALL,
        length: 3,
        cycles: [24, 12],
        operation: |cpu| match cpu.flags.zero {
            true => {
                let address = cpu.get_immediate_16();
                cpu.call(address);
                0
            }
            false => 1,
        },
    },
    Instruction {
        opcode: 0xCD,
        name: InstructionCode::CALL,
        length: 3,
        cycles: [24, 24],
        operation: |cpu| {
            let address = cpu.get_immediate_16();
            cpu.call(address);
            0
        },
    },
    Instruction {
        opcode: 0xCE,
        name: InstructionCode::ADC,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            let value = cpu.get_immediate();
            cpu.adc(value);
            0
        },
    },
    Instruction {
        opcode: 0xCF,
        name: InstructionCode::RST,
        length: 1,
        cycles: [16, 16],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0xD0,
        name: InstructionCode::RET,
        length: 1,
        cycles: [20, 8],
        operation: |cpu| {
            if !cpu.flags.carry {
                cpu.ret();
                0
            } else {
                1
            }
        },
    },
    Instruction {
        opcode: 0xD1,
        name: InstructionCode::POP,
        length: 1,
        cycles: [12, 12],
        operation: |cpu| {
            let value = cpu.pop();
            cpu.registers.set_de(value);
            0
        },
    },
    Instruction {
        opcode: 0xD2,
        name: InstructionCode::JP,
        length: 3,
        cycles: [16, 12],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0xD3,
        name: InstructionCode::ILLEGAL_D3,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| panic!("Illegal instruction D3"),
    },
    Instruction {
        opcode: 0xD4,
        name: InstructionCode::CALL,
        length: 3,
        cycles: [24, 12],
        operation: |cpu| match cpu.flags.carry {
            false => {
                let address = cpu.get_immediate_16();
                cpu.call(address);
                0
            }
            true => 1,
        },
    },
    Instruction {
        opcode: 0xD5,
        name: InstructionCode::PUSH,
        length: 1,
        cycles: [16, 16],
        operation: |cpu| {
            cpu.push(cpu.registers.get_de());
            0
        },
    },
    Instruction {
        opcode: 0xD6,
        name: InstructionCode::SUB,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            let value = cpu.get_immediate();
            cpu.sub(value);
            0
        },
    },
    Instruction {
        opcode: 0xD7,
        name: InstructionCode::RST,
        length: 1,
        cycles: [16, 16],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0xD8,
        name: InstructionCode::RET,
        length: 1,
        cycles: [20, 8],
        operation: |cpu| {
            if !cpu.flags.carry {
                cpu.ret();
                0
            } else {
                1
            }
        },
    },
    Instruction {
        opcode: 0xD9,
        name: InstructionCode::RETI,
        length: 1,
        cycles: [16, 16],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0xDA,
        name: InstructionCode::JP,
        length: 3,
        cycles: [16, 12],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0xDB,
        name: InstructionCode::ILLEGAL_DB,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| panic!("Illegal instruction DB"),
    },
    Instruction {
        opcode: 0xDC,
        name: InstructionCode::CALL,
        length: 3,
        cycles: [24, 12],
        operation: |cpu| match cpu.flags.carry {
            true => {
                let address = cpu.get_immediate_16();
                cpu.call(address);
                0
            }
            false => 1,
        },
    },
    Instruction {
        opcode: 0xDD,
        name: InstructionCode::ILLEGAL_DD,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| panic!("Illegal instruction DD"),
    },
    Instruction {
        opcode: 0xDE,
        name: InstructionCode::SBC,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            let value = cpu.get_immediate();
            cpu.sbc(value);
            0
        },
    },
    Instruction {
        opcode: 0xDF,
        name: InstructionCode::RST,
        length: 1,
        cycles: [16, 16],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0xE0,
        name: InstructionCode::LDH,
        length: 2,
        cycles: [12, 12],
        operation: |cpu| {
            let address = 0xff00 | cpu.get_immediate() as u16;
            cpu.set_value_at(address, cpu.registers.a);
            0
        },
    },
    Instruction {
        opcode: 0xE1,
        name: InstructionCode::POP,
        length: 1,
        cycles: [12, 12],
        operation: |cpu| {
            let value = cpu.pop();
            cpu.registers.set_hl(value);
            0
        },
    },
    Instruction {
        opcode: 0xE2,
        name: InstructionCode::LD,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            let address = 0xff00 | cpu.registers.c as u16;
            cpu.set_value_at(address, cpu.registers.a);
            0
        },
    },
    Instruction {
        opcode: 0xE3,
        name: InstructionCode::ILLEGAL_E3,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| panic!("Illegal instruction E3"),
    },
    Instruction {
        opcode: 0xE4,
        name: InstructionCode::ILLEGAL_E4,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| panic!("Illegal instruction E4"),
    },
    Instruction {
        opcode: 0xE5,
        name: InstructionCode::PUSH,
        length: 1,
        cycles: [16, 16],
        operation: |cpu| {
            cpu.push(cpu.registers.get_hl());
            0
        },
    },
    Instruction {
        opcode: 0xE6,
        name: InstructionCode::AND,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            let value = cpu.get_immediate();
            cpu.and(value);
            0
        },
    },
    Instruction {
        opcode: 0xE7,
        name: InstructionCode::RST,
        length: 1,
        cycles: [16, 16],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0xE8,
        name: InstructionCode::ADD,
        length: 2,
        cycles: [16, 16],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0xE9,
        name: InstructionCode::JP,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0xEA,
        name: InstructionCode::LD,
        length: 3,
        cycles: [16, 16],
        operation: |cpu| {
            let address = cpu.get_immediate_16();
            cpu.set_value_at(address, cpu.registers.a);
            0
        },
    },
    Instruction {
        opcode: 0xEB,
        name: InstructionCode::ILLEGAL_EB,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| panic!("Illegal instruction EB"),
    },
    Instruction {
        opcode: 0xEC,
        name: InstructionCode::ILLEGAL_EC,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| panic!("Illegal instruction EC"),
    },
    Instruction {
        opcode: 0xED,
        name: InstructionCode::ILLEGAL_ED,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| panic!("Illegal instruction ED"),
    },
    Instruction {
        opcode: 0xEE,
        name: InstructionCode::XOR,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            let value = cpu.get_immediate();
            cpu.xor(value);
            0
        },
    },
    Instruction {
        opcode: 0xEF,
        name: InstructionCode::RST,
        length: 1,
        cycles: [16, 16],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0xF0,
        name: InstructionCode::LDH,
        length: 2,
        cycles: [12, 12],
        operation: |cpu| {
            let address = 0xff00 | cpu.get_immediate() as u16;
            cpu.registers.a = cpu.get_value_at(address);
            0
        },
    },
    Instruction {
        opcode: 0xF1,
        name: InstructionCode::POP,
        length: 1,
        cycles: [12, 12],
        operation: |cpu| {
            let value = cpu.pop();
            cpu.set_af(value);
            0
        },
    },
    Instruction {
        opcode: 0xF2,
        name: InstructionCode::LD,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            let address = 0xff00 | cpu.registers.c as u16;
            cpu.registers.a = cpu.get_value_at(address);
            0
        },
    },
    Instruction {
        opcode: 0xF3,
        name: InstructionCode::DI,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0xF4,
        name: InstructionCode::ILLEGAL_F4,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| panic!("Illegal instruction F4"),
    },
    Instruction {
        opcode: 0xF5,
        name: InstructionCode::PUSH,
        length: 1,
        cycles: [16, 16],
        operation: |cpu| {
            cpu.push(cpu.get_af());
            0
        },
    },
    Instruction {
        opcode: 0xF6,
        name: InstructionCode::OR,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            let value = cpu.get_immediate();
            cpu.or(value);
            0
        },
    },
    Instruction {
        opcode: 0xF7,
        name: InstructionCode::RST,
        length: 1,
        cycles: [16, 16],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0xF8,
        name: InstructionCode::LD,
        length: 2,
        cycles: [12, 12],
        operation: |cpu| {
            cpu.flags.clear_flags();
            let value = cpu.get_immediate() as u16;

            cpu.flags.set_half_carry_16(value, cpu.registers.sp);

            let (value, carry) = cpu.registers.sp.overflowing_add(value);
            cpu.registers.set_hl(value);
            cpu.flags.carry = carry;
            0
        },
    },
    Instruction {
        opcode: 0xF9,
        name: InstructionCode::LD,
        length: 1,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.registers.sp = cpu.registers.get_hl();
            0
        },
    },
    Instruction {
        opcode: 0xFA,
        name: InstructionCode::LD,
        length: 3,
        cycles: [16, 16],
        operation: |cpu| {
            let address = cpu.get_immediate_16();
            cpu.registers.a = cpu.get_value_at(address);
            0
        },
    },
    Instruction {
        opcode: 0xFB,
        name: InstructionCode::EI,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0xFC,
        name: InstructionCode::ILLEGAL_FC,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| panic!("Illegal instruction FC"),
    },
    Instruction {
        opcode: 0xFD,
        name: InstructionCode::ILLEGAL_FD,
        length: 1,
        cycles: [4, 4],
        operation: |_cpu| panic!("Illegal instruction FD"),
    },
    Instruction {
        opcode: 0xFE,
        name: InstructionCode::CP,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            let value = cpu.get_immediate();
            cpu.cp(value);
            0
        },
    },
    Instruction {
        opcode: 0xFF,
        name: InstructionCode::RST,
        length: 1,
        cycles: [16, 16],
        operation: |_cpu| 0,
    },
];
