use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

mod sampler;
mod packetizer;
mod network;

fn main() {
    let sample_rate = 48000;
    let ring_capacity = sample_rate;

    let (_stream, mut consumer) = sampler::start_input_stream(ring_capacity);

    let mut net = network::Network::new(
            SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0)),
            SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9000)));
    loop {
        let packet = packetizer::run(&mut consumer, 480);
        net.send_packet(packet);
    }
}
