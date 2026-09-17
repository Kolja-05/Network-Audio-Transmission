use common::AudioPacket;



// displays a volum diagramm in the terminal
pub fn process(packet: &AudioPacket) {
    let sum_squares: f32 = packet.samples.iter().map(|s| s * s).sum();
    let rms = (sum_squares / packet.samples.len() as f32).sqrt();

    let bar_len = (rms * 200.0) as usize;
    let bar: String = "#".repeat(bar_len.min(50));

    println!("{:<50} {:.4}  (seq={})", bar, rms, packet.seq);
}
