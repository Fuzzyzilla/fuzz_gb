use crate::cpu;
use crate::bitmath;

use std::fmt;

use bitmath::join_u8;

pub enum DataSource {
    Value8(u8),
    Value16(u16),
    Register8(cpu::Register),
    Register16(cpu::Register),
    IndirectRegister16(cpu::Register),
    IndirectRegister16Inc(cpu::Register),
    IndirectRegister16Dec(cpu::Register),
}

impl fmt::Display for DataSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataSource::Value8(v) => write!(f, "${:02x}", v),
            DataSource::Value16(v) => write!(f, "${:02x}", v),
            DataSource::Register8(r) | DataSource::Register16(r) =>
                write!(f, "{}", r),
            DataSource::IndirectRegister16(r) =>
                write!(f, "({})", r),
            DataSource::IndirectRegister16Dec(r) =>
                write!(f, "({}-)", r),
            DataSource::IndirectRegister16Inc(r) =>
                write!(f, "({}+)", r)
        }
    }
}

pub enum DataDest {
    Register8(cpu::Register),
    Register16(cpu::Register),
    IndirectRegister16(cpu::Register),
    IndirectRegister16Inc(cpu::Register),
    IndirectRegister16Dec(cpu::Register),
    IndirectValue16(u16)
}

impl fmt::Display for DataDest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataDest::IndirectValue16(v) => write!(f, "(${:04x})", v),
            DataDest::Register8(r) | DataDest::Register16(r) =>
                write!(f, "{}", r),
            DataDest::IndirectRegister16(r) =>
                write!(f, "({})", r),
            DataDest::IndirectRegister16Dec(r) =>
                write!(f, "({}-)", r),
            DataDest::IndirectRegister16Inc(r) =>
                write!(f, "({}+)", r)
        }
    }
}

pub enum Op {
    Nop,
    Stop,
    Halt,
    Load{into : DataDest, from : DataSource},
    Inc{into : DataDest},
    Dec{into : DataDest},
    Ror{into : DataDest},
    RorCarry{into : DataDest},
    Rol{into : DataDest},
    RolCarry{into : DataDest},
    Add{into : DataDest, from : DataSource},
    Sub{into : DataDest, from : DataSource},
    And{into : DataDest, from : DataSource},
    Or{into : DataDest, from : DataSource},
    Xor{into : DataDest, from : DataSource},
    JumpRelative{amount : DataSource},
    JumpRelativeIf{condition : cpu::Flag, amount : DataSource},
    Return,
    ReturnIf{condition : cpu::Flag},

