use crc_any::CRCu16;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::net::UdpSocket;

use ack::Ack;
use config::Config;

mod ack;
mod config;
mod util;

const I32_SIZE: usize = std::mem::size_of::<i32>();
const USIZE_SIZE: usize = std::mem::size_of::<usize>();
const U16_SIZE: usize = std::mem::size_of::<u16>();

fn main() -> io::Result<()> {
    let config_json = std::fs::read_to_string("config.json")?;
    let mut config: Config = serde_json::from_str(&config_json)?;

    let mut buffer_size = I32_SIZE + USIZE_SIZE + config.data_size() + U16_SIZE;

    let socket = UdpSocket::bind(format!("127.0.0.1:{}", config.port()))?;
    let mut seq_num = 0; // 从0开始接收分组
                         // Vec的大小是发送的序列号 + 数据字节数 + 数据 + 校验和
    let mut buffer = vec![0; buffer_size];
    let mut crc_ccitt = CRCu16::crc16ccitt_false(); // 用于计算CRC校验和
    crc_ccitt.reset();

    let mut frame_cnt = 0; // 一共需要接受多少帧

    loop {
        if seq_num > frame_cnt {
            break;
        }
        println!("等待帧序列号为{}的分组...", seq_num);
        let (n, sender) = socket.recv_from(&mut buffer)?;
        println!("接收到来自{}, 大小为{}字节的分组", sender, n);
        let recv_seq_num = i32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
        if recv_seq_num != seq_num {
            println!(
                "不符合期待的帧序列号(期待的: {}, 实际收到的: {}), 丢弃",
                seq_num, recv_seq_num
            );
            if seq_num == 0 {
                continue;
            }
            let ack_data = bincode::serialize(&Ack::new(seq_num - 1)).unwrap();
            socket.send_to(&ack_data, sender).expect("发送ACK失败");
            continue;
        }

        if recv_seq_num == 0 {
            // 第0帧包含了帧数、文件名信息
            let (filename, _frame_cnt) = util::get_filename_and_frame_cnt(&buffer);
            buffer_size = I32_SIZE + USIZE_SIZE + util::get_data_size(&buffer) + U16_SIZE;
            buffer.resize(buffer_size, 0u8);
            frame_cnt = _frame_cnt;
            config.set_saved_filename(filename);
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(config.saved_filename())?;
            // 清空文件
            file.write_all(b"")?;
            println!(
                "准备接收文件...\n文件名: {}, 分组数: {}",
                config.saved_filename(),
                frame_cnt
            );
            socket
                .send_to(&seq_num.to_le_bytes(), sender)
                .expect(format!("发送ACK{}失败", seq_num).as_str());
            seq_num += 1;
            continue;
        }
        let recv_checksum =
            u16::from_le_bytes([buffer[buffer.len() - 2], buffer[buffer.len() - 1]]);
        crc_ccitt.digest(&buffer[..(buffer_size - U16_SIZE)]); // 计算校验码
        let crc = crc_ccitt.get_crc(); // 保存校验码
        crc_ccitt.reset(); // 重置crc状态

        if recv_checksum != crc {
            println!(
                "校验码出错(期待的: {}, 实际收到的: {}), 丢弃",
                crc, recv_checksum
            );
            if seq_num == 0 {
                continue;
            }
            let ack_data = bincode::serialize(&Ack::new(seq_num - 1)).unwrap();
            socket.send_to(&ack_data, sender).expect("发送ACK失败");
            continue;
        }

        let data_size = util::get_data_size(&buffer); // 获取当前帧的有效数据的字节数

        // 将收到的数据写入文件
        println!("正在将分组数据写入{}...", config.saved_filename());
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(config.saved_filename())?;
        let data = &buffer[(I32_SIZE + USIZE_SIZE)..(I32_SIZE + USIZE_SIZE + data_size)];
        file.write_all(data)?;
        println!("写入成功!");
        let ack_data = seq_num.to_le_bytes();
        socket
            .send_to(&ack_data, sender)
            .expect(format!("发送ACK{}失败", seq_num).as_str());
        seq_num += 1;
    }
    println!("文件已保存到{}", config.saved_filename());
    Ok(())
}
