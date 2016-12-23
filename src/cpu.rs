use std::error::Error;
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
    fn set_stack_pointer(&mut self, pc: u16);

    fn tick(&mut self, count: u16);

    fn halt(&mut self);
    fn stop(&mut self);

    fn set_interrupts_enabled(&mut self, enabled: bool);
}

pub trait Memory {
    fn get_memory(&self, addr: u16) -> u8;
    fn set_memory(&mut self, addr: u16, val: u8);
}

pub fn step<C: Cpu + Memory>(cpu: &mut C) -> Result<(), Box<Error>> {
    let instruction = decode_instruction(|| {
        let pc = cpu.get_program_counter();
        if pc == u16::max_value() {
            None
        } else {
            cpu.set_program_counter(pc + 1);
            Some(cpu.get_memory(pc))
        }
    })?;

    do_instruction(cpu, instruction)
}

fn do_instruction<C: Cpu + Memory>(cpu: &mut C, i: Instruction) -> Result<(), Box<Error>> {
    match i {
        LD_R_R(tr, sr) => {
            let sv = cpu.get_register(sr);
            cpu.set_register(tr, sv);
            cpu.tick(4);
        }
        _ => unimplemented!(),
    }

    Ok(())
}
