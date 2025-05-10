use crate::{
    components::*,
    constants::{INNER_CAMERA, OUTER_CAMERA},
    resources::ShownDate,
};
use astro::{planet, time};
use bevy::prelude::*;
use chrono::{Datelike, Duration, Timelike};

pub fn update_date_label(mut text: Single<&mut Text, With<DateLabel>>, date: Res<ShownDate>) {
    text.0 = format!("{}", date.date.format("%Y-%m-%d"));
}

pub fn update_date(timer: Res<Time>, mut date: ResMut<ShownDate>) {
    let seconds_per_second = 2592000.0;
    date.date = date.date + Duration::seconds((timer.delta_secs() * seconds_per_second) as i64);
}

pub fn update_planets(mut planets: Query<(&mut Transform, &Planet)>, date: Res<ShownDate>) {
    let day_of_month = time::DayOfMonth {
        day: date.date.day() as u8,
        hr: date.date.hour() as u8,
        min: date.date.minute() as u8,
        sec: date.date.second() as f64,
        time_zone: 0.0,
    };
    let astro_date = time::Date {
        year: date.date.year() as i16,
        month: date.date.month() as u8,
        decimal_day: time::decimal_day(&day_of_month),
        cal_type: time::CalType::Gregorian,
    };
    let julian_day = time::julian_day(&astro_date);

    for (mut transform, planet) in &mut planets {
        let (helio_long, helio_lat, rad_vec) =
            planet::heliocent_coords(&planet.astro_enum, julian_day);
        let x = rad_vec * f64::cos(helio_lat) * f64::cos(helio_long);
        let y = rad_vec * f64::cos(helio_lat) * f64::sin(helio_long);
        let z = rad_vec * f64::sin(helio_lat);
        transform.translation = Vec3 {
            x: x as f32,
            y: y as f32,
            z: z as f32,
        };
    }
}

pub fn update_planet_labels(
    mut labels: Query<(&mut Node, &PlanetLabel)>,
    camera: Single<(&mut Camera, &mut Transform, &GlobalTransform), With<Camera3d>>,
    labeled: Query<&GlobalTransform>,
) {
    let (camera, _camera_transform, camera_global_transform) = camera.into_inner();

    for (mut node, label) in &mut labels {
        let world_position = labeled.get(label.entity).unwrap().translation()
            + Vec3 {
                x: 0.1,
                y: 0.1,
                z: 0.0,
            };

        let viewport_position = camera
            .world_to_viewport(camera_global_transform, world_position)
            .unwrap();

        node.top = Val::Px(viewport_position.y);
        node.left = Val::Px(viewport_position.x);
    }
}

pub fn inner_button_interactions(
    inner_interaction_query: Query<
        (&Interaction, &mut BorderColor),
        (
            Changed<Interaction>,
            With<Button>,
            With<InnerButton>,
            Without<OuterButton>,
        ),
    >,
    outer_button: Single<&mut BorderColor, (With<Button>, With<OuterButton>, Without<InnerButton>)>,
    camera: Single<&mut Transform, With<Camera3d>>,
) {
    let mut camera_tranform = camera.into_inner();
    let mut outer_border_color = outer_button.into_inner();
    for (inner_interaction, mut inner_border_color) in inner_interaction_query {
        match *inner_interaction {
            Interaction::Pressed => {
                inner_border_color.0 = Color::WHITE;
                outer_border_color.0 = Color::BLACK;
                camera_tranform.translation = Vec3 {
                    x: 0.,
                    y: 0.,
                    z: INNER_CAMERA,
                };
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn outer_button_interactions(
    outer_interaction_query: Query<
        (&Interaction, &mut BorderColor),
        (
            Changed<Interaction>,
            With<Button>,
            With<OuterButton>,
            Without<InnerButton>,
        ),
    >,
    inner_button: Single<&mut BorderColor, (With<Button>, With<InnerButton>, Without<OuterButton>)>,
    camera: Single<&mut Transform, With<Camera3d>>,
) {
    let mut camera_tranform = camera.into_inner();
    let mut inner_border_color = inner_button.into_inner();
    for (outer_interaction, mut outer_border_color) in outer_interaction_query {
        match *outer_interaction {
            Interaction::Pressed => {
                outer_border_color.0 = Color::WHITE;
                inner_border_color.0 = Color::BLACK;
                camera_tranform.translation = Vec3 {
                    x: 0.,
                    y: 0.,
                    z: OUTER_CAMERA,
                };
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
