use crate::cpu;
use crate::bitmath;

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
pub enum DataDest {
    Register8(cpu::Register),
    Register16(cpu::Register),
    IndirectRegister16(cpu::Register),
    IndirectRegister16Inc(cpu::Register),
    IndirectRegister16Dec(cpu::Register),
    IndirectValue16(u16)
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
    JumpRelative{amount : DataSource},
    JumpRelativeIf{condition : cpu::Flag, amount : DataSource},
    Return,
    ReturnIf{condition : cpu::Flag},

    Unimplemented(u8)
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
                => Instruction{  cycles : 0, size : 0, op : Op::Nop },
            [0x10, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Stop },

            [0x01, a, b, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Load{
                    into : DataDest::Register16(cpu::Register::BC),
                    from : DataSource::Value16(join_u8(*a, *b))
                } },
            [0x11, a, b, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Load{
                    into : DataDest::Register16(cpu::Register::DE),
                    from : DataSource::Value16(join_u8(*a, *b))
                } },
            [0x21, a, b, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Load{
                    into : DataDest::Register16(cpu::Register::HL),
                    from : DataSource::Value16(join_u8(*a, *b))
                } },
            [0x31, a, b, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Load{
                    into : DataDest::Register16(cpu::Register::SP),
                    from : DataSource::Value16(join_u8(*a, *b))
                } },

            [0x02, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Load{
                    into : DataDest::IndirectRegister16(cpu::Register::BC),
                    from : DataSource::Register8(cpu::Register::A)
                } },
            [0x12, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Load{
                    into : DataDest::IndirectRegister16(cpu::Register::DE),
                    from : DataSource::Register8(cpu::Register::A)
                } },
            [0x22, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Load{
                    into : DataDest::IndirectRegister16Inc(cpu::Register::HL),
                    from : DataSource::Register8(cpu::Register::A)
                } },
            [0x32, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Load{
                    into : DataDest::IndirectRegister16Dec(cpu::Register::HL),
                    from : DataSource::Register8(cpu::Register::A)
                } },

            [0x03, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Inc{
                    into : DataDest::Register16(cpu::Register::BC)
                } },
            [0x13, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Inc{
                    into : DataDest::Register16(cpu::Register::DE)
                } },
            [0x23, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Inc{
                    into : DataDest::Register16(cpu::Register::HL)
                } },
            [0x33, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Inc{
                    into : DataDest::Register16(cpu::Register::SP)
                } },


            [0x04, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Inc{
                    into : DataDest::Register16(cpu::Register::B)
                } },
            [0x14, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Inc{
                    into : DataDest::Register16(cpu::Register::D)
                } },
            [0x24, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Inc{
                    into : DataDest::Register16(cpu::Register::H)
                } },
            [0x34, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Inc{
                    into : DataDest::IndirectRegister16(cpu::Register::HL)
                } },


            [0x05, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Dec{
                    into : DataDest::Register8(cpu::Register::B)
                } },
            [0x15, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Dec{
                    into : DataDest::Register8(cpu::Register::D)
                } },
            [0x25, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Dec{
                    into : DataDest::Register8(cpu::Register::H)
                } },
            [0x35, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Dec{
                    into : DataDest::IndirectRegister16(cpu::Register::HL)
                } },

            
            [0x06, a, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Load{
                    into : DataDest::Register8(cpu::Register::B),
                    from : DataSource::Value8(*a)
                } },
            [0x16, a, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Load{
                    into : DataDest::Register8(cpu::Register::D),
                    from : DataSource::Value8(*a)
                } },
            [0x26, a, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Load{
                    into : DataDest::Register8(cpu::Register::H),
                    from : DataSource::Value8(*a)
                } },
            [0x36, a, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Load{
                    into : DataDest::IndirectRegister16(cpu::Register::HL),
                    from : DataSource::Value8(*a)
                } },
            
            
            [0x07, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::RolCarry{
                    into : DataDest::Register8(cpu::Register::A),
                } },
            [0x17, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Rol{
                    into : DataDest::Register8(cpu::Register::A),
                } },
            [0x27, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Unimplemented(0x27) },
            [0x37, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Unimplemented(0x37) },
            
