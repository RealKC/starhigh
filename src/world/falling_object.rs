use bevy::{
    core::FixedTimestep,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use super::{
    scoreboard::{Lives, Scoreboard},
    GameState, Wall, PLAYER_DEATH_LABEL,
};

pub(super) struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Speed {
            last_increased_at: 0,
            value: 1.0,
        });

        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::steps_per_second(0.5))
                .with_system(spawn_falling_object.system()),
        );
        app.add_system(falling_object_gravity.system());
        app.add_system(
            falling_object_wall_collision
                .system()
                .after(PLAYER_DEATH_LABEL),
        );
    }
}

pub struct Speed {
    last_increased_at: u64,
    value: f32,
}

impl Speed {
    pub fn increase(&mut self) {
        if (0..100).contains(&self.last_increased_at) {
            self.value += 0.25;
        } else if (100..200).contains(&self.last_increased_at) {
            self.value += 0.5;
        } else if (200..350).contains(&self.last_increased_at) {
            self.value += 0.75;
        } else if (350..500).contains(&self.last_increased_at) {
            self.value += 0.8;
        } else {
            self.value += 1.0;
        }
    }

    pub fn should_increase(&self, scoreboard: Scoreboard) -> bool {
        scoreboard.score() % 10 == 0 && scoreboard.score() >= self.last_increased_at
    }
}

pub enum ObjectKind {
    Star,
    Heart,
}

fn spawn_falling_object(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_running: Res<GameState>,
    scoreboard: Res<Scoreboard>,
    asset_server: Res<AssetServer>,
) {
    if game_running.is_not_running() {
        return;
    }

    const SPAWN_Y: f32 = 260.0;

    use rand::{thread_rng, Rng};
    let mut rng = thread_rng();
    let x = rng.gen_range(-120.0..180.0);

    if scoreboard.score() >= 50 {
        let spawn_heart = {
            let x = rng.gen_range(1..100);
            x == 5
        };
        if spawn_heart {
            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.add(asset_server.load("heart.png").into()),
                    transform: Transform::from_xyz(x, SPAWN_Y, 0.0),
                    sprite: Sprite::new(Vec2::new(40.0, 40.0)),
                    ..Default::default()
                })
                .insert(ObjectKind::Heart);
            return;
        }
    }

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(asset_server.load("star.png").into()),
            transform: Transform::from_xyz(x, SPAWN_Y, 0.0),
            sprite: Sprite::new(Vec2::new(40.0, 40.0)),
            ..Default::default()
        })
        .insert(ObjectKind::Star);
}

fn falling_object_gravity(
    mut falling_objects: Query<&mut Transform, With<ObjectKind>>,
    game_running: Res<GameState>,
    speed: Res<Speed>,
) {
    if game_running.is_not_running() {
        return;
    }

    for mut star_transform in falling_objects.iter_mut() {
        star_transform.translation.y -= speed.value;
    }
}
fn falling_object_wall_collision(
    mut commands: Commands,
    mut lives: ResMut<Lives>,
    mut scoreboard: ResMut<Scoreboard>,
    falling_objects: Query<(Entity, &Transform, &Sprite, &ObjectKind)>,
    wall_query: Query<(&Transform, &Sprite), With<Wall>>,
) {
    for (wall_transform, wall_sprite) in wall_query.iter() {
        for (entity, transform, sprite, kind) in falling_objects.iter() {
            let collision = collide(
                wall_transform.translation,
                wall_sprite.size,
                transform.translation,
                sprite.size,
            );

            let player_missed_object = match collision {
                Some(c) => match c {
                    Collision::Bottom => true,
                    _ => false,
                },
                None => false,
            };

            if player_missed_object {
                match kind {
                    ObjectKind::Star => {
                        lives.remove_life();
                    }
                    ObjectKind::Heart => {
                        scoreboard.remove_point();
                    }
                }
                commands.entity(entity).despawn();
            }
        }
    }
}
