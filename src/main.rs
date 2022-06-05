pub mod cpu;
pub mod bitmath;
mod instructions;


fn main() {
    let data = include_bytes!("data/dmg_boot.bin");

    assert_eq!(data.len(), 256);

    let mut pc : u16 = 0;
    loop {
        let instr_slice = &data[(pc as usize) .. ((pc + 3) as usize)];

        let instruction = instructions::Instruction::new(instr_slice);

        println!("{}", instruction.op);

        pc += instruction.size as u16;

        if instruction.size == 0 {
            panic!("Uh oh!")
        }
    }
}
