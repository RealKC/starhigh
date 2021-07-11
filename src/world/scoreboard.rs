use bevy::prelude::*;
use std::{env, fs, io::Read, io::Write};

use super::player::PLAYER_STAR_COLLISION_SYSTEM_LABEL;

pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Scoreboard::new());
        app.insert_resource(Lives::new());
        app.insert_resource(HighScore::load_from_file());
        app.add_startup_system(spawn_boards.system());
        app.add_system(
            update_boards
                .system()
                .after(PLAYER_STAR_COLLISION_SYSTEM_LABEL),
        );
    }
}

#[derive(PartialEq, Eq)]
pub(super) enum BoardType {
    Score,
    Lives,
    HighScore,
}

fn spawn_boards(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    highscore: Res<HighScore>,
) {
    commands.spawn_bundle(ImageBundle {
        material: materials.add(asset_server.load("star.png").into()),
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
    commands
        .spawn_bundle(TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load("FiraCode-Regular.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.0, 0.0, 0.0),
                    },
                }],
                ..Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(50.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(BoardType::Score);

    commands.spawn_bundle(ImageBundle {
        material: materials.add(asset_server.load("heart.png").into()),
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(45.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });

    commands
        .spawn_bundle(TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load("FiraCode-Regular.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.0, 0.0, 0.0),
                    },
                }],
                ..Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(45.0),
                    left: Val::Px(50.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(BoardType::Lives);

    commands.spawn_bundle(ImageBundle {
        material: materials.add(asset_server.load("high-score.png").into()),
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(100.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn_bundle(ImageBundle {
        material: materials.add(asset_server.load("star.png").into()),
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(145.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
    commands
        .spawn_bundle(TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: highscore.value.to_string(),
                    style: TextStyle {
                        font: asset_server.load("FiraCode-Regular.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.0, 0.0, 0.0),
                    },
                }],
                ..Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(145.0),
                    left: Val::Px(50.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(BoardType::HighScore);
}

fn update_boards(
    scoreboard: Res<Scoreboard>,
    lives: Res<Lives>,
    mut query: Query<(&mut Text, &BoardType)>,
) {
    for (mut text, board_type) in query.iter_mut() {
        match board_type {
            BoardType::Score => text.sections[0].value = scoreboard.0.to_string(),
            BoardType::Lives => text.sections[0].value = lives.0.to_string(),
            BoardType::HighScore => (),
        };
    }
}
#[derive(Clone, Copy)]
pub struct HighScore {
    pub value: u64,
}

impl HighScore {
    fn get_file() -> Option<fs::File> {
        let current_exe = match env::current_exe() {
            Ok(exe) => exe,
            Err(_) => {
                println!("huh?");
                return None;
            }
        };

        let current_exe = match fs::read_link(current_exe.clone()) {
            Ok(exe) => exe,
            Err(_) => current_exe,
        };

        let current_exe_dir = match current_exe.parent() {
            Some(dir) => dir,
            None => return None,
        };

        let high_score_file_path = current_exe_dir.join("highscore.dat");

        let high_score_file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .append(false)
            .open(&high_score_file_path);

        high_score_file.ok()
    }

    pub fn load_from_file() -> Self {
        if let Some(mut file) = Self::get_file() {
            let mut string = String::new();
            match file.read_to_string(&mut string) {
                Ok(_) => match string.parse::<u64>() {
                    Ok(value) => Self { value },
                    _ => Self { value: 0 },
                },
                _ => Self { value: 0 },
            }
        } else {
            Self { value: 0 }
        }
    }

    pub fn update_file(&self) {
        if let Some(mut file) = Self::get_file() {
            match file.write(self.value.to_string().as_bytes()) {
                Ok(_) => (),
                Err(_) => (),
            };
        }
    }
}

#[derive(Clone, Copy)]
pub struct Scoreboard(u64);

impl Scoreboard {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn add_point(&mut self) {
        self.0 += 1;
    }

    pub fn remove_point(&mut self) {
        if self.0 != 0 {
            self.0 -= 1;
        }
    }

    pub fn score(&self) -> u64 {
        self.0
    }
}

pub struct Lives(u64);

impl Lives {
    pub fn new() -> Self {
        Self(3)
    }

    pub fn add_life(&mut self) {
        self.0 += 1;
    }

    pub fn remove_life(&mut self) {
        self.0 -= 1;
    }

    pub fn is_dead(&self) -> bool {
        self.0 == 0
    }
}
