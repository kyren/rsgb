use instructions::types::*;

pub fn decode_instruction<I: Iterator<Item = u8>>(mut i: I) -> Option<Instruction> {
    match i.next() {
        None => None,

        Some(0x7f) => Some(LD_R_R(ARegister, ARegister)),
        Some(0x78) => Some(LD_R_R(ARegister, BRegister)),
        Some(0x79) => Some(LD_R_R(ARegister, CRegister)),
        Some(0x7a) => Some(LD_R_R(ARegister, DRegister)),
        Some(0x7b) => Some(LD_R_R(ARegister, ERegister)),
        Some(0x7c) => Some(LD_R_R(ARegister, HRegister)),
        Some(0x7d) => Some(LD_R_R(ARegister, LRegister)),

        Some(0x47) => Some(LD_R_R(BRegister, ARegister)),
        Some(0x40) => Some(LD_R_R(BRegister, BRegister)),
        Some(0x41) => Some(LD_R_R(BRegister, CRegister)),
        Some(0x42) => Some(LD_R_R(BRegister, DRegister)),
        Some(0x43) => Some(LD_R_R(BRegister, ERegister)),
        Some(0x44) => Some(LD_R_R(BRegister, HRegister)),
        Some(0x45) => Some(LD_R_R(BRegister, LRegister)),

        Some(0x4f) => Some(LD_R_R(CRegister, ARegister)),
        Some(0x48) => Some(LD_R_R(CRegister, BRegister)),
        Some(0x49) => Some(LD_R_R(CRegister, CRegister)),
        Some(0x4a) => Some(LD_R_R(CRegister, DRegister)),
        Some(0x4b) => Some(LD_R_R(CRegister, ERegister)),
        Some(0x4c) => Some(LD_R_R(CRegister, HRegister)),
        Some(0x4d) => Some(LD_R_R(CRegister, LRegister)),

        Some(0x57) => Some(LD_R_R(DRegister, ARegister)),
        Some(0x50) => Some(LD_R_R(DRegister, BRegister)),
        Some(0x51) => Some(LD_R_R(DRegister, CRegister)),
        Some(0x52) => Some(LD_R_R(DRegister, DRegister)),
        Some(0x53) => Some(LD_R_R(DRegister, ERegister)),
        Some(0x54) => Some(LD_R_R(DRegister, HRegister)),
        Some(0x55) => Some(LD_R_R(DRegister, LRegister)),

        Some(0x5f) => Some(LD_R_R(ERegister, ARegister)),
        Some(0x58) => Some(LD_R_R(ERegister, BRegister)),
        Some(0x59) => Some(LD_R_R(ERegister, CRegister)),
        Some(0x5a) => Some(LD_R_R(ERegister, DRegister)),
        Some(0x5b) => Some(LD_R_R(ERegister, ERegister)),
        Some(0x5c) => Some(LD_R_R(ERegister, HRegister)),
        Some(0x5d) => Some(LD_R_R(ERegister, LRegister)),

        Some(0x67) => Some(LD_R_R(HRegister, ARegister)),
        Some(0x60) => Some(LD_R_R(HRegister, BRegister)),
        Some(0x61) => Some(LD_R_R(HRegister, CRegister)),
        Some(0x62) => Some(LD_R_R(HRegister, DRegister)),
        Some(0x63) => Some(LD_R_R(HRegister, ERegister)),
        Some(0x64) => Some(LD_R_R(HRegister, HRegister)),
        Some(0x65) => Some(LD_R_R(HRegister, LRegister)),

        Some(0x6f) => Some(LD_R_R(LRegister, ARegister)),
        Some(0x68) => Some(LD_R_R(LRegister, BRegister)),
        Some(0x69) => Some(LD_R_R(LRegister, CRegister)),
        Some(0x6a) => Some(LD_R_R(LRegister, DRegister)),
        Some(0x6b) => Some(LD_R_R(LRegister, ERegister)),
        Some(0x6c) => Some(LD_R_R(LRegister, HRegister)),
        Some(0x6d) => Some(LD_R_R(LRegister, LRegister)),

        _ => unimplemented!(),
    }
}
