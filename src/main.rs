use std::time::{Instant, Duration};

pub mod cpu;
pub mod bitmath;
pub mod memory;
mod instructions;

use instructions::Instruction;
use memory::Memory;
use ansi_term::Color::Blue;

fn black_box<T>(dummy: T) -> T {
    unsafe {
        std::ptr::read_volatile(&dummy)
    }
}

fn main() {
    let data = include_bytes!("data/dmg_boot.bin");

    let mut cpu_state = cpu::Registers::default();
    let mut memory = Memory::default();

    loop {
        let addr = cpu_state.pc();
        let instruction = Instruction::from_bytes(addr as usize, data);

        if let Some(instruction) = instruction {

            print!("{:04X}: ", addr);

            let instruction_data = &data[(addr as usize)..((addr + instruction.size as u16) as usize)];

            for i in 0..3 {
                if i < instruction_data.len() {
                    print!("{:02X} ", instruction_data[i]);
                } else {
                    print!("   ");
                };
            };

            println!("| {}", Blue.bold().paint(format!("{}", instruction.op)));

            if instruction.size == 0 {
                break
            }

            instruction.execute(&mut cpu_state, &mut memory);
        } else {
            break
        }
    }

    println!{"{}", cpu_state}
}
