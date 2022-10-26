mod game;

use common::{
    buffer2d::{
        text::Font,
        virtual_window::{self, VirtualWindow, VirtualWindowStack, WindowBorder},
        B2DO,
    },
    console::Console,
    image::bmp,
    platform::init_application,
    renderer::{camera::Camera, Renderer},
};
use game::{
    definitions::{
        PRIMARY_HEIGHT, PRIMARY_WIDTH, REFERENCE_HEIGHT, REFERENCE_WIDTH, TEST_A_HEIGHT,
        TEST_A_WIDTH, VW_MAX, VW_PRIMARY, VW_TEST_A, VW_TEST_B,
    },
    player::Player,
    world::World,
    Game, GameState,
};

pub fn create_virtual_windows() -> Vec<VirtualWindow> {
    let mut virtual_windows = vec![];
    for i in 0..VW_MAX {
        virtual_windows.push(match i {
            VW_PRIMARY => {
                VirtualWindow::new(String::from("Primary"), PRIMARY_WIDTH, PRIMARY_HEIGHT).with_xyz(
                    ((REFERENCE_WIDTH - PRIMARY_WIDTH) / 2) as i32,
                    ((REFERENCE_HEIGHT - PRIMARY_HEIGHT) / 2) as i32,
                    0,
                )
            }
            VW_TEST_A => VirtualWindow::new(String::from("Test A"), TEST_A_WIDTH, TEST_A_HEIGHT)
                .with_xyz(64, 32, 1),
            VW_TEST_B => VirtualWindow::new(String::from("Test B"), TEST_A_WIDTH, TEST_A_HEIGHT)
                .with_xyz(200, 40, 2),
            _ => unreachable!(),
        });
    }
    virtual_windows
}

fn load_game() -> Game {
    let world = World::new();
    let texture = bmp::load_bmp("./assets/floor.bmp");
    let border = WindowBorder::new(bmp::load_bmp("./assets/border.bmp"));
    let font = Font::new(bmp::load_bmp("./assets/conchars.bmp"), (8, 8), 0, 2);
    let console_font = Font::new(bmp::load_bmp("./assets/conchars.bmp"), (8, 8), 0, 10);

    let mut console = Console::new(REFERENCE_WIDTH / 3, REFERENCE_HEIGHT);
    console.put_line("Console activated", &console_font);

    let virtual_windows = create_virtual_windows();
    for virtual_window in &virtual_windows {
        console.put_string(
            format!("Virtual window \"{}\" created", virtual_window.name),
            &console_font,
        );
    }
    let renderer = Renderer::new(&virtual_windows[VW_PRIMARY].buffer);
    console.put_line("Renderer created", &console_font);
    let stack = VirtualWindowStack::new(virtual_windows);
    console.put_line("Window stack activated", &console_font);

    let game_state = GameState::Action;
    let camera = Camera::perspective(
        f32::to_radians(90.0),
        PRIMARY_WIDTH as f32 / PRIMARY_HEIGHT as f32,
        0.01,
        100.0,
    );
    let player = Player::new();

    let x = 0;
    let y = 0;

    Game {
        console,
        stack,
        game_state,
        renderer,
        camera,
        player,
        world,
        texture,
        border,
        font,
        x,
        y,
    }
}

fn main() {
    init_application(load_game());
}
