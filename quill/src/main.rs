mod view;

use std::{
    collections::{vec_deque, VecDeque},
    time::Instant,
};

use common::{
    buffer2d::B2DS,
    console::Console,
    platform::{input::Input, Application},
};

struct Quill {
    console: Console,
}

impl Application for Quill {
    fn get_title(&self) -> &'static str {
        "Quill"
    }

    fn main_loop(&mut self, input: &Input, dt: f32, buffer: Option<B2DS>) -> bool {
        todo!()
    }
}

fn main() {
    // init_application(Quill {
    //     console: Console::new(640, 360),
    // });

    let width = 5520usize;
    let height = 5520usize;
    let mut color_buffer = vec![5; width * height * 1];
    let mut color_buffer_clear = vec![5; width * height * 1];

    let mut last_frame = Instant::now();
    {
        color_buffer.rotate_left(width * (height / 2));
        color_buffer[width * (height / 2)..].fill(0);
    }
    println!("Elapsed1: {:?}", last_frame.elapsed().as_secs_f32());

    let mut last_frame = Instant::now();
    {
        // color_buffer_clear.rotate_left(width * (height / 2));
        color_buffer_clear.copy_within(width * (height / 2)..width * height, 0);
        color_buffer_clear[width * (height / 2)..].fill(0);
    }
    println!("Elapsed2: {:?}", last_frame.elapsed().as_secs_f32());

    for i in 0..height * width {
        assert_eq!(color_buffer[i], color_buffer_clear[i]);
    }
}
