use async_std::{
    io::{ReadExt, WriteExt},
    net::TcpStream,
};
use json::JsonValue;

#[derive(Debug)]
pub struct Error(&'static str);

pub async fn receive(stream: &mut TcpStream) -> Result<JsonValue, Error> {
    let mut packet_size_buffer: [u8; 8] = [0; 8];
    stream.read_exact(&mut packet_size_buffer).await.unwrap();

    let mut packet_buffer = vec![0; usize::from_be_bytes(packet_size_buffer)];
    stream.read_exact(&mut packet_buffer).await.unwrap();

    Ok(json::parse(std::str::from_utf8(&packet_buffer).unwrap()).unwrap())
}

pub async fn send(stream: &mut TcpStream, obj: JsonValue) -> Result<(), Error> {
    let string = obj.dump();

    stream.write_all(&string.len().to_be_bytes()).await.unwrap();
    stream.write_all(string.as_bytes()).await.unwrap();

    Ok(())
}
