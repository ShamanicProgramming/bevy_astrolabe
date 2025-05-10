mod components;
mod constants;
mod resources;
mod setup_systems;
mod update_systems;

use bevy::prelude::*;
use chrono::Utc;
use resources::ShownDate;
use setup_systems::*;
use update_systems::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AstrolabePlugin))
        .insert_resource(ShownDate { date: Utc::now() })
        .run();
}

pub struct AstrolabePlugin;

impl Plugin for AstrolabePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                add_planets,
                add_camera,
                add_light,
                add_planet_labels,
                add_date_label,
                add_buttons,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                update_date,
                update_planets,
                update_planet_labels,
                update_date_label,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (inner_button_interactions, outer_button_interactions),
        );
    }
}
