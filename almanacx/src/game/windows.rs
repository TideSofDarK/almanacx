use common::{
    buffer2d::{virtual_window::VirtualWindow, B2DO},
    image::bmp,
};

use super::definitions::{
    PRIMARY_HEIGHT, PRIMARY_WIDTH, REFERENCE_HEIGHT, REFERENCE_WIDTH, TEST_A_HEIGHT, TEST_A_WIDTH,
    VW_MAX,
};

pub fn load_border_texture() -> B2DO {
    bmp::load_bmp("./assets/border.bmp").expect("no such bmp file")
}

pub fn create_virtual_windows() -> Vec<VirtualWindow> {
    let mut virtual_windows = vec![];
    for i in 0..VW_MAX {
        virtual_windows.push(match i {
            super::definitions::VW_PRIMARY => VirtualWindow::new(

                PRIMARY_WIDTH,
                PRIMARY_HEIGHT,
            ).with_xyz(((REFERENCE_WIDTH - PRIMARY_WIDTH) / 2) as i32,
                ((REFERENCE_HEIGHT - PRIMARY_HEIGHT) / 2) as i32,
                0),
            super::definitions::VW_TEST_A => {
                VirtualWindow::new( TEST_A_WIDTH, TEST_A_HEIGHT).with_xyz(64, 32, 1)
            }
            super::definitions::VW_TEST_B => {
                VirtualWindow::new( TEST_A_WIDTH, TEST_A_HEIGHT).with_xyz(200, 40, 2)
            }
            _ => unreachable!(),
        });
    }
    virtual_windows
}
