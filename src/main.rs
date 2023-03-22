pub mod cpu;
pub mod gpu;
pub mod request_response;
use cpu::cpu::CPU;
use cpu::memory_bus::MemoryBus;
use gpu::gpu::GPU;
use gpu::tile::Color;
use request_response::Request;
// use std::env;
// use std::time::Instant;
use winit::dpi::LogicalSize;
use winit::event::VirtualKeyCode;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Fullscreen, WindowBuilder};
use winit_input_helper::WinitInputHelper;
use std::sync::mpsc::{self, Sender, Receiver, channel};
use std::thread;

// use cpu::cpu::CPU;
// use cpu::instruction::Instruction;
use gpu::lcd::LCD;

// fn debug_info(cpu: &CPU, breakpoint: u16, address: u16) {
//     // let now = Instant::now();
//     print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
//     let byte = cpu.bus.read_byte(cpu.pc);
//     println!("Address: 0x{:X}", cpu.pc);
//     println!(
//         "Opcode: 0x{:X}; {:?}; Next Byte: 0x{:X}",
//         byte,
//         Instruction::from_byte(
//             {
//                 match byte == 0xCB {
//                     true => cpu.bus.read_byte(cpu.pc.wrapping_add(1)),
//                     false => byte,
//                 }
//             },
//             byte == 0xCB
//         ),
//         cpu.bus.read_byte(cpu.pc.wrapping_add(match byte == 0xCB {
//             true => 1,
//             false => 2,
//         }))
//     );
//     // Registers
//     println!("Registers: {{");
//     println!("\tA: 0x{:X}", cpu.registers.a);
//     println!("\tB: 0x{:X}", cpu.registers.b);
//     println!("\tC: 0x{:X}", cpu.registers.c);
//     println!("\tD: 0x{:X}", cpu.registers.d);
//     println!("\tE: 0x{:X}", cpu.registers.e);
//     println!("\tH: 0x{:X}", cpu.registers.h);
//     println!("\tL: 0x{:X}", cpu.registers.l);
//     println!("\tF: {{");
//     println!("\t\tZero: {}", cpu.registers.f.zero);
//     println!("\t\tSub: {}", cpu.registers.f.subtract);
//     println!("\t\tHalf-Carry: {}", cpu.registers.f.half_carry);
//     println!("\t\tCarry: {}", cpu.registers.f.carry);
//     println!("\t}}");
//     println!("\tSP: 0x{:X}", cpu.sp);
//     println!("}}");
//     println! {"Watching: 0x{:X}: 0x{:X}", address, cpu.bus.read_byte(address)}
//     // println!("Registers: {:?}, sp: 0x{:X}", cpu.registers, cpu.sp);
//     if cpu.pc == breakpoint {
//         println!("Breakpoint!");
//     }
//     // while now.elapsed().as_millis() <= 16{

//     // }
// }

// fn get_args(args: Vec<String>) -> Result<u16, String> {
//     let test = args.get(1);
//     match test {
//         Some(x) => match x.parse::<u16>() {
//             Ok(y) => Ok(y),
//             Err(_) => Err(format!("Error, cannot parse {}", x)),
//         },
//         None => Err(format!("")),
//     }
// }

fn main() {
    let (request_sender, request_receiver) = channel::<Request>();
    // Create Memory Thread
    thread::spawn(move || {
        let mut memory = MemoryBus::new(request_receiver, String::from("hello-world.gb"));
        loop {
            memory.step();
        }
    });
    let (ppu_timing_sender, cpu_timing_receiver) = channel::<u8>();
    let (cpu_timing_sender, ppu_timing_receiver) = channel::<u8>();
    // Create CPU thread
    let cpu_request_sender = request_sender.clone();
    thread::spawn(move || {
        let mut cpu = CPU::new(cpu_request_sender);
        let mut relative_t = 0;
        loop {
            if relative_t <= 0 {
                let step_t = cpu.step();
                relative_t += step_t as i32;
                cpu_timing_sender.send(step_t).unwrap();
            } else {
                relative_t -= match cpu_timing_receiver.recv() {
                    Ok(x) => x as i32,
                    Err(e) => panic!("{e:}")
                }
            }
            // let relative_t = cpu.step();
        }
    });
    let (lcd_sender, lcd_receiver) = mpsc::channel::<[[[u8; 4]; 160]; 144]>();
    // Create PPU thread
    thread::spawn(move || {
        let mut ppu = GPU::new(request_sender, lcd_sender);
        let mut relative_t = 0;
        loop {
            if relative_t <= 0 {
                let step_t = ppu.step();
                relative_t += step_t as i32;
                ppu_timing_sender.send(step_t).unwrap();
            } else {
                relative_t -= match ppu_timing_receiver.recv() {
                    Ok(x) => x as i32,
                    Err(e) => panic!("{e:}")
                }
            }
        }
    });
    // Create LCD thread
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();


    let window = WindowBuilder::new()
        .with_title("RustGBEmu")
        .with_min_inner_size(LogicalSize::new(160 as f32, 144 as f32))
        .build(&event_loop)
        .unwrap();

    let mut lcd = LCD::new(&window, lcd_receiver);
    lcd.render();

    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            // Close event
            if input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                lcd.resize(size);
            }

            // Fullscreen
            if input.key_pressed(VirtualKeyCode::F11) {
                match window.fullscreen() {
                    None => {
                        window
                            .set_fullscreen(Some(Fullscreen::Borderless(window.current_monitor())));
                    }
                    Some(_) => window.set_fullscreen(None),
                }
            }

            window.request_redraw();
        }
        lcd.push();
    });

    // let args: Vec<String> = env::args().collect();
    // let breakpoint_arg = get_args(args);
    // let mut cpu = CPU::new();
    // let mut logging = false;
    // let mut breakpoint = 0;
    // match &breakpoint_arg {
    //     Ok(x) => breakpoint = *x,
    //     Err(e) => println!("{}", e),
    // }
    // while cpu.pc <= 0xFF {
    //     match &breakpoint_arg {
    //         Ok(x) => {
    //             if x == &cpu.pc {
    //                 logging = true;
    //             }
    //         }
    //         _ => {}
    //     }
    //     if logging {
    //         debug_info(&cpu, breakpoint, 0xFF44);
    //     }
    //     cpu.step();
    // }
}

pub trait ProcessingUnitStep {
    fn step(&mut self) -> u8;
}

fn processing_unit_step(f: impl Fn() -> u8, sender: Sender<u8>, receiver: Receiver<u8>, initial_relative_t: i32) -> i32 {
    // previous t is proccessing_unit.t - other_processing_unit.t
    let mut relative_t = initial_relative_t;
    if relative_t <= 0 {
        let step_t = f();
        relative_t += step_t as i32;
        sender.send(step_t).unwrap();
    }
    else {
        relative_t -= match receiver.recv() {
            Ok(x) => x as i32,
            Err(e) => panic!("{e:}")
        }
    }

    relative_t
}

enum ProcessingUnit {
    CPU(CPU),
    PPU(GPU)
}
