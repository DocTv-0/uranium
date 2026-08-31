mod nbtlib;
mod packetlib;

use packetlib::*;
use tokio::io::{Error, ErrorKind, Result};
use tokio::net::{TcpListener, TcpStream};
use serde_json::json;
use crate::nbtlib::{get_configuration_data};

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
    pub const fn login_start_id(self) -> i32 {
        match self {
            Self::Java26_2 => 0x00,
        }
    }
    pub const fn login_success_id(self) -> i32 {
        match self {
            Self::Java26_2 => 0x02,
        }
    }
}

pub enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Configuration,
    Play
}

#[derive(Clone, Copy)]
pub struct ServerConfig {
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

    let server_options = ServerConfig {
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
            if let Err(error) = handle_client(stream, server_options).await {
                eprintln!("Error handling client {}: {}", addr, error);
            }
        });
    }
}

async fn handle_client(mut stream: TcpStream, config: ServerConfig) -> Result<()> {
    let mut state = ConnectionState::Handshaking;

    loop {
        match state {
            ConnectionState::Handshaking => {
                let _packet_length = read_varint(&mut stream).await?;

                let packet_id = read_varint(&mut stream).await?;

                if packet_id != 0 {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Expected packet ID to be 0"
                    ));
                }

                let _protocol_version = read_varint(&mut stream).await?;

                let _server_address = read_string(&mut stream).await?;

                let _server_port = read_ushort(&mut stream).await?;

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
                let _packet_length = read_varint(&mut stream).await?;

                match read_varint(&mut stream).await? {
                    0 => { // Status Request
                        let mut packet = Packet::new();

                        let status_json = json!({
                            "version": {
                                "name": config.version.version(),
                                "protocol": config.version.protocol(),
                            },
                            "players": {
                                "max": config.max_players,
                                "online": 0,
                                "sample": []
                            },
                            "description": {
                                "text": config.description
                            }
                        });

                        packet.encode_varint(0x00);

                        packet.encode_string(&status_json.to_string() as &str);

                        packet.send(&mut stream).await?;
                    }

                    1 => { // Ping Request
                        let timestamp = read_long(&mut stream).await?;

                        let mut packet = Packet::new();

                        packet.encode_varint(0x01);

                        packet.encode_long(timestamp);

                        packet.send(&mut stream).await?;

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
                let _packet_length = read_varint(&mut stream).await?;

                let packet_id = read_varint(&mut stream).await?;

                if packet_id != config.version.login_start_id() {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("Invalid packet ID. Expected: {}, got: {}", config.version.login_start_id(), packet_id)
                    ))
                }

                let username = read_string(&mut stream).await?;

                let uuid = read_uuid(&mut stream).await?;

                let mut packet = Packet::new();

                if config.verify_players {
                    todo!("Add player verification")
                } else {
                    packet.encode_varint(config.version.login_success_id());

                    packet.encode_uuid(uuid);

                    packet.encode_string(username.as_str());

                    // length of the properties array
                    packet.encode_varint(0);

                    packet.encode_uuid(uuid);
                }

                packet.send(&mut stream).await?;

                let _acknowledgment_length = read_varint(&mut stream).await?;

                let acknowledgment_id = read_varint(&mut stream).await?;

                if acknowledgment_id != 0x03 {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("Invalid acknowledgment ID. Expected: {}, got: {}", 0x03, acknowledgment_id)
                    ))
                }

                state = ConnectionState::Configuration;
            }

            ConnectionState::Configuration => {

                let mut packet = Packet::new();

                packet.encode_varint(0x01);

                packet.encode_string("minecraft:brand");

                packet.encode_string("uranium");

                packet.send(&mut stream).await?;

                for (key, value) in get_configuration_data() {
                    let mut packet = Packet::new();

                    packet.encode_varint(0x07);

                    packet.encode_string(key);

                    packet.encode_varint(value.len() as i32);

                    for (entry_key, entry_value) in value {
                        packet.encode_string(entry_key);

                        packet.push(1);

                        packet.extend_from_slice(&entry_value);
                    }

                    packet.send(&mut stream).await?;
                }
            }

            ConnectionState::Play => {
                todo!()
            }
        }
    }
}
