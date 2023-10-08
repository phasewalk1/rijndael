use crate::constants::{RCON, S_BOX};

pub(crate) fn sub_word(word: u32) -> u32 {
    let mut result = 0;
    log::debug!("sub_word with word = 0x{:x}", word);
    for i in 0..4 {
        log::debug!("sub_word with i = {}", i);
        let byte = (word >> (8 * i)) as u8;
        log::debug!(
            "byte after bit_shift_right by {} bits: 0x{:x}",
            (8 * i),
            byte
        );
        let subbed = s_box_substitution(byte);
        log::debug!("byte after s_box_substitution: 0x{:x}", subbed);
        result |= (subbed as u32) << (8 * i);
        log::debug!(
            "result after bit_shift_right {} bits: 0x{:x}",
            (8 * i),
            result
        );
    }
    return result;
}

pub fn rot_word(word: u32) -> u32 {
    return (word << 8) | (word >> 24);
}

pub fn s_box_substitution(byte: u8) -> u8 {
    let row = (byte >> 4) as usize;
    let col = (byte & 0x0F) as usize;
    log::debug!(
        "s_box_sub: byte = 0x{:x}, row = 0x{:x}, col = 0x{:x}",
        byte,
        row,
        col
    );
    log::debug!("s_box[0x{:x}][0x{:x}] = 0x{:x}", row, col, S_BOX[row][col]);
    return S_BOX[row][col];
}

pub fn aes_key_schedule(key: &[u8; 16]) -> [u32; 44] {
    let mut w: [u32; 44] = [0; 44];

    for i in 0..4 {
        log::debug!("i = {}", i);
        w[i] = u32::from_be_bytes([key[4 * i], key[4 * i + 1], key[4 * i + 2], key[4 * i + 3]]);
    }

    for i in 4..44 {
        log::debug!("i = {}", i);
        let mut temp = w[i - 1];
        if i % 4 == 0 {
            temp = sub_word(rot_word(temp)) ^ (RCON[i / 4 - 1] as u32) << 24;
        }
        w[i] = w[i - 4] ^ temp;
        log::debug!("w[i] = {}", w[i]);
    }

    return w;
}

#[cfg(test)]
mod scheduler_test {
    use super::*;

    #[test]
    fn test_s_box_sub() {
        pretty_env_logger::try_init().ok();

        assert_eq!(s_box_substitution(0xdc), 0x86);
    }

    #[test]
    fn test_rot_word() {}

    #[test]
    fn test_sub_word() {
        pretty_env_logger::try_init().ok();

        assert_eq!(sub_word(0x2b7e1516), 0x63cab704);
    }

    #[test]
    fn test_key_scheduler() {
        pretty_env_logger::try_init().ok();

        let key: [u8; 16] = [
            0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x89, 0x09, 0xcf,
            0x4f, 0x3c,
        ];
        let expanded_key = aes_key_schedule(&key);

        let expected_expansion: [u32; 44] = [
            0x2b7e1516, 0x28aed2a6, 0xabf71588, 0x09cf4f3c, 0xa0fafe17, 0x88542cb1, 0x23a33939,
            0x2a6c7605, 0xf2c295f2, 0x7a96b943, 0x5935807a, 0x7359f67f, 0x3d80477d, 0x4716fe3e,
            0x1e237e44, 0x6d7a883b, 0xef44a541, 0xa8525b7f, 0xb671253b, 0xdb0bad00, 0xd4d1c6f8,
            0x7c839d87, 0xcaf2b8bc, 0x11f915bc, 0x6d88a37a, 0x110b3efd, 0xdbf98641, 0xca0093fd,
            0x4e54f70e, 0x5f5fc9f3, 0x84a64fb2, 0x4ea6dc4f, 0xead27321, 0xb58dbad2, 0x312bf560,
            0x7f8d292f, 0xac7766f3, 0x19fadc21, 0x28d12941, 0x575c006e, 0xd014f9a8, 0xc9ee2589,
            0xe13f0cc8, 0xb6630ca6,
        ];

        for i in 0..44 {
            log::debug!(
                "asserting byte {} of expanded_key matches byte {} of expected_expansion",
                i,
                i
            );
            assert_eq!(expanded_key[i], expected_expansion[i]);
        }
    }
}
