pub fn read_u8(buf: &[u8], offset: usize) -> u8 {
    u8::from_le_bytes(
        buf[offset..offset + 1]
            .try_into()
            .expect("slice with incorrect length"),
    )
}

pub fn read_i16(buf: &[u8], offset: usize) -> i16 {
    i16::from_le_bytes(
        buf[offset..offset + 2]
            .try_into()
            .expect("slice with incorrect length"),
    )
}

pub fn read_u16(buf: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes(
        buf[offset..offset + 2]
            .try_into()
            .expect("slice with incorrect length"),
    )
}

pub fn read_u32(buf: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(
        buf[offset..offset + 4]
            .try_into()
            .expect("slice with incorrect length"),
    )
}

pub fn read_str_4bytes(buf: &[u8], offset: usize) -> String {
    String::from_utf8(
        buf[offset..offset + 4]
            .try_into()
            .expect("slice with incorrect length"),
    )
    .expect("")
    .replace('\0', "")
}

pub fn read_str_8bytes(buf: &[u8], offset: usize) -> String {
    String::from_utf8(
        buf[offset..offset + 8]
            .try_into()
            .expect("slice with incorrect length"),
    )
    .expect("")
    .replace('\0', "")
}

pub fn calculate_index(x: i32, y: i32, width: i32) -> usize {
    (y * width + x) as usize
}
