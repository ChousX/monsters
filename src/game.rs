use bevy::{prelude::*, render::camera::ScalingMode};

use crate::{init::RESOLUTION, AppState};

pub struct GamePlug;
impl Plugin for GamePlug{
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_camera))
            .add_system_set(SystemSet::on_enter(AppState::Init).with_system(load_textur))
            ;
    }
}

pub fn setup(mut commands: Commands){
}

pub fn spawn_camera(mut commands: Commands){
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.top = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}

pub struct MyTextureAtlas(Handle<TextureAtlas>);

pub fn load_textur(mut commands: Commands, assets: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>){
    let image = assets.load("texture.png");
    let atlas = TextureAtlas::from_grid_with_padding(image, Vec2::splat(100.0), 1, 2, Vec2::splat(3.0));

    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(MyTextureAtlas(atlas_handle));
}
