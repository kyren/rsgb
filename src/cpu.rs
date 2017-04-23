use util::*;
use instruction::*;
use decoding::*;

#[derive(Debug, Clone, Copy)]
pub struct Flags {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

pub trait Cpu {
    fn get_register(&self, reg: Register) -> u8;
    fn set_register(&mut self, reg: Register, val: u8);

    fn get_flags(&self) -> Flags;
    fn set_flags(&mut self, flags: Flags);

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
            set_hl(cpu, hl.wrapping_sub(1));
        }
        LDD_ATHL_A => {
            cpu.tick(8);
            let hl = get_hl(cpu);
            let v = cpu.get_register(ARegister);
            cpu.set_memory(hl, v)?;
            set_hl(cpu, hl.wrapping_sub(1));
        }

        LDI_A_ATHL => {
            cpu.tick(8);
            let hl = get_hl(cpu);
            let v = cpu.get_memory(hl)?;
            cpu.set_register(ARegister, v);
            set_hl(cpu, hl.wrapping_add(1));
        }
        LDI_ATHL_A => {
            cpu.tick(8);
            let hl = get_hl(cpu);
            let v = cpu.get_register(ARegister);
            cpu.set_memory(hl, v)?;
            set_hl(cpu, hl.wrapping_add(1));
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
            cpu.set_register(r, res);

            let mut flags = cpu.get_flags();
            flags.zero = res == 0;
            flags.subtract = false;
            flags.half_carry = h;
            cpu.set_flags(flags);
        }
        INC_ATHL => {
            cpu.tick(12);
            let v = get_athl(cpu)?;
            let (res, h, _) = add8(v, 1);
            set_athl(cpu, res)?;

            let mut flags = cpu.get_flags();
            flags.zero = res == 0;
            flags.subtract = false;
            flags.half_carry = h;
            cpu.set_flags(flags);
        }

        DEC_R(r) => {
            cpu.tick(4);
            let v = cpu.get_register(r);
            let (res, h, _) = sub8(v, 1);
            cpu.set_register(r, res);

            let mut flags = cpu.get_flags();
            flags.zero = res == 0;
            flags.subtract = true;
            flags.half_carry = h;
            cpu.set_flags(flags);
        }
        DEC_ATHL => {
            cpu.tick(12);
            let v = get_athl(cpu)?;
            let (res, h, _) = sub8(v, 1);
            set_athl(cpu, res)?;

            let mut flags = cpu.get_flags();
            flags.zero = res == 0;
            flags.subtract = true;
            flags.half_carry = h;
            cpu.set_flags(flags);
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
            cpu.set_stack_pointer(sp);

            let mut flags = cpu.get_flags();
            flags.zero = false;
            flags.subtract = false;
            flags.half_carry = h;
            flags.carry = c;
            cpu.set_flags(flags);
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
            cpu.tick(4);

            let mut a = cpu.get_register(ARegister);
            let mut flags = cpu.get_flags();

            let mut corr = 0;
            if flags.half_carry {
                corr |= 0x06;
            }
            if flags.carry {
                corr |= 0x60;
            }
            if flags.subtract {
                a = a.wrapping_sub(corr);
            } else {
                if a & 0x0f > 0x09 {
                    corr |= 0x06;
                }
                if a > 0x99 {
                    corr |= 0x60;
                }
                a = a.wrapping_add(corr);
            }

            cpu.set_register(ARegister, a);

            flags.zero = a == 0;
            flags.half_carry = false;
            flags.carry = corr & 0x60 != 0;
            cpu.set_flags(flags);
        }
        CPL => {
            cpu.tick(4);
            let a = cpu.get_register(ARegister);
            cpu.set_register(ARegister, !a);

            let mut flags = cpu.get_flags();
            flags.subtract = true;
            flags.half_carry = true;
            cpu.set_flags(flags);
        }
        CCF => {
            cpu.tick(4);

            let mut flags = cpu.get_flags();
            flags.subtract = true;
            flags.half_carry = true;
            flags.carry = !flags.carry;
            cpu.set_flags(flags);
        }
        SCF => {
            cpu.tick(4);

            let mut flags = cpu.get_flags();
            flags.subtract = false;
            flags.half_carry = false;
            flags.carry = true;
            cpu.set_flags(flags);
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

            let mut flags = cpu.get_flags();
            flags.zero = v == 0;
            flags.subtract = false;
            flags.half_carry = false;
            flags.carry = c;
            cpu.set_flags(flags);
        }

