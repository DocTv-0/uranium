use tokio::io::{AsyncReadExt, AsyncWriteExt, Error, ErrorKind, Result};
use tokio::net::{TcpListener, TcpStream};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Version {
    Java26_2
}

impl Version {
    pub const fn protocol(self) -> i32 {
        match self {
            Self::Java26_2 => 776,
        }
    }
}

pub enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Play
}

/// Encodes and sends a packet to the client
pub async fn send_packet(stream: &mut TcpStream, packet: Vec<u8>) -> Result<()> {
    let mut final_packet = Vec::new();
    encode_varint(&mut final_packet, packet.len() as i32);
    final_packet.extend(packet);

    stream.write_all(&final_packet).await?;
    stream.flush().await?;
    Ok(())
}

/// Saves a VarInt to a vector of bytes
pub fn encode_varint(buffer: &mut Vec<u8>, mut value: i32) {
    loop {
        let mut temporary = (value & 0x7F) as u8;
        value = ((value as u32) >> 7) as i32;
        if value != 0 {
            temporary |= 0x80;
        }
        buffer.push(temporary);
        if value == 0 {
            break;
        }
    }
}

/// Saves a String to a vector of bytes
pub fn encode_string(buffer: &mut Vec<u8>, text: &str) {
    let bytes = text.as_bytes();
    encode_varint(buffer, bytes.len() as i32);
    buffer.extend_from_slice(bytes);
}
