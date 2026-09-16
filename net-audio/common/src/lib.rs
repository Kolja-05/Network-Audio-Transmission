pub struct AudioPacket {
    pub seq: u32,
    pub timestamp: u64,
    pub samples: Vec<f32>,
}
