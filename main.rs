/// Encodes and sends a packet to the client
pub async fn send_packet(stream: &mut TcpStream, packet: Vec<u8>) -> Result<()> {
    let mut final_packet = Vec::new();
    encode_varint(&mut final_packet, packet.len() as i32);
    final_packet.extend(packet);

    stream.write_all(&final_packet).await?;
    stream.flush().await?;
    Ok(())
}
