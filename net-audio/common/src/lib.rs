pub struct AudioPacket {
    pub seq: u32,
    pub timestamp: u64,
    pub samples: Vec<f32>,
}
impl AudioPacket {

    pub fn to_bytes (&self) -> Vec<u8>  {
        let mut bytes  = Vec::with_capacity(4 + 8 + self.samples.len()); // 4 for  seq: u32 and 8 for timestamp: u64
        bytes.extend_from_slice(&self.seq.to_be_bytes());
        bytes.extend_from_slice(&self.timestamp.to_be_bytes());

        for sample in &self.samples {
            bytes.extend_from_slice(&sample.to_be_bytes());
        }

        bytes
    }


    pub fn from_bytes (bytes: Vec<u8>) -> Self {
        todo!()
    }


}
