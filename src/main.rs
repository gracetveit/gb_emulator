pub mod cpu;
pub mod gpu;
use std::env;
use std::time::Instant;

use cpu::cpu::CPU;
use cpu::instruction::Instruction;

fn debug_info(cpu: &CPU, breakpoint: u16, address: u16) {
    // let now = Instant::now();
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let byte = cpu.bus.read_byte(cpu.pc);
    println!("Address: 0x{:X}", cpu.pc);
    println!(
        "Opcode: 0x{:X}; {:?}; Next Byte: 0x{:X}",
        byte,
        Instruction::from_byte(
            {
                match byte == 0xCB {
                    true => cpu.bus.read_byte(cpu.pc.wrapping_add(1)),
                    false => byte,
                }
            },
            byte == 0xCB
        ),
        cpu.bus.read_byte(cpu.pc.wrapping_add(match byte == 0xCB {
            true => 1,
            false => 2,
        }))
    );
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
    println!{"Watching: 0x{:X}: 0x{:X}", address, cpu.bus.read_byte(address)}
    // println!("Registers: {:?}, sp: 0x{:X}", cpu.registers, cpu.sp);
    if cpu.pc == breakpoint {
        println!("Breakpoint!");
    }
    // while now.elapsed().as_millis() <= 16{

    // }
}

fn get_args(args: Vec<String>) -> Result<u16, String> {
    let test = args.get(1);
    match test {
        Some(x) => match x.parse::<u16>() {
            Ok(y) => Ok(y),
            Err(_) => Err(format!("Error, cannot parse {}", x)),
        },
        None => Err(format!("")),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let breakpoint_arg = get_args(args);
    let mut cpu = CPU::new();
    let mut logging = false;
    let mut breakpoint = 0;
    match &breakpoint_arg {
        Ok(x) => breakpoint = *x,
        Err(e) => println!("{}", e),
    }
    while cpu.pc <= 0xFF {
    match &breakpoint_arg {
        Ok(x) => {
            if x == &cpu.pc {
                logging = true;
            }
        },
        _ => {}
    }
        if logging {
            debug_info(&cpu, breakpoint, 0xFF44);
        }
        cpu.step();
    }
}
