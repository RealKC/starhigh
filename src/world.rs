mod falling_object;
mod player;
mod scoreboard;

use falling_object::*;
use player::*;
use scoreboard::*;

use bevy::prelude::*;

pub struct World;

static PLAYER_DEATH_LABEL: &str = "player_death_system";

impl Plugin for World {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
        app.add_startup_system(build_arena.system());
        app.add_startup_system(
            (|mut commands: Commands,
              mut materials: ResMut<Assets<ColorMaterial>>,
              mut asset_server: Res<AssetServer>| {
                commands
                    .spawn_bundle(pause_button(&mut materials, &mut asset_server))
                    .insert(Button::Pause);
            })
            .system(),
        );

        app.insert_resource(GameState::NotRunning);
        app.insert_resource(GameStoppedByHandler::No);
        app.add_system(global_keyinput_handler.system().before(PLAYER_DEATH_LABEL));
        app.add_system(handle_player_death.system().label(PLAYER_DEATH_LABEL));

        app.add_plugin(StarPlugin);
        app.add_plugin(ScoreboardPlugin);
        app.add_plugin(PlayerPlugin);
    }
}

fn global_keyinput_handler(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asset_server: Res<AssetServer>,
    mut game_state: ResMut<GameState>,
    mut lives: ResMut<Lives>,
    mut scoreboard: ResMut<Scoreboard>,
    mut game_stopped: ResMut<GameStoppedByHandler>,
    query: Query<(Entity, &Button)>,
    help_box_query: Query<Entity, With<Help>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (entity, button_kind) in query.iter() {
        if keyboard_input.pressed(KeyCode::P) && *button_kind == Button::Play {
            commands.entity(entity).despawn();
            commands
                .spawn_bundle(pause_button(&mut materials, &mut asset_server))
                .insert(Button::Pause);
            game_state.flip();
        } else if keyboard_input.pressed(KeyCode::S) && *button_kind == Button::Pause {
            commands.entity(entity).despawn();
            commands
                .spawn_bundle(play_button(&mut materials, &mut asset_server))
                .insert(Button::Play);
            game_state.flip();

            for help_box in help_box_query.iter() {
                commands.entity(help_box).despawn();
            }

            if *game_stopped == GameStoppedByHandler::Yes {
                *lives = Lives::new();
                *scoreboard = Scoreboard::new();
                *game_stopped = GameStoppedByHandler::No;
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Button {
    Pause,
    Play,
}

fn pause_button(
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &mut Res<AssetServer>,
) -> ImageBundle {
    ImageBundle {
        material: materials.add(asset_server.load("pause.png").into()),
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        calculated_size: CalculatedSize {
            size: Size::new(40.0, 40.0),
        },
        ..Default::default()
    }
}

fn play_button(
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &mut Res<AssetServer>,
) -> ImageBundle {
    ImageBundle {
        material: materials.add(asset_server.load("play.png").into()),
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        calculated_size: CalculatedSize {
            size: Size::new(40.0, 40.0),
        },
        ..Default::default()
    }
}

struct Wall;
struct Help;

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(asset_server.load("backgroun2d.png").into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        sprite: Sprite::new(Vec2::new(600.0, 600.0)),
        ..Default::default()
    });

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(asset_server.load("help.png").into()),
            transform: Transform::from_xyz(40.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(238.0, 150.0)),
            ..Default::default()
        })
        .insert(Help);
}

fn build_arena(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    enum Axis {
        Vertical,
        Horizontal,
    }

    fn wall_at(
        x: f32,
        y: f32,
        axis: Axis,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> SpriteBundle {
        SpriteBundle {
            material: materials.add(ColorMaterial::color(Color::rgb_u8(146, 208, 209))),
            transform: Transform::from_xyz(x, y, 0.0),
            sprite: match axis {
                Axis::Vertical => Sprite::new(Vec2::new(20.0, 1199.0)),
                Axis::Horizontal => Sprite::new(Vec2::new(700.0, 20.0)),
            },
            ..Default::default()
        }
    }

    // right wall
    commands
        .spawn_bundle(wall_at(260.0, -270.0, Axis::Vertical, &mut materials))
        .insert(Wall);
    // bottom wall
    commands
        .spawn_bundle(wall_at(220.0, -290.0, Axis::Horizontal, &mut materials))
        .insert(Wall);
    // left wall
    commands
        .spawn_bundle(wall_at(-140.0, -275.0, Axis::Vertical, &mut materials))
        .insert(Wall);
    // top wall
    commands
        .spawn_bundle(wall_at(200.0, 290.0, Axis::Horizontal, &mut materials))
        .insert(Wall);
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum GameState {
    Running,
    NotRunning,
}

impl GameState {
    pub fn is_not_running(self) -> bool {
        Self::NotRunning == self
    }

    fn flip(&mut self) {
        *self = match *self {
            Self::Running => Self::NotRunning,
            Self::NotRunning => Self::Running,
        };
    }
}

#[derive(PartialEq, Eq)]
enum GameStoppedByHandler {
    Yes,
    No,
}

fn handle_player_death(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asset_server: Res<AssetServer>,
    mut game_running: ResMut<GameState>,
    mut game_stopped: ResMut<GameStoppedByHandler>,
    lives: Res<Lives>,
    mut high_score: ResMut<HighScore>,
    scoreboard: Res<Scoreboard>,
    mut displayed_boards: Query<(&mut Text, &BoardType)>,
    buttons: Query<(Entity, &Button)>,
    falling_objects: Query<Entity, With<ObjectKind>>,
    players: Query<Entity, With<Player>>,
) {
    if lives.is_dead() && *game_stopped == GameStoppedByHandler::No {
        *game_stopped = GameStoppedByHandler::Yes;
        for object in falling_objects.iter() {
            commands.entity(object).despawn();
        }
        *game_running = GameState::NotRunning;
        for (mut text, kind) in displayed_boards.iter_mut() {
            if *kind != BoardType::HighScore {
                continue;
            }
            high_score.value = scoreboard.score();
            text.sections[0].value = high_score.value.to_string();
            high_score.update_file();
        }
        for (button, kind) in buttons.iter() {
            if *kind == Button::Play {
                commands.entity(button).despawn();
            }
        }
        commands
            .spawn_bundle(pause_button(&mut materials, &mut asset_server))
            .insert(Button::Pause);

        for player in players.iter() {
            commands.entity(player).despawn();
        }

        spawn_player(commands, materials, asset_server);
    }
}
