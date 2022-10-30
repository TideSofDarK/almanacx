use std::{
    fs::File,
    io::{self, ErrorKind, Read, Seek, SeekFrom},
    mem, slice,
};

use crate::{buffer2d::B2DO, utils::color_from_tuple};

const SIGNATURE: u16 = 19778;

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
struct BMPHeader {
    signature: u16,
    file_size: u32,
    reserved1: u16,
    reserved2: u16,
    offset: u32,
    header_size: u32,
    width: i32,
    height: i32,
    planes: u16,
    bits_per_pixel: u16,
    compression: u32,
    size_of_bitmap: u32,
    x_res: i32,
    y_res: i32,
    colors_used: u32,
    colors_important: u32,
    red_mask: u32,
    green_mask: u32,
    blue_mask: u32,
    alpha_mask: u32,
}
const HEADER_SIZE: usize = mem::size_of::<BMPHeader>();

pub fn load_bmp(path: &str) -> B2DO {
    let mut f = File::open(path).expect("[BMP] Error reading file!");
    let mut header: BMPHeader = unsafe { mem::zeroed() };

    unsafe {
        let header_slice = slice::from_raw_parts_mut(&mut header as *mut _ as *mut u8, HEADER_SIZE);
        f.read_exact(header_slice).unwrap();
    }

    if header.signature != SIGNATURE {
        panic!("[BMP] Signature didn't match!");
    }

    if header.bits_per_pixel != 24 && header.bits_per_pixel != 32 {
        panic!("[BMP] Unsupported bit depth!");
    }

    if header.compression != 0 && header.compression != 3 {
        panic!("[BMP] Compression mode {:?} is not supported!", {
            header.compression
        });
    }

    let bytes_per_pixel = (header.bits_per_pixel / u8::BITS as u16) as usize;

    let mut color_buf = vec![0; (header.width * header.height) as usize * bytes_per_pixel];
    f.seek(SeekFrom::Start(header.offset as u64))
        .expect("[BMP] Error seeking!");
    f.read_exact(color_buf.as_mut_slice())
        .expect("[BMP] Error reading!");

    let pixels = if header.compression == 0 && header.bits_per_pixel == 24 {
        if header.width != header.height
            || !((header.width & (header.width - 1)) == 0)
            || !((header.height & (header.height - 1)) == 0)
        {
            panic!("[BMP] Width and/or height are not power of two!");
        }

        Vec::from_iter(color_buf.rchunks_exact_mut(bytes_per_pixel).map(|c| {
            color_from_tuple((
                (c[2] as f32 / 256.0 * 32.0) as u16,
                (c[1] as f32 / 256.0 * 32.0) as u16,
                (c[0] as f32 / 256.0 * 32.0) as u16,
            ))
        }))
    } else if header.compression == 3 && header.bits_per_pixel == 32 {
        // let red_mask = header.red_mask;
        // let green_mask = header.green_mask;
        // let blue_mask = header.blue_mask;
        // let alpha_mask = header.alpha_mask;

        // let red_shift = red_mask.trailing_zeros();
        // let green_shift = green_mask.trailing_zeros();
        // let blue_shift = blue_mask.trailing_zeros();
        // let alpha_shift = alpha_mask.trailing_zeros();

        // color_buf
        //     .chunks_exact_mut(bytes_per_pixel)
        //     .for_each(|color| {
        //         shift_channels(color, red_shift, green_shift, blue_shift, alpha_shift)
        //     });

        // un-mirror Y
        color_buf = Vec::from_iter(
            color_buf
                .rchunks(4 * header.width as usize)
                .flatten()
                .cloned(),
        );

        Vec::from_iter(color_buf.chunks_exact_mut(4).map(|c| {
            color_from_tuple((
                (c[2] as f32 / 256.0 * 32.0) as u16,
                (c[1] as f32 / 256.0 * 32.0) as u16,
                (c[0] as f32 / 256.0 * 32.0) as u16,
            ))
        }))
    } else {
        panic!("[BMP] Wrong combination of compression and bit depth!");
    };

    B2DO {
        width: header.width,
        height: header.height,
        bitmap: pixels,
    }
}

fn shift32(value: u32, shift: u32) -> u32 {
    value << shift
}

fn shift_channels(
    array: &mut [u8],
    red_shift: u32,
    green_shift: u32,
    blue_shift: u32,
    alpha_shift: u32,
) {
    let shifted = shift32(array[0] as u32, red_shift)
        + shift32(array[1] as u32, green_shift)
        + shift32(array[2] as u32, blue_shift)
        + shift32(array[3] as u32, alpha_shift);

    array[0] = (shifted >> 0) as u8;
    array[1] = (shifted >> 8) as u8;
    array[2] = (shifted >> 16) as u8;
    array[3] = (shifted >> 24) as u8;
}
