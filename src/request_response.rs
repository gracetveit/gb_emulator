use std::sync::mpsc::{channel, Receiver, Sender};

#[derive(Debug)]
pub struct Request {
    pub request_info: RequestInfo,
    pub responder: Sender<Response>,
}

impl Request {
    fn create_read_byte_request(addr: u16) -> (Self, Receiver<Response>) {
        let (response_sender, response_receiver) = channel::<Response>();
        let request_info = RequestInfo {
            addr,
            request_len: 1,
            request_type: RequestType::Read,
        };
        return (
            Request {
                request_info,
                responder: response_sender,
            },
            response_receiver,
        );
    }

    fn create_read_word_request(addr: u16) -> (Self, Receiver<Response>) {
        let (response_sender, response_receiver) = channel::<Response>();
        let request_info = RequestInfo {
            addr,
            request_len: 2,
            request_type: RequestType::Read,
        };
        return (
            Request {
                request_info,
                responder: response_sender,
            },
            response_receiver,
        );
    }

    fn create_write_byte_request(addr: u16, data: u8) -> (Self, Receiver<Response>) {
        let (response_sender, response_receiver) = channel::<Response>();
        let request_info = RequestInfo {
            addr,
            request_len: 1,
            request_type: RequestType::Write(vec![data]),
        };
        return (
            Request {
                request_info,
                responder: response_sender,
            },
            response_receiver,
        );
    }

    fn create_read_oam_request() -> (Self, Receiver<Response>) {
        let (response_sender, response_receiver) = channel::<Response>();
        let request_info = RequestInfo {
            addr: 0xFE00,
            request_len: 160,
            request_type: RequestType::Read,
        };
        return (
            Request {
                request_info,
                responder: response_sender,
            },
            response_receiver,
        );
    }

    // pub fn get_request_type(&self) -> RequestType {
    //     match self.request_info.request_type {
    //         RequestType::Read => {RequestType::Read},
    //         RequestType::Write(data) => {
    //             let mut new_data: Vec<u8> = vec![];
    //             for x in data {
    //                 new_data.push(x);
    //             }
    //             RequestType::Write(new_data)
    //         }
    //     }
    // }
}

#[derive(Debug)]
pub struct RequestInfo {
    pub addr: u16,
    pub request_len: u8,
    pub request_type: RequestType,
}
#[derive(Debug)]
pub enum RequestType {
    Read,
    Write(Vec<u8>),
    LoadROM,
}

pub enum Response {
    Ok200(Vec<u8>),
    MemError(String),
    RequestError(String),
    Ok204,
}

#[derive(Debug)]
pub struct Bus {
    pub request_sender: Sender<Request>,
}

impl Bus {
    pub fn read_byte(&self, addr: u16) -> u8 {
        let (request, response_receiver) = Request::create_read_byte_request(addr);
        self.request_sender.send(request).unwrap();
        match response_receiver.recv() {
            Ok(response) => match response {
                Response::Ok200(data) => data[0],
                Response::MemError(err) => panic!("{err:}"),
                Response::RequestError(err) => panic!("{err:}"),
                Response::Ok204 => panic!("Error, expected data, received 204"),
            },
            Err(err) => panic!("{err:}"),
        }
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        let (request, response_receiver) = Request::create_read_word_request(addr);
        self.request_sender.send(request).unwrap();
        match response_receiver.recv() {
            Ok(response) => match response {
                Response::Ok200(data) => ((data[1] as u16) << 8) | (data[0] as u16),
                Response::MemError(err) => panic!("{err:}"),
                Response::RequestError(err) => panic!("{err:}"),
                Response::Ok204 => panic!("Error, expected data, received 204"),
            },
            Err(err) => panic!("{err:}"),
        }
    }

    pub fn write_byte(&self, addr: u16, data: u8) {
        let (request, response_receiver) = Request::create_write_byte_request(addr, data);
        self.request_sender.send(request).unwrap();
        match response_receiver.recv() {
            Ok(response) => match response {
                Response::Ok200(data) => panic!("Error, expected 204, received 200 with {data:?}"),
                Response::Ok204 => {}
                Response::MemError(err) => panic!("{err:}"),
                Response::RequestError(err) => panic!("{err:}"),
            },
            Err(err) => panic!("{err:}"),
        }
    }

    pub fn load_rom(&self) {
        let (response_sender, response_receiver) = channel::<Response>();
        let request = Request {
            request_info: RequestInfo {
                addr: 0,
                request_len: 0,
                request_type: RequestType::LoadROM,
            },
            responder: response_sender,
        };
        self.request_sender.send(request).unwrap();
        match response_receiver.recv() {
            Ok(response) => match response {
                Response::Ok204 => {}
                Response::Ok200(data) => panic!("Error, expected 204, received 200 with {data:?}"),
                Response::MemError(err) => panic!("{err:}"),
                Response::RequestError(err) => panic!("{err:}"),
            },
            Err(err) => panic!("{err:}"),
        }
    }

    pub fn read_oam(&self) -> Vec<u8> {
        let (request, response_receiver) = Request::create_read_oam_request();
        self.request_sender.send(request).unwrap();
        match response_receiver.recv() {
            Ok(response) => match response {
                Response::Ok200(data) => data,
                Response::Ok204 => panic!("Error, expected data, received 204 instead"),
                Response::MemError(err) => panic!("{err:}"),
                Response::RequestError(err) => panic!("{err:}"),
            },
            Err(err) => panic!("{err:}"),
        }
    }
}

// ReadByte (I: u8 O: u8)
// ReadWord (I: u8, O: u16)
// Write Byte (I: u8; u16)
// Write Word

// Channel: Sender<x>, Receiver<x>

// CPU/PPU Sends <x> over channel with request info and a new sender (to receive later);
// Mem, waiting for <x>, receives it and executes request;
// Mem uses sender to send requested data back.

// struct x {
// 	request_info: RequestInfo
// 	responder: Sender<Response>
// }

// struct RequestInfo {
// 	addr: u16,
// 	request_len: u8,
// 	type: RequestType,
// 	write_data: Option<Vec<u8>>
// }

// enum RequestType {
// 	Read,
// 	Write
// }

// struct ResponseInfo {
// 	response_code
// 	data:
// }

// enum Response {
// 	Ok200(Vec<u8>),
// 	MemError(String),
// 	RequestError(String),
// 	Ok204
// }
