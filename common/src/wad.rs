use std::fs::File;
use std::io;
use std::io::Read;

use cgmath::{Vector2, Vector3};

use crate::{buffer2d::Buffer2D, utils::*};

const DIR_SIZE: usize = 16;
const VERTICES_SIZE: usize = 4;
const LINEDEFS_SIZE: usize = 14;

pub struct WAD {
    dir_count: usize,
    dir_offset: usize,
    dirs: Vec<Dir>,
    buf: Vec<u8>,
}

struct Dir {
    offset: usize,
    size: usize,
    name: String,
}

#[derive(Default)]
pub struct WorldData {
    pub vertices: Vec<Vector2<i16>>,
    pub linedefs: Vec<Vector2<usize>>,
}

enum PatchColumnState {
    YOffset,
    Length,
    PaddingPre,
    Color,
}

impl WAD {
    pub fn load_texture_into_buffer(&self, name: &str) -> Buffer2D {
        let texture_dir = self
            .dirs
            .iter()
            .find(|dir| (dir.name.as_str() == name))
            .expect("no such texture dir");

        let playpal_dir = self
            .dirs
            .iter()
            .find(|dir| (dir.name.as_str() == "PLAYPAL"))
            .expect("no such palette dir");
        let colors: Vec<Vector3<u8>> = self.buf[playpal_dir.offset..playpal_dir.offset + (3 * 256)]
            .chunks(3)
            .map(|c| (Vector3::new(c[0], c[1], c[2])))
            .collect();

        let header_width = read_u16(&self.buf, texture_dir.offset) as usize;
        let header_height = read_u16(&self.buf, texture_dir.offset + 2) as usize;

        let mut final_colors = vec![255u8; header_width * header_height * 4];

        let header_lo = read_i16(&self.buf, texture_dir.offset + 4);
        let header_to = read_i16(&self.buf, texture_dir.offset + 6);
        let pointer_offset = texture_dir.offset + 8;
        let header_pointer_offsets: Vec<usize> = self.buf
            [pointer_offset..pointer_offset + (header_width * 4)]
            .chunks(4)
            .map(|o| read_u32(o, 0) as usize)
            .collect();

        for x in 0..header_width as usize {
            let patch_offset = texture_dir.offset + header_pointer_offsets[x];

            let mut state = PatchColumnState::YOffset;

            let mut y_offset = 0;
            let mut length = 0;
            let mut c = 0;

            let mut i = 0;
            loop {
                let b = read_u8(&self.buf, patch_offset + i);
                match state {
                    PatchColumnState::YOffset => {
                        if b == 0xFF {
                            break;
                        }
                        y_offset = b;

                        state = PatchColumnState::Length;
                    }
                    PatchColumnState::Length => {
                        length = b;
                        c = 0;

                        state = PatchColumnState::PaddingPre;
                    }
                    PatchColumnState::PaddingPre => {
                        state = PatchColumnState::Color;
                    }
                    PatchColumnState::Color => {
                        if c == length {
                            state = PatchColumnState::YOffset;
                        } else {
                            let y = (y_offset + c) as usize;
                            let index =
                                calculate_index(x as i32, y as i32, header_width as i32) * 4;
                            let color = colors[b as usize];
                            final_colors[index..index + 3]
                                .copy_from_slice(&[color.z, color.y, color.x]);
                            final_colors[index + 3] = 255;

                            c += 1;
                        }
                    }
                }
                i += 1;
            }
        }

        println!(
            "Loading texture \"{}\" with size {}:{} with offsets {}:{}",
            texture_dir.name, header_width, header_height, header_lo, header_to
        );

        Buffer2D::new(header_width, header_height, final_colors)
    }

    pub fn get_map_data(&self, map_name: &str) -> Result<WorldData, ()> {
        let mut data = WorldData::default();

        let mut found = false;
        for dir in &self.dirs {
            if dir.size == 0 {
                if found {
                    break;
                } else {
                    if dir.name.as_str() == map_name {
                        found = true;
                        println!("Loading map \"{}\" with offset {}", dir.name, dir.offset);
                    }
                    continue;
                }
            }

            match dir.name.as_str() {
                "VERTEXES" => {
                    data.vertices = self.buf[dir.offset..dir.offset + dir.size]
                        .chunks(VERTICES_SIZE)
                        .map(|x| (Vector2::new(read_i16(x, 0), read_i16(x, 2))))
                        .collect();
                }
                "LINEDEFS" => {
                    data.linedefs = self.buf[dir.offset..dir.offset + dir.size]
                        .chunks(LINEDEFS_SIZE)
                        .map(|x| (Vector2::new(read_u16(x, 0) as usize, read_u16(x, 2) as usize)))
                        .collect();
                }
                _ => (),
            }
        }

        match found {
            true => Ok(data),
            _ => Err(()),
        }
    }
}

pub fn load(path: &str) -> io::Result<WAD> {
    let mut buf = vec![0];
    let bytes_read = File::open(path)?.read_to_end(&mut buf)?;
    buf.remove(0);

    println!("Loaded WAD: {}, Total {:?} bytes!", path, bytes_read);

    let dir_count = read_u32(&buf, 4) as usize;
    let dir_offset = read_u32(&buf, 8) as usize;

    let dir = &buf[dir_offset..buf.len()];
    let dirs = dir
        .chunks(DIR_SIZE)
        .map(|x| Dir {
            offset: read_u32(x, 0) as usize,
            size: read_u32(x, 4) as usize,
            name: read_str_8bytes(x, 8),
        })
        .collect();

    Ok(WAD {
        dir_count,
        dir_offset,
        dirs,
        buf,
    })
}
