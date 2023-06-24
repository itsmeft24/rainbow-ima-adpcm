use std::io::Write;

use binrw::{BinRead, BinWrite, BinWriterExt};

mod decode;
mod encode;
mod wav;

fn main() {
    let mut pcm_file = std::fs::File::open("./data/04 Boy In Luv.wav").unwrap();
    let pcm = wav::WavFile::read_le(&mut pcm_file).unwrap();

    let mut cursor = std::io::Cursor::new(&pcm.data.data);
    let input_samples = Vec::<i16>::read_options(
        &mut cursor,
        binrw::Endian::Little,
        binrw::VecArgs::builder()
            .count(pcm.data.data.len() / 2 as usize)
            .finalize(),
    )
    .unwrap();

    let encoded = encode::compress_adpcm(&input_samples);
    let encoded_len = encoded.len() as u32;

    let data = wav::DataChunk {
        magic: [0x64, 0x61, 0x74, 0x61],
        len: encoded_len,
        data: encoded,
    };

    let wav_file = wav::WavFile {
        riff: 0x46464952,
        size_remaining: encoded_len + 40,
        wave: 0x45564157,
        fmt: wav::FmtChunk {
            magic: [0x66, 0x6D, 0x74, 0x20],
            len: 20,
            audio_format: 2,
            channel_count: 2,
            sampling_rate: 44100,
            bytes_per_second: 176400,
            bytes_per_sample: 4,
            bits_per_sample: 16,
            extra_data: vec![0x00, 0x00, 0x40, 0x00],
        },
        data,
    };
    let mut out = std::fs::File::create("./data/Boy_In_Luv_CARS.wav").unwrap();
    wav_file.write_le(&mut out).unwrap();
}
