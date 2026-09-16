mod sampler;
mod packetizer;
mod network;

fn main() {
    let sample_rate = 48000;
    let ring_capacity = sample_rate;

    let (_stream, consumer) = sampler::start_input_stream(ring_capacity);
    
    loop {
        let packet = packetizer::run(consumer, 480);
        network::send(packet);
    }
}
