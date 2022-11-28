use std::sync::mpsc::{channel, Sender};

#[derive(Debug)]
pub struct BusChannel {
    sender: Sender<BusData>,
}

impl BusChannel {
    pub fn new(sender: Sender<BusData>) -> Self {
        BusChannel { sender }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        let (bus_sender, bus_receiver) = channel::<Option<ByteWord>>();
        let bus_data = BusData {
            sender: Some(bus_sender),
            addr,
            val: None,
            instruction: ReadWrite::ReadByte,
        };
        self.sender.send(bus_data).unwrap();
        if let Some(ByteWord::Byte(x)) = bus_receiver.recv().unwrap() {
            return x;
        } else {
            panic!()
        };
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        let (bus_sender, bus_receiver) = channel::<Option<ByteWord>>();
        let bus_data = BusData {
            sender: Some(bus_sender),
            addr,
            val: None,
            instruction: ReadWrite::ReadWord,
        };

        self.sender.send(bus_data).unwrap();
        if let Some(ByteWord::Word(x)) = bus_receiver.recv().unwrap() {
            return x;
        } else {
            panic!()
        }
    }

    pub fn write_byte(&self, addr: u16, val: u8) {
        let bus_data = BusData {
            sender: None,
            addr,
            val: Some(ByteWord::Byte(val)),
            instruction: ReadWrite::WriteByte,
        };

        self.sender.send(bus_data).unwrap();
    }

    pub fn write_word(&self, addr: u16, val: u16) {
        let bus_data = BusData {
            sender: None,
            addr,
            val: Some(ByteWord::Word(val)),
            instruction: ReadWrite::WriteWord,
        };

        self.sender.send(bus_data).unwrap()
    }
}

pub struct BusData {
    pub sender: Option<Sender<Option<ByteWord>>>,
    pub addr: u16,
    pub val: Option<ByteWord>,
    pub instruction: ReadWrite,
}

pub enum ByteWord {
    Byte(u8),
    Word(u16),
}

pub enum ReadWrite {
    ReadByte,
    ReadWord,
    WriteByte,
    WriteWord,
}

// ReadByte (I: u8 O: u8)
// ReadWord (I: u8, O: u16)
// Write Byte (I: u8; u16)
// Write Word
