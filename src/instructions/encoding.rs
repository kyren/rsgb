use std::ops::Deref;
use instructions::types::*;

/// Gameboy CPU Instructions may take up one to three bytes
pub struct InstructionBytes([u8; 3], u8);

impl InstructionBytes {
    pub fn one(a: u8) -> InstructionBytes {
        InstructionBytes([a, 0, 0], 1)
    }

    pub fn two(a: u8, b: u8) -> InstructionBytes {
        InstructionBytes([a, b, 0], 2)
    }

    pub fn three(a: u8, b: u8, c: u8) -> InstructionBytes {
        InstructionBytes([a, b, c], 3)
    }
}

impl Deref for InstructionBytes {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.0[0..self.1 as usize]
    }
}

pub fn encode_instruction(instruction: Instruction) -> InstructionBytes {
    match instruction {
        LD_R_R(ARegister, ARegister) => InstructionBytes::one(0x7f),
        LD_R_R(ARegister, BRegister) => InstructionBytes::one(0x78),
        LD_R_R(ARegister, CRegister) => InstructionBytes::one(0x79),
        LD_R_R(ARegister, DRegister) => InstructionBytes::one(0x7a),
        LD_R_R(ARegister, ERegister) => InstructionBytes::one(0x7b),
        LD_R_R(ARegister, HRegister) => InstructionBytes::one(0x7c),
        LD_R_R(ARegister, LRegister) => InstructionBytes::one(0x7d),

        LD_R_R(BRegister, ARegister) => InstructionBytes::one(0x47),
        LD_R_R(BRegister, BRegister) => InstructionBytes::one(0x40),
        LD_R_R(BRegister, CRegister) => InstructionBytes::one(0x41),
        LD_R_R(BRegister, DRegister) => InstructionBytes::one(0x42),
        LD_R_R(BRegister, ERegister) => InstructionBytes::one(0x43),
        LD_R_R(BRegister, HRegister) => InstructionBytes::one(0x44),
        LD_R_R(BRegister, LRegister) => InstructionBytes::one(0x45),

        LD_R_R(CRegister, ARegister) => InstructionBytes::one(0x4f),
        LD_R_R(CRegister, BRegister) => InstructionBytes::one(0x48),
        LD_R_R(CRegister, CRegister) => InstructionBytes::one(0x49),
        LD_R_R(CRegister, DRegister) => InstructionBytes::one(0x4a),
        LD_R_R(CRegister, ERegister) => InstructionBytes::one(0x4b),
        LD_R_R(CRegister, HRegister) => InstructionBytes::one(0x4c),
        LD_R_R(CRegister, LRegister) => InstructionBytes::one(0x4d),

        LD_R_R(DRegister, ARegister) => InstructionBytes::one(0x57),
        LD_R_R(DRegister, BRegister) => InstructionBytes::one(0x50),
        LD_R_R(DRegister, CRegister) => InstructionBytes::one(0x51),
        LD_R_R(DRegister, DRegister) => InstructionBytes::one(0x52),
        LD_R_R(DRegister, ERegister) => InstructionBytes::one(0x53),
        LD_R_R(DRegister, HRegister) => InstructionBytes::one(0x54),
        LD_R_R(DRegister, LRegister) => InstructionBytes::one(0x55),

        LD_R_R(ERegister, ARegister) => InstructionBytes::one(0x5f),
        LD_R_R(ERegister, BRegister) => InstructionBytes::one(0x58),
        LD_R_R(ERegister, CRegister) => InstructionBytes::one(0x59),
        LD_R_R(ERegister, DRegister) => InstructionBytes::one(0x5a),
        LD_R_R(ERegister, ERegister) => InstructionBytes::one(0x5b),
        LD_R_R(ERegister, HRegister) => InstructionBytes::one(0x5c),
        LD_R_R(ERegister, LRegister) => InstructionBytes::one(0x5d),

        LD_R_R(HRegister, ARegister) => InstructionBytes::one(0x67),
        LD_R_R(HRegister, BRegister) => InstructionBytes::one(0x60),
        LD_R_R(HRegister, CRegister) => InstructionBytes::one(0x61),
        LD_R_R(HRegister, DRegister) => InstructionBytes::one(0x62),
        LD_R_R(HRegister, ERegister) => InstructionBytes::one(0x63),
        LD_R_R(HRegister, HRegister) => InstructionBytes::one(0x64),
        LD_R_R(HRegister, LRegister) => InstructionBytes::one(0x65),

        LD_R_R(LRegister, ARegister) => InstructionBytes::one(0x6f),
        LD_R_R(LRegister, BRegister) => InstructionBytes::one(0x68),
        LD_R_R(LRegister, CRegister) => InstructionBytes::one(0x69),
        LD_R_R(LRegister, DRegister) => InstructionBytes::one(0x6a),
        LD_R_R(LRegister, ERegister) => InstructionBytes::one(0x6b),
        LD_R_R(LRegister, HRegister) => InstructionBytes::one(0x6c),
        LD_R_R(LRegister, LRegister) => InstructionBytes::one(0x6d),

        _ => unimplemented!(),
    }
}
