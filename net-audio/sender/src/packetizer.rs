use ringbuf::HeapCons;
use ringbuf::traits::Consumer;

use common::AudioPacket;



pub fn run(consumer: &mut HeapCons<f32>, packet_size: usize) -> AudioPacket {
    let mut buf = vec![0.0f32; packet_size];

    loop {
        let len  = consumer.pop_slice(&mut buf);
        if len == packet_size {
            println!("Packet ready for transmission. {} samples", packet_size);


            let packet = AudioPacket {
                seq: 0, // Sequence number will be set by network.rs
                timestamp: 0, //timestamp will be set by network.rs
                samples: buf.clone(),
            };

            return packet;
        }

        else {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }
}
