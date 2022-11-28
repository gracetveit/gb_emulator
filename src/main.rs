pub mod cpu;
pub mod gpu;
pub mod bus_channel;
// use std::env;
// use std::time::Instant;
use winit::dpi::LogicalSize;
use winit::event::VirtualKeyCode;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Fullscreen, WindowBuilder};
use winit_input_helper::WinitInputHelper;

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
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = WindowBuilder::new()
        .with_title("RustGBEmu")
        .with_min_inner_size(LogicalSize::new(160 as f32, 144 as f32))
        .build(&event_loop)
        .unwrap();

    let mut lcd = LCD::new(&window);
    lcd.hello_world();
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
