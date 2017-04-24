use util::*;
use cpu::*;
use instruction::*;
use screen::*;

pub struct Emulator {
    pub interrupts_enabled: u8,
    pub stack_pointer: u16,
    pub program_counter: u16,

    pub a_register: u8,
    pub b_register: u8,
    pub c_register: u8,
    pub d_register: u8,
    pub e_register: u8,
    pub h_register: u8,
    pub l_register: u8,

    pub flags: Flags,

    pub cartridge_rom_bank0: [u8; 0x4000],
    pub cartridge_rom_bank1: [u8; 0x4000],

    pub internal_ram_bank0: [u8; 0x1000],
    pub internal_ram_bank1: [u8; 0x1000],
    pub zero_page: [u8; 0x7f],

    pub character_ram: [u8; 0x1800],
    pub bg_map_data: [u8; 0x800],
    pub sprite_attribute_data: [u8; 0x80],
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            interrupts_enabled: 0x0f,
            stack_pointer: 0xfffe,
            program_counter: 0x100,
            a_register: 0x0,
            b_register: 0x0,
            c_register: 0x0,
            d_register: 0x0,
            e_register: 0x0,
            h_register: 0x0,
            l_register: 0x0,
            flags: Flags {
                zero: false,
                subtract: false,
                half_carry: false,
                carry: false,
            },
            cartridge_rom_bank0: [0x0; 0x4000],
            cartridge_rom_bank1: [0x0; 0x4000],
            internal_ram_bank0: [0x0; 0x1000],
            internal_ram_bank1: [0x0; 0x1000],
            zero_page: [0x0; 0x7f],
            character_ram: [0x0; 0x1800],
            bg_map_data: [0x0; 0x800],
            sprite_attribute_data: [0x0; 0x80],
        }
    }

    pub fn load_rom(rom: &[u8]) -> Result<Emulator> {
        if rom.len() < 0x4000 {
            return Err("rom size invalid".into());
        }

        let mut state = Emulator::new();

        let cart_type = rom[0x147];
        let rom_size = rom[0x148];

        // TODO: Actually implement MBC1
        if cart_type != 0 && cart_type != 1 {
            return Err(format!("mbc / ram unsupported, cart type {:x}", cart_type).into());
        }

        match rom_size {
            0 => {
                if rom.len() != 0x8000 {
                    return Err("rom size mismatch".into());
                }

                state
                    .cartridge_rom_bank0
                    .copy_from_slice(&rom[0..0x4000]);
                state
                    .cartridge_rom_bank1
                    .copy_from_slice(&rom[0x4000..0x8000]);
            }
            s => return Err(format!("unsupported rom_size code {:x}", s).into()),
        };

        Ok(state)
    }

    pub fn step(&mut self) -> Result<()> {
        step_cpu(self)
    }

    pub fn get_screen(&self) -> Screen {
        let mut screen = Screen::new();
        for y in 0..VERTICAL_SCREEN_PIXELS / 8 {
            for x in 0..HORIZONTAL_SCREEN_PIXELS / 8 {
                let tile = self.bg_map_data[y as usize * 32 + x as usize];
                for ytile in 0..8 {
                    let b1 = self.character_ram[tile as usize * 16 + ytile as usize * 2];
                    let b2 = self.character_ram[tile as usize * 16 + ytile as usize * 2 + 1];
                    for xtile in 0..8 {
                        let column = 7 - xtile;
                        screen.set_pixel(x * 8 + xtile,
                                         y * 8 + ytile,
                                         match (get_bit(b1, column), get_bit(b2, column)) {
                                             (false, false) => Pixel::White,
                                             (false, true) => Pixel::LightGray,
                                             (true, false) => Pixel::DarkGray,
                                             (true, true) => Pixel::Black,
                                         });
                    }
                }
            }
        }
        screen
    }
}

impl Cpu for Emulator {
    fn get_register(&self, reg: Register) -> u8 {
        match reg {
            Register::ARegister => self.a_register,
            Register::BRegister => self.b_register,
            Register::CRegister => self.c_register,
            Register::DRegister => self.d_register,
            Register::ERegister => self.e_register,
            Register::HRegister => self.h_register,
            Register::LRegister => self.l_register,
        }
    }

