
use std::fmt::Display;

#[derive(Debug)]
pub enum Register {
    A, B, C, D, E, H, L, BC, DE, HL, SP, PC,
}
impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::A => write!(f, "A"),
            Register::B => write!(f, "B"),
            Register::C => write!(f, "C"),
            Register::D => write!(f, "D"),
            Register::E => write!(f, "E"),
            Register::H => write!(f, "H"),
            Register::L => write!(f, "L"),
            Register::BC => write!(f, "BC"),
            Register::DE => write!(f, "DE"),
            Register::HL => write!(f, "HL"),
            Register::SP => write!(f, "SP"),
            Register::PC => write!(f, "PC")
        }
    }
}
pub enum Flag {
    Zero, Negative, HalfCarry, Carry,
    NotZero, NotNegative, NotHalfCarry, NotCarry,
}
impl Display for Flag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Flag::Zero => write!(f, "Z"),
            Flag::Negative => write!(f, "N"),
            Flag::HalfCarry => write!(f, "H"),
            Flag::Carry => write!(f, "C"),
            Flag::NotZero => write!(f, "NZ"),
            Flag::NotNegative => write!(f, "NN"),
            Flag::NotHalfCarry => write!(f, "NH"),
            Flag::NotCarry => write!(f, "NC"),
        }
    }
}
pub struct Registers {
    a : u8,
    b : u8,
    c : u8,
    d : u8,
    e : u8,
    h : u8,
    l : u8,
    flags : u8,
    sp : u16,
    pc : u16
}

impl Registers {
    pub fn get_bc(&self) -> u16 {
        Registers::get_u8s_into_u16(self.b, self.c)
    }
    pub fn set_bc(&mut self, value : u16) {
        Registers::set_u16_into_u8s(value, &mut self.b, &mut self.c);
    }
    pub fn get_de(&self) -> u16 {
        Registers::get_u8s_into_u16(self.d, self.e)
    }
    pub fn set_de(&mut self, value : u16) {
        Registers::set_u16_into_u8s(value, &mut self.d, &mut self.e);
    }
    pub fn get_hl(&self) -> u16 {
        Registers::get_u8s_into_u16(self.h, self.l)
    }
    pub fn set_hl(&mut self, value : u16) {
        Registers::set_u16_into_u8s(value, &mut self.h, &mut self.l);
    }
    pub fn set_u8_register(&mut self, r : &Register, value : u8) {
        match r {
            Register::A => self.a = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::E => self.e = value,
            Register::H => self.h = value,
            Register::L => self.l = value,
            _ => panic!("Bad register to set to u8 value! {:?}", r)
        };
    }
    pub fn set_u16_register(&mut self, r : &Register, value : u16) {
        match r {
            Register::BC => self.set_bc(value),
            Register::DE => self.set_de(value),
            Register::HL => self.set_hl(value),
            Register::SP => self.sp = value,
            Register::PC => self.pc = value,
            _ => panic!("Bad register to set to u16 value! {:?}", r)
        };
    }
    pub fn get_u8_register(&self, r : &Register) -> u8 {
        match r {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::H => self.h,
            Register::L => self.l,
            _ => panic!("Bad register to get u8 value! {:?}", r)
        }
    }
    pub fn get_u16_register(&self, r : &Register) -> u16 {
        match r {
            Register::BC => self.get_bc(),
            Register::DE => self.get_de(),
            Register::HL => self.get_hl(),
            Register::SP => self.sp,
            Register::PC => self.pc,
            _ => panic!("Bad register to get u16 value! {:?}", r)
        }
    }
    pub fn increment_u16_register(&mut self, r : &Register) {
        self.set_u16_register(r, self.get_u16_register(r).wrapping_add(1));
    }
    pub fn increment_u8_register(&mut self, r : &Register) {
        self.set_u8_register(r, self.get_u8_register(r).wrapping_add(1));
    }
    fn set_u16_into_u8s(value : u16, high : &mut u8, low : &mut u8) {
        *high = ((value >> 8) & 0xff) as u8;
        *low = (value & 0xff) as u8;
    }
    fn get_u8s_into_u16(high : u8, low : u8) -> u16 {
        (high as u16) << 8 | (low as u16)
    }
}