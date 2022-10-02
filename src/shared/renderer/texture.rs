pub struct Texture {
    width: usize,
    height: usize,
    colors: Box<[u8]>,
}

impl Texture {
    pub fn new(width: usize, height: usize, colors: Box<[u8]>) -> Self {
        Self {
            width: width,
            height: height,
            colors: colors
        }
    }
}