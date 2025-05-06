use astro::*;
use bevy::{color::palettes::css::ORANGE, prelude::*};
use chrono::{DateTime, Datelike, Duration, Timelike, Utc};

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

#[derive(Component)]
struct Planet {
    name: String,
    astro_enum: planet::Planet,
    radius: f32,
}

#[derive(Component)]
struct DateLabel;

#[derive(Component)]
struct PlanetLabel {
    entity: Entity,
}

#[derive(Resource)]
struct ShownDate {
    date: DateTime<Utc>,
}

#[derive(Component)]
struct InnerButton;

#[derive(Component)]
struct OuterButton;

const INNER_CAMERA: f32 = 5.0;
const OUTER_CAMERA: f32 = 60.0;

fn add_buttons(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Button,
        InnerButton,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Percent(20.0),
            border: UiRect::all(Val::Px(3.)),
            padding: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        BorderColor(Color::WHITE),
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        BorderRadius::all(Val::Px(10.0)),
        children![(
            Text::new("Inner"),
            TextFont {
                font: asset_server.load("fonts/good timing bd.otf"),
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            TextShadow::default(),
        )],
    ));

    commands.spawn((
        Button,
        OuterButton,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Percent(30.0),
            border: UiRect::all(Val::Px(3.)),
            padding: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        BorderColor(Color::BLACK),
        BorderRadius::all(Val::Px(10.0)),
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        children![(
            Text::new("Outer"),
            TextFont {
                font: asset_server.load("fonts/good timing bd.otf"),
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            TextShadow::default(),
        )],
    ));
}

fn add_planets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("Adding planets");
    let planet_material = materials.add(Color::srgb_u8(255, 255, 255));

    let planets = [
        Planet {
            name: "Mercury".to_string(),
            astro_enum: planet::Planet::Mercury,
            radius: 0.01,
        },
        Planet {
            name: "Venus".to_string(),
            astro_enum: planet::Planet::Venus,
            radius: 0.03,
        },
        Planet {
            name: "Earth".to_string(),
            astro_enum: planet::Planet::Earth,
            radius: 0.03,
        },
        Planet {
            name: "Mars".to_string(),
            astro_enum: planet::Planet::Mars,
            radius: 0.01,
        },
        Planet {
            name: "Jupiter".to_string(),
            astro_enum: planet::Planet::Jupiter,
            radius: 0.05,
        },
        Planet {
            name: "Saturn".to_string(),
            astro_enum: planet::Planet::Saturn,
            radius: 0.05,
        },
        Planet {
            name: "Uranus".to_string(),
            astro_enum: planet::Planet::Uranus,
            radius: 0.05,
        },
        Planet {
            name: "Neptune".to_string(),
            astro_enum: planet::Planet::Neptune,
            radius: 0.05,
        },
    ];

    for planet in planets {
        let radius = planet.radius;
        commands.spawn((
            planet,
            Mesh3d(meshes.add(Sphere::new(radius))),
            MeshMaterial3d(planet_material.clone()),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));
    }

    // add sun
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.06))),
        MeshMaterial3d(planet_material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn add_camera(mut commands: Commands) {
    info!("Adding camera");
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, INNER_CAMERA).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn add_light(mut commands: Commands) {
    info!("Adding light");
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn add_planet_labels(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &Planet)>,
) {
    info!("Adding labels to planets");

    let text_style = TextFont {
        font: asset_server.load("fonts/good timing bd.otf"),
        font_size: 16.0,
        ..default()
    };

    let label_text_style = (text_style.clone(), TextColor(ORANGE.into()));

    let mut label = |entity: Entity, label: &str| {
        commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    ..default()
                },
                PlanetLabel { entity },
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new(label),
                    label_text_style.clone(),
                    Node {
                        position_type: PositionType::Absolute,
                        bottom: Val::ZERO,
                        ..default()
                    },
                    TextLayout::default().with_no_wrap(),
                ));
            });
    };

    for (entity, planet) in query.iter() {
        label(entity, planet.name.as_str());
    }
}

fn add_date_label(mut commands: Commands) {
    commands.spawn((
        DateLabel,
        Text::default(),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

fn update_date_label(mut text: Single<&mut Text, With<DateLabel>>, date: Res<ShownDate>) {
    text.0 = format!("{}", date.date.format("%Y-%m-%d"));
}

fn update_date(timer: Res<Time>, mut date: ResMut<ShownDate>) {
    let seconds_per_second = 2592000.0;
    date.date = date.date + Duration::seconds((timer.delta_secs() * seconds_per_second) as i64);
}

fn update_planets(mut planets: Query<(&mut Transform, &Planet)>, date: Res<ShownDate>) {
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

fn update_planet_labels(
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

fn inner_button_interactions(
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

fn outer_button_interactions(
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
