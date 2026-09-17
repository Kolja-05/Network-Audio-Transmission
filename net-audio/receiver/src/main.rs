use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

mod network;
mod processor;


fn main() {
    println!("Starting receiver!");

    let packet_size_samples = 480;

    let max_packet_size = 4 + 8 + packet_size_samples * 4;

    let mut net = network::Network::new(
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 9000)),
        max_packet_size);

    loop {
        let packet = net.recv_packet();
        processor::process(&packet);
    }


}
