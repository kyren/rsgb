#![allow(non_camel_case_types)]

#[derive(PartialEq, Eq, Debug)]
pub enum Bit {
    Bit0,
    Bit1,
    Bit2,
    Bit3,
    Bit4,
    Bit5,
    Bit6,
    Bit7,
}
pub use self::Bit::*;

/// All registers that the 8 bit opcodes operate on, excludes the F flag register.
#[derive(PartialEq, Eq, Debug)]
pub enum Register {
    ARegister,
    BRegister,
    CRegister,
    DRegister,
    ERegister,
    HRegister,
    LRegister,
}
pub use self::Register::*;

#[derive(PartialEq, Eq, Debug)]
pub enum Cond {
    Zero,
    NZero,
    Carry,
    NCarry,
}
pub use self::Cond::*;

#[derive(PartialEq, Eq, Debug)]
pub enum ResetAddress {
    Reset00,
    Reset08,
    Reset10,
    Reset18,
    Reset20,
    Reset28,
    Reset30,
    Reset38,
}
pub use self::ResetAddress::*;

#[derive(PartialEq, Eq, Debug)]
pub enum Instruction {
    LD_R_R(Register, Register),
    LD_R_N(Register, u8),
    LD_R_ATHL(Register),

    LD_ATHL_R(Register),
    LD_ATHL_N(u8),

    LD_A_ATC,
    LD_A_ATBC,
    LD_A_ATDE,
    LD_A_ATNN(u16),

    LD_ATC_A,
    LD_ATBC_A,
    LD_ATDE_A,
    LD_ATNN_A(u16),

    LDD_A_ATHL,
    LDD_ATHL_A,

    LDI_A_ATHL,
    LDI_ATHL_A,

    LDH_A_ATN(u8),
    LDH_ATN_A(u8),

    LD_BC_NN(u16),
    LD_DE_NN(u16),
    LD_HL_NN(u16),
    LD_SP_NN(u16),

    LD_SP_HL,
    LDHL_SP_N(i8),
    LD_ATNN_SP(u16),

    PUSH_AF,
    PUSH_BC,
    PUSH_DE,
    PUSH_HL,

    POP_AF,
    POP_BC,
    POP_DE,
    POP_HL,

    ADD_A_R(Register),
    ADD_A_N(u8),
    ADD_A_ATHL,

    ADC_A_R(Register),
    ADC_A_N(u8),
    ADC_A_ATHL,

    SUB_R(Register),
    SUB_N(u8),
    SUB_ATHL,

    SBC_A_R(Register),
    SBC_A_N(u8),
    SBC_A_ATHL,

    AND_R(Register),
    AND_N(u8),
    AND_ATHL,

    OR_R(Register),
    OR_ATHL,
    OR_N(u8),

    XOR_R(Register),
    XOR_N(u8),
    XOR_ATHL,

    CP_R(Register),
    CP_N(u8),
    CP_ATHL,

    INC_R(Register),
    INC_ATHL,

    DEC_R(Register),
    DEC_ATHL,

    ADD_HL_BC,
    ADD_HL_DE,
    ADD_HL_HL,
    ADD_HL_SP,

    ADD_SP_N(i8),

    INC_BC,
    INC_DE,
    INC_HL,
    INC_SP,

    DEC_BC,
    DEC_DE,
    DEC_HL,
    DEC_SP,

    SWAP_R(Register),
    SWAP_ATHL,

    DAA,
    CPL,
    CCF,
    SCF,

    NOP,
    HALT,
    STOP,
    DI,
    EI,

    RLCA,
    RLA,
    RRCA,
    RRA,

    RLC_R(Register),
    RLC_ATHL,

    RL_R(Register),
    RL_ATHL,

    RRC_R(Register),
    RRC_ATHL,

    RR_R(Register),
    RR_ATHL,

    SLA_R(Register),
    SLA_ATHL,

    SRA_R(Register),
    SRA_ATHL,

    SRL_R(Register),
    SRL_ATHL,

    BIT_B_R(Bit, Register),
    BIT_B_ATHL(Bit),

    SET_B_R(Bit, Register),
    SET_B_ATHL(Bit),

    RES_B_R(Bit, Register),
    RES_B_ATHL(Bit),

    JP_NN(u16),
    JP_C_NN(Cond, u16),
    JP_ATHL,

    JR_N(i8),
    JR_C_N(Cond, i8),

    CALL_NN(u16),
    CALL_C_NN(Cond, u16),

    RST_RA(ResetAddress),

    RET,
    RET_C(Cond),

    RETI,
}
pub use self::Instruction::*;
