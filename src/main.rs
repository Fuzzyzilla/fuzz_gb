pub mod cpu;
pub mod bitmath;
mod instructions;


fn main() {
    let data = include_bytes!("data/dmg_boot.bin");

    assert_eq!(data.len(), 256);

    let mut pc : u16 = 0xe6;
    loop {
        let instr_slice = &data[(pc as usize)..];

        let instruction = instructions::Instruction::new(instr_slice);

        let instruction_data = &data[(pc as usize) .. ((pc + instruction.size as u16) as usize)];

        print!("{:04X}: ", pc);

        for i in 0..3 {
            if i < instruction_data.len() {
                print!("{:02X} ", instruction_data[i]);
            } else {
                print!("   ");
            };
        };

        println!("| {}", instruction.op);

        pc += instruction.size as u16;

        if instruction.size == 0 {
            panic!("Uh oh!")
        }
    }
}
