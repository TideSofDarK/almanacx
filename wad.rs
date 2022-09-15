use std::convert::TryInto;
use std::fs::File;
use std::io;
use std::io::Read;

use cgmath::Vector2;

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

impl WAD {
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
                    println!(
                        "Loading map \"{}\" of size {} with offset {}",
                        dir.name, dir.size, dir.offset
                    );
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

        if !found {
            return Err(());
        }

        Ok(data)
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
    .replace("\0", "")
}

fn read_str_8bytes(buf: &[u8], offset: usize) -> String {
    String::from_utf8(
        buf[offset..offset + 8]
            .try_into()
            .expect("slice with incorrect length"),
    )
    .expect("")
    .replace("\0", "")
}
