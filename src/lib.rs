pub enum Encryption { None, AES256 }

pub struct ProtocolCodec { encryption: Encryption }

impl ProtocolCodec {
    pub fn new(encryption: Encryption) -> Self { Self { encryption } }
    
    pub fn encode(&self, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut frame = vec![0u8; 4 + data.len()];
        frame[0..4].copy_from_slice(&(data.len() as u32).to_be_bytes());
        frame[4..].copy_from_slice(data);
        Ok(frame)
    }
    
    pub fn decode(&self, frame: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        if frame.len() < 4 { return Err("frame too short".into()); }
        let len = u32::from_be_bytes([frame[0], frame[1], frame[2], frame[3]]) as usize;
        Ok(frame[4..4+len].to_vec())
    }
}
