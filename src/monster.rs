use bevy::prelude::*;

pub struct Health{
    current: f32,
    max: f32
}

pub struct ExperiencePoint(u32);

pub enum Rarity{
    Common,
}

pub struct Level(u32);


pub struct Monsters{
    sprite: SpriteBundle,
}
