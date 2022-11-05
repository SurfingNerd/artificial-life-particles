use bevy::{prelude::App, DefaultPlugins};

mod particle_forces;

fn main() {
    // create a bevy app.
    let mut app = App::new().add_plugins(DefaultPlugins).run();
}
