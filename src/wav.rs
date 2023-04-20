use std::error::Error;
use std::fs::File;
use std::io::Write;

pub struct WavHeader {
    chunk_id: [u8; 4],
    chunk_size: u32,
    format: [u8; 4],
    sub_chunk1_id: [u8;4],
    sub_chunk1_size: u32,
    audio_format: u16,
    channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    block_align: u16,
    bits_per_sample: u16,
    sub_chunk2_id: [u8;4],
    sub_chunk2_size: u32,
}

impl Default for WavHeader {
    fn default() -> Self {
        WavHeader {
            chunk_id: <[u8; 4]>::try_from("RIFF".as_bytes()).unwrap(),
            chunk_size: 0,
            format: <[u8; 4]>::try_from("WAVE".as_bytes()).unwrap(),
            sub_chunk1_id: <[u8; 4]>::try_from("fmt ".as_bytes()).unwrap(),
            sub_chunk1_size: 16,
            audio_format: 1,
            channels: 1,
            sample_rate: 16000,
            byte_rate: 32000,
            block_align: 2,
            bits_per_sample: 16,
            sub_chunk2_id: <[u8; 4]>::try_from("data".as_bytes()).unwrap(),
            sub_chunk2_size: 0,
        }
    }
}

impl WavHeader {
    pub fn new(channels:u16, sample_rate:u32, data_size: u32) -> Self {
        WavHeader {
            channels,
            chunk_size: data_size + 36,
            sample_rate,
            byte_rate: sample_rate * 2 * channels as u32,
            block_align: 2 * channels,
            sub_chunk2_size: data_size,
            ..WavHeader::default()
        }
    }

    pub fn write(self, mut file: &File) -> WavResult {
        file.write(&self.chunk_id)?;
        file.write(&self.chunk_size.to_le_bytes())?;
        file.write(&self.format)?;
        file.write(&self.sub_chunk1_id)?;
        file.write(&self.sub_chunk1_size.to_le_bytes())?;
        file.write(&self.audio_format.to_le_bytes())?;
        file.write(&self.channels.to_le_bytes())?;
        file.write(&self.sample_rate.to_le_bytes())?;
        file.write(&self.byte_rate.to_le_bytes())?;
        file.write(&self.block_align.to_le_bytes())?;
        file.write(&self.bits_per_sample.to_le_bytes())?;
        file.write(&self.sub_chunk2_id)?;
        file.write(&self.sub_chunk2_size.to_le_bytes())?;
        Ok(())
    }
}

pub type WavResult = Result<(), Box<dyn Error>>;
