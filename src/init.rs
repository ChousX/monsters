use bevy::prelude::*;
use crate::AppState;

pub const WINDOW_WIDTH: f32 = 1600.;
pub const WINDOW_HEIGHT: f32 = 900.;
pub const RESOLUTION: f32 = WINDOW_WIDTH / WINDOW_HEIGHT;

pub struct InitPlug;
impl Plugin for InitPlug{
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultPlugins)
            .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.4)))
            .insert_resource(WindowDescriptor{
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                title: "Monsters".to_string(),
                resizable: false,
                ..Default::default()
            })
            
            ;
    }
}

