use crate::cpu;
use crate::bitmath;
use crate::memory::Memory;

use std::fmt;

use bitmath::join_u8;

pub enum MutableData8 {
    Register8(cpu::Register8),
    IndirectRegister16(cpu::Register16),
    IndirectRegister16Inc(cpu::Register16),
    IndirectRegister16Dec(cpu::Register16),
    IndirectRegister8(cpu::Register8),
    IndirectValue8(u8),
    IndirectValue16(u16)
}

impl MutableData8 {
    pub fn get(&self, state : &mut cpu::Registers, memory : &Memory) -> u8 {
        match &self {
            Self::Register8(reg)
                => state.get_u8_register(reg),
            Self::IndirectRegister16(reg)
                => memory.read(state.get_u16_register(reg)),
            Self::IndirectRegister16Inc(reg)
                => memory.read(state.increment_u16_register(reg)),
            Self::IndirectRegister16Dec(reg)
                => memory.read(state.decrement_u16_register(reg)),
            Self::IndirectRegister8(reg)
                => memory.read(0xFF00 + state.get_u8_register(reg) as u16),
            Self::IndirectValue16(addr)
                => memory.read(*addr),
            Self::IndirectValue8(addr)
                => memory.read(0xFF00 + *addr as u16),
        }
    }
    pub fn set(&self, value : u8, state : &mut cpu::Registers, memory : &mut Memory) {
        match &self {
            Self::Register8(reg)
                => state.set_u8_register(reg, value),
            Self::IndirectRegister16(reg)
                => memory.write(state.get_u16_register(reg), value),
            Self::IndirectRegister16Inc(reg)
                => memory.write(state.increment_u16_register(reg), value),
            Self::IndirectRegister16Dec(reg)
                => memory.write(state.decrement_u16_register(reg), value),
            Self::IndirectRegister8(reg)
                => memory.write(0xFF00 + state.get_u8_register(reg) as u16, value),
            Self::IndirectValue16(addr)
                => memory.write(*addr, value),
            Self::IndirectValue8(addr)
                => memory.write(0xFF00 + *addr as u16, value),
        }
    }
}

impl fmt::Display for MutableData8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Register8(reg)
                => write!(f, "{}", reg),
            Self::IndirectRegister16(reg)
                => write!(f, "({})", reg),
            Self::IndirectRegister16Inc(reg)
                => write!(f, "({}+)", reg),
            Self::IndirectRegister16Dec(reg)
                => write!(f, "({}-)", reg),
            Self::IndirectValue8(value)
                => write!(f, "($FF00 + {:02X})", value),
            Self::IndirectRegister8(reg)
                => write!(f, "($FF00 + {})", reg),
            Self::IndirectValue16(value)
                => write!(f, "(${:04X})", value),
        }
    }
}
pub enum Data8 {
    Immutable(u8),
    Mutable(MutableData8)
}

impl Data8 {
    pub fn get(&self, state : &mut cpu::Registers, memory : &Memory) -> u8 {
        match &self {
            Self::Immutable(data) => *data,
            Self::Mutable(mutable) => mutable.get(state, memory),
        }
    }
}

impl fmt::Display for Data8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Immutable(value)
                => write!(f, "${:02X}", value),
            Self::Mutable(mutable)
                => mutable.fmt(f)
        }
    }
}

pub enum MutableData16 {
    Register16(cpu::Register16),
    IndirectValue16(u16)
}

impl MutableData16 {
    pub fn get(&self, state : &cpu::Registers, memory : &Memory) -> u16 {
        match &self {
            Self::Register16(reg) => state.get_u16_register(reg),
            Self::IndirectValue16(addr) => memory.read_u16(*addr),
        }
    }
    pub fn set(&self, value : u16, state : &mut cpu::Registers, memory : &mut Memory) {
        match &self {
            Self::Register16(reg) => state.set_u16_register(reg, value),
            Self::IndirectValue16(addr) => memory.write_u16(*addr, value),
        }
    }
}

