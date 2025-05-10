use astro::planet;
use bevy::ecs::{component::Component, entity::Entity};

#[derive(Component)]
pub struct Planet {
    pub name: String,
    pub astro_enum: planet::Planet,
    pub radius: f32,
}

#[derive(Component)]
pub struct DateLabel;

#[derive(Component)]
pub struct PlanetLabel {
    pub entity: Entity,
}

#[derive(Component)]
pub struct InnerButton;

#[derive(Component)]
pub struct OuterButton;
