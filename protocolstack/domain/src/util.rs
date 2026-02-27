use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use log::debug;
fn get_hex_rep(byte_array: &[u8]) -> Vec<String> {
    byte_array
        .iter()
        .enumerate()
        .map(|(i, val)| {
            if i == 7 {
                format!("{:02x} ", val)
            } else {
                format!("{:02x}", val)
            }
        })
        .collect()
}

fn get_ascii_rep(byte_array: &[u8]) -> Vec<String> {
    byte_array
        .iter()
        .map(|num| {
            if *num >= 32 && *num <= 126 {
                (*num as char).to_string()
            } else {
                '.'.to_string()
            }
        })
        .collect()
}

fn hexdump(byte_array: &[u8]) -> String {
    let dumped_str: Vec<String> = byte_array
        .chunks(16)
        .enumerate()
        .map(|(i, val)| {
            let hex: String = get_hex_rep(val).join(" ");
            let ascii: String = get_ascii_rep(val).join("");
            format!("{:08x}  {}  |{}|", i * 16, hex, ascii)
        })
        .collect();
    dumped_str.join("\n")
}

pub fn debugdump(data: &[u8]) {
    debug!("\n{}", hexdump(data));
}

#[cfg(test)]
mod tests {
    use super::hexdump;
    use alloc::vec;
    #[test]
    fn hexdump_test() {
        let test_data = vec![
            0x45, 0x00, 0x00, 0x30, 0x00, 0x80, 0x00, 0x00, 0xff, 0x01, 0xbd, 0x4a, 0x7f, 0x00,
            0x00, 0x01, 0x7f, 0x00, 0x00, 0x01, 0x08, 0x00, 0x35, 0x64, 0x00, 0x80, 0x00, 0x01,
            0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x21, 0x40, 0x23, 0x24,
            0x25, 0x5e, 0x26, 0x2a, 0x28, 0x29,
        ];
        let result =
            "00000000  45 00 00 30 00 80 00 00  ff 01 bd 4a 7f 00 00 01  |E..0.......J....|
00000010  7f 00 00 01 08 00 35 64  00 80 00 01 31 32 33 34  |......5d....1234|
00000020  35 36 37 38 39 30 21 40  23 24 25 5e 26 2a 28 29  |567890!@#$%^&*()|";
        assert_eq!(result, hexdump(&test_data));
    }
}