impl fmt::Display for MutableData16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Register16(reg)
                => write!(f, "{}", reg),
            Self::IndirectValue16(addr)
                => write!(f, "(${:04X})", addr),
        }
    }
}

pub enum Data16 {
    Immutable(u16),
    Mutable(MutableData16)
}

impl Data16 {

}

impl fmt::Display for Data16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Immutable(value)
                => write!(f, "${:04X}", value),
            Self::Mutable(mutable)
                => mutable.fmt(f)
        }
    }
}
pub enum Op {
    Nop,
    Stop,
    Halt,
    Load8{into : MutableData8, from : Data8},
    Load16{into : MutableData16, from : Data16},
    Inc8{into : MutableData8},
    Dec8{into : MutableData8},
    Inc16{into : MutableData16},
    Dec16{into : MutableData16},
    Ror{into : MutableData8},
    RorCarry{into : MutableData8},
    Rol{into : MutableData8},
    RolCarry{into : MutableData8},
    Add{into : MutableData8, from : Data8},
    Add16{into : MutableData16, from : Data16},
    AddCarry{into : MutableData8, from : Data8},
    Sub{into : MutableData8, from : Data8},
    SubCarry{into : MutableData8, from : Data8},
    And{into : MutableData8, from : Data8},
    Or{into : MutableData8, from : Data8},
    Xor{into : MutableData8, from : Data8},
    Compare{into : Data8, from : Data8},
    JumpRelative{amount : i8},
    JumpRelativeIf{condition : cpu::Flag, amount : i8},
    Call{address : Data16},
    CallIf{condition : cpu::Flag, address : Data16},
    Return,
    ReturnIf{condition : cpu::Flag},
    Push{from : Data16},
    Pop{into : MutableData16},

    ShiftLeftAccumulator{into : MutableData8},
    ShiftRightLogical{into : MutableData8},
    ShiftRightAccumulator{into : MutableData8},
    Swap{into : MutableData8},

    Bit{into : Data8, bit : u8},
    Reset{into : MutableData8, bit : u8},
    Set{into : MutableData8, bit : u8},

    Unimplemented(u8)
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Nop => write!(f, "NOP"),
            Op::Stop => write!(f, "STOP"),
            Op::Halt => write!(f, "HALT"),
            Op::Load8{into, from} =>
                write!(f, "LD {}, {}", into, from),
            Op::Load16{into, from} =>
                write!(f, "LD {}, {}", into, from),
            Op::Inc8{into} =>
                write!(f, "INC {}", into),
            Op::Dec8{into} =>
                write!(f, "DEC {}", into),
            Op::Inc16{into} =>
                write!(f, "INC {}", into),
            Op::Dec16{into} =>
                write!(f, "DEC {}", into),
            Op::Ror{into} =>
                write!(f, "ROR {}", into),
            Op::RorCarry{into} =>
                write!(f, "RRC {}", into),
            Op::Rol{into} =>
                write!(f, "ROL {}", into),
            Op::RolCarry{into} =>
                write!(f, "RLC {}", into),
            Op::Add{from, into} =>
                write!(f, "ADD {}, {}", into, from),
            Op::Add16{from, into} =>
                write!(f, "ADD {}, {}", into, from),
            Op::AddCarry{from, into} =>
                write!(f, "ADC {}, {}", into, from),
            Op::Sub{from, into} =>
                write!(f, "SUB {}, {}", into, from),
            Op::SubCarry{from, into} =>
                write!(f, "SBC {}, {}", into, from),
            Op::And{from, into} =>
                write!(f, "AND {}, {}", into, from),
            Op::Or{from, into} =>
                write!(f, "OR  {}, {}", into, from),
            Op::Xor{from, into} =>
                write!(f, "XOR {}, {}", into, from),
            Op::Compare{from, into} =>
                write!(f, "CMP {} {}", into, from),
            Op::JumpRelative{amount} => 
                write!(f, "JR  {}", amount),
            Op::JumpRelativeIf{amount, condition} => 
                write!(f, "JR {condition} {}", amount),
            Op::Call{address} =>
                write!(f, "CALL {}", address),
            Op::CallIf{condition, address} =>
                write!(f, "CALL {condition} {}", address),
            Op::Return =>
                write!(f, "RET"),
            Op::ReturnIf{condition} =>
                write!(f, "RET {condition}"),
            Op::Push{from} =>
                write!(f, "PUSH {}", from),
            Op::Pop{into} =>
                write!(f, "POP {}", into),
            Op::Swap{into} =>
                write!(f, "SWAP {}", into),
            Op::ShiftLeftAccumulator{into} =>
                write!(f, "SLA {}", into),
            Op::ShiftRightAccumulator{into} =>
                write!(f, "SRA {}", into),
            Op::ShiftRightLogical{into} =>
                write!(f, "SRL {}", into),
            Op::Bit{into, bit} =>
                write!(f, "BIT {bit}, {}", into),
            Op::Set{into, bit} =>
                write!(f, "SET {bit}, {}", into),
            Op::Reset{into, bit} =>
                write!(f, "RES {bit}, {}", into),

            
            Op::Unimplemented(instr) =>
                write!(f, "UNIMPLEMENTED {:02x}", instr)
        }
    }
}

