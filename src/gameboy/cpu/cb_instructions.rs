use super::instructions::{Instruction, InstructionCode};
use super::registers::RegisterNames;

pub const CB_INSTRUCTION_TABLE: [Instruction; 0x100] = [
    Instruction {
        opcode: 0x00,
        name: InstructionCode::RLC,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.rlc(RegisterNames::B);
            0
        },
    },
    Instruction {
        opcode: 0x01,
        name: InstructionCode::RLC,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.rlc(RegisterNames::C);
            0
        },
    },
    Instruction {
        opcode: 0x02,
        name: InstructionCode::RLC,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.rlc(RegisterNames::D);
            0
        },
    },
    Instruction {
        opcode: 0x03,
        name: InstructionCode::RLC,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.rlc(RegisterNames::E);
            0
        },
    },
    Instruction {
        opcode: 0x04,
        name: InstructionCode::RLC,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.rlc(RegisterNames::H);
            0
        },
    },
    Instruction {
        opcode: 0x05,
        name: InstructionCode::RLC,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.rlc(RegisterNames::L);
            0
        },
    },
    Instruction {
        opcode: 0x06,
        name: InstructionCode::RLC,
        length: 2,
        cycles: [16, 16],
        operation: |cpu| {
            cpu.rlc(RegisterNames::IndirectHL);
            0
        },
    },
    Instruction {
        opcode: 0x07,
        name: InstructionCode::RLC,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.rlc(RegisterNames::A);
            0
        },
    },
    Instruction {
        opcode: 0x08,
        name: InstructionCode::RRC,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.rrc(RegisterNames::B);
            0
        },
    },
    Instruction {
        opcode: 0x09,
        name: InstructionCode::RRC,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.rrc(RegisterNames::C);
            0
        },
    },
    Instruction {
        opcode: 0x0A,
        name: InstructionCode::RRC,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.rrc(RegisterNames::D);
            0
        },
    },
    Instruction {
        opcode: 0x0B,
        name: InstructionCode::RRC,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.rrc(RegisterNames::E);
            0
        },
    },
    Instruction {
        opcode: 0x0C,
        name: InstructionCode::RRC,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.rrc(RegisterNames::H);
            0
        },
    },
    Instruction {
        opcode: 0x0D,
        name: InstructionCode::RRC,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.rrc(RegisterNames::L);
            0
        },
    },
    Instruction {
        opcode: 0x0E,
        name: InstructionCode::RRC,
        length: 2,
        cycles: [16, 16],
        operation: |cpu| {
            cpu.rrc(RegisterNames::IndirectHL);
            0
        },
    },
    Instruction {
        opcode: 0x0F,
        name: InstructionCode::RRC,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.rrc(RegisterNames::A);
            0
        },
    },
    Instruction {
        opcode: 0x10,
        name: InstructionCode::RL,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.rl(RegisterNames::B);
            0
        },
    },
    Instruction {
        opcode: 0x11,
        name: InstructionCode::RL,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.rl(RegisterNames::C);
            0
        },
    },
    Instruction {
        opcode: 0x12,
        name: InstructionCode::RL,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.rl(RegisterNames::D);
            0
        },
    },
    Instruction {
        opcode: 0x13,
        name: InstructionCode::RL,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.rl(RegisterNames::E);
            0
        },
    },
    Instruction {
        opcode: 0x14,
        name: InstructionCode::RL,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.rl(RegisterNames::H);
            0
        },
    },
    Instruction {
        opcode: 0x15,
        name: InstructionCode::RL,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.rl(RegisterNames::L);
            0
        },
    },
    Instruction {
        opcode: 0x16,
        name: InstructionCode::RL,
        length: 2,
        cycles: [16, 16],
        operation: |cpu| {
            cpu.rl(RegisterNames::IndirectHL);
            0
        },
    },
    Instruction {
        opcode: 0x17,
        name: InstructionCode::RL,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.rl(RegisterNames::A);
            0
        },
    },
    Instruction {
        opcode: 0x18,
        name: InstructionCode::RR,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.rr(RegisterNames::B);
            0
        },
    },
    Instruction {
        opcode: 0x19,
        name: InstructionCode::RR,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.rr(RegisterNames::C);
            0
        },
    },
    Instruction {
        opcode: 0x1A,
        name: InstructionCode::RR,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.rr(RegisterNames::D);
            0
        },
    },
    Instruction {
        opcode: 0x1B,
        name: InstructionCode::RR,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.rr(RegisterNames::E);
            0
        },
    },
    Instruction {
        opcode: 0x1C,
        name: InstructionCode::RR,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.rr(RegisterNames::H);
            0
        },
    },
    Instruction {
        opcode: 0x1D,
        name: InstructionCode::RR,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.rr(RegisterNames::L);
            0
        },
    },
    Instruction {
        opcode: 0x1E,
        name: InstructionCode::RR,
        length: 2,
        cycles: [16, 16],
        operation: |cpu| {
            cpu.rr(RegisterNames::IndirectHL);
            0
        },
    },
    Instruction {
        opcode: 0x1F,
        name: InstructionCode::RR,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.rr(RegisterNames::A);
            0
        },
    },
    Instruction {
        opcode: 0x20,
        name: InstructionCode::SLA,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.sla(RegisterNames::B);
            0
        },
    },
    Instruction {
        opcode: 0x21,
        name: InstructionCode::SLA,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.sla(RegisterNames::C);
            0
        },
    },
    Instruction {
        opcode: 0x22,
        name: InstructionCode::SLA,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.sla(RegisterNames::D);
            0
        },
    },
    Instruction {
        opcode: 0x23,
        name: InstructionCode::SLA,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.sla(RegisterNames::E);
            0
        },
    },
    Instruction {
        opcode: 0x24,
        name: InstructionCode::SLA,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.sla(RegisterNames::H);
            0
        },
    },
    Instruction {
        opcode: 0x25,
        name: InstructionCode::SLA,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.sla(RegisterNames::L);
            0
        },
    },
    Instruction {
        opcode: 0x26,
        name: InstructionCode::SLA,
        length: 2,
        cycles: [16, 16],
        operation: |cpu| {
            cpu.sla(RegisterNames::IndirectHL);
            0
        },
    },
    Instruction {
        opcode: 0x27,
        name: InstructionCode::SLA,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.sla(RegisterNames::A);
            0
        },
    },
    Instruction {
        opcode: 0x28,
        name: InstructionCode::SRA,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.sra(RegisterNames::B);
            0
        },
    },
    Instruction {
        opcode: 0x29,
        name: InstructionCode::SRA,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.sra(RegisterNames::C);
            0
        },
    },
    Instruction {
        opcode: 0x2A,
        name: InstructionCode::SRA,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.sra(RegisterNames::D);
            0
        },
    },
    Instruction {
        opcode: 0x2B,
        name: InstructionCode::SRA,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.sra(RegisterNames::E);
            0
        },
    },
    Instruction {
        opcode: 0x2C,
        name: InstructionCode::SRA,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.sra(RegisterNames::H);
            0
        },
    },
    Instruction {
        opcode: 0x2D,
        name: InstructionCode::SRA,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.sra(RegisterNames::L);
            0
        },
    },
    Instruction {
        opcode: 0x2E,
        name: InstructionCode::SRA,
        length: 2,
        cycles: [16, 16],
        operation: |cpu| {
            cpu.sra(RegisterNames::IndirectHL);
            0
        },
    },
    Instruction {
        opcode: 0x2F,
        name: InstructionCode::SRA,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.sra(RegisterNames::A);
            0
        },
    },
    Instruction {
        opcode: 0x30,
        name: InstructionCode::SWAP,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.swap(RegisterNames::B);
            0
        },
    },
    Instruction {
        opcode: 0x31,
        name: InstructionCode::SWAP,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.swap(RegisterNames::C);
            0
        },
    },
    Instruction {
        opcode: 0x32,
        name: InstructionCode::SWAP,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.swap(RegisterNames::D);
            0
        },
    },
    Instruction {
        opcode: 0x33,
        name: InstructionCode::SWAP,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.swap(RegisterNames::E);
            0
        },
    },
    Instruction {
        opcode: 0x34,
        name: InstructionCode::SWAP,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.swap(RegisterNames::H);
            0
        },
    },
    Instruction {
        opcode: 0x35,
        name: InstructionCode::SWAP,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.swap(RegisterNames::L);
            0
        },
    },
    Instruction {
        opcode: 0x36,
        name: InstructionCode::SWAP,
        length: 2,
        cycles: [16, 16],
        operation: |cpu| {
            cpu.swap(RegisterNames::IndirectHL);
            0
        },
    },
    Instruction {
        opcode: 0x37,
        name: InstructionCode::SWAP,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.swap(RegisterNames::A);
            0
        },
    },
    Instruction {
        opcode: 0x38,
        name: InstructionCode::SRL,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.srl(RegisterNames::B);
            0
        },
    },
    Instruction {
        opcode: 0x39,
        name: InstructionCode::SRL,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.srl(RegisterNames::C);
            0
        },
    },
    Instruction {
        opcode: 0x3A,
        name: InstructionCode::SRL,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.srl(RegisterNames::D);
            0
        },
    },
    Instruction {
        opcode: 0x3B,
        name: InstructionCode::SRL,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.srl(RegisterNames::E);
            0
        },
    },
    Instruction {
        opcode: 0x3C,
        name: InstructionCode::SRL,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.srl(RegisterNames::H);
            0
        },
    },
    Instruction {
        opcode: 0x3D,
        name: InstructionCode::SRL,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.srl(RegisterNames::L);
            0
        },
    },
    Instruction {
        opcode: 0x3E,
        name: InstructionCode::SRL,
        length: 2,
        cycles: [16, 16],
        operation: |cpu| {
            cpu.srl(RegisterNames::IndirectHL);
            0
        },
    },
    Instruction {
        opcode: 0x3F,
        name: InstructionCode::SRL,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.srl(RegisterNames::A);
            0
        },
    },
    Instruction {
        opcode: 0x40,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x41,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x42,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x43,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x44,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x45,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x46,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [12, 12],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x47,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x48,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x49,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x4A,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x4B,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x4C,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x4D,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x4E,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [12, 12],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x4F,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x50,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x51,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x52,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x53,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x54,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x55,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x56,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [12, 12],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x57,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x58,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x59,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x5A,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x5B,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x5C,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x5D,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x5E,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [12, 12],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x5F,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x60,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x61,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x62,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x63,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x64,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x65,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x66,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [12, 12],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x67,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x68,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x69,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x6A,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x6B,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x6C,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x6D,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x6E,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [12, 12],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x6F,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x70,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x71,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x72,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x73,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x74,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x75,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x76,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [12, 12],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x77,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x78,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x79,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x7A,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x7B,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x7C,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x7D,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x7E,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [12, 12],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x7F,
        name: InstructionCode::BIT,
        length: 2,
        cycles: [8, 8],
        operation: |_cpu| 0,
    },
    Instruction {
        opcode: 0x80,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::B, 0);
            0
        },
    },
    Instruction {
        opcode: 0x81,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::C, 0);
            0
        },
    },
    Instruction {
        opcode: 0x82,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::D, 0);
            0
        },
    },
    Instruction {
        opcode: 0x83,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::E, 0);
            0
        },
    },
    Instruction {
        opcode: 0x84,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::H, 0);
            0
        },
    },
    Instruction {
        opcode: 0x85,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::L, 0);
            0
        },
    },
    Instruction {
        opcode: 0x86,
        name: InstructionCode::RES,
        length: 2,
        cycles: [16, 16],
        operation: |cpu| {
            cpu.res(RegisterNames::IndirectHL, 0);
            0
        },
    },
    Instruction {
        opcode: 0x87,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::A, 0);
            0
        },
    },
    Instruction {
        opcode: 0x88,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::B, 1);
            0
        },
    },
    Instruction {
        opcode: 0x89,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::C, 1);
            0
        },
    },
    Instruction {
        opcode: 0x8A,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::D, 1);
            0
        },
    },
    Instruction {
        opcode: 0x8B,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::E, 1);
            0
        },
    },
    Instruction {
        opcode: 0x8C,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::H, 1);
            0
        },
    },
    Instruction {
        opcode: 0x8D,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::L, 1);
            0
        },
    },
    Instruction {
        opcode: 0x8E,
        name: InstructionCode::RES,
        length: 2,
        cycles: [16, 16],
        operation: |cpu| {
            cpu.res(RegisterNames::IndirectHL, 1);
            0
        },
    },
    Instruction {
        opcode: 0x8F,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::A, 1);
            0
        },
    },
    Instruction {
        opcode: 0x90,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::B, 2);
            0
        },
    },
    Instruction {
        opcode: 0x91,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::C, 2);
            0
        },
    },
    Instruction {
        opcode: 0x92,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::D, 2);
            0
        },
    },
    Instruction {
        opcode: 0x93,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::E, 2);
            0
        },
    },
    Instruction {
        opcode: 0x94,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::H, 2);
            0
        },
    },
    Instruction {
        opcode: 0x95,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::L, 2);
            0
        },
    },
    Instruction {
        opcode: 0x96,
        name: InstructionCode::RES,
        length: 2,
        cycles: [16, 16],
        operation: |cpu| {
            cpu.res(RegisterNames::IndirectHL, 2);
            0
        },
    },
    Instruction {
        opcode: 0x97,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::A, 2);
            0
        },
    },
    Instruction {
        opcode: 0x98,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::B, 3);
            0
        },
    },
    Instruction {
        opcode: 0x99,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::C, 3);
            0
        },
    },
    Instruction {
        opcode: 0x9A,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::D, 3);
            0
        },
    },
    Instruction {
        opcode: 0x9B,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::E, 3);
            0
        },
    },
    Instruction {
        opcode: 0x9C,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::H, 3);
            0
        },
    },
    Instruction {
        opcode: 0x9D,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::L, 3);
            0
        },
    },
    Instruction {
        opcode: 0x9E,
        name: InstructionCode::RES,
        length: 2,
        cycles: [16, 16],
        operation: |cpu| {
            cpu.res(RegisterNames::IndirectHL, 3);
            0
        },
    },
    Instruction {
        opcode: 0x9F,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::A, 3);
            0
        },
    },
    Instruction {
        opcode: 0xA0,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::B, 4);
            0
        },
    },
    Instruction {
        opcode: 0xA1,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::C, 4);
            0
        },
    },
    Instruction {
        opcode: 0xA2,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::D, 4);
            0
        },
    },
    Instruction {
        opcode: 0xA3,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::E, 4);
            0
        },
    },
    Instruction {
        opcode: 0xA4,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::H, 4);
            0
        },
    },
    Instruction {
        opcode: 0xA5,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::L, 4);
            0
        },
    },
    Instruction {
        opcode: 0xA6,
        name: InstructionCode::RES,
        length: 2,
        cycles: [16, 16],
        operation: |cpu| {
            cpu.res(RegisterNames::IndirectHL, 4);
            0
        },
    },
    Instruction {
        opcode: 0xA7,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::A, 4);
            0
        },
    },
    Instruction {
        opcode: 0xA8,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::B, 5);
            0
        },
    },
    Instruction {
        opcode: 0xA9,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::C, 5);
            0
        },
    },
    Instruction {
        opcode: 0xAA,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::D, 5);
            0
        },
    },
    Instruction {
        opcode: 0xAB,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::E, 5);
            0
        },
    },
    Instruction {
        opcode: 0xAC,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::H, 5);
            0
        },
    },
    Instruction {
        opcode: 0xAD,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::L, 5);
            0
        },
    },
    Instruction {
        opcode: 0xAE,
        name: InstructionCode::RES,
        length: 2,
        cycles: [16, 16],
        operation: |cpu| {
            cpu.res(RegisterNames::IndirectHL, 5);
            0
        },
    },
    Instruction {
        opcode: 0xAF,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::A, 5);
            0
        },
    },
    Instruction {
        opcode: 0xB0,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::B, 6);
            0
        },
    },
    Instruction {
        opcode: 0xB1,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::C, 6);
            0
        },
    },
    Instruction {
        opcode: 0xB2,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::D, 6);
            0
        },
    },
    Instruction {
        opcode: 0xB3,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::E, 6);
            0
        },
    },
    Instruction {
        opcode: 0xB4,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::H, 6);
            0
        },
    },
    Instruction {
        opcode: 0xB5,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::L, 6);
            0
        },
    },
    Instruction {
        opcode: 0xB6,
        name: InstructionCode::RES,
        length: 2,
        cycles: [16, 16],
        operation: |cpu| {
            cpu.res(RegisterNames::IndirectHL, 6);
            0
        },
    },
    Instruction {
        opcode: 0xB7,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::A, 6);
            0
        },
    },
    Instruction {
        opcode: 0xB8,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::B, 7);
            0
        },
    },
    Instruction {
        opcode: 0xB9,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::C, 7);
            0
        },
    },
    Instruction {
        opcode: 0xBA,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::D, 7);
            0
        },
    },
    Instruction {
        opcode: 0xBB,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::E, 7);
            0
        },
    },
    Instruction {
        opcode: 0xBC,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::H, 7);
            0
        },
    },
    Instruction {
        opcode: 0xBD,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::L, 7);
            0
        },
    },
    Instruction {
        opcode: 0xBE,
        name: InstructionCode::RES,
        length: 2,
        cycles: [16, 16],
        operation: |cpu| {
            cpu.res(RegisterNames::IndirectHL, 7);
            0
        },
    },
    Instruction {
        opcode: 0xBF,
        name: InstructionCode::RES,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.res(RegisterNames::A, 7);
            0
        },
    },
    Instruction {
        opcode: 0xC0,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::B, 0);
            0
        },
    },
    Instruction {
        opcode: 0xC1,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::C, 0);
            0
        },
    },
    Instruction {
        opcode: 0xC2,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::D, 0);
            0
        },
    },
    Instruction {
        opcode: 0xC3,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::E, 0);
            0
        },
    },
    Instruction {
        opcode: 0xC4,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::H, 0);
            0
        },
    },
    Instruction {
        opcode: 0xC5,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::L, 0);
            0
        },
    },
    Instruction {
        opcode: 0xC6,
        name: InstructionCode::SET,
        length: 2,
        cycles: [16, 16],
        operation: |cpu| {
            cpu.set(RegisterNames::IndirectHL, 0);
            0
        },
    },
    Instruction {
        opcode: 0xC7,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::A, 0);
            0
        },
    },
    Instruction {
        opcode: 0xC8,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::B, 1);
            0
        },
    },
    Instruction {
        opcode: 0xC9,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::C, 1);
            0
        },
    },
    Instruction {
        opcode: 0xCA,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::D, 1);
            0
        },
    },
    Instruction {
        opcode: 0xCB,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::E, 1);
            0
        },
    },
    Instruction {
        opcode: 0xCC,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::H, 1);
            0
        },
    },
    Instruction {
        opcode: 0xCD,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::L, 1);
            0
        },
    },
    Instruction {
        opcode: 0xCE,
        name: InstructionCode::SET,
        length: 2,
        cycles: [16, 16],
        operation: |cpu| {
            cpu.set(RegisterNames::IndirectHL, 1);
            0
        },
    },
    Instruction {
        opcode: 0xCF,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::A, 1);
            0
        },
    },
    Instruction {
        opcode: 0xD0,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::B, 2);
            0
        },
    },
    Instruction {
        opcode: 0xD1,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::C, 2);
            0
        },
    },
    Instruction {
        opcode: 0xD2,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::D, 2);
            0
        },
    },
    Instruction {
        opcode: 0xD3,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::E, 2);
            0
        },
    },
    Instruction {
        opcode: 0xD4,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::H, 2);
            0
        },
    },
    Instruction {
        opcode: 0xD5,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::L, 2);
            0
        },
    },
    Instruction {
        opcode: 0xD6,
        name: InstructionCode::SET,
        length: 2,
        cycles: [16, 16],
        operation: |cpu| {
            cpu.set(RegisterNames::IndirectHL, 2);
            0
        },
    },
    Instruction {
        opcode: 0xD7,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::A, 2);
            0
        },
    },
    Instruction {
        opcode: 0xD8,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::B, 3);
            0
        },
    },
    Instruction {
        opcode: 0xD9,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::C, 3);
            0
        },
    },
    Instruction {
        opcode: 0xDA,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::D, 3);
            0
        },
    },
    Instruction {
        opcode: 0xDB,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::E, 3);
            0
        },
    },
    Instruction {
        opcode: 0xDC,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::H, 3);
            0
        },
    },
    Instruction {
        opcode: 0xDD,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::L, 3);
            0
        },
    },
    Instruction {
        opcode: 0xDE,
        name: InstructionCode::SET,
        length: 2,
        cycles: [16, 16],
        operation: |cpu| {
            cpu.set(RegisterNames::IndirectHL, 3);
            0
        },
    },
    Instruction {
        opcode: 0xDF,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::A, 3);
            0
        },
    },
    Instruction {
        opcode: 0xE0,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::B, 4);
            0
        },
    },
    Instruction {
        opcode: 0xE1,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::C, 4);
            0
        },
    },
    Instruction {
        opcode: 0xE2,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::D, 4);
            0
        },
    },
    Instruction {
        opcode: 0xE3,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::E, 4);
            0
        },
    },
    Instruction {
        opcode: 0xE4,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::H, 4);
            0
        },
    },
    Instruction {
        opcode: 0xE5,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::L, 4);
            0
        },
    },
    Instruction {
        opcode: 0xE6,
        name: InstructionCode::SET,
        length: 2,
        cycles: [16, 16],
        operation: |cpu| {
            cpu.set(RegisterNames::IndirectHL, 4);
            0
        },
    },
    Instruction {
        opcode: 0xE7,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::A, 4);
            0
        },
    },
    Instruction {
        opcode: 0xE8,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::B, 5);
            0
        },
    },
    Instruction {
        opcode: 0xE9,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::C, 5);
            0
        },
    },
    Instruction {
        opcode: 0xEA,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::D, 5);
            0
        },
    },
    Instruction {
        opcode: 0xEB,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::E, 5);
            0
        },
    },
    Instruction {
        opcode: 0xEC,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::H, 5);
            0
        },
    },
    Instruction {
        opcode: 0xED,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::L, 5);
            0
        },
    },
    Instruction {
        opcode: 0xEE,
        name: InstructionCode::SET,
        length: 2,
        cycles: [16, 16],
        operation: |cpu| {
            cpu.set(RegisterNames::IndirectHL, 5);
            0
        },
    },
    Instruction {
        opcode: 0xEF,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::A, 5);
            0
        },
    },
    Instruction {
        opcode: 0xF0,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::B, 6);
            0
        },
    },
    Instruction {
        opcode: 0xF1,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::C, 6);
            0
        },
    },
    Instruction {
        opcode: 0xF2,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::D, 6);
            0
        },
    },
    Instruction {
        opcode: 0xF3,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::E, 6);
            0
        },
    },
    Instruction {
        opcode: 0xF4,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::H, 6);
            0
        },
    },
    Instruction {
        opcode: 0xF5,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::L, 6);
            0
        },
    },
    Instruction {
        opcode: 0xF6,
        name: InstructionCode::SET,
        length: 2,
        cycles: [16, 16],
        operation: |cpu| {
            cpu.set(RegisterNames::IndirectHL, 6);
            0
        },
    },
    Instruction {
        opcode: 0xF7,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::A, 6);
            0
        },
    },
    Instruction {
        opcode: 0xF8,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::B, 7);
            0
        },
    },
    Instruction {
        opcode: 0xF9,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::C, 7);
            0
        },
    },
    Instruction {
        opcode: 0xFA,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::D, 7);
            0
        },
    },
    Instruction {
        opcode: 0xFB,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::E, 7);
            0
        },
    },
    Instruction {
        opcode: 0xFC,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::H, 7);
            0
        },
    },
    Instruction {
        opcode: 0xFD,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::L, 7);
            0
        },
    },
    Instruction {
        opcode: 0xFE,
        name: InstructionCode::SET,
        length: 2,
        cycles: [16, 16],
        operation: |cpu| {
            cpu.set(RegisterNames::IndirectHL, 7);
            0
        },
    },
    Instruction {
        opcode: 0xFF,
        name: InstructionCode::SET,
        length: 2,
        cycles: [8, 8],
        operation: |cpu| {
            cpu.set(RegisterNames::A, 7);
            0
        },
    },
];
