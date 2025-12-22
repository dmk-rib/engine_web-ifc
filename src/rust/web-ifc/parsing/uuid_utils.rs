//! UUID utilities for IFC GUIDs.

use uuid::Uuid;

const BASE16_MASK: [i8; 128] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, -1, -1, -1, -1, -1, -1, -1, 10, 11, 12, 13, 14, 15, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 10,
    11, 12, 13, 14, 15, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1,
];

const BASE64_MASK: [i8; 128] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 63, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, -1, -1, -1, -1, -1, -1, -1, 10, 11, 12, 13, 14, 15, 16, 17, 18,
    19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, -1, -1, -1, -1, 62, -1, 36,
    37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60,
    61, -1, -1, -1, -1, -1,
];

const BASE16_CHARS: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];

const BASE64_CHARS: [char; 64] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z', '_', '$',
];

pub fn generate_string_uuid() -> String {
    Uuid::new_v4().to_string()
}

pub fn expand_ifc_guid(guid: &str) -> String {
    let guid_bytes = guid.as_bytes();
    let mut temp = [0u8; 34];
    let mut ii = 0usize;

    let len = guid_bytes.len().min(22);
    let mut i = 0usize;
    while i + 1 < len {
        let n = (BASE64_MASK[guid_bytes[i] as usize] as i16) << 6
            | (BASE64_MASK[guid_bytes[i + 1] as usize] as i16);
        let t = n / 16;
        temp[ii + 2] = BASE16_CHARS[(n % 16) as usize] as u8;
        temp[ii + 1] = BASE16_CHARS[(t % 16) as usize] as u8;
        temp[ii] = BASE16_CHARS[(t / 16) as usize] as u8;
        ii += 3;
        i += 2;
    }

    let mut result = String::with_capacity(36);
    for idx in 1..36 {
        if idx == 9 || idx == 13 || idx == 17 || idx == 21 {
            result.push('-');
        }
        result.push(temp[idx] as char);
    }
    result
}

pub fn compress_ifc_guid(guid: &str) -> String {
    let mut temp = String::from("0");
    for c in guid.chars() {
        if c != '-' {
            temp.push(c);
        }
    }

    let mut result = [0u8; 23];
    let bytes = temp.as_bytes();
    let mut oi = 0usize;
    let mut i = 0usize;
    while i + 2 < 32 {
        let mut n = (BASE16_MASK[bytes[i] as usize] as i16) << 8;
        n += (BASE16_MASK[bytes[i + 1] as usize] as i16) << 4;
        n += BASE16_MASK[bytes[i + 2] as usize] as i16;
        result[oi + 1] = BASE64_CHARS[(n % 64) as usize] as u8;
        result[oi] = BASE64_CHARS[(n / 64) as usize] as u8;
        oi += 2;
        i += 3;
    }
    String::from_utf8_lossy(&result[..22]).to_string()
}
