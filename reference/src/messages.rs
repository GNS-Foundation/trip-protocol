//! Protocol messages - TODO: Implement
//! See spec/MESSAGES.md for details

/// Message type codes
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    I1 = 0x01,
    R1 = 0x02,
    I2 = 0x03,
    R2 = 0x04,
    Data = 0x10,
    Ack = 0x11,
    Ping = 0x12,
    Pong = 0x13,
    Close = 0x14,
    Update = 0x20,
    UpdateAck = 0x21,
    Error = 0xF0,
}

/// Protocol message
pub struct Message {
    pub msg_type: MessageType,
    pub payload: Vec<u8>,
}

