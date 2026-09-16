use cpal::Stream;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use ringbuf::{HeapRb, HeapCons};
use ringbuf::traits::{Producer, Split};


pub fn start_input_stream(ring_capacity: usize) -> (Stream, HeapCons<f32>) {
    let host = cpal::default_host();

    let device = host
        .default_input_device()
        .expect("Error: No Microphone found.");

    println!("Microphone: {}", device.name().unwrap());

    let config = device
        .default_input_config()
        .expect("No input configuration.");

    println!("Configuration: {:?}", config);
    


    let ringbuf = HeapRb::<f32>::new(ring_capacity);
    let (mut producer, mut consumer) = ringbuf.split();


    let stream = device
        .build_input_stream(
            &config.into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                // Write Audio Input stream to ringbuffer
                let pushed = producer.push_slice(data);
                if pushed < data.len() {
                    eprintln!("Rinbuffer full! {} Samples list.", data.len() - pushed);
                }
            },
            move |err| {
                eprintln!("Error in Audio Stream: {}", err);
            },
            None, // Timeout, None = no timeout
        )
        .expect("Failed to initialize stream.");

    stream.play().expect("Failed to start stream.");

    (stream, consumer)
}
