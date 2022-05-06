use bevy::prelude::*;

mod init;
mod game;
mod map;
mod monster;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState{
    Init,
    Game
}


fn main() {
    App::new()
        .add_plugin(init::InitPlug)
        .add_plugin(game::GamePlug)
        
        .add_state(AppState::Init)
        .run();
}