        RL_R(r) => {
            cpu.tick(8);
            do_rl(cpu, r);
        }
        RL_ATHL => {
            cpu.tick(16);

            let mut flags = cpu.get_flags();
            let v = get_athl(cpu)?;
            let c = flags.carry;
            let (v, c) = rotl(v, c);
            set_athl(cpu, v)?;

            flags.zero = v == 0;
            flags.subtract = false;
            flags.half_carry = false;
            flags.carry = c;
            cpu.set_flags(flags);
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

            let mut flags = cpu.get_flags();
            flags.zero = v == 0;
            flags.subtract = false;
            flags.half_carry = false;
            flags.carry = c;
            cpu.set_flags(flags);
        }

        RR_R(r) => {
            cpu.tick(8);
            do_rr(cpu, r);
        }
        RR_ATHL => {
            cpu.tick(16);
            let mut flags = cpu.get_flags();
            let v = get_athl(cpu)?;
            let c = flags.carry;
            let (v, c) = rotr(v, c);
            set_athl(cpu, v)?;

            flags.zero = v == 0;
            flags.subtract = false;
            flags.half_carry = false;
            flags.carry = c;
            cpu.set_flags(flags);
        }

        SLA_R(r) => {
            cpu.tick(8);
            let v = cpu.get_register(r);
            let c = get_bit(v, 7);
            let v = v << 1;
            cpu.set_register(r, v);

            let mut flags = cpu.get_flags();
            flags.zero = v == 0;
            flags.subtract = false;
            flags.half_carry = false;
            flags.carry = c;
            cpu.set_flags(flags);
        }
        SLA_ATHL => {
            cpu.tick(16);
            let v = get_athl(cpu)?;
            let c = get_bit(v, 7);
            let v = v << 1;
            set_athl(cpu, v)?;

            let mut flags = cpu.get_flags();
            flags.zero = v == 0;
            flags.subtract = false;
            flags.half_carry = false;
            flags.carry = c;
            cpu.set_flags(flags);
        }

        SRA_R(r) => {
            cpu.tick(8);
            let v = cpu.get_register(r);
            let lsb = get_bit(v, 0);
            let msb = get_bit(v, 7);
            let v = set_bit(v >> 1, 7, msb);
            cpu.set_register(r, v);

            let mut flags = cpu.get_flags();
            flags.zero = v == 0;
            flags.subtract = false;
            flags.half_carry = false;
            flags.carry = lsb;
            cpu.set_flags(flags);
        }
        SRA_ATHL => {
            cpu.tick(16);
            let v = get_athl(cpu)?;
            let lsb = get_bit(v, 0);
            let msb = get_bit(v, 7);
            let v = set_bit(v >> 1, 7, msb);
            set_athl(cpu, v)?;

            let mut flags = cpu.get_flags();
            flags.zero = v == 0;
            flags.subtract = false;
            flags.half_carry = false;
            flags.carry = lsb;
            cpu.set_flags(flags);
        }

        SRL_R(r) => {
            cpu.tick(8);
            let v = cpu.get_register(r);
            let lsb = get_bit(v, 0);
            let v = v >> 1;
            cpu.set_register(r, v);

            let mut flags = cpu.get_flags();
            flags.zero = v == 0;
            flags.subtract = false;
            flags.half_carry = false;
            flags.carry = lsb;
            cpu.set_flags(flags);
        }
        SRL_ATHL => {
            cpu.tick(16);
            let v = get_athl(cpu)?;
            let lsb = get_bit(v, 0);
            let v = v >> 1;
            set_athl(cpu, v)?;

            let mut flags = cpu.get_flags();
            flags.zero = v == 0;
            flags.subtract = false;
            flags.half_carry = false;
            flags.carry = lsb;
            cpu.set_flags(flags);
        }

