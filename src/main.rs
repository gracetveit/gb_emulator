pub mod cpu;
pub mod gpu;
use std::time::Instant;

use cpu::cpu::CPU;
use cpu::instruction::Instruction;

fn debug_info(cpu: &CPU) {
    let now = Instant::now();
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        let byte = cpu.bus.read_byte(cpu.pc);
        println!("Address: 0x{:X}", cpu.pc);
        println!("Opcode: 0x{:X}; {:?}; Next Byte: 0x{:X}", byte, Instruction::from_byte({
            match byte == 0xCB {
                true => cpu.bus.read_byte(cpu.pc.wrapping_add(1)),
                false => byte
            }
        }, byte == 0xCB), cpu.bus.read_byte(cpu.pc.wrapping_add(match byte == 0xCB {true => 1, false => 2})));
        // Registers
        println!("Registers: {{");
        println!("\tA: 0x{:X}", cpu.registers.a);
        println!("\tB: 0x{:X}", cpu.registers.b);
        println!("\tC: 0x{:X}", cpu.registers.c);
        println!("\tD: 0x{:X}", cpu.registers.d);
        println!("\tE: 0x{:X}", cpu.registers.e);
        println!("\tH: 0x{:X}", cpu.registers.h);
        println!("\tL: 0x{:X}", cpu.registers.l);
        println!("\tF: {{");
        println!("\t\tZero: {}", cpu.registers.f.zero);
        println!("\t\tSub: {}", cpu.registers.f.subtract);
        println!("\t\tHalf-Carry: {}", cpu.registers.f.half_carry);
        println!("\t\tCarry: {}", cpu.registers.f.carry);
        println!("\t}}");
        println!("\tSP: 0x{:X}", cpu.sp);
        println!("}}");
        // println!("Registers: {:?}, sp: 0x{:X}", cpu.registers, cpu.sp);
        if cpu.registers.f.zero {
            println!("Breakpoint!");
        }
        while now.elapsed().as_millis() <= 16{

        }
}

fn main() {
    let mut cpu = CPU::new();
    while cpu.pc <= 0x64 {
        debug_info(&cpu);
        cpu.step();
    }
}
