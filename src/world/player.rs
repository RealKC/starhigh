use bevy::{prelude::*, sprite::collide_aabb::collide};

use super::{
    falling_object::{ObjectKind, Speed},
    scoreboard::{Lives, Scoreboard},
    GameState, Wall,
};

pub static PLAYER_STAR_COLLISION_SYSTEM_LABEL: &str = "player_star_collision_system";
static PLAYER_WALL_COLLISION_LABEL: &str = "player_wall_collision_system";
static KEYBORD_INPUT_LABEL: &str = "keyboard_input_system";
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_player.system());
        app.add_system(keyboard_input.system().label(KEYBORD_INPUT_LABEL));
        app.add_system(
            handle_player_falling_object_collision
                .system()
                .label(PLAYER_STAR_COLLISION_SYSTEM_LABEL),
        );
        app.add_system(
            handle_player_wall_collision
                .system()
                .label(PLAYER_WALL_COLLISION_LABEL)
                .after(KEYBORD_INPUT_LABEL),
        );
        app.add_system(apply_delta.system().after(PLAYER_WALL_COLLISION_LABEL));
    }
}

pub struct Player;

struct PositionDelta(f32);

pub fn spawn_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(asset_server.load("cloud.png").into()),
            transform: Transform::from_xyz(10.0, -200.0, 10.0),
            sprite: Sprite::new(Vec2::new(115.5, 57.0)),
            ..Default::default()
        })
        .insert(Player)
        .insert(PositionDelta(0.0));
}

fn keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    game_running: Res<GameState>,
    mut query: Query<&mut PositionDelta, With<Player>>,
) {
    if game_running.is_not_running() {
        return;
    }

    const STEP: f32 = 7.0;
    for mut position_delta in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            position_delta.0 -= STEP;
        }
        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            position_delta.0 += STEP;
        }
    }
}

fn apply_delta(mut query: Query<(&mut Transform, &mut PositionDelta), With<Player>>) {
    for (mut transform, mut delta) in query.iter_mut() {
        transform.translation.x += delta.0;
        delta.0 = 0.0;
    }
}

fn handle_player_falling_object_collision(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    mut lives: ResMut<Lives>,
    mut speed: ResMut<Speed>,
    players: Query<(&Transform, &Sprite), With<Player>>,
    falling_objects: Query<(Entity, &Transform, &Sprite, &ObjectKind)>,
) {
    for (player_transform, player_sprite) in players.iter() {
        for (entity, transform, sprite, kind) in falling_objects.iter() {
            let collision = collide(
                player_transform.translation,
                player_sprite.size,
                transform.translation,
                sprite.size,
            );

            if collision.is_some() {
                match kind {
                    ObjectKind::Star => {
                        scoreboard.add_point();
                        if speed.should_increase(*scoreboard) {
                            speed.increase();
                        }
                    }
                    ObjectKind::Heart => {
                        lives.add_life();
                    }
                };
                commands.entity(entity).despawn();
            }
        }
    }
}

fn handle_player_wall_collision(
    mut players: Query<(&mut PositionDelta, &Transform, &Sprite), With<Player>>,
    walls: Query<(&Transform, &Sprite), With<Wall>>,
) {
    for (mut delta, transform, sprite) in players.iter_mut() {
        for (wall_transform, wall_sprite) in walls.iter() {
            let collision = collide(
                transform.translation,
                sprite.size,
                wall_transform.translation,
                wall_sprite.size,
            );

            if let Some(collision) = collision {
                use bevy::sprite::collide_aabb::Collision::{Left, Right};
                delta.0 = match collision {
                    Left => -1.0,
                    Right => 1.0,
                    _ => unreachable!(),
                }
            }
        }
    }
}
