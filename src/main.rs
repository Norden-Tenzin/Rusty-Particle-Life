use bevy::{ ecs::component, prelude::*, sprite::MaterialMesh2dBundle, window::PresentMode };
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rand::prelude::*;

const WIDTH: f32 = 1920.0;
const HEIGHT: f32 = 1080.0;
const CIRCLE_RADIUS: f32 = 10.0;
const CIRCLE_COUNT: i32 = 2;

#[derive(Component)]
struct Circle;

#[derive(Component)]
struct Velocity(Vec2);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "App".into(),
                    resolution: (WIDTH, HEIGHT).into(),
                    present_mode: PresentMode::AutoVsync,
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, (setup, spawn_circles))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_circles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let mut rng = thread_rng();
    let bounds: (f32, f32) = (WIDTH / 2.0, HEIGHT / 2.0);
    for _ in 0..CIRCLE_COUNT {
        let x: f32 = rng.gen_range(-(bounds.0 as i32)..bounds.0 as i32) as f32;
        let y: f32 = rng.gen_range(-(bounds.1 as i32)..bounds.1 as i32) as f32;

        println!("x -> {} and y -> {}", x, y);

        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(CIRCLE_RADIUS).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
            ..default()
        });
    }
}
