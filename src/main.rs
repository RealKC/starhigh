mod world;

use bevy::prelude::*;
use rand::{thread_rng, Rng};

static TITLES: [&str; 3] = [
    "Starhigh! Colectează cât mai multe stele!",
    "Starhigh! Dacă nu prinzi o stea, pierzi o viață.",
    "Starhigh! Dacă nu prinzi o inimă, pierzi un punct.",
];

fn main() {
    let title_index = thread_rng().gen_range(0..TITLES.len());

    App::build()
        .insert_resource(WindowDescriptor {
            title: TITLES[title_index].to_string(),
            vsync: true,
            width: 540.0,
            height: 600.0,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(world::World)
        .run();
    println!("Hello, world!");
}
