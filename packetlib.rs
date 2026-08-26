use tokio::io::{AsyncReadExt, AsyncWriteExt, Error, ErrorKind, Result};
use tokio::net::{TcpStream};

/// Reads an Unsigned Short from the stream
pub async fn read_ushort(stream: &mut TcpStream) -> Result<u16> {
    stream.read_u16().await
}

/// Reads a Long from the stream
pub async fn read_long(stream: &mut TcpStream) -> Result<i64> {
    stream.read_i64().await
}

/// Reads a UUID from the stream
pub async fn read_uuid(stream: &mut TcpStream) -> Result<u128> {
    stream.read_u128().await
}

/// Reads a VarInt from the stream
pub async fn read_varint(stream: &mut TcpStream) -> Result<i32> {
    let mut value = 0;
    let mut position = 0;

    loop {
        let mut current_byte = [0; 1];
        stream.read_exact(&mut current_byte).await?;
        let byte = current_byte[0];

        value |= ((byte & 0x7F) as i32) << position;

        if (byte & 0x80) == 0 {
            return Ok(value);
        }

        position += 7;

        if position >= 35 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "VarInt too long, max VarInt length 5 bytes",
            ));
        }
    }
}

/// Reads a String from the stream
pub async fn read_string(stream: &mut TcpStream) -> Result<String> {
    let length = read_varint(stream).await?;

    if length < 0 || length > 32767 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("Invalid String length : {}", length),
        ));
    }

    let length = length as usize;
    let mut string_buffer = vec![0; length];
    stream.read_exact(&mut string_buffer).await?;

    String::from_utf8(string_buffer).map_err(|e| {
        Error::new(
            ErrorKind::InvalidData,
            format!("String contains invalid UTF-8: {}", e),
        )
    })
}

pub struct Packet {
    packet: Vec<u8>
}

impl Packet {
    pub fn new() -> Self {
        Packet { packet: Vec::new() }
    }
    
    /// Saves a Bool to a packet
    pub fn encode_bool(&mut self, value: bool) {
        self.packet.push(value as u8);
    }
    
    /// Saves a Long to a packet
    pub fn encode_long(&mut self, value: i64) {
        for byte in value.to_be_bytes() {
            self.packet.push(byte);
        }
    }
    
    /// Saves a UUID to a packet
    pub fn encode_uuid(&mut self, value: u128) {
        for byte in value.to_be_bytes() {
            self.packet.push(byte);
        }
    }
    
    /// Saves a VarInt to a packet
    pub fn encode_varint(&mut self, mut value: i32) {
        loop {
            let mut temporary = (value & 0x7F) as u8;
            value = ((value as u32) >> 7) as i32;
            if value != 0 {
                temporary |= 0x80;
            }
            self.packet.push(temporary);
            if value == 0 {
                break;
            }
        }
    }
    
    pub fn encode_string(&mut self, value: &str) {
        let bytes = value.as_bytes();
        self.encode_varint(bytes.len() as i32);
        self.packet.extend_from_slice(bytes);
    }

    /// Encodes and sends a packet to the client
    pub async fn send(self, stream: &mut TcpStream) -> Result<()> {
        let mut final_packet = Packet::new();
        final_packet.encode_varint(self.packet.len() as i32);
        final_packet.extend(self);

        stream.write_all(&final_packet.packet).await?;
        stream.flush().await?;
        Ok(())
    }

    pub fn extend(&mut self, other: Packet) {
        self.packet.extend(other.packet);
    }
    
    pub fn push(&mut self, value: u8) {
        self.packet.push(value)
    }
    
    pub fn extend_from_slice(&mut self, other: &[u8]) {
        self.packet.extend_from_slice(other)
    }
}
