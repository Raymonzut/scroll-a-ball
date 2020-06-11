use amethyst::{
    core::transform::Transform,
    prelude::*,
    renderer::{Camera},
};

pub const GAME_HEIGHT: f32 = 100.0;
pub const GAME_WIDTH: f32 = 100.0;

pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialise_camera(world);
    }
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(GAME_WIDTH * 0.5, GAME_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(GAME_WIDTH, GAME_HEIGHT))
        .with(transform)
        .build();
}

