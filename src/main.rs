use std::fs::File;
use std::io::{self, Write};
use std::net::UdpSocket;

use config::Config;

mod config;
mod pdu;

fn main() -> io::Result<()> {
    let config_json = std::fs::read_to_string("config.json")?;
    let config: Config = serde_json::from_str(&config_json)?;

    let socket = UdpSocket::bind(format!("127.0.0.1:{}", config.port()))?;
    let mut seq_num = 0; // 从0开始接收分组
    let mut buffer = [0u8; 2048];
    loop {
        let (n, sender) = socket.recv_from(buf)?;
    }

    let mut file = File::create("received_data.txt")?;

    loop {
        let (n, _) = socket.recv_from(&mut buffer)?;
        file.write_all(&buffer[..n])?;
    }
}