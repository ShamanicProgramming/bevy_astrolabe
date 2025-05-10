use bevy::ecs::resource::Resource;
use chrono::{DateTime, Utc};

#[derive(Resource)]
pub struct ShownDate {
    pub date: DateTime<Utc>,
}
