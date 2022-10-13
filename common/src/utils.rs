pub fn calculate_index(x: i32, y: i32, width: usize, height: usize) -> Option<usize> {
    if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
        return Some((y * height as i32 + x) as usize);
    }
    None
}
