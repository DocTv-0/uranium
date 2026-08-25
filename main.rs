use tokio::io::{AsyncReadExt, AsyncWriteExt, Error, ErrorKind, Result};
use tokio::net::{TcpListener, TcpStream};
use valence_nbt::{compound, Compound, List, to_binary};
use serde_json::json;

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
    pub const fn version(self) -> &'static str {
        match self {
            Self::Java26_2 => "26.2",
        }
    }
}

pub enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Play
}

#[derive(Clone, Copy)]
pub struct ServerOptions {
    description: &'static str,
    version: Version,
    max_players: i32,
    verify_players: bool,
    hardcore: bool,
    render_distance: i32,
    simulation_distance: i32,
}

#[tokio::main]
async fn main() -> Result<()> {

    let server_options = ServerOptions {
        description: "§aA Rust Minecraft Server",
        version: Version::Java26_2,
        max_players: 24,
        verify_players: false,
        hardcore: false,
        render_distance: 16,
        simulation_distance: 5,
    };
    
    let listener = TcpListener::bind("127.0.0.1:25565").await?;
    
    loop {
        let (stream, addr) = listener.accept().await?;

        tokio::spawn(async move {
            if let Err(error) = handle_client(stream).await {
                eprintln!("Errod handling client {}: {}", addr, error);
            }
        });
    }
}

async fn handle_client(mut stream: TcpStream) -> Result<()> {
    let mut state = ConnectionState::Handshaking;

    loop {
        match state {
            ConnectionState::Handshaking => {
                let packet_length = read_varint(&mut stream).await?;

                let packet_id = read_varint(&mut stream).await?;

                if packet_id != 0 {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Expected packet ID to be 0"
                    ));
                }

                let protocol_version = read_varint(&mut stream).await?;

                let server_address = read_string(&mut stream).await?;

                let server_port = read_ushort(&mut stream).await?;

                match read_varint(&mut stream).await? {
                    1 => {state = ConnectionState::Status;},
                    2 | 3 => {state = ConnectionState::Login;},
                    _ => {
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            "Expected 1, 2 or 3"
                        ))
                    }
                }
            }

            ConnectionState::Status => {
                let packet_length = read_varint(&mut stream).await?;

                match read_varint(&mut stream).await? {
                    0 => { // Status Request
                        let mut packet: Vec<u8> = Vec::new();

                        let status_json = json!({
                            "version": {
                                "name": options.version.version(),
                                "protocol": options.version.protocol(),
                            },
                            "players": {
                                "max": options.max_players,
                                "online": 0,
                                "sample": []
                            },
                            "description": {
                                "text": options.description
                            }
                        });

                        encode_varint(&mut packet, 0x00);

                        encode_string(&mut packet, &status_json.to_string() as &str);

                        send_packet(&mut stream, packet).await?;
                    }

                    1 => { // Ping Request
                        let timestamp = read_long(&mut stream).await?;

                        let mut packet: Vec<u8> = Vec::new();

                        encode_varint(&mut packet, 0x01);

                        encode_long(&mut packet, timestamp);

                        send_packet(&mut stream, packet).await?;

                        return Ok(())
                    }

                    _ => {
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            "Invalid request"
                        ))
                    }
                }
            }

            ConnectionState::Login => {
                todo!()
            }

            ConnectionState::Play => {
                todo!()
            }
        }
    }
}

/// Reads an Unsigned Short from the stream
pub async fn read_ushort(stream: &mut TcpStream) -> Result<u16> {
    stream.read_u16().await
}

/// Reads a Long from the stream
pub async fn read_long(stream: &mut TcpStream) -> Result<i64> {
    stream.read_i64().await
}

/// Saves a Long to a vector of bytes
pub fn encode_long(buffer: &mut Vec<u8>, value: i64) {
    for byte in value.to_be_bytes() {
        buffer.push(byte);
    }
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

/// Saves a String to a vector of bytes
pub fn encode_string(buffer: &mut Vec<u8>, text: &str) {
    let bytes = text.as_bytes();
    encode_varint(buffer, bytes.len() as i32);
    buffer.extend_from_slice(bytes);
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