        BIT_B_R(b, r) => {
            cpu.tick(8);
            let v = cpu.get_register(r);
            let btest = get_bit(v, bit_number(b));

            let mut flags = cpu.get_flags();
            flags.zero = !btest;
            flags.subtract = false;
            flags.half_carry = true;
            cpu.set_flags(flags);
        }
        BIT_B_ATHL(b) => {
            cpu.tick(16);
            let v = get_athl(cpu)?;
            let btest = get_bit(v, bit_number(b));

            let mut flags = cpu.get_flags();
            flags.zero = !btest;
            flags.subtract = false;
            flags.half_carry = true;
            cpu.set_flags(flags);
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
    let h = cpu.get_memory(addr.checked_add(1).ok_or("address overflow")?)?;
    Ok(make_word16(h, l))
}

fn set_memory16<C: Cpu>(cpu: &mut C, addr: u16, nn: u16) -> Result<()> {
    cpu.set_memory(addr, low_byte(nn))?;
    cpu.set_memory(addr.checked_add(1).ok_or("address overflow")?,
                    high_byte(nn))?;
    Ok(())
}

fn push_stack16<C: Cpu>(cpu: &mut C, nn: u16) -> Result<()> {
    let sp = cpu.get_stack_pointer();
    let sp_dec = sp.checked_sub(2).ok_or("stack overflow")?;
    set_memory16(cpu, sp_dec, nn)?;
    cpu.set_stack_pointer(sp_dec);
    Ok(())
}

fn pop_stack16<C: Cpu>(cpu: &mut C) -> Result<u16> {
    let sp = cpu.get_stack_pointer();
    let nn = get_memory16(cpu, sp)?;
    cpu.set_stack_pointer(sp.checked_add(2).ok_or("stack underflow")?);
    Ok(nn)
}

fn get_af<C: Cpu>(cpu: &C) -> u16 {
    let flags = cpu.get_flags();
    let mut f = 0;
    f = set_bit(f, 7, flags.zero);
    f = set_bit(f, 6, flags.subtract);
    f = set_bit(f, 5, flags.half_carry);
    f = set_bit(f, 4, flags.carry);

    make_word16(cpu.get_register(ARegister), f)
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

    let f = low_byte(v);
    cpu.set_flags(Flags {
                      zero: get_bit(f, 7),
                      subtract: get_bit(f, 6),
                      half_carry: get_bit(f, 5),
                      carry: get_bit(f, 4),
                  });
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

fn test_cond<C: Cpu>(cpu: &C, c: Cond) -> bool {
    let flags = cpu.get_flags();
    match c {
        Cond::Zero => flags.zero,
        Cond::NZero => !flags.zero,
        Cond::Carry => flags.carry,
        Cond::NCarry => !flags.carry,
    }
}

fn add_a<C: Cpu>(cpu: &mut C, n: u8) {
    let v = cpu.get_register(ARegister);
    let (v, h, c) = add8(v, n);
    cpu.set_register(ARegister, v);

    let mut flags = cpu.get_flags();
    flags.zero = v == 0;
    flags.subtract = false;
    flags.half_carry = h;
    flags.carry = c;
    cpu.set_flags(flags);
}

fn add_ca<C: Cpu>(cpu: &mut C, mut n: u8) {
    let v = cpu.get_register(ARegister);
    let mut flags = cpu.get_flags();

    let mut carry = false;
    if flags.carry {
        if n == 0xff {
            carry = true;
        }
        n = n.wrapping_add(1);
    }
    let (v, h, c) = add8(v, n);
    carry |= c;

    cpu.set_register(ARegister, v);

    flags.zero = v == 0;
    flags.subtract = false;
    flags.half_carry = h;
    flags.carry = carry;
    cpu.set_flags(flags);
}

fn sub_a<C: Cpu>(cpu: &mut C, n: u8) {
    let v = cpu.get_register(ARegister);
    let (v, h, c) = sub8(v, n);
    cpu.set_register(ARegister, v);

    let mut flags = cpu.get_flags();
    flags.zero = v == 0;
    flags.subtract = true;
    flags.half_carry = h;
    flags.carry = c;
    cpu.set_flags(flags);
}

fn sub_ca<C: Cpu>(cpu: &mut C, mut n: u8) {
    let v = cpu.get_register(ARegister);
    let mut flags = cpu.get_flags();

    let mut carry = false;
    if flags.carry {
        if n == 0x00 {
            carry = true;
        }
        n = n.wrapping_sub(1);
    }
    let (v, h, c) = sub8(v, n);
    carry |= c;

    cpu.set_register(ARegister, v);

    flags.zero = v == 0;
    flags.subtract = true;
    flags.half_carry = h;
    flags.carry = carry;
    cpu.set_flags(flags);
}

fn do_and_a<C: Cpu>(cpu: &mut C, n: u8) {
    let v = cpu.get_register(ARegister);
    let v = v & n;
    cpu.set_register(ARegister, v);

    let mut flags = cpu.get_flags();
    flags.zero = v == 0;
    flags.subtract = false;
    flags.half_carry = true;
    flags.carry = false;
    cpu.set_flags(flags);
}

fn do_or_a<C: Cpu>(cpu: &mut C, n: u8) {
    let v = cpu.get_register(ARegister);
    let v = v | n;
    cpu.set_register(ARegister, v);

    let mut flags = cpu.get_flags();
    flags.zero = v == 0;
    flags.subtract = false;
    flags.half_carry = false;
    flags.carry = false;
    cpu.set_flags(flags);
}

fn do_xor_a<C: Cpu>(cpu: &mut C, n: u8) {
    let v = cpu.get_register(ARegister);
    let v = v ^ n;
    cpu.set_register(ARegister, v);

    let mut flags = cpu.get_flags();
    flags.zero = v == 0;
    flags.subtract = false;
    flags.half_carry = false;
    flags.carry = false;
    cpu.set_flags(flags);
}

fn do_cp_a<C: Cpu>(cpu: &mut C, n: u8) {
    let v = cpu.get_register(ARegister);
    let (v, h, c) = sub8(v, n);

    let mut flags = cpu.get_flags();
    flags.zero = v == 0;
    flags.subtract = true;
    flags.half_carry = h;
    flags.carry = c;
    cpu.set_flags(flags);
}

fn do_add_hl<C: Cpu>(cpu: &mut C, nn: u16) {
    let vv = get_hl(cpu);
    let (vv, h, c) = add16(vv, nn);
    set_hl(cpu, vv);

    let mut flags = cpu.get_flags();
    flags.subtract = false;
    flags.half_carry = h;
    flags.carry = c;
    cpu.set_flags(flags);
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
    cpu.set_register(r, v);

    let mut flags = cpu.get_flags();
    flags.zero = v == 0;
    flags.subtract = false;
    flags.half_carry = false;
    flags.carry = c;
    cpu.set_flags(flags);
}

fn do_rl<C: Cpu>(cpu: &mut C, r: Register) {
    let v = cpu.get_register(r);
    let mut flags = cpu.get_flags();
    let (v, c) = rotl(v, flags.carry);
    cpu.set_register(r, v);

    flags.zero = v == 0;
    flags.subtract = false;
    flags.half_carry = false;
    flags.carry = c;
    cpu.set_flags(flags);
}

fn do_rrc<C: Cpu>(cpu: &mut C, r: Register) {
    let v = cpu.get_register(r);
    let (v, c) = rotrc(v);
    cpu.set_register(r, v);

    let mut flags = cpu.get_flags();
    flags.zero = v == 0;
    flags.subtract = false;
    flags.half_carry = false;
    flags.carry = c;
    cpu.set_flags(flags);
}

fn do_rr<C: Cpu>(cpu: &mut C, r: Register) {
    let v = cpu.get_register(r);
    let mut flags = cpu.get_flags();
    let (v, c) = rotr(v, flags.carry);
    cpu.set_register(r, v);

    flags.zero = v == 0;
    flags.subtract = false;
    flags.half_carry = false;
    flags.carry = c;
    cpu.set_flags(flags);
}
