pub mod cpu;
pub mod bitmath;
mod instructions;


fn main() {
    let hello = cpu::Register::A;
    println!("Hello, world!");
}
