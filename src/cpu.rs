use util::*;
use instruction::*;
use decoding::*;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Flag {
    Z,
    N,
    H,
    C,
}

pub fn flag_bit(f: Flag) -> u8 {
    match f {
        Flag::Z => 7,
        Flag::N => 6,
        Flag::H => 5,
        Flag::C => 4,
    }
}

pub fn bit_number(b: Bit) -> u8 {
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

    fn get_memory16(&self, addr: u16) -> Result<u16> {
        let l = self.get_memory(addr)?;
        let h = self.get_memory(addr + 1)?;
        Ok(make_word16(h, l))
    }

    fn set_memory16(&mut self, addr: u16, nn: u16) -> Result<()> {
        self.set_memory(addr, low_byte(nn))?;
        self.set_memory(addr + 1, high_byte(nn))?;
        Ok(())
    }

    fn push_stack16(&mut self, nn: u16) -> Result<()> {
        let sp = self.get_stack_pointer();
        self.set_memory16(sp - 2, nn)?;
        self.set_stack_pointer(sp - 2);
        Ok(())
    }

    fn pop_stack16(&mut self) -> Result<u16> {
        let sp = self.get_stack_pointer();
        let nn = self.get_memory16(sp)?;
        self.set_stack_pointer(sp + 2);
        Ok(nn)
    }

    fn get_af(&self) -> u16 {
        make_word16(self.get_register(ARegister), self.get_flags_register())
    }

    fn get_bc(&self) -> u16 {
        make_word16(self.get_register(BRegister), self.get_register(CRegister))
    }

    fn get_de(&self) -> u16 {
        make_word16(self.get_register(DRegister), self.get_register(ERegister))
    }

    fn get_hl(&self) -> u16 {
        make_word16(self.get_register(HRegister), self.get_register(LRegister))
    }

    fn set_af(&mut self, v: u16) {
        self.set_register(ARegister, high_byte(v));
        self.set_flags_register(low_byte(v));
    }

    fn set_bc(&mut self, v: u16) {
        self.set_register(BRegister, high_byte(v));
        self.set_register(CRegister, low_byte(v));
    }

    fn set_de(&mut self, v: u16) {
        self.set_register(DRegister, high_byte(v));
        self.set_register(ERegister, low_byte(v));
    }

    fn set_hl(&mut self, v: u16) {
        self.set_register(HRegister, high_byte(v));
        self.set_register(LRegister, low_byte(v));
    }

    fn get_atbc(&self) -> Result<u8> {
        self.get_memory(self.get_bc())
    }

    fn get_atde(&self) -> Result<u8> {
        self.get_memory(self.get_de())
    }

    fn get_athl(&self) -> Result<u8> {
        self.get_memory(self.get_hl())
    }

    fn get_atc(&self) -> Result<u8> {
        let c = self.get_register(CRegister);
        self.get_memory(make_word16(0xff, c))
    }

    fn set_atbc(&mut self, v: u8) -> Result<()> {
        let bc = self.get_bc();
        self.set_memory(bc, v)
    }

    fn set_atde(&mut self, v: u8) -> Result<()> {
        let de = self.get_de();
        self.set_memory(de, v)
    }

    fn set_athl(&mut self, v: u8) -> Result<()> {
        let hl = self.get_hl();
        self.set_memory(hl, v)
    }

    fn set_atc(&mut self, v: u8) -> Result<()> {
        let c = self.get_register(CRegister);
        self.set_memory(make_word16(0xff, c), v)
    }

    fn get_flag(&self, flag: Flag) -> bool {
        let f = self.get_flags_register();
        get_bit(f, flag_bit(flag))
    }

    fn set_flags(&mut self, fs: &[(Flag, bool)]) {
        let mut flags = self.get_flags_register();
        for &(f, v) in fs {
            flags = set_bit(flags, flag_bit(f), v);
        }
        self.set_flags_register(flags);
    }

    fn test_cond(&self, c: Cond) -> bool {
        match c {
            Cond::Zero => self.get_flag(Flag::Z),
            Cond::NZero => !self.get_flag(Flag::Z),
            Cond::Carry => self.get_flag(Flag::C),
            Cond::NCarry => !self.get_flag(Flag::C),
        }
    }

    fn add_a(&mut self, n: u8) {
        let a = self.get_register(ARegister);
        let (res, h, c) = add8(a, n);
        self.set_flags(&[(Flag::Z, res == 0),
                         (Flag::N, false),
                         (Flag::H, h),
                         (Flag::C, c)]);
        self.set_register(ARegister, res);
    }

    fn add_ca(&mut self, mut n: u8) {
        let a = self.get_register(ARegister);

        let mut carry = false;
        if self.get_flag(Flag::C) {
            if n == 0xff {
                carry = true;
            }
            n += 1;
        }
        let (res, h, c) = add8(a, n);
        carry |= c;

        self.set_flags(&[(Flag::Z, res == 0),
                         (Flag::N, false),
                         (Flag::H, h),
                         (Flag::C, carry)]);
        self.set_register(ARegister, res);
    }

    fn sub_a(&mut self, n: u8) {
        let a = self.get_register(ARegister);
        let (res, h, c) = sub8(a, n);
        self.set_flags(&[(Flag::Z, res == 0),
                         (Flag::N, true),
                         (Flag::H, h),
                         (Flag::C, c)]);
        self.set_register(ARegister, res);
    }

    fn sub_ca(&mut self, mut n: u8) {
        let a = self.get_register(ARegister);

        let mut carry = false;
        if self.get_flag(Flag::C) {
            if n == 0x00 {
                carry = true;
            }
            n -= 1;
        }
        let (res, h, c) = sub8(a, n);
        carry |= c;

        self.set_flags(&[(Flag::Z, res == 0),
                         (Flag::N, true),
                         (Flag::H, h),
                         (Flag::C, carry)]);
        self.set_register(ARegister, res);
    }

    fn do_and_a(&mut self, n: u8) {
        let a = self.get_register(ARegister);
        let r = a & n;
        self.set_flags(&[(Flag::Z, r == 0),
                         (Flag::N, false),
                         (Flag::H, true),
                         (Flag::C, false)]);
        self.set_register(ARegister, r);
    }

    fn do_or_a(&mut self, n: u8) {
        let a = self.get_register(ARegister);
        let r = a | n;
        self.set_flags(&[(Flag::Z, r == 0),
                         (Flag::N, false),
                         (Flag::H, false),
                         (Flag::C, false)]);
        self.set_register(ARegister, r);
    }

    fn do_xor_a(&mut self, n: u8) {
        let a = self.get_register(ARegister);
        let r = a ^ n;
        self.set_flags(&[(Flag::Z, r == 0),
                         (Flag::N, false),
                         (Flag::H, false),
                         (Flag::C, false)]);
        self.set_register(ARegister, r);
    }

    fn do_cp_a(&mut self, n: u8) {
        let a = self.get_register(ARegister);
        let (r, h, c) = sub8(a, n);
        self.set_flags(&[(Flag::Z, r == 0),
                         (Flag::N, true),
                         (Flag::H, h),
                         (Flag::C, c)]);
    }

    fn do_add_hl(&mut self, nn: u16) {
        let hl = self.get_hl();
        let (res, h, c) = add16(hl, nn);
        self.set_flags(&[(Flag::N, false), (Flag::H, h), (Flag::C, c)]);
        self.set_hl(res);
    }

    fn do_rr(&mut self, r: Register) {
        let v = self.get_register(r);
        let c = self.get_flag(Flag::C);
        let nc = get_bit(v, 0);
        let l = set_bit(v >> 1, 7, c);
        self.set_register(r, l);
        self.set_flags(&[(Flag::Z, l == 0),
                         (Flag::N, false),
                         (Flag::H, false),
                         (Flag::C, nc)]);
    }

    fn step(&mut self) -> Result<()> {
        let instruction = decode_instruction(|| {
            let pc = self.get_program_counter();
            if pc == u16::max_value() {
                Err("program counter wrapped at max address".into())
            } else {
                self.set_program_counter(pc + 1);
                self.get_memory(pc)
            }
        })?;

        self.do_instruction(instruction)
    }

    fn do_instruction(&mut self, i: Instruction) -> Result<()> {
        match i {
            LD_R_R(tr, sr) => {
                let sv = self.get_register(sr);
                self.set_register(tr, sv);
                self.tick(4);
            }
            LD_R_N(tr, n) => {
                self.set_register(tr, n);
                self.tick(8);
            }
            LD_R_ATHL(tr) => {
                let v = self.get_athl()?;
                self.set_register(tr, v);
                self.tick(8);
            }
            LD_ATHL_R(sr) => {
                let v = self.get_register(sr);
                self.set_athl(v)?;
                self.tick(8);
            }
            LD_ATHL_N(n) => {
                self.set_athl(n)?;
                self.tick(8);
            }
            LD_A_ATC => {
                let v = self.get_atc()?;
                self.set_register(ARegister, v);
                self.tick(8);
            }
            LD_A_ATBC => {
                let v = self.get_atbc()?;
                self.set_register(ARegister, v);
                self.tick(8);
            }
            LD_A_ATDE => {
                let v = self.get_atde()?;
                self.set_register(ARegister, v);
                self.tick(8);
            }
            LD_A_ATNN(nn) => {
                let v = self.get_memory(nn)?;
                self.set_register(ARegister, v);
                self.tick(16);
            }
            LD_ATC_A => {
                let v = self.get_register(ARegister);
                self.set_atc(v)?;
                self.tick(8);
            }
            LD_ATBC_A => {
                let v = self.get_register(ARegister);
                self.set_atbc(v)?;
                self.tick(8);
            }
            LD_ATDE_A => {
                let v = self.get_register(ARegister);
                self.set_atde(v)?;
                self.tick(8);
            }
            LD_ATNN_A(nn) => {
                let v = self.get_register(ARegister);
                self.set_memory(nn, v)?;
                self.tick(16);
            }
            LDD_A_ATHL => {
                let hl = self.get_hl();
                let v = self.get_memory(hl)?;
                self.set_register(ARegister, v);
                self.set_hl(hl - 1);
                self.tick(8);
            }
            LDD_ATHL_A => {
                let hl = self.get_hl();
                let v = self.get_register(ARegister);
                self.set_memory(hl, v)?;
                self.set_hl(hl - 1);
                self.tick(8);
            }
            LDI_A_ATHL => {
                let hl = self.get_hl();
                let v = self.get_memory(hl)?;
                self.set_register(ARegister, v);
                self.set_hl(hl + 1);
                self.tick(8);
            }
            LDI_ATHL_A => {
                let hl = self.get_hl();
                let v = self.get_register(ARegister);
                self.set_memory(hl, v)?;
                self.set_hl(hl + 1);
                self.tick(8);
            }
            LDH_A_ATN(n) => {
                let v = self.get_memory(make_word16(0xff, n))?;
                self.set_register(ARegister, v);
                self.tick(12);
            }
            LDH_ATN_A(n) => {
                let v = self.get_register(ARegister);
                self.set_memory(make_word16(0xff, n), v)?;
                self.tick(12);
            }
            LD_BC_NN(nn) => {
                self.set_bc(nn);
                self.tick(12);
            }
            LD_DE_NN(nn) => {
                self.set_de(nn);
                self.tick(12);
            }
            LD_HL_NN(nn) => {
                self.set_hl(nn);
                self.tick(12);
            }
            LD_SP_NN(nn) => {
                self.set_stack_pointer(nn);
                self.tick(12);
            }
            LD_SP_HL => {
                let sp = self.get_stack_pointer();
                self.set_hl(sp);
                self.tick(8);
            }
            LDHL_SP_N(n) => {
                let sp = self.get_stack_pointer();
                self.set_hl(((sp as i32) + (n as i32)) as u16);
                self.tick(12);
            }
            LD_ATNN_SP(nn) => {
                let sp = self.get_stack_pointer();
                self.set_memory16(nn, sp)?;
                self.tick(20);
            }
            PUSH_AF => {
                let af = self.get_af();
                self.push_stack16(af)?;
                self.tick(16);
            }
            PUSH_BC => {
                let bc = self.get_bc();
                self.push_stack16(bc)?;
                self.tick(16);
            }
            PUSH_DE => {
                let de = self.get_de();
                self.push_stack16(de)?;
                self.tick(16);
            }
            PUSH_HL => {
                let hl = self.get_hl();
                self.push_stack16(hl)?;
                self.tick(16);
            }
            POP_AF => {
                let nn = self.pop_stack16()?;
                self.set_af(nn);
                self.tick(12);
            }
            POP_BC => {
                let nn = self.pop_stack16()?;
                self.set_bc(nn);
                self.tick(12);
            }
            POP_DE => {
                let nn = self.pop_stack16()?;
                self.set_de(nn);
                self.tick(12);
            }
            POP_HL => {
                let nn = self.pop_stack16()?;
                self.set_hl(nn);
                self.tick(12);
            }
            ADD_A_R(r) => {
                let n = self.get_register(r);
                self.add_a(n);
                self.tick(4);
            }
            ADD_A_N(n) => {
                self.add_a(n);
                self.tick(8);
            }
            ADD_A_ATHL => {
                let athl = self.get_athl()?;
                self.add_a(athl);
                self.tick(8);
            }
            ADC_A_R(r) => {
                let n = self.get_register(r);
                self.add_ca(n);
                self.tick(4);
            }
            ADC_A_N(n) => {
                self.add_ca(n);
                self.tick(8);
            }
            ADC_A_ATHL => {
                let athl = self.get_athl()?;
                self.add_ca(athl);
                self.tick(8);
            }
            SUB_R(r) => {
                let n = self.get_register(r);
                self.sub_a(n);
                self.tick(4);
            }
            SUB_N(n) => {
                self.sub_a(n);
                self.tick(8);
            }
            SUB_ATHL => {
                let athl = self.get_athl()?;
                self.sub_a(athl);
                self.tick(8);
            }
            SBC_A_R(r) => {
                let n = self.get_register(r);
                self.sub_ca(n);
                self.tick(4);
            }
            SBC_A_N(n) => {
                self.sub_ca(n);
                self.tick(8);
            }
            SBC_A_ATHL => {
                let athl = self.get_athl()?;
                self.sub_ca(athl);
                self.tick(8);
            }
            NOP => {
                self.tick(4);
            }
            INC_R(r) => {
                let v = self.get_register(r);
                let (res, h, _) = add8(v, 1);
                self.set_flags(&[(Flag::Z, res == 0), (Flag::N, false), (Flag::H, h)]);
                self.set_register(r, res);
                self.tick(4);
            }
            DEC_R(r) => {
                let v = self.get_register(r);
                let (res, h, _) = sub8(v, 1);
                self.set_flags(&[(Flag::Z, res == 0), (Flag::N, true), (Flag::H, h)]);
                self.set_register(r, res);
                self.tick(4);
            }
            INC_ATHL => {
                let v = self.get_athl()?;
                let (res, h, _) = add8(v, 1);
                self.set_flags(&[(Flag::Z, res == 0), (Flag::N, false), (Flag::H, h)]);
                self.set_athl(res)?;
                self.tick(12);
            }
            DEC_ATHL => {
                let v = self.get_athl()?;
                let (res, h, _) = sub8(v, 1);
                self.set_flags(&[(Flag::Z, res == 0), (Flag::N, true), (Flag::H, h)]);
                self.set_athl(res)?;
                self.tick(12);
            }
            ADD_HL_HL => {
                let hl = self.get_hl();
                self.do_add_hl(hl);
                self.tick(8);
            }
            JP_NN(nn) => {
                self.set_program_counter(nn);
                self.tick(12);
            }
            JP_ATHL => {
                let hl = self.get_hl();
                self.set_program_counter(hl);
                self.tick(4);
            }
            INC_HL => {
                let hl = self.get_hl();
                self.set_hl(hl.wrapping_add(1));
                self.tick(8);
            }
            INC_BC => {
                let hl = self.get_bc();
                self.set_bc(hl.wrapping_add(1));
                self.tick(8);
            }
            INC_DE => {
                let hl = self.get_de();
                self.set_de(hl.wrapping_add(1));
                self.tick(8);
            }
            AND_R(r) => {
                let r = self.get_register(r);
                self.do_and_a(r);
                self.tick(4);
            }
            AND_N(n) => {
                self.do_and_a(n);
                self.tick(8);
            }
            OR_R(r) => {
                let r = self.get_register(r);
                self.do_or_a(r);
                self.tick(4);
            }
            OR_ATHL => {
                let v = self.get_athl()?;
                self.do_or_a(v);
                self.tick(8);
            }
            XOR_R(r) => {
                let r = self.get_register(r);
                self.do_xor_a(r);
                self.tick(4);
            }
            XOR_N(n) => {
                self.do_xor_a(n);
                self.tick(8);
            }
            XOR_ATHL => {
                let athl = self.get_athl()?;
                self.do_xor_a(athl);
                self.tick(8);
            }
            CP_N(n) => {
                self.do_cp_a(n);
                self.tick(8);
            }
            CP_ATHL => {
                let athl = self.get_athl()?;
                self.do_cp_a(athl);
                self.tick(8);
            }
            JR_N(n) => {
                let pc = self.get_program_counter();
                self.set_program_counter(add_u16_i8(pc, n as i8));
                self.tick(8)
            }
            JR_C_N(c, n) => {
                if self.test_cond(c) {
                    let pc = self.get_program_counter();
                    self.set_program_counter(add_u16_i8(pc, n as i8));
                }
                self.tick(8);
            }
            CALL_NN(nn) => {
                let pc = self.get_program_counter();
                self.push_stack16(pc)?;
                self.set_program_counter(nn);
                self.tick(12);
            }
            CALL_C_NN(c, nn) => {
                if self.test_cond(c) {
                    let pc = self.get_program_counter();
                    self.push_stack16(pc)?;
                    self.set_program_counter(nn);
                }
                self.tick(12);
            }
            RRA => {
                self.do_rr(ARegister);
                self.tick(4);
            }
            RR_R(r) => {
                self.do_rr(r);
                self.tick(8);
            }
            SRL_R(r) => {
                let v = self.get_register(r);
                let lsb = get_bit(v, 0);
                let v = v >> 1;
                self.set_flags(&[(Flag::Z, v == 0),
                                 (Flag::N, false),
                                 (Flag::H, false),
                                 (Flag::C, lsb)]);
                self.set_register(r, v);
                self.tick(8);
            }
            SRL_ATHL => {
                let v = self.get_athl()?;
                let lsb = get_bit(v, 0);
                let v = v >> 1;
                self.set_flags(&[(Flag::Z, v == 0),
                                 (Flag::N, false),
                                 (Flag::H, false),
                                 (Flag::C, lsb)]);
                self.set_athl(v)?;
                self.tick(16);
            }
            BIT_B_R(b, r) => {
                let v = self.get_register(r);
                let btest = get_bit(v, bit_number(b));
                self.set_flags(&[(Flag::Z, !btest), (Flag::N, false), (Flag::H, true)]);
                self.tick(8);
            }
            SET_B_R(b, r) => {
                let v = self.get_register(r);
                let v = set_bit(v, bit_number(b), true);
                self.set_register(r, v);
                self.tick(8);
            }
            RES_B_R(b, r) => {
                let v = self.get_register(r);
                let v = set_bit(v, bit_number(b), false);
                self.set_register(r, v);
                self.tick(8);
            }
            RET => {
                let pc = self.pop_stack16()?;
                self.set_program_counter(pc);
                self.tick(8);
            }
            RET_C(c) => {
                if self.test_cond(c) {
                    let pc = self.pop_stack16()?;
                    self.set_program_counter(pc);
                }
                self.tick(8);
            }
            DI => {
                self.set_interrupts_enabled(false);
            }
            EI => {
                self.set_interrupts_enabled(true);
            }
            ins => {
                return Err(format!("instruction {:?} unimplemented", ins).into());
            }
        }

        Ok(())
    }
}
