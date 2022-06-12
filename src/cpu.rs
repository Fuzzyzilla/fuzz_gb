
use std::fmt::Display;

#[derive(Debug)]
pub enum Register8 {
    A, B, C, D, E, H, L,
}
pub enum Register16 {
    AF, BC, DE, HL, SP, PC,
}
impl Display for Register8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Register8::A => write!(f, "A"),
            Register8::B => write!(f, "B"),
            Register8::C => write!(f, "C"),
            Register8::D => write!(f, "D"),
            Register8::E => write!(f, "E"),
            Register8::H => write!(f, "H"),
            Register8::L => write!(f, "L"),
        }
    }
}
impl Display for Register16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Register16::AF => write!(f, "AF"),
            Register16::BC => write!(f, "BC"),
            Register16::DE => write!(f, "DE"),
            Register16::HL => write!(f, "HL"),
            Register16::SP => write!(f, "SP"),
            Register16::PC => write!(f, "PC")
        }
    }
}
pub enum Flag {
    Zero = 7, Negative = 6, HalfCarry = 5, Carry = 4,
    
    //Inverted forms are 4 less than their positive conterparts
    NotZero = 3 , NotNegative = 2, NotHalfCarry = 1, NotCarry = 0,
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
    pub a : u8,
    pub b : u8,
    pub c : u8,
    pub d : u8,
    pub e : u8,
    pub h : u8,
    pub l : u8,
    pub flags : u8,
    pub sp : u16,
    pub pc : u16
}

impl Registers {
    pub fn af(&self) -> u16 {
        Registers::get_u8s_into_u16(self.a, self.flags)
    }
    pub fn set_af(&mut self, value : u16) {
        Registers::set_u16_into_u8s(value, &mut self.a, &mut self.flags);
    }
    pub fn bc(&self) -> u16 {
        Registers::get_u8s_into_u16(self.b, self.c)
    }
    pub fn set_bc(&mut self, value : u16) {
        Registers::set_u16_into_u8s(value, &mut self.b, &mut self.c);
    }
    pub fn de(&self) -> u16 {
        Registers::get_u8s_into_u16(self.d, self.e)
    }
    pub fn set_de(&mut self, value : u16) {
        Registers::set_u16_into_u8s(value, &mut self.d, &mut self.e);
    }
    pub fn hl(&self) -> u16 {
        Registers::get_u8s_into_u16(self.h, self.l)
    }
    pub fn set_hl(&mut self, value : u16) {
        Registers::set_u16_into_u8s(value, &mut self.h, &mut self.l);
    }
    pub fn set_pc(&mut self, value : u16) {
        self.pc = value;
    }
    pub fn pc(&self) -> u16 {
        self.pc
    }
    pub fn set_sp(&mut self, value : u16) {
        self.sp = value;
    }
    pub fn sp(&self) -> u16 {
        self.sp
    }
    pub fn flag(&self, f : Flag) -> bool {
        match f {
            Flag::Zero | Flag::Carry | Flag::HalfCarry | Flag::Negative =>
                //Test the bit as determined by the flag index
                self.flags & (f as u8) != 0,
            Flag::NotZero | Flag::NotCarry | Flag::NotHalfCarry | Flag::NotNegative =>
                //Inverted constants are four less than their positive counterparts
                self.flags & (f as u8 + 4) == 0
        }
    }
    pub fn set_flag(&mut self, f : Flag) {
        match f {
            Flag::Zero | Flag::Carry | Flag::HalfCarry | Flag::Negative =>
                //Test the bit as determined by the flag index
                self.flags |= 1 << f as u8,
            Flag::NotZero =>
                self.reset_flag(Flag::Zero),
            Flag::NotCarry =>
                self.reset_flag(Flag::Carry),
            Flag::NotHalfCarry =>
                self.reset_flag(Flag::HalfCarry),
            Flag::NotNegative =>
                self.reset_flag(Flag::Negative),
        }
    }
    pub fn reset_flag(&mut self, f : Flag) {
        match f {
            Flag::Zero | Flag::Carry | Flag::HalfCarry | Flag::Negative =>
                //Test the bit as determined by the flag index
                self.flags &= !(1 << f as u8),
            Flag::NotZero =>
                self.set_flag(Flag::Zero),
            Flag::NotCarry =>
                self.set_flag(Flag::Carry),
            Flag::NotHalfCarry =>
                self.set_flag(Flag::HalfCarry),
            Flag::NotNegative =>
                self.set_flag(Flag::Negative),
        }
    }
    pub fn set_u8_register(&mut self, r : &Register8, value : u8) {
        match r {
            Register8::A => self.a = value,
            Register8::B => self.b = value,
            Register8::C => self.c = value,
            Register8::D => self.d = value,
            Register8::E => self.e = value,
            Register8::H => self.h = value,
            Register8::L => self.l = value,
        };
    }
    pub fn set_u16_register(&mut self, r : &Register16, value : u16) {
        match r {
            Register16::AF => self.set_af(value),
            Register16::BC => self.set_bc(value),
            Register16::DE => self.set_de(value),
            Register16::HL => self.set_hl(value),
            Register16::SP => self.sp = value,
            Register16::PC => self.pc = value,
        };
    }
    pub fn get_u8_register(&self, r : &Register8) -> u8 {
        match r {
            Register8::A => self.a,
            Register8::B => self.b,
            Register8::C => self.c,
            Register8::D => self.d,
            Register8::E => self.e,
            Register8::H => self.h,
            Register8::L => self.l,
        }
    }
    pub fn get_u8_register_mut<'a>(&'a mut self, r : &Register8) -> &'a mut u8 {
        match r {
            Register8::A => &mut self.a,
            Register8::B => &mut self.b,
            Register8::C => &mut self.c,
            Register8::D => &mut self.d,
            Register8::E => &mut self.e,
            Register8::H => &mut self.h,
            Register8::L => &mut self.l,
        }
    }
    pub fn get_u16_register(&self, r : &Register16) -> u16 {
        match r {
            Register16::AF => self.af(),
            Register16::BC => self.bc(),
            Register16::DE => self.de(),
            Register16::HL => self.hl(),
            Register16::SP => self.sp,
            Register16::PC => self.pc,
        }
    }
    pub fn increment_u16_register(&mut self, r : &Register16) -> u16 {
        let prev = self.get_u16_register(r);
        self.set_u16_register(r, prev.wrapping_add(1));

        prev
    }
    pub fn decrement_u16_register(&mut self, r : &Register16) -> u16 {
        let prev = self.get_u16_register(r);
        self.set_u16_register(r, prev.wrapping_sub(1));

        prev
    }
    fn set_u16_into_u8s(value : u16, high : &mut u8, low : &mut u8) {
        *high = ((value >> 8) & 0xff) as u8;
        *low = (value & 0xff) as u8;
    }
    fn get_u8s_into_u16(high : u8, low : u8) -> u16 {
        (high as u16) << 8 | (low as u16)
    }
}

impl Default for Registers {
    fn default() -> Self {
        return Registers { a: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, flags: 0, sp: 0, pc: 0 }
    }
}

impl Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "A: {:02X} F: {:02X}, AF: {:04X}", self.a, self.flags, self.af())?;
        writeln!(f, "B: {:02X} C: {:02X}, BC: {:04X}", self.b, self.c, self.bc())?;
        writeln!(f, "H: {:02X} L: {:02X}, HL: {:04X}", self.h, self.l, self.hl())?;
        writeln!(f, "E: {:02X}", self.e)?;
        write!(f, "SP: {:04X}  PC : {:04X}", self.sp, self.pc)?;

        Ok(())
    }
}