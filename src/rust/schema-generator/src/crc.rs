pub fn make_crc_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    for n in 0..256u32 {
        let mut c = n;
        for _ in 0..8 {
            if c & 1 == 1 {
                c = 0xEDB88320 ^ (c >> 1);
            } else {
                c >>= 1;
            }
        }
        table[n as usize] = c;
    }
    table
}

pub fn crc32(value: &str, table: &[u32; 256]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for byte in value.bytes() {
        let index = (crc ^ u32::from(byte)) & 0xFF;
        crc = (crc >> 8) ^ table[index as usize];
    }
    crc ^ 0xFFFF_FFFFu32
}
