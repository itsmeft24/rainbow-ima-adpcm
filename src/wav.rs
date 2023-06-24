use binrw::*;

#[derive(BinRead, BinWrite, Debug, Default, Clone)]
pub struct FmtChunk {
    pub magic: [u8; 4],
    pub len: u32,
    pub audio_format: u16,
    pub channel_count: u16,
    pub sampling_rate: u32,
    pub bytes_per_second: u32,
    pub bytes_per_sample: u16,
    pub bits_per_sample: u16,

    #[br(count = len - 16)]
    pub extra_data: Vec<u8>,
}

#[derive(BinRead, BinWrite, Debug, Default, Clone)]
pub struct DataChunk {
    pub magic: [u8; 4],
    pub len: u32,

    #[br(count = len)]
    pub data: Vec<u8>,
}

#[derive(BinRead, BinWrite, Debug, Default, Clone)]
pub struct WavFile {
    pub riff: u32,
    pub size_remaining: u32,
    pub wave: u32,
    pub fmt: FmtChunk,
    pub data: DataChunk,
}
