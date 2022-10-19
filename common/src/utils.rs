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

pub fn blit_buffer_to_buffer(
    dest: &mut [u8],
    dest_width: i32,
    dest_height: i32,
    source: &[u8],
    source_width: i32,
    source_height: i32,
    offset_x: i32,
    offset_y: i32,
) {
    let mut source_offset_x = 0;
    if offset_x < 0 {
        source_offset_x = offset_x.abs();
    }
    let mut image_length_x = source_width - source_offset_x;
    image_length_x = image_length_x.min(dest_width - offset_x);
    if image_length_x <= 0 {
        return;
    }

    let mut source_offset_y = 0;
    if offset_y < 0 {
        source_offset_y = offset_y.abs();
    }
    let mut image_length_y = source_height - source_offset_y;
    image_length_y = image_length_y.min(dest_height - offset_y);
    if image_length_y <= 0 {
        return;
    }

    let slice_length = image_length_x as usize * 4;

    let dest_offset_x = offset_x.max(0);
    let dest_offset_y = offset_y.max(0);

    for y in 0..image_length_y {
        let dest_index = calculate_index(dest_offset_x, y + dest_offset_y, dest_width) * 4;
        let source_index = calculate_index(source_offset_x, y + source_offset_y, source_width) * 4;
        dest[dest_index..dest_index + slice_length]
            .copy_from_slice(&source[source_index..source_index + slice_length])
    }
}