    fn set_register(&mut self, reg: Register, val: u8) {
        match reg {
            Register::ARegister => self.a_register = val,
            Register::BRegister => self.b_register = val,
            Register::CRegister => self.c_register = val,
            Register::DRegister => self.d_register = val,
            Register::ERegister => self.e_register = val,
            Register::HRegister => self.h_register = val,
            Register::LRegister => self.l_register = val,
        }
    }

    fn get_flags(&self) -> Flags {
        self.flags
    }

    fn set_flags(&mut self, flags: Flags) {
        self.flags = flags;
    }

    fn get_program_counter(&self) -> u16 {
        self.program_counter
    }

    fn set_program_counter(&mut self, pc: u16) {
        self.program_counter = pc;
    }

    fn get_stack_pointer(&self) -> u16 {
        self.stack_pointer
    }

    fn set_stack_pointer(&mut self, pc: u16) {
        self.stack_pointer = pc;
    }

    fn set_interrupts_enabled(&mut self, _enabled: bool) {}

    fn tick(&mut self, _count: u8) {}

    fn halt(&mut self) {}

    fn stop(&mut self) {}

    fn get_memory(&self, addr: u16) -> Result<u8> {
        match addr {
            0...0x3fff => Ok(self.cartridge_rom_bank0[addr as usize]),
            0x4000...0x7fff => Ok(self.cartridge_rom_bank1[addr as usize - 0x4000]),
            0x8000...0x97ff => Ok(self.character_ram[addr as usize - 0x8000]),
            0x9800...0x9fff => Ok(self.bg_map_data[addr as usize - 0x9800]),
            0xa000...0xbfff => Err(format!("Illegal read from cartridge ram bank {}", addr).into()),
            0xc000...0xcfff => Ok(self.internal_ram_bank0[addr as usize - 0xc000]),
            0xd000...0xdfff => Ok(self.internal_ram_bank1[addr as usize - 0xd000]),
            0xe000...0xfdff => self.get_memory(addr - 0x2000),
            0xfe00...0xfe99 => Ok(self.sprite_attribute_data[addr as usize - 0xfe00]),
            0xfea0...0xfeff => {
                Err(format!("Illegal read from unusable memory region {}", addr).into())
            }
            0xff00...0xff7f => Ok(0x0), // TODO: Implement hardware registers
            0xff80...0xfffe => Ok(self.zero_page[addr as usize - 0xff80]),
            _ => {
                assert_eq!(addr, 0xffff);
                Ok(self.interrupts_enabled)
            }
        }
    }

    fn set_memory(&mut self, addr: u16, n: u8) -> Result<()> {
        match addr {
            0...0x3fff => Err(format!("Illegal write to cartridge rom {}", addr).into()),
            0x4000...0x7fff => Err(format!("Illegal write to cartridge rom {}", addr).into()),
            0x8000...0x97ff => Ok(self.character_ram[addr as usize - 0x8000] = n),
            0x9800...0x9fff => Ok(self.bg_map_data[addr as usize - 0x9800] = n),
            0xa000...0xbfff => Err(format!("Illegal write to cartridge ram bank {}", addr).into()),
            0xc000...0xcfff => Ok(self.internal_ram_bank0[addr as usize - 0xc000] = n),
            0xd000...0xdfff => Ok(self.internal_ram_bank1[addr as usize - 0xd000] = n),
            0xe000...0xfdff => self.set_memory(addr - 0x2000, n),
            0xfe00...0xfe99 => Ok(self.sprite_attribute_data[addr as usize - 0xfe00] = n),
            0xfea0...0xfeff => {
                Err(format!("Illegal write to unusable memory region {}", addr).into())
            }
            0xff00...0xff7f => Ok(()), // TODO: Implement hardware registers
            0xff80...0xfffe => Ok(self.zero_page[addr as usize - 0xff80] = n),
            _ => {
                assert_eq!(addr, 0xffff);
                Ok(self.interrupts_enabled = n)
            }
        }
    }
}
