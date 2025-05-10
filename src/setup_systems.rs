use crate::{components::*, constants::INNER_CAMERA};
use astro::planet;
use bevy::{color::palettes::css::ORANGE, prelude::*};

pub fn add_buttons(mut commands: Commands, asset_server: Res<AssetServer>) {
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

pub fn add_planets(
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

pub fn add_camera(mut commands: Commands) {
    info!("Adding camera");
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, INNER_CAMERA).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub fn add_light(mut commands: Commands) {
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

pub fn add_planet_labels(
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

pub fn add_date_label(mut commands: Commands) {
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
