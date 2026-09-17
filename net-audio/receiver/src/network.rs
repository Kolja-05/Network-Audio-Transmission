use std::net::{SocketAddr, UdpSocket};
use common::AudioPacket;



pub struct Network {
    socket: UdpSocket,
    recv_buf: Vec<u8>,
    last_seq: Option<u32>,
}


impl Network {

    pub fn new (bind_addr: SocketAddr, max_packet_size: usize) -> Self {
        let socket = UdpSocket::bind(bind_addr).expect("Failed to bind.");
        Network {
            socket: socket,
            recv_buf: vec![0u8; max_packet_size],
            last_seq: None,
        }
    }
    pub fn recv_packet(&mut self, ) -> common::AudioPacket {
        let (number_of_bytes, src_addr) = self.socket.recv_from(&mut self.recv_buf).expect("recv_from failed");
        let filled_buf = &self.recv_buf[..number_of_bytes];

        AudioPacket::from_bytes(filled_buf)
    }
}