    Unimplemented(u8)
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Nop => write!(f, "NOP"),
            Op::Stop => write!(f, "STOP"),
            Op::Halt => write!(f, "HALT"),
            Op::Load{into, from} =>
                write!(f, "LD {}, {}", into, from),
            Op::Inc{into} =>
                write!(f, "INC {}", into),
            Op::Dec{into} =>
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
            Op::Sub{from, into} =>
                write!(f, "SUB {}, {}", into, from),
            Op::And{from, into} =>
                write!(f, "AND {}, {}", into, from),
            Op::Or{from, into} =>
                write!(f, "OR  {}, {}", into, from),
            Op::Xor{from, into} =>
                write!(f, "XOR {}, {}", into, from),
            Op::JumpRelative{amount} => 
                write!(f, "JR  {}", amount),
            Op::JumpRelativeIf{amount, condition} => 
                write!(f, "JR{condition} {}", amount),
            Op::Return =>
                write!(f, "RET"),
            Op::ReturnIf{condition} =>
                write!(f, "RET{condition}"),
            
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
    pub fn new(data : &[u8]) -> Instruction {
        match data {

            [0x00, ..]
                => Instruction{ size : 1, cycles : 4, op : Op::Nop },
            [0x10, ..]
                => Instruction{ size : 2, cycles : 1, op : Op::Stop },
            [0x20, a, ..]
                => Instruction{ size : 2, cycles : 1, op : Op::JumpRelativeIf {
                    amount : DataSource::Value8(*a),
                    condition : cpu::Flag::NotZero 
                } },
            [0x30, a, ..]
                => Instruction{ size : 2, cycles : 1, op : Op::JumpRelativeIf {
                    amount : DataSource::Value8(*a),
                    condition : cpu::Flag::NotCarry 
                } },

            [0x01, a, b, ..]
                => Instruction{ size : 3, cycles : 3, op : Op::Load{
                    into : DataDest::Register16(cpu::Register::BC),
                    from : DataSource::Value16(join_u8(*a, *b))
                } },
            [0x11, a, b, ..]
                => Instruction{ size : 3, cycles : 3, op : Op::Load{
                    into : DataDest::Register16(cpu::Register::DE),
                    from : DataSource::Value16(join_u8(*a, *b))
                } },
            [0x21, a, b, ..]
                => Instruction{ size : 3, cycles : 3, op : Op::Load{
                    into : DataDest::Register16(cpu::Register::HL),
                    from : DataSource::Value16(join_u8(*a, *b))
                } },
            [0x31, a, b, ..]
                => Instruction{ size : 3, cycles : 3, op : Op::Load{
                    into : DataDest::Register16(cpu::Register::SP),
                    from : DataSource::Value16(join_u8(*a, *b))
                } },

            [0x02, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Load{
                    into : DataDest::IndirectRegister16(cpu::Register::BC),
                    from : DataSource::Register8(cpu::Register::A)
                } },
            [0x12, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Load{
                    into : DataDest::IndirectRegister16(cpu::Register::DE),
                    from : DataSource::Register8(cpu::Register::A)
                } },
            [0x22, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Load{
                    into : DataDest::IndirectRegister16Inc(cpu::Register::HL),
                    from : DataSource::Register8(cpu::Register::A)
                } },
            [0x32, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Load{
                    into : DataDest::IndirectRegister16Dec(cpu::Register::HL),
                    from : DataSource::Register8(cpu::Register::A)
                } },

            [0x03, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Inc{
                    into : DataDest::Register16(cpu::Register::BC)
                } },
            [0x13, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Inc{
                    into : DataDest::Register16(cpu::Register::DE)
                } },
            [0x23, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Inc{
                    into : DataDest::Register16(cpu::Register::HL)
                } },
            [0x33, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Inc{
                    into : DataDest::Register16(cpu::Register::SP)
                } },


            [0x04, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Inc{
                    into : DataDest::Register16(cpu::Register::B)
                } },
            [0x14, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Inc{
                    into : DataDest::Register16(cpu::Register::D)
                } },
            [0x24, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Inc{
                    into : DataDest::Register16(cpu::Register::H)
                } },
            [0x34, ..]
                => Instruction{ size : 1, cycles : 3, op : Op::Inc{
                    into : DataDest::IndirectRegister16(cpu::Register::HL)
                } },


            [0x05, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Dec{
                    into : DataDest::Register8(cpu::Register::B)
                } },
            [0x15, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Dec{
                    into : DataDest::Register8(cpu::Register::D)
                } },
            [0x25, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Dec{
                    into : DataDest::Register8(cpu::Register::H)
                } },
            [0x35, ..]
                => Instruction{ size : 1, cycles : 3, op : Op::Dec{
                    into : DataDest::IndirectRegister16(cpu::Register::HL)
                } },

            
            [0x06, a, ..]
                => Instruction{ size : 2, cycles : 2, op : Op::Load{
                    into : DataDest::Register8(cpu::Register::B),
                    from : DataSource::Value8(*a)
                } },
            [0x16, a, ..]
                => Instruction{ size : 2, cycles : 2, op : Op::Load{
                    into : DataDest::Register8(cpu::Register::D),
                    from : DataSource::Value8(*a)
                } },
            [0x26, a, ..]
                => Instruction{ size : 2, cycles : 2, op : Op::Load{
                    into : DataDest::Register8(cpu::Register::H),
                    from : DataSource::Value8(*a)
                } },
            [0x36, a, ..]
                => Instruction{ size : 2, cycles : 3, op : Op::Load{
                    into : DataDest::IndirectRegister16(cpu::Register::HL),
                    from : DataSource::Value8(*a)
                } },
            
            
            [0x07, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::RolCarry{
                    into : DataDest::Register8(cpu::Register::A),
                } },
            [0x17, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Rol{
                    into : DataDest::Register8(cpu::Register::A),
                } },
            [0x27, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Unimplemented(0x27) },
            [0x37, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Unimplemented(0x37) },
            
            [0x08, a, b, ..]
                => Instruction{ size : 3, cycles : 5, op : Op::Load{
                    into : DataDest::IndirectValue16(join_u8(*a, *b)),
                    from : DataSource::Register16(cpu::Register::SP)
                } },
            [0x18, a, ..]
                => Instruction{ size : 2, cycles : 3, op : Op::JumpRelative{
                    amount : DataSource::Value8(*a)
                } },
            [0x28, a, ..]
                => Instruction{ size : 2, cycles : 2, op : Op::JumpRelativeIf{
                    condition : cpu::Flag::Zero,
                    amount : DataSource::Value8(*a)
                } },
            [0x38, a, ..]
                => Instruction{ size : 2, cycles : 2, op : Op::JumpRelativeIf{
                    condition : cpu::Flag::Carry,
                    amount : DataSource::Value8(*a)
                } },
            
            [0x09, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Add{
                    into : DataDest::Register16(cpu::Register::HL),
                    from : DataSource::Register16(cpu::Register::BC)
                } },
            [0x19, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Add{
                    into : DataDest::Register16(cpu::Register::HL),
                    from : DataSource::Register16(cpu::Register::DE)
                } },
            [0x29, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Add{
                    into : DataDest::Register16(cpu::Register::HL),
                    from : DataSource::Register16(cpu::Register::HL)
                } },
            [0x39, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Add{
                    into : DataDest::Register16(cpu::Register::HL),
                    from : DataSource::Register16(cpu::Register::SP)
                } },
            
            [0x0A, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Load{
                    into : DataDest::Register8(cpu::Register::A),
                    from : DataSource::IndirectRegister16(cpu::Register::BC)
                } },
            [0x1A, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Load{
                    into : DataDest::Register8(cpu::Register::A),
                    from : DataSource::IndirectRegister16(cpu::Register::DE)
                } },
            [0x2A, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Load{
                    into : DataDest::Register8(cpu::Register::A),
                    from : DataSource::IndirectRegister16Inc(cpu::Register::HL)
                } },
            [0x3A, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Load{
                    into : DataDest::Register8(cpu::Register::HL),
                    from : DataSource::IndirectRegister16Dec(cpu::Register::HL)
                } },

            
            [0x0B, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Dec{
                    into : DataDest::Register16(cpu::Register::BC)
                } },
            [0x1B, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Dec{
                    into : DataDest::Register16(cpu::Register::DE)
                } },
            [0x2B, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Dec{
                    into : DataDest::Register16(cpu::Register::HL)
                } },
            [0x3B, ..]
                => Instruction{ size : 1, cycles : 2, op : Op::Dec{
                    into : DataDest::Register16(cpu::Register::SP)
                } },

            [0x0C, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Inc{
                    into : DataDest::Register8(cpu::Register::C)
                } },
            [0x1C, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Inc{
                    into : DataDest::Register8(cpu::Register::E)
                } },
            [0x2C, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Inc{
                    into : DataDest::Register8(cpu::Register::L)
                } },
            [0x3C, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Inc{
                    into : DataDest::Register8(cpu::Register::A)
                } },

            [0x0D, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Dec{
                    into : DataDest::Register8(cpu::Register::C)
                } },
            [0x1D, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Dec{
                    into : DataDest::Register8(cpu::Register::E)
                } },
            [0x2D, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Dec{
                    into : DataDest::Register8(cpu::Register::L)
                } },
            [0x3D, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Dec{
                    into : DataDest::Register8(cpu::Register::A)
                } },
            
            
            [0x0E, a, ..]
                => Instruction{ size : 2, cycles : 2, op : Op::Load{
                    into : DataDest::Register8(cpu::Register::C),
                    from : DataSource::Value8(*a)
                } },
            [0x1E, a, ..]
                => Instruction{ size : 2, cycles : 2, op : Op::Load{
                    into : DataDest::Register8(cpu::Register::E),
                    from : DataSource::Value8(*a)
                } },
            [0x2E, a, ..]
                => Instruction{ size : 2, cycles : 2, op : Op::Load{
                    into : DataDest::Register8(cpu::Register::L),
                    from : DataSource::Value8(*a)
                } },
            [0x3E, a, ..]
                => Instruction{ size : 2, cycles : 2, op : Op::Load{
                    into : DataDest::Register8(cpu::Register::A),
                    from : DataSource::Value8(*a)
                } },

            [0x0F, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::RorCarry{
                    into : DataDest::Register8(cpu::Register::A),
                } },
            [0x1F, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Ror{
                    into : DataDest::Register8(cpu::Register::A),
                } },
            [0x2F, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Unimplemented(0x2F) },
            [0x3F, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Unimplemented(0x3F) },

            [0xAF, ..]
                => Instruction{ size : 1, cycles : 4, op : Op::Xor{
                    into : DataDest::Register8(cpu::Register::A),
                    from : DataSource::Register8(cpu::Register::A)
                } },
            
            [0xCB, a, ..]
                => Instruction{ size : 2, cycles : 2, op :Op::Unimplemented(0xCB)},

            /*[0x40..=0xBf, ..]
                => Instruction{ size : 1, cycles : 1, op : Op::Unimplemented(0x01) },
*/
            [a, ..] => Instruction{ size : 0, cycles : 0, op : Op::Unimplemented(*a) },

            _ => Instruction{ size : 0, cycles : 0, op : Op::Unimplemented(0) }
        }
    }
}