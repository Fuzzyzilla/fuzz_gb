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
/*
    LoadIndirect16{address_register : cpu::Register, dest_register : cpu::Register},
    LoadIndirect8{address_register : cpu::Register, dest_register : cpu::Register},
    Inc16{register : cpu::Register},
    Inc8{register : cpu::Register},
    Dec16{register : cpu::Register},
    Dec8{register : cpu::Register},
    RolCarry{register : cpu::Register},
    Rol{register : cpu::Register},
    RorCarry{register : cpu::Register},
    Ror{register : cpu::Register},
    StoreIndirect16{address_register : cpu::Register, value_register : cpu::Register},
    StoreIndirect8{address_register : cpu::Register, value_register : cpu::Register},
    Add16{add_into : cpu::Register, add_from : cpu::Register},
    Stop,
    JumpRelative{amount : u8},
    JumpRelativeConditional{condition : cpu::Flag, amount : u8}
*/
}

struct Instruction {
    pub op : Op,
    pub cycles : u8,
    pub size : u8
}

pub fn parse_instruction(data : &[u8]) -> Op {
    match data {
        
        [0x00, ..]
            => Op::Nop,
        [0x10, ..]
            => Op::Stop,

        [0x01, a, b, ..]
            => Op::Load{
                into : DataDest::Register16(cpu::Register::BC),
                from : DataSource::Value16(join_u8(*a, *b))
            },
        [0x11, a, b, ..]
            => Op::Load{
                into : DataDest::Register16(cpu::Register::DE),
                from : DataSource::Value16(join_u8(*a, *b))
            },
        [0x21, a, b, ..]
            => Op::Load{
                into : DataDest::Register16(cpu::Register::HL),
                from : DataSource::Value16(join_u8(*a, *b))
            },
        [0x31, a, b, ..]
            => Op::Load{
                into : DataDest::Register16(cpu::Register::SP),
                from : DataSource::Value16(join_u8(*a, *b))
            },

        [0x02, ..]
            => Op::Load{
                into : DataDest::IndirectRegister16(cpu::Register::BC),
                from : DataSource::Register8(cpu::Register::A)
            },
        [0x12, ..]
            => Op::Load{
                into : DataDest::IndirectRegister16(cpu::Register::DE),
                from : DataSource::Register8(cpu::Register::A)
            },
        [0x22, ..]
            => Op::Load{
                into : DataDest::IndirectRegister16Inc(cpu::Register::HL),
                from : DataSource::Register8(cpu::Register::A)
            },
        [0x32, ..]
            => Op::Load{
                into : DataDest::IndirectRegister16Dec(cpu::Register::HL),
                from : DataSource::Register8(cpu::Register::A)
            },

        [0x03, ..]
            => Op::Inc{
                into : DataDest::Register16(cpu::Register::BC)
            },
        [0x13, ..]
            => Op::Inc{
                into : DataDest::Register16(cpu::Register::DE)
            },
        [0x23, ..]
            => Op::Inc{
                into : DataDest::Register16(cpu::Register::HL)
            },
        [0x33, ..]
            => Op::Inc{
                into : DataDest::Register16(cpu::Register::SP)
            },


        [0x04, ..]
            => Op::Inc{
                into : DataDest::Register16(cpu::Register::B)
            },
        [0x14, ..]
            => Op::Inc{
                into : DataDest::Register16(cpu::Register::D)
            },
        [0x24, ..]
            => Op::Inc{
                into : DataDest::Register16(cpu::Register::H)
            },
        [0x34, ..]
            => Op::Inc{
                into : DataDest::IndirectRegister16(cpu::Register::HL)
            },


        [0x05, ..]
            => Op::Dec{
                into : DataDest::Register8(cpu::Register::B)
            },
        [0x15, ..]
            => Op::Dec{
                into : DataDest::Register8(cpu::Register::D)
            },
        [0x25, ..]
            => Op::Dec{
                into : DataDest::Register8(cpu::Register::H)
            },
        [0x35, ..]
            => Op::Dec{
                into : DataDest::IndirectRegister16(cpu::Register::HL)
            },

        
        [0x06, a, ..]
            => Op::Load{
                into : DataDest::Register8(cpu::Register::B),
                from : DataSource::Value8(*a)
            },
        [0x16, a, ..]
            => Op::Load{
                into : DataDest::Register8(cpu::Register::D),
                from : DataSource::Value8(*a)
            },
        [0x26, a, ..]
            => Op::Load{
                into : DataDest::Register8(cpu::Register::H),
                from : DataSource::Value8(*a)
            },
        [0x36, a, ..]
            => Op::Load{
                into : DataDest::IndirectRegister16(cpu::Register::HL),
                from : DataSource::Value8(*a)
            },
        
        
        [0x07, ..]
            => Op::RolCarry{
                into : DataDest::Register8(cpu::Register::A),
            },
        [0x17, ..]
            => Op::Rol{
                into : DataDest::Register8(cpu::Register::A),
            },
        [0x27, ..]
            => Op::Unimplemented(0x27),
        [0x37, ..]
            => Op::Unimplemented(0x37),
        
        [0x08, a, b, ..]
            => Op::Load{
                into : DataDest::IndirectValue16(join_u8(*a, *b)),
                from : DataSource::Register16(cpu::Register::SP)
            },
        [0x18, a, ..]
            => Op::JumpRelative{
                amount : DataSource::Value8(*a)
            },
        [0x28, a, ..]
            => Op::JumpRelativeIf{
                condition : cpu::Flag::Zero,
                amount : DataSource::Value8(*a)
            },
        [0x38, a, ..]
            => Op::JumpRelativeIf{
                condition : cpu::Flag::Carry,
                amount : DataSource::Value8(*a)
            },
        
        [0x09, ..]
            => Op::Add{
                into : DataDest::Register16(cpu::Register::HL),
                from : DataSource::Register16(cpu::Register::BC)
            },
        [0x19, ..]
            => Op::Add{
                into : DataDest::Register16(cpu::Register::HL),
                from : DataSource::Register16(cpu::Register::DE)
            },
        [0x29, ..]
            => Op::Add{
                into : DataDest::Register16(cpu::Register::HL),
                from : DataSource::Register16(cpu::Register::HL)
            },
        [0x39, ..]
            => Op::Add{
                into : DataDest::Register16(cpu::Register::HL),
                from : DataSource::Register16(cpu::Register::SP)
            },
        
        [0x0A, ..]
            => Op::Load{
                into : DataDest::Register8(cpu::Register::A),
                from : DataSource::IndirectRegister16(cpu::Register::BC)
            },
        [0x1A, ..]
            => Op::Load{
                into : DataDest::Register8(cpu::Register::A),
                from : DataSource::IndirectRegister16(cpu::Register::DE)
            },
        [0x2A, ..]
            => Op::Load{
                into : DataDest::Register8(cpu::Register::A),
                from : DataSource::IndirectRegister16Inc(cpu::Register::HL)
            },
        [0x3A, ..]
            => Op::Load{
                into : DataDest::Register8(cpu::Register::HL),
                from : DataSource::IndirectRegister16Dec(cpu::Register::HL)
            },

        
        [0x0B, ..]
            => Op::Dec{
                into : DataDest::Register16(cpu::Register::BC)
            },
        [0x1B, ..]
            => Op::Dec{
                into : DataDest::Register16(cpu::Register::DE)
            },
        [0x2B, ..]
            => Op::Dec{
                into : DataDest::Register16(cpu::Register::HL)
            },
        [0x3B, ..]
            => Op::Dec{
                into : DataDest::Register16(cpu::Register::SP)
            },

        [0x0C, ..]
            => Op::Inc{
                into : DataDest::Register8(cpu::Register::C)
            },
        [0x1C, ..]
            => Op::Inc{
                into : DataDest::Register8(cpu::Register::E)
            },
        [0x2C, ..]
            => Op::Inc{
                into : DataDest::Register8(cpu::Register::L)
            },
        [0x3C, ..]
            => Op::Inc{
                into : DataDest::Register8(cpu::Register::A)
            },

        [0x0D, ..]
            => Op::Dec{
                into : DataDest::Register8(cpu::Register::C)
            },
        [0x1D, ..]
            => Op::Dec{
                into : DataDest::Register8(cpu::Register::E)
            },
        [0x2D, ..]
            => Op::Dec{
                into : DataDest::Register8(cpu::Register::L)
            },
        [0x3D, ..]
            => Op::Dec{
                into : DataDest::Register8(cpu::Register::A)
            },
        
        
        [0x0E, a, ..]
            => Op::Load{
                into : DataDest::Register8(cpu::Register::C),
                from : DataSource::Value8(*a)
            },
        [0x1E, a, ..]
            => Op::Load{
                into : DataDest::Register8(cpu::Register::E),
                from : DataSource::Value8(*a)
            },
        [0x2E, a, ..]
            => Op::Load{
                into : DataDest::Register8(cpu::Register::L),
                from : DataSource::Value8(*a)
            },
        [0x3E, a, ..]
            => Op::Load{
                into : DataDest::Register8(cpu::Register::A),
                from : DataSource::Value8(*a)
            },

        [0x0F, ..]
            => Op::RorCarry{
                into : DataDest::Register8(cpu::Register::A),
            },
        [0x1F, ..]
            => Op::Ror{
                into : DataDest::Register8(cpu::Register::A),
            },
        [0x2F, ..]
            => Op::Unimplemented(0x2F),
        [0x3F, ..]
            => Op::Unimplemented(0x3F),

        [a, ..] => Op::Unimplemented(*a),

        _ => Op::Unimplemented(0)
    }
}