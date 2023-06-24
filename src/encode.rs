use crate::decode::{IMA_INDEX_TABLE, STEP_SIZE_TABLE};

pub fn compress_adpcm(input_samples: &[i16]) -> Vec<u8> {
    let mut output_buffer = Vec::with_capacity(input_samples.len() / 2);
    let mut predictors: [i32; 2] = [0, 0];
    let mut step_indices: [i32; 2] = [0, 0];

    for i in 0..(input_samples.len() / 2) {
        let left = input_samples[2 * i];
        let right = input_samples[2 * i + 1];

        let mut delta = [left as i32 - predictors[0], right as i32 - predictors[1]];
        let mut sample_nibble: u8 = 0;

        for j in 0..2 {
            let mut step = STEP_SIZE_TABLE[step_indices[j] as usize];
            let mut nibble_delta: u8 = 0;

            if delta[j] < 0 {
                nibble_delta = 8;
                delta[j] = -delta[j];
            }

            if delta[j] >= step {
                nibble_delta |= 4;
                delta[j] -= step;
            }

            step >>= 1;

            if delta[j] >= step {
                nibble_delta |= 2;
                delta[j] -= step;
            }

            step >>= 1;

            if delta[j] >= step {
                nibble_delta |= 1;
            }

            step_indices[j] += IMA_INDEX_TABLE[nibble_delta as usize];
            step_indices[j] = step_indices[j].clamp(0, 88);

            sample_nibble |= nibble_delta << (4 * j);
        }

        predictors = [left as i32, right as i32];
        output_buffer.push(sample_nibble);
    }

    output_buffer
}
