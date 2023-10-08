use crate::constants::{RCON, S_BOX};

// 128-bit (16-byte) block
pub type LayerInput = [[u8; 4]; 4];

pub struct SubstitutionLayer {
    round_key: [u32; 44],
    state: LayerInput,
}

impl SubstitutionLayer {
    pub fn foward(&mut self) {
        // round_key XOR {S_1,...,S_8}
        // perm_layer.foward()
    }
}

pub struct PermuationLayer;

pub fn byte_sub_mut(state: &mut LayerInput) {
    for i in 0..4 {
        for j in 0..4 {
            let byte = state[i][j];
            let row = (byte >> 4) as usize;
            let col = (byte & 0x0F) as usize;
            let lookup = S_BOX[row][col];
            state[i][j] = lookup;
        }
    }
    
}

impl PermuationLayer {
    pub fn foward(&self, state: &mut LayerInput) {
        byte_sub_mut(state);
        self.shift_rows(state);
        self.mix_columns(state);
    }

    pub fn forward_final(&self, state: &mut LayerInput) -> LayerInput {
        byte_sub_mut(state);
        self.shift_rows(state);
        return state.clone();
    }


    pub fn shift_rows(&self, state: &mut LayerInput) {
        let mut temp: [u8; 4] = [0; 4];

        // Shift row 1 by 1 to the right
        temp[0] = state[1][3];
        state[1][3] = state[1][2];
        state[1][2] = state[1][1];
        state[1][1] = state[1][0];
        state[1][0] = temp[0];

        // Shift row 2 by 2 to the right
        temp[0] = state[2][2];
        temp[1] = state[2][3];
        state[2][2] = state[2][0];
        state[2][3] = state[2][1];
        state[2][0] = temp[0];
        state[2][1] = temp[1];

        // Shift row 3 by 3 to the right
        temp[0] = state[3][0];
        temp[1] = state[3][1];
        temp[2] = state[3][2];
        state[3][0] = state[3][3];
        state[3][1] = temp[0];
        state[3][2] = temp[1];
        state[3][3] = temp[2];

        log::debug!("after shift_rows: {:?}", state);
    }

    fn galois_field_mul(&self, mut a: u8, mut b: u8) -> u8 {
        let mut p: u8 = 0;
        let mut hi_bit_set: u8;

        for _ in 0..8 {
            if (b & 1) != 0 {
                p ^= a;
            }
            hi_bit_set = a & 0x80;
            a <<= 1;
            if hi_bit_set != 0 {
                a ^= 0x1b;
            }
            b >>= 1;
        }
        return p;
    }

    pub fn mix_columns(&self, state: &mut LayerInput) {
        let mix_mat = [
            [0x02, 0x03, 0x01, 0x01],
            [0x01, 0x02, 0x03, 0x01],
            [0x01, 0x01, 0x02, 0x03],
            [0x03, 0x01, 0x01, 0x02],
        ];

        let mut result: LayerInput = [[0; 4]; 4];

        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result[j][i] ^= self.galois_field_mul(state[k][i], mix_mat[j][k]);
                }
            }
        }

        *state = result;
    }
}

#[cfg(test)]
mod layer_test {
    use super::*;

    #[test]
    fn test_byte_sub_mut() {
        let mut state: LayerInput = [
            [0x19, 0xa0, 0x9a, 0xe9],
            [0x3d, 0xf4, 0xc6, 0xf8],
            [0xe3, 0xe2, 0x8d, 0x48],
            [0xbe, 0x2b, 0x2a, 0x08],
        ];
        byte_sub_mut(&mut state);
        assert_eq!(
            state,
            [
            [0xd4, 0xe0, 0xb8, 0x1e],
            [0x27, 0xbf, 0xb4, 0x41],
            [0x11, 0x98, 0x5d, 0x52],
            [0xae, 0xf1, 0xe5, 0x30],
            ]
        );
    }

    #[test]
    fn test_shift_rows() {
        let mut state: LayerInput = [
            [0xd4, 0xe0, 0xb8, 0x1e],
            [0x27, 0xbf, 0xb4, 0x41],
            [0x11, 0x98, 0x5d, 0x52],
            [0xae, 0xf1, 0xe5, 0x30],
        ];

        println!("original sate {:?}", state);
        let perm_layer = PermuationLayer {};
        perm_layer.shift_rows(&mut state);
        assert_eq!(
            state,
            [
            [0xd4, 0xe0, 0xb8, 0x1e],
            [0x41, 0x27, 0xbf, 0xb4],
            [0x5d, 0x52, 0x11, 0x98],
            [0x30, 0xae, 0xf1, 0xe5],
            ]
        );
    }

    #[test]
    fn test_mix_columns() {
        pretty_env_logger::try_init().ok();

        let mut state: LayerInput = [
            [0xdb, 0xf2, 0x01, 0x2d],
            [0x13, 0x0a, 0x01, 0x26],
            [0x53, 0x22, 0x01, 0x31],
            [0x45, 0x5c, 0x01, 0x4c],
        ];

        log::debug!("testing mix_columns with state: {:?}", state);

        let expected_after_mix_columns: LayerInput = [
            [0x8e, 0x9f, 0x01, 0x4d],
            [0x4d, 0xdc, 0x01, 0x7e],
            [0xa1, 0x58, 0x01, 0xbd],
            [0xbc, 0x9d, 0x01, 0xf8],
        ];

        let perm_layer = PermuationLayer {};
        log::debug!("applying permuation-layer.mix_columns");
        perm_layer.mix_columns(&mut state);
        log::debug!("after mix_columns: {:?}", state);
        assert_eq!(state, expected_after_mix_columns);
        log::info!("mix_columns smoke test passed");
    }
}
