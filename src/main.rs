pub mod cpu;
pub mod bitmath;
mod instructions;

use ansi_term::Color::Blue;

fn disassemble(base_address : u16, data : &[u8]) {
    let mut pc : u16 = 0x00;
    loop {
        let instr_slice = &data[(pc as usize)..];

        let instruction = instructions::Instruction::from_bytes(instr_slice);

        let instruction_data = &data[(pc as usize) .. ((pc + instruction.size as u16) as usize)];

        print!("{:04X}: ", pc + base_address);

        for i in 0..3 {
            if i < instruction_data.len() {
                print!("{:02X} ", instruction_data[i]);
            } else {
                print!("   ");
            };
        };

        println!("| {}", Blue.bold().paint(format!("{}", instruction.op)));

        pc += instruction.size as u16;

        if instruction.size == 0 {
            break
        }
    }
}

fn main() {
    let data = include_bytes!("data/dmg_boot.bin");

    assert_eq!(data.len(), 256);

    disassemble(0x0000, &data[0x00..0xA8]);
    disassemble(0x00E0, &data[0xE0..]);
}
