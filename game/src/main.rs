mod game;
mod player;
mod world;

use common::application::init_application;

use game::Game;

fn main() {
    init_application(Game::new());
}
