use std::{net::{SocketAddr, UdpSocket}, time::UNIX_EPOCH};

use common::AudioPacket;


pub struct Network {
    socket: UdpSocket,
    target_addr: SocketAddr,
    seq: u32,
}

impl Network {
    pub fn new (bind_addr: SocketAddr, target_addr: SocketAddr) -> Self {
        let socket = UdpSocket::bind(bind_addr).expect("failed to bind");
        Network {
            socket,
            target_addr: target_addr,
            seq: 0,
        }
    }


    pub fn send_packet(&mut self, mut packet: AudioPacket) {
        packet.seq = self.seq;
        packet.timestamp = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let bytes = packet.to_bytes();
        self.socket.send_to(&bytes, &self.target_addr).ok();
    }

}