pub struct Instruction {
    pub op : Op,
    pub cycles : u8,
    pub size : u8
}

impl Instruction {
    pub fn from_bytes(addr : usize, data : &[u8]) -> Option<Instruction> {
        Some(match &data[addr..] {

            [0x00, ..]
                => Instruction{ size : 1, cycles : 4, op : Op::Nop },
            [0x10, ..]
                => Instruction{ size : 2, cycles : 1, op : Op::Stop },
            [0x20, a, ..]
                => Instruction{ size : 2, cycles : 1, op : Op::JumpRelativeIf {
                    amount : *a as i8,
                    condition : cpu::Flag::NotZero 
                } },
            [0x30, a, ..]
                => Instruction{ size : 2, cycles : 1, op : Op::JumpRelativeIf {
                    amount : *a as i8,
                    condition : cpu::Flag::NotCarry 
                } },

            [0x01, a, b, ..]
                => Instruction{ size : 3, cycles : 3, op : Op::Load16{
                    into : MutableData16::Register16(cpu::Register16::BC),
                    from : Data16::Immutable(join_u8(*a, *b))
                } },
            [0x11, a, b, ..]
                => Instruction{ size : 3, cycles : 3, op : Op::Load16{
                    into : MutableData16::Register16(cpu::Register16::DE),
                    from : Data16::Immutable(join_u8(*a, *b))
                } },
            [0x21, a, b, ..]
                => Instruction{ size : 3, cycles : 3, op : Op::Load16{
                    into : MutableData16::Register16(cpu::Register16::HL),
                    from : Data16::Immutable(join_u8(*a, *b))
                } },
            [0x31, a, b, ..]
                => Instruction{ size : 3, cycles : 3, op : Op::Load16{
                    into : MutableData16::Register16(cpu::Register16::SP),
                    from : Data16::Immutable(join_u8(*a, *b))
                } },

            [0x02, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Load8{
                    into : MutableData8::IndirectRegister16(cpu::Register16::BC),
                    from : Data8::Mutable(MutableData8::Register8(cpu::Register8::A))
                } },
            [0x12, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Load8{
                    into : MutableData8::IndirectRegister16(cpu::Register16::DE),
                    from : Data8::Mutable(MutableData8::Register8(cpu::Register8::A))
                } },
            [0x22, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Load8{
                    into : MutableData8::IndirectRegister16Inc(cpu::Register16::HL),
                    from : Data8::Mutable(MutableData8::Register8(cpu::Register8::A))
                } },
            [0x32, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Load8{
                    into : MutableData8::IndirectRegister16Dec(cpu::Register16::HL),
                    from : Data8::Mutable(MutableData8::Register8(cpu::Register8::A))
                } },

            [0x03, ..]
                => Instruction{ size : 1, cycles : 8, op : Op::Inc16{
                    into : MutableData16::Register16(cpu::Register16::BC)
                } },
            [0x13, ..]
                => Instruction{ size : 1, cycles : 8, op : Op::Inc16{
                    into : MutableData16::Register16(cpu::Register16::DE)
                } },
            [0x23, ..]
                => Instruction{ size : 1, cycles : 8, op : Op::Inc16{
                    into : MutableData16::Register16(cpu::Register16::HL)
                } },
            [0x33, ..]
                => Instruction{ size : 1, cycles : 8, op : Op::Inc16{
                    into : MutableData16::Register16(cpu::Register16::SP)
                } },


            [0x04, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Inc8{
                    into : MutableData8::Register8(cpu::Register8::B)
                } },
            [0x14, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Inc8{
                    into : MutableData8::Register8(cpu::Register8::D)
                } },
            [0x24, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Inc8{
                    into : MutableData8::Register8(cpu::Register8::H)
                } },
            [0x34, ..]
                => Instruction{ size : 1, cycles : 3, op : Op::Inc8{
                    into : MutableData8::IndirectRegister16(cpu::Register16::HL)
                } },


            [0x05, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Dec8{
                    into : MutableData8::Register8(cpu::Register8::B)
                } },
            [0x15, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Dec8{
                    into : MutableData8::Register8(cpu::Register8::D)
                } },
            [0x25, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Dec8{
                    into : MutableData8::Register8(cpu::Register8::H)
                } },
            [0x35, ..]
                => Instruction{ size : 1, cycles : 3, op : Op::Dec8{
                    into : MutableData8::IndirectRegister16(cpu::Register16::HL)
                } },

            
            [0x06, a, ..]
                => Instruction{ size : 2, cycles : 2, op : Op::Load8{
                    into : MutableData8::Register8(cpu::Register8::B),
                    from : Data8::Immutable(*a)
                } },
            [0x16, a, ..]
                => Instruction{ size : 2, cycles : 2, op : Op::Load8{
                    into : MutableData8::Register8(cpu::Register8::D),
                    from : Data8::Immutable(*a)
                } },
            [0x26, a, ..]
                => Instruction{ size : 2, cycles : 2, op : Op::Load8{
                    into : MutableData8::Register8(cpu::Register8::H),
                    from : Data8::Immutable(*a)
                } },
            [0x36, a, ..]
                => Instruction{ size : 2, cycles : 3, op : Op::Load8{
                    into : MutableData8::IndirectRegister16(cpu::Register16::HL),
                    from : Data8::Immutable(*a)
                } },
            
            
            [0x07, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::RolCarry{
                    into : MutableData8::Register8(cpu::Register8::A),
                } },
            [0x17, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Rol{
                    into : MutableData8::Register8(cpu::Register8::A),
                } },
            [0x27, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Unimplemented(0x27) },
            [0x37, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Unimplemented(0x37) },
            
            [0x08, a, b, ..]
                => Instruction{ size : 3, cycles : 5, op : Op::Load16{
                    into : MutableData16::IndirectValue16(join_u8(*a, *b)),
                    from : Data16::Mutable(MutableData16::Register16(cpu::Register16::SP))
                } },
            [0x18, a, ..]
                => Instruction{ size : 2, cycles : 3, op : Op::JumpRelative{
                    amount : *a as i8
                } },
            [0x28, a, ..]
                => Instruction{ size : 2, cycles : 2, op : Op::JumpRelativeIf{
                    condition : cpu::Flag::Zero,
                    amount : *a as i8
                } },
            [0x38, a, ..]
                => Instruction{ size : 2, cycles : 2, op : Op::JumpRelativeIf{
                    condition : cpu::Flag::Carry,
                    amount : *a as i8
                } },
            
            [0x09, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Add16{
                    into : MutableData16::Register16(cpu::Register16::HL),
                    from : Data16::Mutable(MutableData16::Register16(cpu::Register16::BC))
                } },
            [0x19, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Add16{
                    into : MutableData16::Register16(cpu::Register16::HL),
                    from : Data16::Mutable(MutableData16::Register16(cpu::Register16::DE))
                } },
            [0x29, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Add16{
                    into : MutableData16::Register16(cpu::Register16::HL),
                    from : Data16::Mutable(MutableData16::Register16(cpu::Register16::HL))
                } },
            [0x39, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Add16{
                    into : MutableData16::Register16(cpu::Register16::HL),
                    from : Data16::Mutable(MutableData16::Register16(cpu::Register16::SP))
                } },
            
            [0x0A, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Load8{
                    into : MutableData8::Register8(cpu::Register8::A),
                    from : Data8::Mutable(MutableData8::IndirectRegister16(cpu::Register16::BC))
                } },
            [0x1A, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Load8{
                    into : MutableData8::Register8(cpu::Register8::A),
                    from : Data8::Mutable(MutableData8::IndirectRegister16(cpu::Register16::DE))
                } },
            [0x2A, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Load8{
                    into : MutableData8::Register8(cpu::Register8::A),
                    from : Data8::Mutable(MutableData8::IndirectRegister16Inc(cpu::Register16::HL))
                } },
            [0x3A, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Load8{
                    into : MutableData8::Register8(cpu::Register8::A),
                    from : Data8::Mutable(MutableData8::IndirectRegister16Dec(cpu::Register16::HL))
                } },

            
            [0x0B, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Dec16{
                    into : MutableData16::Register16(cpu::Register16::BC)
                } },
            [0x1B, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Dec16{
                    into : MutableData16::Register16(cpu::Register16::DE)
                } },
            [0x2B, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Dec16{
                    into : MutableData16::Register16(cpu::Register16::HL)
                } },
            [0x3B, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Dec16{
                    into : MutableData16::Register16(cpu::Register16::SP)
                } },

            [0x0C, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Inc8{
                    into : MutableData8::Register8(cpu::Register8::C)
                } },
            [0x1C, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Inc8{
                    into : MutableData8::Register8(cpu::Register8::E)
                } },
            [0x2C, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Inc8{
                    into : MutableData8::Register8(cpu::Register8::L)
                } },
            [0x3C, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Inc8{
                    into : MutableData8::Register8(cpu::Register8::A)
                } },

            [0x0D, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Dec8{
                    into : MutableData8::Register8(cpu::Register8::C)
                } },
            [0x1D, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Dec8{
                    into : MutableData8::Register8(cpu::Register8::E)
                } },
            [0x2D, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Dec8{
                    into : MutableData8::Register8(cpu::Register8::L)
                } },
            [0x3D, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Dec8{
                    into : MutableData8::Register8(cpu::Register8::A)
                } },
            
            
            [0x0E, a, ..]
                => Instruction{ size : 2, cycles : 2, op : Op::Load8{
                    into : MutableData8::Register8(cpu::Register8::C),
                    from : Data8::Immutable(*a)
                } },
            [0x1E, a, ..]
                => Instruction{ size : 2, cycles : 2, op : Op::Load8{
                    into : MutableData8::Register8(cpu::Register8::E),
                    from : Data8::Immutable(*a)
                } },
            [0x2E, a, ..]
                => Instruction{ size : 2, cycles : 2, op : Op::Load8{
                    into : MutableData8::Register8(cpu::Register8::L),
                    from : Data8::Immutable(*a)
                } },
            [0x3E, a, ..]
                => Instruction{ size : 2, cycles : 2, op : Op::Load8{
                    into : MutableData8::Register8(cpu::Register8::A),
                    from : Data8::Immutable(*a)
                } },

            [0x0F, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::RorCarry{
                    into : MutableData8::Register8(cpu::Register8::A),
                } },
            [0x1F, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Ror{
                    into : MutableData8::Register8(cpu::Register8::A),
                } },
            [0x2F, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Unimplemented(0x2F) },
            [0x3F, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Unimplemented(0x3F) },

            [0xAF, ..]
                => Instruction{ size : 1, cycles : 4, op : Op::Xor{
                    into : MutableData8::Register8(cpu::Register8::A),
                    from : Data8::Mutable(MutableData8::Register8(cpu::Register8::A))
                } },

            [0xCC, a, b, ..]
                => Instruction{ size : 3, cycles : 6, op : Op::CallIf {
                    condition : cpu::Flag::Zero,
                    address : Data16::Immutable(join_u8(*a, *b))
                } },
            [0xDC, a, b, ..]
                => Instruction{ size : 3, cycles : 6, op : Op::CallIf {
                    condition : cpu::Flag::Carry,
                    address : Data16::Immutable(join_u8(*a, *b))
                } },
            [0xCD, a, b, ..]
                => Instruction{ size : 3, cycles : 6, op : Op::Call {
                    address : Data16::Immutable(join_u8(*a, *b))
                } },
            
            [0x40..=0x7f, ..]
                => {
                    let opcode = data[0];

                    if opcode == 0x76 {
                        Instruction{ size : 1, cycles : 4, op : Op::Halt }
                    } else {
                        let data_source = match opcode & 0b0111 {
                            0x0 => Data8::Mutable(MutableData8::Register8(cpu::Register8::B)),
                            0x1 => Data8::Mutable(MutableData8::Register8(cpu::Register8::C)),
                            0x2 => Data8::Mutable(MutableData8::Register8(cpu::Register8::D)),
                            0x3 => Data8::Mutable(MutableData8::Register8(cpu::Register8::E)),
                            0x4 => Data8::Mutable(MutableData8::Register8(cpu::Register8::H)),
                            0x5 => Data8::Mutable(MutableData8::Register8(cpu::Register8::L)),
                            0x6 => Data8::Mutable(MutableData8::IndirectRegister16(cpu::Register16::HL)),
                            0x7 => Data8::Mutable(MutableData8::Register8(cpu::Register8::A)),
                
                            //We masked the lower three bits, it will only ever be 0..=7
                            _ => unreachable!()
                        };
                        //Operation takes 8 cycles if it's indirected, 4 otherwise
                        let mut cycles = if let Data8::Mutable(MutableData8::IndirectRegister16(_)) = data_source {8} else {4};
                
                        //Top5 bits indicate operation
                        let operation = opcode >> 3;
                        let op = {
                            let data_dest = match operation & 0b111 {
                                0x0 => MutableData8::Register8(cpu::Register8::B),
                                0x1 => MutableData8::Register8(cpu::Register8::C),
                                0x2 => MutableData8::Register8(cpu::Register8::D),
                                0x3 => MutableData8::Register8(cpu::Register8::E),
                                0x4 => MutableData8::Register8(cpu::Register8::H),
                                0x5 => MutableData8::Register8(cpu::Register8::L),
                                0x6 => {
                                    cycles = 8;
                                    MutableData8::IndirectRegister16(cpu::Register16::HL)
                                },
                                0x7 => MutableData8::Register8(cpu::Register8::A),
                    
                                //We masked the lower three bits, it will only ever be 0..=7
                                _ => unreachable!()
                            };
                            Op::Load8{ into : data_dest, from : data_source }
                        };
                        Instruction { size : 1, cycles, op }
                    }
                },

            [0x80..=0xBF, ..]
                => {
                    let opcode = data[addr];

                    let data_source = match opcode & 0b0111 {
                        0x0 => Data8::Mutable(MutableData8::Register8(cpu::Register8::B)),
                        0x1 => Data8::Mutable(MutableData8::Register8(cpu::Register8::C)),
                        0x2 => Data8::Mutable(MutableData8::Register8(cpu::Register8::D)),
                        0x3 => Data8::Mutable(MutableData8::Register8(cpu::Register8::E)),
                        0x4 => Data8::Mutable(MutableData8::Register8(cpu::Register8::H)),
                        0x5 => Data8::Mutable(MutableData8::Register8(cpu::Register8::L)),
                        0x6 => Data8::Mutable(MutableData8::IndirectRegister16(cpu::Register16::HL)),
                        0x7 => Data8::Mutable(MutableData8::Register8(cpu::Register8::A)),
            
                        //We masked the lower three bits, it will only ever be 0..=7
                        _ => unreachable!()
                    };
                    //Operation takes 8 cycles if it's indirected, 4 otherwise
                    let cycles = if let Data8::Mutable(MutableData8::IndirectRegister16(_)) = data_source {8} else {4};
            
                    //Top5 bits indicate operation
                    let operation = (opcode - 0x80) >> 3;
                    let op = match operation {
                        //ADD A, _
                        0x0 => Op::Add{ into : MutableData8::Register8(cpu::Register8::A), from : data_source},
                        //ADC A, _
                        0x1 => Op::AddCarry{ into : MutableData8::Register8(cpu::Register8::A), from : data_source},
                        //SUB A, _
                        0x2 => Op::Sub{ into : MutableData8::Register8(cpu::Register8::A), from : data_source},
                        //SBC A, _
                        0x3 => Op::SubCarry{ into : MutableData8::Register8(cpu::Register8::A), from : data_source},
                        //AND A, _
                        0x4 => Op::And{ into : MutableData8::Register8(cpu::Register8::A), from : data_source},
                        //XOR A, _
                        0x5 => Op::Xor{ into : MutableData8::Register8(cpu::Register8::A), from : data_source},
                        //OR A, _
                        0x6 => Op::Or{ into : MutableData8::Register8(cpu::Register8::A), from : data_source},
                        //CP A, _
                        0x7 => Op::Compare{ into : Data8::Mutable(MutableData8::Register8(cpu::Register8::A)), from : data_source},
                        
                        //If opcode is in the range [0x80..=0xbf] minus 80, the top 5 bits will range from 0x00 to 0x07
                        _ => unreachable!()
                    };
                    Instruction { size : 1, cycles, op }
                }

            [0xCB, opcode, ..]
                => Instruction::extended_instruction_from_opcode(*opcode),

            [0xE0, a, ..]
                => Instruction{ size : 2, cycles : 3, op : Op::Load8{
                    into : MutableData8::IndirectValue8(*a),
                    from : Data8::Mutable(MutableData8::Register8(cpu::Register8::A))
                } },
            [0xF0, a, ..]
                => Instruction{ size : 2, cycles : 3, op : Op::Load8{
                    into : MutableData8::Register8(cpu::Register8::A),
                    from : Data8::Mutable(MutableData8::IndirectValue8(*a))
                } },
            [0xE2, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Load8{
                    into : MutableData8::IndirectRegister8(cpu::Register8::C),
                    from : Data8::Mutable(MutableData8::Register8(cpu::Register8::A))
                } },
            [0xF2, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Load8{
                    into : MutableData8::Register8(cpu::Register8::A),
                    from : Data8::Mutable(MutableData8::IndirectRegister8(cpu::Register8::C))
                } },

            
            
            [0xC1, ..]
                => Instruction{ size : 1, cycles : 3, op : Op::Pop{
                    into : MutableData16::Register16(cpu::Register16::BC)
                } },
            [0xD1, ..]
                => Instruction{ size : 1, cycles : 3, op : Op::Pop{
                    into : MutableData16::Register16(cpu::Register16::DE)
                } },
            [0xE1, ..]
                => Instruction{ size : 1, cycles : 3, op : Op::Pop{
                    into : MutableData16::Register16(cpu::Register16::HL)
                } },
            [0xF1, ..]
                => Instruction{ size : 1, cycles : 43, op : Op::Pop{
                    into : MutableData16::Register16(cpu::Register16::AF)
                } },
            
            [0xC5, ..]
                => Instruction{ size : 1, cycles : 4, op : Op::Push{
                    from : Data16::Mutable(MutableData16::Register16(cpu::Register16::BC))
                } },
            [0xD5, ..]
                => Instruction{ size : 1, cycles : 4, op : Op::Push{
                    from : Data16::Mutable(MutableData16::Register16(cpu::Register16::DE))
                } },
            [0xE5, ..]
                => Instruction{ size : 1, cycles : 4, op : Op::Push{
                    from : Data16::Mutable(MutableData16::Register16(cpu::Register16::HL))
                } },
            [0xF5, ..]
                => Instruction{ size : 1, cycles : 4, op : Op::Push{
                    from : Data16::Mutable(MutableData16::Register16(cpu::Register16::AF))
                } },

            [0xC9, ..]
                => Instruction{ size : 1, cycles : 4, op : Op::Return },

            [0xEA, a, b, ..]
                => Instruction{ size : 3, cycles : 4, op : Op::Load8{
                    into : MutableData8::IndirectValue16(join_u8(*a, *b)),
                    from : Data8::Mutable(MutableData8::Register8(cpu::Register8::A))
                } },
            [0xFA, a, b, ..]
                => Instruction{ size : 3, cycles : 4, op : Op::Load8{
                    into : MutableData8::Register8(cpu::Register8::A),
                    from : Data8::Mutable(MutableData8::IndirectValue16(join_u8(*a, *b)))
                } },

            [0xFE, a, ..]
                => Instruction{ size : 2, cycles : 2, op : Op::Compare {
                    from : Data8::Mutable(MutableData8::Register8(cpu::Register8::A)),
                    into : Data8::Immutable(*a)
                }},

            [a, ..] => Instruction{ size : 0, cycles : 0, op : Op::Unimplemented(*a) },

            _ => Instruction{ size : 0, cycles : 0, op : Op::Unimplemented(0) }
        })
    }
    fn extended_instruction_from_opcode(opcode : u8) -> Instruction {
        //Bottom 3 bits determines which register to operate on
        let data_dest = match opcode & 0b0111 {
            0x0 => MutableData8::Register8(cpu::Register8::B),
            0x1 => MutableData8::Register8(cpu::Register8::C),
            0x2 => MutableData8::Register8(cpu::Register8::D),
            0x3 => MutableData8::Register8(cpu::Register8::E),
            0x4 => MutableData8::Register8(cpu::Register8::H),
            0x5 => MutableData8::Register8(cpu::Register8::L),
            0x6 => MutableData8::IndirectRegister16(cpu::Register16::HL),
            0x7 => MutableData8::Register8(cpu::Register8::A),

            //We masked the lower three bits, it will only ever be 0..=7
            _ => unreachable!()
        };
        //Operation takes 16 cycles if it's indirected, 8 otherwise
        let cycles = if let MutableData8::IndirectRegister16(_) = data_dest {16} else {8};

        //Top5 bits indicate operation
        let operation = opcode >> 3;
        let op = match operation {
            //RLC
            0x00 => Op::RolCarry{ into : data_dest },
            //RRC
            0x01 => Op::RorCarry{ into : data_dest },
            //RL
            0x02 => Op::Rol{ into : data_dest },
            //RR
            0x03 => Op::Ror{ into : data_dest },
            //SLA
            0x04 => Op::ShiftLeftAccumulator{ into : data_dest },
            //SRA
            0x05 => Op::ShiftRightAccumulator{ into : data_dest },
            //SWAP
            0x06 => Op::Swap{ into : data_dest },
            //SRL
            0x07 => Op::ShiftRightLogical{ into : data_dest },
            //BIT[0..7]
            0x08..=0x0F => Op::Bit{ into : Data8::Mutable(data_dest), bit : operation - 0x08 },
            //RES[0..7]
            0x10..=0x17 =>  Op::Reset{ into : data_dest, bit : operation - 0x16 },
            //SET[0..7]
            0x18..=0x1F => Op::Set{ into : data_dest, bit : operation - 0x24 },

            //We masked to the top 5 bits, will always range 0..=31
            _ => unreachable!(),
        };
        Instruction { size : 2, cycles, op }
    }

    pub fn execute(&self, state : &mut cpu::Registers, memory : &mut Memory) {
        let (default_addr, default_cycles) = (state.pc() + self.size as u16, self.cycles);
        let (new_addr, cycles) : (u16, u8) = match &self.op {
            
            _ => (default_addr, default_cycles)
        };

        state.set_pc(new_addr);
    }
}