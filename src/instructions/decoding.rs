use std::error::Error;
use instructions::types::*;

pub fn make_word16(l: u8, h: u8) -> u16 {
    ((h as u16) << 8) | (l as u16)
}

pub fn decode_instruction<I: Iterator<Item = u8>>(mut i: I) -> Result<Instruction, Box<Error>> {
    fn getu<I: Iterator<Item = u8>>(mut i: I) -> Result<u8, Box<Error>> {
        Ok(i.next().ok_or("Decoding error, expected 1 byte unsigned argument")?)
    }

    fn geti<I: Iterator<Item = u8>>(mut i: I) -> Result<i8, Box<Error>> {
        Ok(i.next().ok_or("Decoding error, expected 1 byte signed argument")? as i8)
    }

    fn getuu<I: Iterator<Item = u8>>(mut i: I) -> Result<u16, Box<Error>> {
        let err = "Decoding error, expected 2 byte unsigned argument";
        let l = i.next().ok_or(err)?;
        let h = i.next().ok_or(err)?;
        Ok(make_word16(l, h))
    }

    match i.next() {
        Some(0x7f) => Ok(LD_R_R(ARegister, ARegister)),
        Some(0x78) => Ok(LD_R_R(ARegister, BRegister)),
        Some(0x79) => Ok(LD_R_R(ARegister, CRegister)),
        Some(0x7a) => Ok(LD_R_R(ARegister, DRegister)),
        Some(0x7b) => Ok(LD_R_R(ARegister, ERegister)),
        Some(0x7c) => Ok(LD_R_R(ARegister, HRegister)),
        Some(0x7d) => Ok(LD_R_R(ARegister, LRegister)),

        Some(0x47) => Ok(LD_R_R(BRegister, ARegister)),
        Some(0x40) => Ok(LD_R_R(BRegister, BRegister)),
        Some(0x41) => Ok(LD_R_R(BRegister, CRegister)),
        Some(0x42) => Ok(LD_R_R(BRegister, DRegister)),
        Some(0x43) => Ok(LD_R_R(BRegister, ERegister)),
        Some(0x44) => Ok(LD_R_R(BRegister, HRegister)),
        Some(0x45) => Ok(LD_R_R(BRegister, LRegister)),

        Some(0x4f) => Ok(LD_R_R(CRegister, ARegister)),
        Some(0x48) => Ok(LD_R_R(CRegister, BRegister)),
        Some(0x49) => Ok(LD_R_R(CRegister, CRegister)),
        Some(0x4a) => Ok(LD_R_R(CRegister, DRegister)),
        Some(0x4b) => Ok(LD_R_R(CRegister, ERegister)),
        Some(0x4c) => Ok(LD_R_R(CRegister, HRegister)),
        Some(0x4d) => Ok(LD_R_R(CRegister, LRegister)),

        Some(0x57) => Ok(LD_R_R(DRegister, ARegister)),
        Some(0x50) => Ok(LD_R_R(DRegister, BRegister)),
        Some(0x51) => Ok(LD_R_R(DRegister, CRegister)),
        Some(0x52) => Ok(LD_R_R(DRegister, DRegister)),
        Some(0x53) => Ok(LD_R_R(DRegister, ERegister)),
        Some(0x54) => Ok(LD_R_R(DRegister, HRegister)),
        Some(0x55) => Ok(LD_R_R(DRegister, LRegister)),

        Some(0x5f) => Ok(LD_R_R(ERegister, ARegister)),
        Some(0x58) => Ok(LD_R_R(ERegister, BRegister)),
        Some(0x59) => Ok(LD_R_R(ERegister, CRegister)),
        Some(0x5a) => Ok(LD_R_R(ERegister, DRegister)),
        Some(0x5b) => Ok(LD_R_R(ERegister, ERegister)),
        Some(0x5c) => Ok(LD_R_R(ERegister, HRegister)),
        Some(0x5d) => Ok(LD_R_R(ERegister, LRegister)),

        Some(0x67) => Ok(LD_R_R(HRegister, ARegister)),
        Some(0x60) => Ok(LD_R_R(HRegister, BRegister)),
        Some(0x61) => Ok(LD_R_R(HRegister, CRegister)),
        Some(0x62) => Ok(LD_R_R(HRegister, DRegister)),
        Some(0x63) => Ok(LD_R_R(HRegister, ERegister)),
        Some(0x64) => Ok(LD_R_R(HRegister, HRegister)),
        Some(0x65) => Ok(LD_R_R(HRegister, LRegister)),

        Some(0x6f) => Ok(LD_R_R(LRegister, ARegister)),
        Some(0x68) => Ok(LD_R_R(LRegister, BRegister)),
        Some(0x69) => Ok(LD_R_R(LRegister, CRegister)),
        Some(0x6a) => Ok(LD_R_R(LRegister, DRegister)),
        Some(0x6b) => Ok(LD_R_R(LRegister, ERegister)),
        Some(0x6c) => Ok(LD_R_R(LRegister, HRegister)),
        Some(0x6d) => Ok(LD_R_R(LRegister, LRegister)),

        Some(0x3e) => Ok(LD_R_N(ARegister, getu(i)?)),
        Some(0x06) => Ok(LD_R_N(BRegister, getu(i)?)),
        Some(0x0e) => Ok(LD_R_N(CRegister, getu(i)?)),
        Some(0x16) => Ok(LD_R_N(DRegister, getu(i)?)),
        Some(0x1e) => Ok(LD_R_N(ERegister, getu(i)?)),
        Some(0x26) => Ok(LD_R_N(HRegister, getu(i)?)),
        Some(0x2e) => Ok(LD_R_N(LRegister, getu(i)?)),

        Some(0x7e) => Ok(LD_R_ATHL(ARegister)),
        Some(0x46) => Ok(LD_R_ATHL(BRegister)),
        Some(0x4e) => Ok(LD_R_ATHL(CRegister)),
        Some(0x56) => Ok(LD_R_ATHL(DRegister)),
        Some(0x5e) => Ok(LD_R_ATHL(ERegister)),
        Some(0x66) => Ok(LD_R_ATHL(HRegister)),
        Some(0x6e) => Ok(LD_R_ATHL(LRegister)),

        Some(0x77) => Ok(LD_ATHL_R(ARegister)),
        Some(0x70) => Ok(LD_ATHL_R(BRegister)),
        Some(0x71) => Ok(LD_ATHL_R(CRegister)),
        Some(0x72) => Ok(LD_ATHL_R(DRegister)),
        Some(0x73) => Ok(LD_ATHL_R(ERegister)),
        Some(0x74) => Ok(LD_ATHL_R(HRegister)),
        Some(0x75) => Ok(LD_ATHL_R(LRegister)),

        Some(0x36) => Ok(LD_ATHL_N(getu(i)?)),

        Some(0xf2) => Ok(LD_A_ATC),
        Some(0x0a) => Ok(LD_A_ATBC),
        Some(0x1a) => Ok(LD_A_ATDE),
        Some(0xfa) => Ok(LD_A_ATNN(getuu(i)?)),

        Some(0xe2) => Ok(LD_ATC_A),
        Some(0x02) => Ok(LD_ATBC_A),
        Some(0x12) => Ok(LD_ATDE_A),
        Some(0xea) => Ok(LD_ATNN_A(getuu(i)?)),

        Some(0x3a) => Ok(LDD_A_ATHL),
        Some(0x32) => Ok(LDD_ATHL_A),

        Some(0x2a) => Ok(LDI_A_ATHL),
        Some(0x22) => Ok(LDI_ATHL_A),

        Some(0xf0) => Ok(LDH_A_ATN(getu(i)?)),
        Some(0xe0) => Ok(LDH_ATN_A(getu(i)?)),

        Some(0x01) => Ok(LD_BC_NN(getuu(i)?)),
        Some(0x11) => Ok(LD_DE_NN(getuu(i)?)),
        Some(0x21) => Ok(LD_HL_NN(getuu(i)?)),
        Some(0x31) => Ok(LD_SP_NN(getuu(i)?)),

        Some(0xf9) => Ok(LD_SP_HL),
        Some(0xf8) => Ok(LDHL_SP_N(geti(i)?)),
        Some(0x08) => Ok(LD_ATNN_SP(getuu(i)?)),

        Some(0xf5) => Ok(PUSH_AF),
        Some(0xc5) => Ok(PUSH_BC),
        Some(0xd5) => Ok(PUSH_DE),
        Some(0xe5) => Ok(PUSH_HL),

        Some(0xf1) => Ok(POP_AF),
        Some(0xc1) => Ok(POP_BC),
        Some(0xd1) => Ok(POP_DE),
        Some(0xe1) => Ok(POP_HL),

        Some(0x87) => Ok(ADD_A_R(ARegister)),
        Some(0x80) => Ok(ADD_A_R(BRegister)),
        Some(0x81) => Ok(ADD_A_R(CRegister)),
        Some(0x82) => Ok(ADD_A_R(DRegister)),
        Some(0x83) => Ok(ADD_A_R(ERegister)),
        Some(0x84) => Ok(ADD_A_R(HRegister)),
        Some(0x85) => Ok(ADD_A_R(LRegister)),
        Some(0xc6) => Ok(ADD_A_N(getu(i)?)),
        Some(0x86) => Ok(ADD_A_ATHL),

        Some(0x8f) => Ok(ADC_A_R(ARegister)),
        Some(0x88) => Ok(ADC_A_R(BRegister)),
        Some(0x89) => Ok(ADC_A_R(CRegister)),
        Some(0x8a) => Ok(ADC_A_R(DRegister)),
        Some(0x8b) => Ok(ADC_A_R(ERegister)),
        Some(0x8c) => Ok(ADC_A_R(HRegister)),
        Some(0x8d) => Ok(ADC_A_R(LRegister)),
        Some(0xce) => Ok(ADC_A_N(getu(i)?)),
        Some(0x8e) => Ok(ADC_A_ATHL),

        Some(0x97) => Ok(SUB_R(ARegister)),
        Some(0x90) => Ok(SUB_R(BRegister)),
        Some(0x91) => Ok(SUB_R(CRegister)),
        Some(0x92) => Ok(SUB_R(DRegister)),
        Some(0x93) => Ok(SUB_R(ERegister)),
        Some(0x94) => Ok(SUB_R(HRegister)),
        Some(0x95) => Ok(SUB_R(LRegister)),
        Some(0xd6) => Ok(SUB_N(getu(i)?)),
        Some(0x96) => Ok(SUB_ATHL),

        Some(0x9f) => Ok(SBC_A_R(ARegister)),
        Some(0x98) => Ok(SBC_A_R(BRegister)),
        Some(0x99) => Ok(SBC_A_R(CRegister)),
        Some(0x9a) => Ok(SBC_A_R(DRegister)),
        Some(0x9b) => Ok(SBC_A_R(ERegister)),
        Some(0x9c) => Ok(SBC_A_R(HRegister)),
        Some(0x9d) => Ok(SBC_A_R(LRegister)),
        Some(0xde) => Ok(SBC_A_N(getu(i)?)),
        Some(0x9e) => Ok(SBC_A_ATHL),

        Some(0xa7) => Ok(AND_R(ARegister)),
        Some(0xa0) => Ok(AND_R(BRegister)),
        Some(0xa1) => Ok(AND_R(CRegister)),
        Some(0xa2) => Ok(AND_R(DRegister)),
        Some(0xa3) => Ok(AND_R(ERegister)),
        Some(0xa4) => Ok(AND_R(HRegister)),
        Some(0xa5) => Ok(AND_R(LRegister)),
        Some(0xe6) => Ok(AND_N(getu(i)?)),
        Some(0xa6) => Ok(AND_ATHL),

        Some(0xb7) => Ok(OR_R(ARegister)),
        Some(0xb0) => Ok(OR_R(BRegister)),
        Some(0xb1) => Ok(OR_R(CRegister)),
        Some(0xb2) => Ok(OR_R(DRegister)),
        Some(0xb3) => Ok(OR_R(ERegister)),
        Some(0xb4) => Ok(OR_R(HRegister)),
        Some(0xb5) => Ok(OR_R(LRegister)),
        Some(0xf6) => Ok(OR_N(getu(i)?)),
        Some(0xb6) => Ok(OR_ATHL),

        Some(0xaf) => Ok(XOR_R(ARegister)),
        Some(0xa8) => Ok(XOR_R(BRegister)),
        Some(0xa9) => Ok(XOR_R(CRegister)),
        Some(0xaa) => Ok(XOR_R(DRegister)),
        Some(0xab) => Ok(XOR_R(ERegister)),
        Some(0xac) => Ok(XOR_R(HRegister)),
        Some(0xad) => Ok(XOR_R(LRegister)),
        Some(0xee) => Ok(XOR_N(getu(i)?)),
        Some(0xae) => Ok(XOR_ATHL),

        Some(0xbf) => Ok(CP_R(ARegister)),
        Some(0xb8) => Ok(CP_R(BRegister)),
        Some(0xb9) => Ok(CP_R(CRegister)),
        Some(0xba) => Ok(CP_R(DRegister)),
        Some(0xbb) => Ok(CP_R(ERegister)),
        Some(0xbc) => Ok(CP_R(HRegister)),
        Some(0xbd) => Ok(CP_R(LRegister)),
        Some(0xfe) => Ok(CP_N(getu(i)?)),
        Some(0xbe) => Ok(CP_ATHL),

        Some(0x3c) => Ok(INC_R(ARegister)),
        Some(0x04) => Ok(INC_R(BRegister)),
        Some(0x0c) => Ok(INC_R(CRegister)),
        Some(0x14) => Ok(INC_R(DRegister)),
        Some(0x1c) => Ok(INC_R(ERegister)),
        Some(0x24) => Ok(INC_R(HRegister)),
        Some(0x2c) => Ok(INC_R(LRegister)),
        Some(0x34) => Ok(INC_ATHL),

        Some(0x3d) => Ok(DEC_R(ARegister)),
        Some(0x05) => Ok(DEC_R(BRegister)),
        Some(0x0d) => Ok(DEC_R(CRegister)),
        Some(0x15) => Ok(DEC_R(DRegister)),
        Some(0x1d) => Ok(DEC_R(ERegister)),
        Some(0x25) => Ok(DEC_R(HRegister)),
        Some(0x2d) => Ok(DEC_R(LRegister)),
        Some(0x35) => Ok(DEC_ATHL),

        Some(0x09) => Ok(ADD_HL_BC),
        Some(0x19) => Ok(ADD_HL_DE),
        Some(0x29) => Ok(ADD_HL_HL),
        Some(0x39) => Ok(ADD_HL_SP),

        Some(0xe8) => Ok(ADD_SP_N(geti(i)?)),

        Some(0x03) => Ok(INC_BC),
        Some(0x13) => Ok(INC_DE),
        Some(0x23) => Ok(INC_HL),
        Some(0x33) => Ok(INC_SP),

        Some(0x0b) => Ok(DEC_BC),
        Some(0x1b) => Ok(DEC_DE),
        Some(0x2b) => Ok(DEC_HL),
        Some(0x3b) => Ok(DEC_SP),

        Some(0x27) => Ok(DAA),
        Some(0x2f) => Ok(CPL),
        Some(0x3f) => Ok(CCF),
        Some(0x37) => Ok(SCF),

        Some(0x00) => Ok(NOP),
        Some(0x76) => Ok(HALT),

        Some(0xf3) => Ok(DI),
        Some(0xfb) => Ok(EI),

        Some(0x07) => Ok(RLCA),
        Some(0x17) => Ok(RLA),
        Some(0x0f) => Ok(RRCA),
        Some(0x1f) => Ok(RRA),

        Some(0xc3) => Ok(JP_NN(getuu(i)?)),
        Some(0xc2) => Ok(JP_C_NN(NZero, getuu(i)?)),
        Some(0xca) => Ok(JP_C_NN(Zero, getuu(i)?)),
        Some(0xd2) => Ok(JP_C_NN(NCarry, getuu(i)?)),
        Some(0xda) => Ok(JP_C_NN(Carry, getuu(i)?)),
        Some(0xe9) => Ok(JP_ATHL),

        Some(0x18) => Ok(JR_N(geti(i)?)),
        Some(0x20) => Ok(JR_C_N(NZero, geti(i)?)),
        Some(0x28) => Ok(JR_C_N(Zero, geti(i)?)),
        Some(0x30) => Ok(JR_C_N(NCarry, geti(i)?)),
        Some(0x38) => Ok(JR_C_N(Carry, geti(i)?)),

        Some(0xcd) => Ok(CALL_NN(getuu(i)?)),
        Some(0xc4) => Ok(CALL_C_NN(NZero, getuu(i)?)),
        Some(0xcc) => Ok(CALL_C_NN(Zero, getuu(i)?)),
        Some(0xd4) => Ok(CALL_C_NN(NCarry, getuu(i)?)),
        Some(0xdc) => Ok(CALL_C_NN(Carry, getuu(i)?)),

        Some(0xc7) => Ok(RST_RA(Reset00)),
        Some(0xcf) => Ok(RST_RA(Reset08)),
        Some(0xd7) => Ok(RST_RA(Reset10)),
        Some(0xdf) => Ok(RST_RA(Reset18)),
        Some(0xe7) => Ok(RST_RA(Reset20)),
        Some(0xef) => Ok(RST_RA(Reset28)),
        Some(0xf7) => Ok(RST_RA(Reset30)),
        Some(0xff) => Ok(RST_RA(Reset38)),

        Some(0xc9) => Ok(RET),
        Some(0xc0) => Ok(RET_C(NZero)),
        Some(0xc8) => Ok(RET_C(Zero)),
        Some(0xd0) => Ok(RET_C(NCarry)),
        Some(0xd8) => Ok(RET_C(Carry)),

        Some(0xd9) => Ok(RETI),

        Some(0x10) => {
            match i.next() {
                Some(0x00) => Ok(STOP),
                None => Err("Decoding error, no byte following 0x10".into()),
                _ => Err("Decoding error, improper byte following 0x10".into()),
            }
        }

        Some(0xcb) => {
            match i.next() {
                Some(0x37) => Ok(SWAP_R(ARegister)),
                Some(0x30) => Ok(SWAP_R(BRegister)),
                Some(0x31) => Ok(SWAP_R(CRegister)),
                Some(0x32) => Ok(SWAP_R(DRegister)),
                Some(0x33) => Ok(SWAP_R(ERegister)),
                Some(0x34) => Ok(SWAP_R(HRegister)),
                Some(0x35) => Ok(SWAP_R(LRegister)),
                Some(0x36) => Ok(SWAP_ATHL),

                Some(0x07) => Ok(RLC_R(ARegister)),
                Some(0x00) => Ok(RLC_R(BRegister)),
                Some(0x01) => Ok(RLC_R(CRegister)),
                Some(0x02) => Ok(RLC_R(DRegister)),
                Some(0x03) => Ok(RLC_R(ERegister)),
                Some(0x04) => Ok(RLC_R(HRegister)),
                Some(0x05) => Ok(RLC_R(LRegister)),
                Some(0x06) => Ok(RLC_ATHL),

                Some(0x17) => Ok(RL_R(ARegister)),
                Some(0x10) => Ok(RL_R(BRegister)),
                Some(0x11) => Ok(RL_R(CRegister)),
                Some(0x12) => Ok(RL_R(DRegister)),
                Some(0x13) => Ok(RL_R(ERegister)),
                Some(0x14) => Ok(RL_R(HRegister)),
                Some(0x15) => Ok(RL_R(LRegister)),
                Some(0x16) => Ok(RL_ATHL),

                Some(0x0f) => Ok(RRC_R(ARegister)),
                Some(0x08) => Ok(RRC_R(BRegister)),
                Some(0x09) => Ok(RRC_R(CRegister)),
                Some(0x0a) => Ok(RRC_R(DRegister)),
                Some(0x0b) => Ok(RRC_R(ERegister)),
                Some(0x0c) => Ok(RRC_R(HRegister)),
                Some(0x0d) => Ok(RRC_R(LRegister)),
                Some(0x0e) => Ok(RRC_ATHL),

                Some(0x1f) => Ok(RR_R(ARegister)),
                Some(0x18) => Ok(RR_R(BRegister)),
                Some(0x19) => Ok(RR_R(CRegister)),
                Some(0x1a) => Ok(RR_R(DRegister)),
                Some(0x1b) => Ok(RR_R(ERegister)),
                Some(0x1c) => Ok(RR_R(HRegister)),
                Some(0x1d) => Ok(RR_R(LRegister)),
                Some(0x1e) => Ok(RR_ATHL),

                Some(0x27) => Ok(SLA_R(ARegister)),
                Some(0x20) => Ok(SLA_R(BRegister)),
                Some(0x21) => Ok(SLA_R(CRegister)),
                Some(0x22) => Ok(SLA_R(DRegister)),
                Some(0x23) => Ok(SLA_R(ERegister)),
                Some(0x24) => Ok(SLA_R(HRegister)),
                Some(0x25) => Ok(SLA_R(LRegister)),
                Some(0x26) => Ok(SLA_ATHL),

                Some(0x2f) => Ok(SRA_R(ARegister)),
                Some(0x28) => Ok(SRA_R(BRegister)),
                Some(0x29) => Ok(SRA_R(CRegister)),
                Some(0x2a) => Ok(SRA_R(DRegister)),
                Some(0x2b) => Ok(SRA_R(ERegister)),
                Some(0x2c) => Ok(SRA_R(HRegister)),
                Some(0x2d) => Ok(SRA_R(LRegister)),
                Some(0x2e) => Ok(SRA_ATHL),

                Some(0x3f) => Ok(SRL_R(ARegister)),
                Some(0x38) => Ok(SRL_R(BRegister)),
                Some(0x39) => Ok(SRL_R(CRegister)),
                Some(0x3a) => Ok(SRL_R(DRegister)),
                Some(0x3b) => Ok(SRL_R(ERegister)),
                Some(0x3c) => Ok(SRL_R(HRegister)),
                Some(0x3d) => Ok(SRL_R(LRegister)),
                Some(0x3e) => Ok(SRL_ATHL),

                Some(0x47) => Ok(BIT_B_R(Bit0, ARegister)),
                Some(0x40) => Ok(BIT_B_R(Bit0, BRegister)),
                Some(0x41) => Ok(BIT_B_R(Bit0, CRegister)),
                Some(0x42) => Ok(BIT_B_R(Bit0, DRegister)),
                Some(0x43) => Ok(BIT_B_R(Bit0, ERegister)),
                Some(0x44) => Ok(BIT_B_R(Bit0, HRegister)),
                Some(0x45) => Ok(BIT_B_R(Bit0, LRegister)),
                Some(0x46) => Ok(BIT_B_ATHL(Bit0)),

                Some(0x4f) => Ok(BIT_B_R(Bit1, ARegister)),
                Some(0x48) => Ok(BIT_B_R(Bit1, BRegister)),
                Some(0x49) => Ok(BIT_B_R(Bit1, CRegister)),
                Some(0x4a) => Ok(BIT_B_R(Bit1, DRegister)),
                Some(0x4b) => Ok(BIT_B_R(Bit1, ERegister)),
                Some(0x4c) => Ok(BIT_B_R(Bit1, HRegister)),
                Some(0x4d) => Ok(BIT_B_R(Bit1, LRegister)),
                Some(0x4e) => Ok(BIT_B_ATHL(Bit1)),

                Some(0x57) => Ok(BIT_B_R(Bit2, ARegister)),
                Some(0x50) => Ok(BIT_B_R(Bit2, BRegister)),
                Some(0x51) => Ok(BIT_B_R(Bit2, CRegister)),
                Some(0x52) => Ok(BIT_B_R(Bit2, DRegister)),
                Some(0x53) => Ok(BIT_B_R(Bit2, ERegister)),
                Some(0x54) => Ok(BIT_B_R(Bit2, HRegister)),
                Some(0x55) => Ok(BIT_B_R(Bit2, LRegister)),
                Some(0x56) => Ok(BIT_B_ATHL(Bit2)),

                Some(0x5f) => Ok(BIT_B_R(Bit3, ARegister)),
                Some(0x58) => Ok(BIT_B_R(Bit3, BRegister)),
                Some(0x59) => Ok(BIT_B_R(Bit3, CRegister)),
                Some(0x5a) => Ok(BIT_B_R(Bit3, DRegister)),
                Some(0x5b) => Ok(BIT_B_R(Bit3, ERegister)),
                Some(0x5c) => Ok(BIT_B_R(Bit3, HRegister)),
                Some(0x5d) => Ok(BIT_B_R(Bit3, LRegister)),
                Some(0x5e) => Ok(BIT_B_ATHL(Bit3)),

                Some(0x67) => Ok(BIT_B_R(Bit4, ARegister)),
                Some(0x60) => Ok(BIT_B_R(Bit4, BRegister)),
                Some(0x61) => Ok(BIT_B_R(Bit4, CRegister)),
                Some(0x62) => Ok(BIT_B_R(Bit4, DRegister)),
                Some(0x63) => Ok(BIT_B_R(Bit4, ERegister)),
                Some(0x64) => Ok(BIT_B_R(Bit4, HRegister)),
                Some(0x65) => Ok(BIT_B_R(Bit4, LRegister)),
                Some(0x66) => Ok(BIT_B_ATHL(Bit4)),

                Some(0x6f) => Ok(BIT_B_R(Bit5, ARegister)),
                Some(0x68) => Ok(BIT_B_R(Bit5, BRegister)),
                Some(0x69) => Ok(BIT_B_R(Bit5, CRegister)),
                Some(0x6a) => Ok(BIT_B_R(Bit5, DRegister)),
                Some(0x6b) => Ok(BIT_B_R(Bit5, ERegister)),
                Some(0x6c) => Ok(BIT_B_R(Bit5, HRegister)),
                Some(0x6d) => Ok(BIT_B_R(Bit5, LRegister)),
                Some(0x6e) => Ok(BIT_B_ATHL(Bit5)),

                Some(0x77) => Ok(BIT_B_R(Bit6, ARegister)),
                Some(0x70) => Ok(BIT_B_R(Bit6, BRegister)),
                Some(0x71) => Ok(BIT_B_R(Bit6, CRegister)),
                Some(0x72) => Ok(BIT_B_R(Bit6, DRegister)),
                Some(0x73) => Ok(BIT_B_R(Bit6, ERegister)),
                Some(0x74) => Ok(BIT_B_R(Bit6, HRegister)),
                Some(0x75) => Ok(BIT_B_R(Bit6, LRegister)),
                Some(0x76) => Ok(BIT_B_ATHL(Bit6)),

                Some(0x7f) => Ok(BIT_B_R(Bit7, ARegister)),
                Some(0x78) => Ok(BIT_B_R(Bit7, BRegister)),
                Some(0x79) => Ok(BIT_B_R(Bit7, CRegister)),
                Some(0x7a) => Ok(BIT_B_R(Bit7, DRegister)),
                Some(0x7b) => Ok(BIT_B_R(Bit7, ERegister)),
                Some(0x7c) => Ok(BIT_B_R(Bit7, HRegister)),
                Some(0x7d) => Ok(BIT_B_R(Bit7, LRegister)),
                Some(0x7e) => Ok(BIT_B_ATHL(Bit7)),

                Some(0xc7) => Ok(SET_B_R(Bit0, ARegister)),
                Some(0xc0) => Ok(SET_B_R(Bit0, BRegister)),
                Some(0xc1) => Ok(SET_B_R(Bit0, CRegister)),
                Some(0xc2) => Ok(SET_B_R(Bit0, DRegister)),
                Some(0xc3) => Ok(SET_B_R(Bit0, ERegister)),
                Some(0xc4) => Ok(SET_B_R(Bit0, HRegister)),
                Some(0xc5) => Ok(SET_B_R(Bit0, LRegister)),
                Some(0xc6) => Ok(SET_B_ATHL(Bit0)),

                Some(0xcf) => Ok(SET_B_R(Bit1, ARegister)),
                Some(0xc8) => Ok(SET_B_R(Bit1, BRegister)),
                Some(0xc9) => Ok(SET_B_R(Bit1, CRegister)),
                Some(0xca) => Ok(SET_B_R(Bit1, DRegister)),
                Some(0xcb) => Ok(SET_B_R(Bit1, ERegister)),
                Some(0xcc) => Ok(SET_B_R(Bit1, HRegister)),
                Some(0xcd) => Ok(SET_B_R(Bit1, LRegister)),
                Some(0xce) => Ok(SET_B_ATHL(Bit1)),

                Some(0xd7) => Ok(SET_B_R(Bit2, ARegister)),
                Some(0xd0) => Ok(SET_B_R(Bit2, BRegister)),
                Some(0xd1) => Ok(SET_B_R(Bit2, CRegister)),
                Some(0xd2) => Ok(SET_B_R(Bit2, DRegister)),
                Some(0xd3) => Ok(SET_B_R(Bit2, ERegister)),
                Some(0xd4) => Ok(SET_B_R(Bit2, HRegister)),
                Some(0xd5) => Ok(SET_B_R(Bit2, LRegister)),
                Some(0xd6) => Ok(SET_B_ATHL(Bit2)),

                Some(0xdf) => Ok(SET_B_R(Bit3, ARegister)),
                Some(0xd8) => Ok(SET_B_R(Bit3, BRegister)),
                Some(0xd9) => Ok(SET_B_R(Bit3, CRegister)),
                Some(0xda) => Ok(SET_B_R(Bit3, DRegister)),
                Some(0xdb) => Ok(SET_B_R(Bit3, ERegister)),
                Some(0xdc) => Ok(SET_B_R(Bit3, HRegister)),
                Some(0xdd) => Ok(SET_B_R(Bit3, LRegister)),
                Some(0xde) => Ok(SET_B_ATHL(Bit3)),

                Some(0xe7) => Ok(SET_B_R(Bit4, ARegister)),
                Some(0xe0) => Ok(SET_B_R(Bit4, BRegister)),
                Some(0xe1) => Ok(SET_B_R(Bit4, CRegister)),
                Some(0xe2) => Ok(SET_B_R(Bit4, DRegister)),
                Some(0xe3) => Ok(SET_B_R(Bit4, ERegister)),
                Some(0xe4) => Ok(SET_B_R(Bit4, HRegister)),
                Some(0xe5) => Ok(SET_B_R(Bit4, LRegister)),
                Some(0xe6) => Ok(SET_B_ATHL(Bit4)),

                Some(0xef) => Ok(SET_B_R(Bit5, ARegister)),
                Some(0xe8) => Ok(SET_B_R(Bit5, BRegister)),
                Some(0xe9) => Ok(SET_B_R(Bit5, CRegister)),
                Some(0xea) => Ok(SET_B_R(Bit5, DRegister)),
                Some(0xeb) => Ok(SET_B_R(Bit5, ERegister)),
                Some(0xec) => Ok(SET_B_R(Bit5, HRegister)),
                Some(0xed) => Ok(SET_B_R(Bit5, LRegister)),
                Some(0xee) => Ok(SET_B_ATHL(Bit5)),

                Some(0xf7) => Ok(SET_B_R(Bit6, ARegister)),
                Some(0xf0) => Ok(SET_B_R(Bit6, BRegister)),
                Some(0xf1) => Ok(SET_B_R(Bit6, CRegister)),
                Some(0xf2) => Ok(SET_B_R(Bit6, DRegister)),
                Some(0xf3) => Ok(SET_B_R(Bit6, ERegister)),
                Some(0xf4) => Ok(SET_B_R(Bit6, HRegister)),
                Some(0xf5) => Ok(SET_B_R(Bit6, LRegister)),
                Some(0xf6) => Ok(SET_B_ATHL(Bit6)),

                Some(0xff) => Ok(SET_B_R(Bit7, ARegister)),
                Some(0xf8) => Ok(SET_B_R(Bit7, BRegister)),
                Some(0xf9) => Ok(SET_B_R(Bit7, CRegister)),
                Some(0xfa) => Ok(SET_B_R(Bit7, DRegister)),
                Some(0xfb) => Ok(SET_B_R(Bit7, ERegister)),
                Some(0xfc) => Ok(SET_B_R(Bit7, HRegister)),
                Some(0xfd) => Ok(SET_B_R(Bit7, LRegister)),
                Some(0xfe) => Ok(SET_B_ATHL(Bit7)),

                Some(0x87) => Ok(RES_B_R(Bit0, ARegister)),
                Some(0x80) => Ok(RES_B_R(Bit0, BRegister)),
                Some(0x81) => Ok(RES_B_R(Bit0, CRegister)),
                Some(0x82) => Ok(RES_B_R(Bit0, DRegister)),
                Some(0x83) => Ok(RES_B_R(Bit0, ERegister)),
                Some(0x84) => Ok(RES_B_R(Bit0, HRegister)),
                Some(0x85) => Ok(RES_B_R(Bit0, LRegister)),
                Some(0x86) => Ok(RES_B_ATHL(Bit0)),

                Some(0x8f) => Ok(RES_B_R(Bit1, ARegister)),
                Some(0x88) => Ok(RES_B_R(Bit1, BRegister)),
                Some(0x89) => Ok(RES_B_R(Bit1, CRegister)),
                Some(0x8a) => Ok(RES_B_R(Bit1, DRegister)),
                Some(0x8b) => Ok(RES_B_R(Bit1, ERegister)),
                Some(0x8c) => Ok(RES_B_R(Bit1, HRegister)),
                Some(0x8d) => Ok(RES_B_R(Bit1, LRegister)),
                Some(0x8e) => Ok(RES_B_ATHL(Bit1)),

                Some(0x97) => Ok(RES_B_R(Bit2, ARegister)),
                Some(0x90) => Ok(RES_B_R(Bit2, BRegister)),
                Some(0x91) => Ok(RES_B_R(Bit2, CRegister)),
                Some(0x92) => Ok(RES_B_R(Bit2, DRegister)),
                Some(0x93) => Ok(RES_B_R(Bit2, ERegister)),
                Some(0x94) => Ok(RES_B_R(Bit2, HRegister)),
                Some(0x95) => Ok(RES_B_R(Bit2, LRegister)),
                Some(0x96) => Ok(RES_B_ATHL(Bit2)),

                Some(0x9f) => Ok(RES_B_R(Bit3, ARegister)),
                Some(0x98) => Ok(RES_B_R(Bit3, BRegister)),
                Some(0x99) => Ok(RES_B_R(Bit3, CRegister)),
                Some(0x9a) => Ok(RES_B_R(Bit3, DRegister)),
                Some(0x9b) => Ok(RES_B_R(Bit3, ERegister)),
                Some(0x9c) => Ok(RES_B_R(Bit3, HRegister)),
                Some(0x9d) => Ok(RES_B_R(Bit3, LRegister)),
                Some(0x9e) => Ok(RES_B_ATHL(Bit3)),

                Some(0xa7) => Ok(RES_B_R(Bit4, ARegister)),
                Some(0xa0) => Ok(RES_B_R(Bit4, BRegister)),
                Some(0xa1) => Ok(RES_B_R(Bit4, CRegister)),
                Some(0xa2) => Ok(RES_B_R(Bit4, DRegister)),
                Some(0xa3) => Ok(RES_B_R(Bit4, ERegister)),
                Some(0xa4) => Ok(RES_B_R(Bit4, HRegister)),
                Some(0xa5) => Ok(RES_B_R(Bit4, LRegister)),
                Some(0xa6) => Ok(RES_B_ATHL(Bit4)),

                Some(0xaf) => Ok(RES_B_R(Bit5, ARegister)),
                Some(0xa8) => Ok(RES_B_R(Bit5, BRegister)),
                Some(0xa9) => Ok(RES_B_R(Bit5, CRegister)),
                Some(0xaa) => Ok(RES_B_R(Bit5, DRegister)),
                Some(0xab) => Ok(RES_B_R(Bit5, ERegister)),
                Some(0xac) => Ok(RES_B_R(Bit5, HRegister)),
                Some(0xad) => Ok(RES_B_R(Bit5, LRegister)),
                Some(0xae) => Ok(RES_B_ATHL(Bit5)),

                Some(0xb7) => Ok(RES_B_R(Bit6, ARegister)),
                Some(0xb0) => Ok(RES_B_R(Bit6, BRegister)),
                Some(0xb1) => Ok(RES_B_R(Bit6, CRegister)),
                Some(0xb2) => Ok(RES_B_R(Bit6, DRegister)),
                Some(0xb3) => Ok(RES_B_R(Bit6, ERegister)),
                Some(0xb4) => Ok(RES_B_R(Bit6, HRegister)),
                Some(0xb5) => Ok(RES_B_R(Bit6, LRegister)),
                Some(0xb6) => Ok(RES_B_ATHL(Bit6)),

                Some(0xbf) => Ok(RES_B_R(Bit7, ARegister)),
                Some(0xb8) => Ok(RES_B_R(Bit7, BRegister)),
                Some(0xb9) => Ok(RES_B_R(Bit7, CRegister)),
                Some(0xba) => Ok(RES_B_R(Bit7, DRegister)),
                Some(0xbb) => Ok(RES_B_R(Bit7, ERegister)),
                Some(0xbc) => Ok(RES_B_R(Bit7, HRegister)),
                Some(0xbd) => Ok(RES_B_R(Bit7, LRegister)),
                Some(0xbe) => Ok(RES_B_ATHL(Bit7)),

                None => Err("No byte following 0xcb".into()),
                _ => Err("Decoding error, improper byte following 0xcb".into()),
            }
        }

        None => Err("No first byte of instruction available".into()),
        _ => Err("Decoding error, improper first byte".into()),
    }
}
