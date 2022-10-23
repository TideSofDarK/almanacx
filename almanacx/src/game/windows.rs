use common::{
    buffer2d::{virtual_window::VirtualWindow, B2DO},
    image::bmp,
};

use super::definitions::{
    PRIMARY_HEIGHT, PRIMARY_WIDTH, REFERENCE_HEIGHT, REFERENCE_WIDTH, TEST_A_HEIGHT, TEST_A_WIDTH,
    VW_MAX, VW_TEST_A,
};

pub fn load_border_texture() -> B2DO {
    bmp::load_bmp("./assets/border.bmp").expect("no such bmp file")
}

pub fn create_virtual_windows() -> Vec<VirtualWindow> {
    let mut virtual_windows = vec![];
    for i in 0..VW_MAX {
        virtual_windows.push(match i {
            super::definitions::VW_PRIMARY => VirtualWindow::new(
                ((REFERENCE_WIDTH - PRIMARY_WIDTH) / 2) as i32,
                ((REFERENCE_HEIGHT - PRIMARY_HEIGHT) / 2) as i32,
                PRIMARY_WIDTH,
                PRIMARY_HEIGHT,
            ),
            super::definitions::VW_TEST_A => {
                VirtualWindow::new(64, 32, TEST_A_WIDTH, TEST_A_HEIGHT)
            }
            _ => VirtualWindow::default(),
        });
    }
    virtual_windows
}

// pub fn create_buffers() -> Vec<B2DO> {
//     let mut buffers = vec![];
//     for i in 0..VW_MAX {
//         virtual_windows.push(match i {
//             VW_PRIMARY => VirtualWindow {
//                 x: ((REFERENCE_WIDTH - PRIMARY_WIDTH) / 2) as i32,
//                 y: ((REFERENCE_HEIGHT - PRIMARY_HEIGHT) / 2) as i32,
//                 z: 0,
//                 minimized: false,
//             },
//             _ => VirtualWindow::default(),
//         });
//     }
//     virtual_windows
// }
