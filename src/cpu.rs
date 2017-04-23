use util::*;
use instruction::*;
use decoding::*;

pub trait Cpu {
    fn get_register(&self, reg: Register) -> u8;
    fn set_register(&mut self, reg: Register, val: u8);

    fn get_flags_register(&self) -> u8;
    fn set_flags_register(&mut self, flags: u8);

    fn get_program_counter(&self) -> u16;
    fn set_program_counter(&mut self, pc: u16);

    fn get_stack_pointer(&self) -> u16;
    fn set_stack_pointer(&mut self, sp: u16);

    fn tick(&mut self, count: u8);

    fn halt(&mut self);
    fn stop(&mut self);

    fn set_interrupts_enabled(&mut self, enabled: bool);

    fn get_memory(&self, addr: u16) -> Result<u8>;
    fn set_memory(&mut self, addr: u16, n: u8) -> Result<()>;
}

pub fn step_cpu<C: Cpu>(cpu: &mut C) -> Result<()> {
    let instruction = decode_instruction(|| {
                                             let pc = cpu.get_program_counter();
                                             cpu.set_program_counter(pc.checked_add(1).ok_or("program counter wrapped at 0xffff")?);
                                             cpu.get_memory(pc)
                                         })?;

    match instruction {
        LD_R_R(tr, sr) => {
            cpu.tick(4);
            let sv = cpu.get_register(sr);
            cpu.set_register(tr, sv);
        }
        LD_R_N(tr, n) => {
            cpu.tick(8);
            cpu.set_register(tr, n);
        }
        LD_R_ATHL(tr) => {
            cpu.tick(8);
            let v = get_athl(cpu)?;
            cpu.set_register(tr, v);
        }

        LD_ATHL_R(sr) => {
            cpu.tick(8);
            let v = cpu.get_register(sr);
            set_athl(cpu, v)?;
        }
        LD_ATHL_N(n) => {
            cpu.tick(8);
            set_athl(cpu, n)?;
        }

        LD_A_ATC => {
            cpu.tick(8);
            let v = get_atc(cpu)?;
            cpu.set_register(ARegister, v);
        }
        LD_A_ATBC => {
            cpu.tick(8);
            let v = get_atbc(cpu)?;
            cpu.set_register(ARegister, v);
        }
        LD_A_ATDE => {
            cpu.tick(8);
            let v = get_atde(cpu)?;
            cpu.set_register(ARegister, v);
        }
        LD_A_ATNN(nn) => {
            cpu.tick(16);
            let v = cpu.get_memory(nn)?;
            cpu.set_register(ARegister, v);
        }

        LD_ATC_A => {
            cpu.tick(8);
            let v = cpu.get_register(ARegister);
            set_atc(cpu, v)?;
        }
        LD_ATBC_A => {
            cpu.tick(8);
            let v = cpu.get_register(ARegister);
            set_atbc(cpu, v)?;
        }
        LD_ATDE_A => {
            cpu.tick(8);
            let v = cpu.get_register(ARegister);
            set_atde(cpu, v)?;
        }
        LD_ATNN_A(nn) => {
            cpu.tick(16);
            let v = cpu.get_register(ARegister);
            cpu.set_memory(nn, v)?;
        }

        LDD_A_ATHL => {
            cpu.tick(8);
            let hl = get_hl(cpu);
            let v = cpu.get_memory(hl)?;
            cpu.set_register(ARegister, v);
            set_hl(cpu, hl - 1);
        }
        LDD_ATHL_A => {
            cpu.tick(8);
            let hl = get_hl(cpu);
            let v = cpu.get_register(ARegister);
            cpu.set_memory(hl, v)?;
            set_hl(cpu, hl - 1);
        }

        LDI_A_ATHL => {
            cpu.tick(8);
            let hl = get_hl(cpu);
            let v = cpu.get_memory(hl)?;
            cpu.set_register(ARegister, v);
            set_hl(cpu, hl + 1);
        }
        LDI_ATHL_A => {
            cpu.tick(8);
            let hl = get_hl(cpu);
            let v = cpu.get_register(ARegister);
            cpu.set_memory(hl, v)?;
            set_hl(cpu, hl + 1);
        }

        LDH_A_ATN(n) => {
            cpu.tick(12);
            let v = cpu.get_memory(make_word16(0xff, n))?;
            cpu.set_register(ARegister, v);
        }
        LDH_ATN_A(n) => {
            cpu.tick(12);
            let v = cpu.get_register(ARegister);
            cpu.set_memory(make_word16(0xff, n), v)?;
        }

        LD_BC_NN(nn) => {
            cpu.tick(12);
            set_bc(cpu, nn);
        }
        LD_DE_NN(nn) => {
            cpu.tick(12);
            set_de(cpu, nn);
        }
        LD_HL_NN(nn) => {
            cpu.tick(12);
            set_hl(cpu, nn);
        }
        LD_SP_NN(nn) => {
            cpu.tick(12);
            cpu.set_stack_pointer(nn);
        }

        LD_SP_HL => {
            cpu.tick(8);
            let sp = cpu.get_stack_pointer();
            set_hl(cpu, sp);
        }
        LDHL_SP_N(n) => {
            cpu.tick(12);
            let sp = cpu.get_stack_pointer();
            set_hl(cpu, ((sp as i32) + (n as i32)) as u16);
        }
        LD_ATNN_SP(nn) => {
            cpu.tick(20);
            let sp = cpu.get_stack_pointer();
            set_memory16(cpu, nn, sp)?;
        }

        PUSH_AF => {
            cpu.tick(16);
            let af = get_af(cpu);
            push_stack16(cpu, af)?;
        }
        PUSH_BC => {
            cpu.tick(16);
            let bc = get_bc(cpu);
            push_stack16(cpu, bc)?;
        }
        PUSH_DE => {
            cpu.tick(16);
            let de = get_de(cpu);
            push_stack16(cpu, de)?;
        }
        PUSH_HL => {
            cpu.tick(16);
            let hl = get_hl(cpu);
            push_stack16(cpu, hl)?;
        }

        POP_AF => {
            cpu.tick(12);
            let nn = pop_stack16(cpu)?;
            set_af(cpu, nn);
        }
        POP_BC => {
            cpu.tick(12);
            let nn = pop_stack16(cpu)?;
            set_bc(cpu, nn);
        }
        POP_DE => {
            cpu.tick(12);
            let nn = pop_stack16(cpu)?;
            set_de(cpu, nn);
        }
        POP_HL => {
            cpu.tick(12);
            let nn = pop_stack16(cpu)?;
            set_hl(cpu, nn);
        }

        ADD_A_R(r) => {
            cpu.tick(4);
            let n = cpu.get_register(r);
            add_a(cpu, n);
        }
        ADD_A_N(n) => {
            cpu.tick(8);
            add_a(cpu, n);
        }
        ADD_A_ATHL => {
            cpu.tick(8);
            let athl = get_athl(cpu)?;
            add_a(cpu, athl);
        }

        ADC_A_R(r) => {
            cpu.tick(4);
            let n = cpu.get_register(r);
            add_ca(cpu, n);
        }
        ADC_A_N(n) => {
            cpu.tick(8);
            add_ca(cpu, n);
        }
        ADC_A_ATHL => {
            cpu.tick(8);
            let athl = get_athl(cpu)?;
            add_ca(cpu, athl);
        }

        SUB_R(r) => {
            cpu.tick(4);
            let n = cpu.get_register(r);
            sub_a(cpu, n);
        }
        SUB_N(n) => {
            cpu.tick(8);
            sub_a(cpu, n);
        }
        SUB_ATHL => {
            cpu.tick(8);
            let athl = get_athl(cpu)?;
            sub_a(cpu, athl);
        }

        SBC_A_R(r) => {
            cpu.tick(4);
            let n = cpu.get_register(r);
            sub_ca(cpu, n);
        }
        SBC_A_N(n) => {
            cpu.tick(8);
            sub_ca(cpu, n);
        }
        SBC_A_ATHL => {
            cpu.tick(8);
            let athl = get_athl(cpu)?;
            sub_ca(cpu, athl);
        }

        AND_R(r) => {
            cpu.tick(4);
            let r = cpu.get_register(r);
            do_and_a(cpu, r);
        }
        AND_N(n) => {
            cpu.tick(8);
            do_and_a(cpu, n);
        }
        AND_ATHL => {
            cpu.tick(8);
            let v = get_athl(cpu)?;
            do_and_a(cpu, v);
        }

        OR_R(r) => {
            cpu.tick(4);
            let r = cpu.get_register(r);
            do_or_a(cpu, r);
        }
        OR_N(n) => {
            cpu.tick(8);
            do_or_a(cpu, n);
        }
        OR_ATHL => {
            cpu.tick(8);
            let v = get_athl(cpu)?;
            do_or_a(cpu, v);
        }

        XOR_R(r) => {
            cpu.tick(4);
            let r = cpu.get_register(r);
            do_xor_a(cpu, r);
        }
        XOR_N(n) => {
            cpu.tick(8);
            do_xor_a(cpu, n);
        }
        XOR_ATHL => {
            cpu.tick(8);
            let athl = get_athl(cpu)?;
            do_xor_a(cpu, athl);
        }

        CP_R(r) => {
            cpu.tick(4);
            let n = cpu.get_register(r);
            do_cp_a(cpu, n);
        }
        CP_N(n) => {
            cpu.tick(8);
            do_cp_a(cpu, n);
        }
        CP_ATHL => {
            cpu.tick(8);
            let athl = get_athl(cpu)?;
            do_cp_a(cpu, athl);
        }

        INC_R(r) => {
            cpu.tick(4);
            let v = cpu.get_register(r);
            let (res, h, _) = add8(v, 1);
            set_flags(cpu, &[(Flag::Z, res == 0), (Flag::N, false), (Flag::H, h)]);
            cpu.set_register(r, res);
        }
        INC_ATHL => {
            cpu.tick(12);
            let v = get_athl(cpu)?;
            let (res, h, _) = add8(v, 1);
            set_flags(cpu, &[(Flag::Z, res == 0), (Flag::N, false), (Flag::H, h)]);
            set_athl(cpu, res)?;
        }

        DEC_R(r) => {
            cpu.tick(4);
            let v = cpu.get_register(r);
            let (res, h, _) = sub8(v, 1);
            set_flags(cpu, &[(Flag::Z, res == 0), (Flag::N, true), (Flag::H, h)]);
            cpu.set_register(r, res);
        }
        DEC_ATHL => {
            cpu.tick(12);
            let v = get_athl(cpu)?;
            let (res, h, _) = sub8(v, 1);
            set_flags(cpu, &[(Flag::Z, res == 0), (Flag::N, true), (Flag::H, h)]);
            set_athl(cpu, res)?;
        }

        ADD_HL_BC => {
            cpu.tick(8);
            let bc = get_bc(cpu);
            do_add_hl(cpu, bc);
        }
        ADD_HL_DE => {
            cpu.tick(8);
            let de = get_de(cpu);
            do_add_hl(cpu, de);
        }
        ADD_HL_HL => {
            cpu.tick(8);
            let hl = get_hl(cpu);
            do_add_hl(cpu, hl);
        }
        ADD_HL_SP => {
            cpu.tick(8);
            let sp = cpu.get_stack_pointer();
            do_add_hl(cpu, sp);
        }

        ADD_SP_N(n) => {
            cpu.tick(16);
            let sp = cpu.get_stack_pointer();
            let (sp, h, c) = add16(sp, n as u16);
            set_flags(cpu,
                      &[(Flag::Z, false),
                        (Flag::N, false),
                        (Flag::H, h),
                        (Flag::C, c)]);
            cpu.set_stack_pointer(sp);
        }

        INC_BC => {
            cpu.tick(8);
            let hl = get_bc(cpu);
            set_bc(cpu, hl.wrapping_add(1));
        }
        INC_DE => {
            cpu.tick(8);
            let hl = get_de(cpu);
            set_de(cpu, hl.wrapping_add(1));
        }
        INC_HL => {
            cpu.tick(8);
            let hl = get_hl(cpu);
            set_hl(cpu, hl.wrapping_add(1));
        }
        INC_SP => {
            cpu.tick(8);
            let sp = cpu.get_stack_pointer();
            cpu.set_stack_pointer(sp.wrapping_add(1));
        }

        DEC_BC => {
            cpu.tick(8);
            let hl = get_bc(cpu);
            set_bc(cpu, hl.wrapping_sub(1));
        }
        DEC_DE => {
            cpu.tick(8);
            let hl = get_de(cpu);
            set_de(cpu, hl.wrapping_sub(1));
        }
        DEC_HL => {
            cpu.tick(8);
            let hl = get_hl(cpu);
            set_hl(cpu, hl.wrapping_sub(1));
        }
        DEC_SP => {
            cpu.tick(8);
            let sp = cpu.get_stack_pointer();
            cpu.set_stack_pointer(sp.wrapping_sub(1));
        }

        SWAP_R(r) => {
            cpu.tick(8);
            let v = cpu.get_register(r);
            let v = make_word8(low_nibble(v), high_nibble(v));
            cpu.set_register(r, v);
        }
        SWAP_ATHL => {
            cpu.tick(16);
            let v = get_athl(cpu)?;
            let v = make_word8(low_nibble(v), high_nibble(v));
            set_athl(cpu, v)?;
        }

        DAA => {
            return Err("DAA is unimplemented".into());
        }
        CPL => {
            cpu.tick(4);
            let a = cpu.get_register(ARegister);
            cpu.set_register(ARegister, !a);
            set_flags(cpu, &[(Flag::N, true), (Flag::H, true)]);
        }
        CCF => {
            cpu.tick(4);
            let c = get_flag(cpu, Flag::C);
            set_flags(cpu, &[(Flag::N, true), (Flag::H, true), (Flag::C, !c)]);
        }
        SCF => {
            cpu.tick(4);
            set_flags(cpu, &[(Flag::N, false), (Flag::H, false), (Flag::C, true)]);
        }

        NOP => {
            cpu.tick(4);
        }
        HALT => {
            cpu.tick(4);
            cpu.halt();
        }
        STOP => {
            cpu.tick(4);
            cpu.stop();
        }
        DI => {
            cpu.tick(4);
            cpu.set_interrupts_enabled(false);
        }
        EI => {
            cpu.tick(4);
            cpu.set_interrupts_enabled(true);
        }

        RLCA => {
            cpu.tick(4);
            do_rlc(cpu, ARegister);
        }
        RLA => {
            cpu.tick(4);
            do_rl(cpu, ARegister);
        }
        RRCA => {
            cpu.tick(4);
            do_rrc(cpu, ARegister);
        }
        RRA => {
            cpu.tick(4);
            do_rr(cpu, ARegister);
        }

        RLC_R(r) => {
            cpu.tick(8);
            do_rlc(cpu, r);
        }
        RLC_ATHL => {
            cpu.tick(16);
            let v = get_athl(cpu)?;
            let (v, c) = rotlc(v);
            set_athl(cpu, v)?;
            set_flags(cpu,
                      &[(Flag::Z, v == 0),
                        (Flag::N, false),
                        (Flag::H, false),
                        (Flag::C, c)]);
        }

        RL_R(r) => {
            cpu.tick(8);
            do_rl(cpu, r);
        }
        RL_ATHL => {
            cpu.tick(16);
            let v = get_athl(cpu)?;
            let c = get_flag(cpu, Flag::C);
            let (v, c) = rotl(v, c);
            set_athl(cpu, v)?;
            set_flags(cpu,
                      &[(Flag::Z, v == 0),
                        (Flag::N, false),
                        (Flag::H, false),
                        (Flag::C, c)]);
        }

        RRC_R(r) => {
            cpu.tick(8);
            do_rrc(cpu, r);
        }
        RRC_ATHL => {
            cpu.tick(16);
            let v = get_athl(cpu)?;
            let (v, c) = rotrc(v);
            set_athl(cpu, v)?;
            set_flags(cpu,
                      &[(Flag::Z, v == 0),
                        (Flag::N, false),
                        (Flag::H, false),
                        (Flag::C, c)]);
        }

        RR_R(r) => {
            cpu.tick(8);
            do_rr(cpu, r);
        }
        RR_ATHL => {
            cpu.tick(16);
            let v = get_athl(cpu)?;
            let c = get_flag(cpu, Flag::C);
            let (v, c) = rotr(v, c);
            set_athl(cpu, v)?;
            set_flags(cpu,
                      &[(Flag::Z, v == 0),
                        (Flag::N, false),
                        (Flag::H, false),
                        (Flag::C, c)]);
        }

        SLA_R(r) => {
            cpu.tick(8);
            let v = cpu.get_register(r);
            let c = get_bit(v, 7);
            let v = v << 1;
            cpu.set_register(r, v);
            set_flags(cpu,
                      &[(Flag::Z, v == 0),
                        (Flag::N, false),
                        (Flag::H, false),
                        (Flag::C, c)]);
        }
        SLA_ATHL => {
            cpu.tick(16);
            let v = get_athl(cpu)?;
            let c = get_bit(v, 7);
            let v = v << 1;
            set_athl(cpu, v)?;
            set_flags(cpu,
                      &[(Flag::Z, v == 0),
                        (Flag::N, false),
                        (Flag::H, false),
                        (Flag::C, c)]);
        }

        SRA_R(r) => {
            cpu.tick(8);
            let v = cpu.get_register(r);
            let lsb = get_bit(v, 0);
            let msb = get_bit(v, 7);
            let v = set_bit(v >> 1, 7, msb);
            cpu.set_register(r, v);
            set_flags(cpu,
                      &[(Flag::Z, v == 0),
                        (Flag::N, false),
                        (Flag::H, false),
                        (Flag::C, lsb)]);
        }
        SRA_ATHL => {
            cpu.tick(16);
            let v = get_athl(cpu)?;
            let lsb = get_bit(v, 0);
            let msb = get_bit(v, 7);
            let v = set_bit(v >> 1, 7, msb);
            set_athl(cpu, v)?;
            set_flags(cpu,
                      &[(Flag::Z, v == 0),
                        (Flag::N, false),
                        (Flag::H, false),
                        (Flag::C, lsb)]);
        }

        SRL_R(r) => {
            cpu.tick(8);
            let v = cpu.get_register(r);
            let lsb = get_bit(v, 0);
            let v = v >> 1;
            set_flags(cpu,
                      &[(Flag::Z, v == 0),
                        (Flag::N, false),
                        (Flag::H, false),
                        (Flag::C, lsb)]);
            cpu.set_register(r, v);
        }
        SRL_ATHL => {
            cpu.tick(16);
            let v = get_athl(cpu)?;
            let lsb = get_bit(v, 0);
            let v = v >> 1;
            set_flags(cpu,
                      &[(Flag::Z, v == 0),
                        (Flag::N, false),
                        (Flag::H, false),
                        (Flag::C, lsb)]);
            set_athl(cpu, v)?;
        }

        BIT_B_R(b, r) => {
            cpu.tick(8);
            let v = cpu.get_register(r);
            let btest = get_bit(v, bit_number(b));
            set_flags(cpu, &[(Flag::Z, !btest), (Flag::N, false), (Flag::H, true)]);
        }
        BIT_B_ATHL(b) => {
            cpu.tick(16);
            let v = get_athl(cpu)?;
            let btest = get_bit(v, bit_number(b));
            set_flags(cpu, &[(Flag::Z, !btest), (Flag::N, false), (Flag::H, true)]);
        }

        SET_B_R(b, r) => {
            cpu.tick(8);
            let v = cpu.get_register(r);
            let v = set_bit(v, bit_number(b), true);
            cpu.set_register(r, v);
        }
        SET_B_ATHL(b) => {
            cpu.tick(16);
            let v = get_athl(cpu)?;
            let v = set_bit(v, bit_number(b), true);
            set_athl(cpu, v)?;
        }

        RES_B_R(b, r) => {
            cpu.tick(8);
            let v = cpu.get_register(r);
            let v = set_bit(v, bit_number(b), false);
            cpu.set_register(r, v);
        }
        RES_B_ATHL(b) => {
            cpu.tick(16);
            let v = get_athl(cpu)?;
            let v = set_bit(v, bit_number(b), false);
            set_athl(cpu, v)?;
        }

        JP_NN(nn) => {
            cpu.tick(12);
            cpu.set_program_counter(nn);
        }
        JP_C_NN(c, nn) => {
            cpu.tick(12);
            if test_cond(cpu, c) {
                cpu.set_program_counter(nn);
            }
        }
        JP_ATHL => {
            cpu.tick(4);
            let hl = get_hl(cpu);
            cpu.set_program_counter(hl);
        }

        JR_N(n) => {
            cpu.tick(8);
            let pc = cpu.get_program_counter();
            cpu.set_program_counter(pc.wrapping_add(n as u16));
        }
        JR_C_N(c, n) => {
            cpu.tick(8);
            if test_cond(cpu, c) {
                let pc = cpu.get_program_counter();
                cpu.set_program_counter(pc.wrapping_add(n as u16));
            }
        }

        CALL_NN(nn) => {
            cpu.tick(12);
            let pc = cpu.get_program_counter();
            push_stack16(cpu, pc)?;
            cpu.set_program_counter(nn);
        }
        CALL_C_NN(c, nn) => {
            cpu.tick(12);
            if test_cond(cpu, c) {
                let pc = cpu.get_program_counter();
                push_stack16(cpu, pc)?;
                cpu.set_program_counter(nn);
            }
        }

        RST_RA(ra) => {
            cpu.tick(32);
            let pc = cpu.get_program_counter();
            push_stack16(cpu, pc)?;
            cpu.set_program_counter(reset_address(ra));
        }

        RET => {
            cpu.tick(8);
            let pc = pop_stack16(cpu)?;
            cpu.set_program_counter(pc);
        }
        RET_C(c) => {
            cpu.tick(8);
            if test_cond(cpu, c) {
                let pc = pop_stack16(cpu)?;
                cpu.set_program_counter(pc);
            }
        }

        RETI => {
            cpu.tick(8);
            let nn = pop_stack16(cpu)?;
            cpu.set_program_counter(nn);
            cpu.set_interrupts_enabled(true);
        }
    }

    Ok(())
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Flag {
    Z,
    N,
    H,
    C,
}

fn flag_bit(f: Flag) -> u8 {
    match f {
        Flag::Z => 7,
        Flag::N => 6,
        Flag::H => 5,
        Flag::C => 4,
    }
}

fn bit_number(b: Bit) -> u8 {
    match b {
        Bit0 => 0,
        Bit1 => 1,
        Bit2 => 2,
        Bit3 => 3,
        Bit4 => 4,
        Bit5 => 5,
        Bit6 => 6,
        Bit7 => 7,
    }
}

fn reset_address(r: ResetAddress) -> u16 {
    match r {
        Reset00 => 0x00,
        Reset08 => 0x08,
        Reset10 => 0x10,
        Reset18 => 0x18,
        Reset20 => 0x20,
        Reset28 => 0x28,
        Reset30 => 0x30,
        Reset38 => 0x38,
    }
}

fn get_memory16<C: Cpu>(cpu: &C, addr: u16) -> Result<u16> {
    let l = cpu.get_memory(addr)?;
    let h = cpu.get_memory(addr + 1)?;
    Ok(make_word16(h, l))
}

fn set_memory16<C: Cpu>(cpu: &mut C, addr: u16, nn: u16) -> Result<()> {
    cpu.set_memory(addr, low_byte(nn))?;
    cpu.set_memory(addr + 1, high_byte(nn))?;
    Ok(())
}

fn push_stack16<C: Cpu>(cpu: &mut C, nn: u16) -> Result<()> {
    let sp = cpu.get_stack_pointer();
    set_memory16(cpu, sp - 2, nn)?;
    cpu.set_stack_pointer(sp - 2);
    Ok(())
}

fn pop_stack16<C: Cpu>(cpu: &mut C) -> Result<u16> {
    let sp = cpu.get_stack_pointer();
    let nn = get_memory16(cpu, sp)?;
    cpu.set_stack_pointer(sp + 2);
    Ok(nn)
}

fn get_af<C: Cpu>(cpu: &C) -> u16 {
    make_word16(cpu.get_register(ARegister), cpu.get_flags_register())
}

fn get_bc<C: Cpu>(cpu: &C) -> u16 {
    make_word16(cpu.get_register(BRegister), cpu.get_register(CRegister))
}

fn get_de<C: Cpu>(cpu: &C) -> u16 {
    make_word16(cpu.get_register(DRegister), cpu.get_register(ERegister))
}

fn get_hl<C: Cpu>(cpu: &C) -> u16 {
    make_word16(cpu.get_register(HRegister), cpu.get_register(LRegister))
}

fn set_af<C: Cpu>(cpu: &mut C, v: u16) {
    cpu.set_register(ARegister, high_byte(v));
    cpu.set_flags_register(low_byte(v));
}

fn set_bc<C: Cpu>(cpu: &mut C, v: u16) {
    cpu.set_register(BRegister, high_byte(v));
    cpu.set_register(CRegister, low_byte(v));
}

fn set_de<C: Cpu>(cpu: &mut C, v: u16) {
    cpu.set_register(DRegister, high_byte(v));
    cpu.set_register(ERegister, low_byte(v));
}

fn set_hl<C: Cpu>(cpu: &mut C, v: u16) {
    cpu.set_register(HRegister, high_byte(v));
    cpu.set_register(LRegister, low_byte(v));
}

fn get_atbc<C: Cpu>(cpu: &C) -> Result<u8> {
    cpu.get_memory(get_bc(cpu))
}

fn get_atde<C: Cpu>(cpu: &C) -> Result<u8> {
    cpu.get_memory(get_de(cpu))
}

fn get_athl<C: Cpu>(cpu: &C) -> Result<u8> {
    cpu.get_memory(get_hl(cpu))
}

fn get_atc<C: Cpu>(cpu: &C) -> Result<u8> {
    let c = cpu.get_register(CRegister);
    cpu.get_memory(make_word16(0xff, c))
}

fn set_atbc<C: Cpu>(cpu: &mut C, v: u8) -> Result<()> {
    let bc = get_bc(cpu);
    cpu.set_memory(bc, v)
}

fn set_atde<C: Cpu>(cpu: &mut C, v: u8) -> Result<()> {
    let de = get_de(cpu);
    cpu.set_memory(de, v)
}

fn set_athl<C: Cpu>(cpu: &mut C, v: u8) -> Result<()> {
    let hl = get_hl(cpu);
    cpu.set_memory(hl, v)
}

fn set_atc<C: Cpu>(cpu: &mut C, v: u8) -> Result<()> {
    let c = cpu.get_register(CRegister);
    cpu.set_memory(make_word16(0xff, c), v)
}

fn get_flag<C: Cpu>(cpu: &C, flag: Flag) -> bool {
    let f = cpu.get_flags_register();
    get_bit(f, flag_bit(flag))
}

fn set_flags<C: Cpu>(cpu: &mut C, fs: &[(Flag, bool)]) {
    let mut flags = cpu.get_flags_register();
    for &(f, v) in fs {
        flags = set_bit(flags, flag_bit(f), v);
    }
    cpu.set_flags_register(flags);
}

fn test_cond<C: Cpu>(cpu: &C, c: Cond) -> bool {
    match c {
        Cond::Zero => get_flag(cpu, Flag::Z),
        Cond::NZero => !get_flag(cpu, Flag::Z),
        Cond::Carry => get_flag(cpu, Flag::C),
        Cond::NCarry => !get_flag(cpu, Flag::C),
    }
}

fn add_a<C: Cpu>(cpu: &mut C, n: u8) {
    let a = cpu.get_register(ARegister);
    let (res, h, c) = add8(a, n);
    set_flags(cpu,
              &[(Flag::Z, res == 0),
                (Flag::N, false),
                (Flag::H, h),
                (Flag::C, c)]);
    cpu.set_register(ARegister, res);
}

fn add_ca<C: Cpu>(cpu: &mut C, mut n: u8) {
    let a = cpu.get_register(ARegister);

    let mut carry = false;
    if get_flag(cpu, Flag::C) {
        if n == 0xff {
            carry = true;
        }
        n += 1;
    }
    let (res, h, c) = add8(a, n);
    carry |= c;

    set_flags(cpu,
              &[(Flag::Z, res == 0),
                (Flag::N, false),
                (Flag::H, h),
                (Flag::C, carry)]);
    cpu.set_register(ARegister, res);
}

fn sub_a<C: Cpu>(cpu: &mut C, n: u8) {
    let a = cpu.get_register(ARegister);
    let (res, h, c) = sub8(a, n);
    set_flags(cpu,
              &[(Flag::Z, res == 0),
                (Flag::N, true),
                (Flag::H, h),
                (Flag::C, c)]);
    cpu.set_register(ARegister, res);
}

fn sub_ca<C: Cpu>(cpu: &mut C, mut n: u8) {
    let a = cpu.get_register(ARegister);

    let mut carry = false;
    if get_flag(cpu, Flag::C) {
        if n == 0x00 {
            carry = true;
        }
        n -= 1;
    }
    let (res, h, c) = sub8(a, n);
    carry |= c;

    set_flags(cpu,
              &[(Flag::Z, res == 0),
                (Flag::N, true),
                (Flag::H, h),
                (Flag::C, carry)]);
    cpu.set_register(ARegister, res);
}

fn do_and_a<C: Cpu>(cpu: &mut C, n: u8) {
    let a = cpu.get_register(ARegister);
    let r = a & n;
    set_flags(cpu,
              &[(Flag::Z, r == 0),
                (Flag::N, false),
                (Flag::H, true),
                (Flag::C, false)]);
    cpu.set_register(ARegister, r);
}

fn do_or_a<C: Cpu>(cpu: &mut C, n: u8) {
    let a = cpu.get_register(ARegister);
    let r = a | n;
    set_flags(cpu,
              &[(Flag::Z, r == 0),
                (Flag::N, false),
                (Flag::H, false),
                (Flag::C, false)]);
    cpu.set_register(ARegister, r);
}

fn do_xor_a<C: Cpu>(cpu: &mut C, n: u8) {
    let a = cpu.get_register(ARegister);
    let r = a ^ n;
    set_flags(cpu,
              &[(Flag::Z, r == 0),
                (Flag::N, false),
                (Flag::H, false),
                (Flag::C, false)]);
    cpu.set_register(ARegister, r);
}

fn do_cp_a<C: Cpu>(cpu: &mut C, n: u8) {
    let a = cpu.get_register(ARegister);
    let (r, h, c) = sub8(a, n);
    set_flags(cpu,
              &[(Flag::Z, r == 0),
                (Flag::N, true),
                (Flag::H, h),
                (Flag::C, c)]);
}

fn do_add_hl<C: Cpu>(cpu: &mut C, nn: u16) {
    let hl = get_hl(cpu);
    let (res, h, c) = add16(hl, nn);
    set_flags(cpu, &[(Flag::N, false), (Flag::H, h), (Flag::C, c)]);
    set_hl(cpu, res);
}

fn rotlc(b: u8) -> (u8, bool) {
    (b.rotate_left(1), get_bit(b, 7))
}

fn rotl(b: u8, c: bool) -> (u8, bool) {
    (set_bit(b << 1, 0, c), get_bit(b, 7))
}

fn rotrc(b: u8) -> (u8, bool) {
    (b.rotate_right(1), get_bit(b, 0))
}

fn rotr(b: u8, c: bool) -> (u8, bool) {
    (set_bit(b >> 1, 7, c), get_bit(b, 0))
}

fn do_rlc<C: Cpu>(cpu: &mut C, r: Register) {
    let v = cpu.get_register(r);
    let (v, c) = rotlc(v);
    set_flags(cpu,
              &[(Flag::Z, v == 0),
                (Flag::N, false),
                (Flag::H, false),
                (Flag::C, c)]);
    cpu.set_register(r, v);
}

fn do_rl<C: Cpu>(cpu: &mut C, r: Register) {
    let v = cpu.get_register(r);
    let c = get_flag(cpu, Flag::C);
    let (v, c) = rotl(v, c);
    set_flags(cpu,
              &[(Flag::Z, v == 0),
                (Flag::N, false),
                (Flag::H, false),
                (Flag::C, c)]);
    cpu.set_register(r, v);
}

fn do_rrc<C: Cpu>(cpu: &mut C, r: Register) {
    let v = cpu.get_register(r);
    let (v, c) = rotrc(v);
    set_flags(cpu,
              &[(Flag::Z, v == 0),
                (Flag::N, false),
                (Flag::H, false),
                (Flag::C, c)]);
    cpu.set_register(r, v);
}

fn do_rr<C: Cpu>(cpu: &mut C, r: Register) {
    let v = cpu.get_register(r);
    let c = get_flag(cpu, Flag::C);
    let (v, c) = rotr(v, c);
    set_flags(cpu,
              &[(Flag::Z, v == 0),
                (Flag::N, false),
                (Flag::H, false),
                (Flag::C, c)]);
    cpu.set_register(r, v);
}
