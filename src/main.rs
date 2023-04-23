use std::fs::OpenOptions;
use std::io::{self, Write};
use std::net::UdpSocket;
use crc_any::CRCu16;

use config::Config;
use ack::Ack;

mod config;
mod pdu;
mod ack;

fn main() -> io::Result<()> {
    let config_json = std::fs::read_to_string("config.json")?;
    let config: Config = serde_json::from_str(&config_json)?;

    let socket = UdpSocket::bind(format!("127.0.0.1:{}", config.port()))?;
    let mut seq_num = 0; // 从0开始接收分组
    // Vec的大小是发送的序列号 + 数据字节数 + 数据 + 校验和
    let mut buffer = vec![0; std::mem::size_of::<i32>() * 2 + config.data_size() + std::mem::size_of::<i16>()];
    let mut crc_ccitt = CRCu16::crc16ccitt_false(); // 用于计算CRC校验和
    loop {
        println!("等待帧序列号为{}的分组...", seq_num);
        let (n, sender) = socket.recv_from(&mut buffer)?;
        println!("接收到来自{}, 大小为{}字节的分组", sender, n);
        let recv_seq_num = i32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
        if recv_seq_num != seq_num {
            println!("不符合期待的帧序列号(期待的: {}, 实际收到的: {}), 丢弃", seq_num, recv_seq_num);
            if seq_num == 0 {
                continue;
            }
            let ack_data = bincode::serialize(&Ack::new(seq_num - 1)).unwrap();
            socket.send_to(&ack_data, sender).expect("发送ACK失败");
            continue;
        }
        let recv_checksum = u16::from_le_bytes([buffer[buffer.len() - 2], buffer[buffer.len() - 1]]);
        let data_to_check = &buffer[..(std::mem::size_of::<i32>() * 2 + config.data_size())]; // 要计算校验码的部分
        crc_ccitt.digest(data_to_check); // 计算校验码
        if recv_checksum != crc_ccitt.get_crc() {
            println!("校验码出错(期待的: {}, 实际收到的: {}), 丢弃", crc_ccitt.get_crc(), recv_checksum);
            if seq_num == 0 {
                continue;
            }
            let ack_data = bincode::serialize(&Ack::new(seq_num - 1)).unwrap();
            socket.send_to(&ack_data, sender).expect("发送ACK失败");
            continue;
        }
        // 将收到的数据写入文件
        println!("正在将分组数据写入{}...", config.saved_filename());
        let mut file = OpenOptions::new().append(true).create(true).open(config.saved_filename())?;
        let data = &buffer[(std::mem::size_of::<i32>() * 2)..(std::mem::size_of::<i32>() * 2 + config.data_size())];
        file.write_all(data)?;
        println!("写入成功!");
        let ack_data = bincode::serialize(&Ack::new(seq_num)).unwrap();
        socket.send_to(&ack_data, sender).expect("发送ACK失败");
        seq_num += 1;
    }
}