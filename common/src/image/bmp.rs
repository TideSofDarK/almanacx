use std::{
    fs::File,
    io::{self, ErrorKind, Read, Seek, SeekFrom},
    mem, slice,
};

use crate::buffer2d::B2DO;

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

pub fn load_bmp(path: &str) -> io::Result<B2DO> {
    let mut f = File::open(path)?;
    let mut header: BMPHeader = unsafe { mem::zeroed() };

    unsafe {
        let header_slice = slice::from_raw_parts_mut(&mut header as *mut _ as *mut u8, HEADER_SIZE);
        f.read_exact(header_slice).unwrap();
    }

    if header.signature != SIGNATURE {
        return Err(std::io::Error::new(
            ErrorKind::InvalidData,
            "BMP signature didn't match!",
        ));
    }

    if header.bits_per_pixel != 24 && header.bits_per_pixel != 32 {
        return Err(std::io::Error::new(
            ErrorKind::InvalidData,
            "Unsupported BMP bit depth!",
        ));
    }

    if header.compression != 0 && header.compression != 3 {
        return Err(std::io::Error::new(
            ErrorKind::InvalidData,
            format!("BMP compression mode {:?} is not supported!", {
                header.compression
            }),
        ));
    }

    let bytes_per_pixel = (header.bits_per_pixel / 8) as usize;

    let mut color_buf = vec![0; (header.width * header.height) as usize * bytes_per_pixel];
    f.seek(SeekFrom::Start(header.offset as u64))?;
    f.read_exact(color_buf.as_mut_slice())?;

    if header.compression == 0 && header.bits_per_pixel == 24 {
        color_buf = Vec::from_iter(
            color_buf
                .chunks_exact_mut(bytes_per_pixel)
                .map(|c| {
                    let mut n = c.to_owned();
                    // n.reverse();
                    n.push(255);
                    n
                })
                .flatten(),
        );

        // // un-mirror X
        // color_buf = Vec::from_iter(
        //     color_buf
        //         .chunks(4 * header.width as usize)
        //         .flat_map(|row| row.rchunks(4).flatten())
        //         .cloned(),
        // );
    } else if header.compression == 3 && header.bits_per_pixel == 32 {
        let red_mask = header.red_mask;
        let green_mask = header.green_mask;
        let blue_mask = header.blue_mask;
        let alpha_mask = header.alpha_mask;

        let red_shift = red_mask.trailing_zeros();
        let green_shift = green_mask.trailing_zeros();
        let blue_shift = blue_mask.trailing_zeros();
        let alpha_shift = alpha_mask.trailing_zeros();

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
    }

    Ok(B2DO {
        width: header.width,
        height: header.height,
        colors: color_buf,
    })
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
