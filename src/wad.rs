use std::convert::TryInto;
use std::fs::File;
use std::io;
use std::io::Read;

use cgmath::{Vector2, Vector3};

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

#[derive(Default)]
pub struct TextureData {
    pub width: usize,
    pub height: usize,
    pub colors: Vec<Vector3<u8>>,
}

enum PatchColumnState {
    YOffset,
    Length,
    PaddingPre,
    Color,
}

impl WAD {
    pub fn get_texture_data(&self, name: &str) -> TextureData {
        let mut texture_data = TextureData::default();

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
        texture_data.colors = vec![Vector3::new(255, 255, 255); header_width * header_height];
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
                            texture_data.colors[(y * header_width + x)] = colors[b as usize];

                            c += 1;
                        }
                    }
                    _ => unreachable!(),
                }
                i += 1;
            }
        }

        texture_data.width = header_width;
        texture_data.height = header_height;

        println!(
            "Loading texture \"{}\" with size {}:{} with offsets {}:{}",
            texture_dir.name, header_width, header_height, header_lo, header_to
        );

        texture_data
    }

    pub fn get_map_data(&self, map_name: &str) -> Result<WorldData, ()> {
        let mut data = WorldData::default();

        let mut found = false;
        for dir in &self.dirs {
            if (dir.size == 0) {
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
        .map(|x| {
            (Dir {
                offset: read_u32(x, 0) as usize,
                size: read_u32(x, 4) as usize,
                name: read_str_8bytes(x, 8),
            })
        })
        .collect();

    Ok(WAD {
        dir_count,
        dir_offset,
        dirs,
        buf,
    })
}

fn read_u8(buf: &[u8], offset: usize) -> u8 {
    u8::from_le_bytes(
        buf[offset..offset + 1]
            .try_into()
            .expect("slice with incorrect length"),
    )
}

fn read_i16(buf: &[u8], offset: usize) -> i16 {
    i16::from_le_bytes(
        buf[offset..offset + 2]
            .try_into()
            .expect("slice with incorrect length"),
    )
}

fn read_u16(buf: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes(
        buf[offset..offset + 2]
            .try_into()
            .expect("slice with incorrect length"),
    )
}

fn read_u32(buf: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(
        buf[offset..offset + 4]
            .try_into()
            .expect("slice with incorrect length"),
    )
}

fn read_str_4bytes(buf: &[u8], offset: usize) -> String {
    String::from_utf8(
        buf[offset..offset + 4]
            .try_into()
            .expect("slice with incorrect length"),
    )
    .expect("")
    .replace('\0', "")
}

fn read_str_8bytes(buf: &[u8], offset: usize) -> String {
    String::from_utf8(
        buf[offset..offset + 8]
            .try_into()
            .expect("slice with incorrect length"),
    )
    .expect("")
    .replace('\0', "")
}
