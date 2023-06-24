pub const STEP_SIZE_TABLE: [i32; 90] = [
    7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 19, 21, 23, 25, 28, 31, 34, 37, 41, 45, 50, 55, 60, 66,
    73, 80, 88, 97, 107, 118, 130, 143, 157, 173, 190, 209, 230, 253, 279, 307, 337, 371, 408, 449,
    494, 544, 598, 658, 724, 796, 876, 963, 1060, 1166, 1282, 1411, 1552, 1707, 1878, 2066, 2272,
    2499, 2749, 3024, 3327, 3660, 4026, 4428, 4871, 5358, 5894, 6484, 7132, 7845, 8630, 9493,
    10442, 11487, 12635, 13899, 15289, 16818, 18500, 20350, 22385, 24623, 27086, 29794, 32767, 0,
];

pub const IMA_INDEX_TABLE: [i32; 16] = [-1, -1, -1, -1, 2, 4, 6, 8, -1, -1, -1, -1, 2, 4, 6, 8];

pub fn decompress_adpcm(input_bytes: &[u8]) -> Vec<i16> {
    let mut output_buffer = vec![0i16; input_bytes.len() * 2];
    let mut predictors: [i32; 2] = [0, 0];
    let mut step_indices: [i32; 2] = [0, 0];
    for i in 0..input_bytes.len() {
        let mut sample_nibble = input_bytes[i];
        let mut step = STEP_SIZE_TABLE[step_indices[0] as usize];
        step_indices[0] += IMA_INDEX_TABLE[(sample_nibble & 7) as usize];
        step_indices[0] = step_indices[0].clamp(0, 88);
        let mut delta = step >> 3;
        if (sample_nibble & 4) != 0 {
            delta += step;
        }
        if (sample_nibble & 2) != 0 {
            delta += step >> 1;
        }
        if (sample_nibble & 1) != 0 {
            delta += step >> 2;
        }
        if (sample_nibble & 8) != 0 {
            delta = -delta;
        }
        predictors[0] += delta;
        predictors[0] = predictors[0].clamp(i16::MIN as i32, i16::MAX as i32);
        output_buffer[2 * i] = predictors[0] as i16;

        step = STEP_SIZE_TABLE[step_indices[1] as usize];
        sample_nibble = input_bytes[i] >> 4;

        step_indices[1] += IMA_INDEX_TABLE[(sample_nibble & 7) as usize];
        step_indices[1] = step_indices[1].clamp(0, 88);
        delta = step >> 3;
        if (sample_nibble & 4) != 0 {
            delta += step;
        }
        if (sample_nibble & 2) != 0 {
            delta += step >> 1;
        }
        if (sample_nibble & 1) != 0 {
            delta += step >> 2;
        }
        if (sample_nibble & 8) != 0 {
            delta = -delta;
        }
        predictors[1] += delta;
        predictors[1] = predictors[1].clamp(i16::MIN as i32, i16::MAX as i32);
        output_buffer[2 * i + 1] = predictors[1] as i16;
    }
    output_buffer
}
