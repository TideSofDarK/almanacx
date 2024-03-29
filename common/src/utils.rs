use cgmath::Vector3;

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

// Rect is (x, y, width, height)
pub const fn is_inside(point: (i32, i32), rect: (i32, i32, i32, i32)) -> bool {
    point.0 >= rect.0
        && point.0 <= rect.0 + rect.2
        && point.1 >= rect.1
        && point.1 <= rect.1 + rect.3
}

pub fn calculate_index(x: i32, y: i32, width: i32) -> usize {
    (y * width + x) as usize
}

pub fn color_from_vec(color: Vector3<f32>) -> u16 {
    color_from_tuple((
        (color.x * 31.0) as u16,
        (color.y * 31.0) as u16,
        (color.z * 31.0) as u16,
    ))
}

pub const fn color_from_tuple(color: (u16, u16, u16)) -> u16 {
    (color.0 << 10) + (color.1 << 5) + color.2
}