            [0x08, a, b, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Load{
                    into : DataDest::IndirectValue16(join_u8(*a, *b)),
                    from : DataSource::Register16(cpu::Register::SP)
                } },
            [0x18, a, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::JumpRelative{
                    amount : DataSource::Value8(*a)
                } },
            [0x28, a, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::JumpRelativeIf{
                    condition : cpu::Flag::Zero,
                    amount : DataSource::Value8(*a)
                } },
            [0x38, a, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::JumpRelativeIf{
                    condition : cpu::Flag::Carry,
                    amount : DataSource::Value8(*a)
                } },
            
            [0x09, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Add{
                    into : DataDest::Register16(cpu::Register::HL),
                    from : DataSource::Register16(cpu::Register::BC)
                } },
            [0x19, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Add{
                    into : DataDest::Register16(cpu::Register::HL),
                    from : DataSource::Register16(cpu::Register::DE)
                } },
            [0x29, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Add{
                    into : DataDest::Register16(cpu::Register::HL),
                    from : DataSource::Register16(cpu::Register::HL)
                } },
            [0x39, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Add{
                    into : DataDest::Register16(cpu::Register::HL),
                    from : DataSource::Register16(cpu::Register::SP)
                } },
            
            [0x0A, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Load{
                    into : DataDest::Register8(cpu::Register::A),
                    from : DataSource::IndirectRegister16(cpu::Register::BC)
                } },
            [0x1A, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Load{
                    into : DataDest::Register8(cpu::Register::A),
                    from : DataSource::IndirectRegister16(cpu::Register::DE)
                } },
            [0x2A, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Load{
                    into : DataDest::Register8(cpu::Register::A),
                    from : DataSource::IndirectRegister16Inc(cpu::Register::HL)
                } },
            [0x3A, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Load{
                    into : DataDest::Register8(cpu::Register::HL),
                    from : DataSource::IndirectRegister16Dec(cpu::Register::HL)
                } },

            
            [0x0B, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Dec{
                    into : DataDest::Register16(cpu::Register::BC)
                } },
            [0x1B, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Dec{
                    into : DataDest::Register16(cpu::Register::DE)
                } },
            [0x2B, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Dec{
                    into : DataDest::Register16(cpu::Register::HL)
                } },
            [0x3B, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Dec{
                    into : DataDest::Register16(cpu::Register::SP)
                } },

            [0x0C, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Inc{
                    into : DataDest::Register8(cpu::Register::C)
                } },
            [0x1C, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Inc{
                    into : DataDest::Register8(cpu::Register::E)
                } },
            [0x2C, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Inc{
                    into : DataDest::Register8(cpu::Register::L)
                } },
            [0x3C, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Inc{
                    into : DataDest::Register8(cpu::Register::A)
                } },

            [0x0D, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Dec{
                    into : DataDest::Register8(cpu::Register::C)
                } },
            [0x1D, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Dec{
                    into : DataDest::Register8(cpu::Register::E)
                } },
            [0x2D, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Dec{
                    into : DataDest::Register8(cpu::Register::L)
                } },
            [0x3D, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Dec{
                    into : DataDest::Register8(cpu::Register::A)
                } },
            
            
            [0x0E, a, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Load{
                    into : DataDest::Register8(cpu::Register::C),
                    from : DataSource::Value8(*a)
                } },
            [0x1E, a, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Load{
                    into : DataDest::Register8(cpu::Register::E),
                    from : DataSource::Value8(*a)
                } },
            [0x2E, a, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Load{
                    into : DataDest::Register8(cpu::Register::L),
                    from : DataSource::Value8(*a)
                } },
            [0x3E, a, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Load{
                    into : DataDest::Register8(cpu::Register::A),
                    from : DataSource::Value8(*a)
                } },

            [0x0F, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::RorCarry{
                    into : DataDest::Register8(cpu::Register::A),
                } },
            [0x1F, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Ror{
                    into : DataDest::Register8(cpu::Register::A),
                } },
            [0x2F, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Unimplemented(0x2F) },
            [0x3F, ..]
                => Instruction{  cycles : 0, size : 0, op : Op::Unimplemented(0x3F) },

            [a, ..] => Instruction{  cycles : 0, size : 0, op : Op::Unimplemented(*a) },

            _ => Instruction{  cycles : 0, size : 0, op : Op::Unimplemented(0) }
        }
    }
}