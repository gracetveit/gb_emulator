use crate::{gpu::gpu::GPU, request_response::{Request, Response, RequestType}};
use std::{fs, sync::mpsc::{Receiver, Sender}};

/*

Memory map

0x0000 - 0x3FFF: Rom Bank #0
0x4000 - 0x7FFF: swithcable ROM bank
0x8000 - 0x9FFF: Video RAM
0xA000 - 0xBFFF: Switchable RAM bank
0xC000 - 0xDFFF: Internal RAM
0xE000 - 0xFDFF: Echo of Internal RAM
0xFE00 - 0xFE9F: Sprite Attrib Mem (OAM)
0xFEA0 - 0xFEFF: Empty but unusable for I/O (Ignore)
0xFF00 - 0xFF7F: I/O Ports
0xFF80 - 0xFFFE: Internal / High RAM
0xFFFF: Interrupt Enable Register

*/

#[derive(Debug)]
pub struct MemoryBus {
    memory: [u8; 0x10000],
    request_receiver: Receiver<Request>,
    rom_name: String
}

impl MemoryBus {
    pub fn new(request_receiver: Receiver<Request>, rom_name: String) -> MemoryBus {
        let data = MemoryBus::read_data("gb_bios.bin");
        let mut memory_bus = MemoryBus { memory: [0; 0x10000], request_receiver, rom_name};
        memory_bus.load_data(data);
        memory_bus
    }

    pub fn step(&mut self) {
        match self.request_receiver.recv() {
            Ok(request) => {
                let request_info = request.request_info;
                match request_info.request_type {
                    RequestType::Read => {
                        self.send_read(request_info.addr, request_info.request_len, request.responder)
                    }
                    RequestType::Write(data) => {
                        self.send_write(request_info.addr, data, request.responder)
                    }
                    RequestType::LoadROM => {
                        let data = MemoryBus::read_data(&self.rom_name);
                        self.load_data(data);
                        request.responder.send(Response::Ok204).unwrap();
                    }
                }
            }
            Err(err) => panic!("{err:}")
        };
    }

    // fn resolve_request(&mut self, request: Request) {

    // }

    fn send_read(&self, addr: u16, request_len: u8, responder: Sender<Response>) {
        let addr = addr as usize;
        let request_len = request_len as usize;
        let data = self.memory[addr..addr + request_len].to_vec();

        responder.send(Response::Ok200(data)).unwrap();
    }
    fn send_write(&mut self, addr: u16, data: Vec<u8>, responder: Sender<Response>) {
        let mut  addr = addr as usize;
        for x in data {
            self.memory[addr] = x;
            addr += 1;
        }

        responder.send(Response::Ok204).unwrap();
    }

    fn read_data(filename: &str) -> Vec<u8> {
        let file = fs::read(format!("./roms/{filename:}"));
        match file {
            Ok(data) => {
                data
            }
            Err(err) => panic!("{err:}")
        }
    }

    fn load_data(&mut self, data: Vec<u8>) {
        let mut i = 0;
        while i < data.len() {
            self.memory[i] = data[i];
            i += 1;
        }
    }
}

// #[cfg(test)]
// #[test]
// fn write_read_byte_test() {
//     let mut test_bus = MemoryBus::new();

//     test_bus.in_bios = false;
//     let test_addr = 0xA12C;
//     let test_byte = 0x93;

//     test_bus.write_byte(test_addr, test_byte);
//     assert_eq!(test_bus.read_byte(test_addr), test_byte)
// }

// #[test]
// fn write_read_word_test() {
//     let mut test_bus = MemoryBus::new();

//     test_bus.in_bios = false;
//     let test_addr = 0xA12C;
//     let test_word = 0x9383;

//     test_bus.write_word(test_addr, test_word);
//     assert_eq!(test_bus.read_word(test_addr), test_word)
// }
// #[test]
// fn memory_bus_init_test() {
//     MemoryBus::new();
// }

// #[test]
// fn read_bios_test() {
//     let test_bus = MemoryBus::new();

//     assert_eq!(test_bus.read_byte(0), 0x31);
//     assert_eq!(test_bus.read_byte(1), 0xFE);
//     assert_eq!(test_bus.read_byte(2), 0xFF);
// }
