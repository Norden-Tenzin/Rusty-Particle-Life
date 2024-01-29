use bevy::{
    ecs::{ component, entity },
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::PresentMode,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rand::prelude::*;

const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 720.0;
const CIRCLE_RADIUS: f32 = 10.0;
const CIRCLE_COUNT: i32 = 10;

const BALL_SPEED: f32 = 400.0;
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);
const GRAVITATIONAL_VALUE: f32 = -1.0;

#[derive(Component)]
struct Circle;

#[derive(Component, Deref, DerefMut, Debug)]
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
        .add_systems(Update, force)
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

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(CIRCLE_RADIUS).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                ..default()
            },
            Circle,
            Velocity(Vec2::ZERO),
            // Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED),
        ));
    }
}

fn force(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Circle>>
) {
    let mut combinations = query.iter_combinations_mut();

    while let Some([mut c1, mut c2]) = combinations.fetch_next() {
        // update velocity and acceleration
        let (c1_transform, c1_velocity) = &mut c1;
        let (c2_transform, c2_velocity) = &mut c2;
        let mut f1x = 0.0;
        let mut f1y = 0.0;

        let mut f2x = 0.0;
        let mut f2y = 0.0;

        // get the x from transforms for both circlesj
        let dx = c1_transform.translation.x - c2_transform.translation.x;
        let dy = c1_transform.translation.y - c2_transform.translation.y;
        let d = (dx.powi(2) + dy.powi(2)).sqrt();

        // check if distance is great enough
        if d > 0.0 && d < 200.0 {
            let force1 = GRAVITATIONAL_VALUE / d;
            f1x += force1 * dx;
            f1y += force1 * dy;
            c1_velocity.x += f1x * 0.5;
            c1_velocity.y += f1y * 0.5;
            c1_transform.translation.x += c1_velocity.x;
            c1_transform.translation.y += c1_velocity.y;

            let force2 = GRAVITATIONAL_VALUE / d;
            f2x += force2 * -dx;
            f2y += force2 * -dy;
            c2_velocity.x += f2x * 0.5;
            c2_velocity.y += f2y * 0.5;
            c2_transform.translation.x += c2_velocity.x;
            c2_transform.translation.y += c2_velocity.y;
        }

        // bounds
        if (c1_transform.translation.x <= -WIDTH/2.0 || c1_transform.translation.x >= WIDTH/2.0) {c1_velocity.x *= -1.0} 
        if (c1_transform.translation.y <= -HEIGHT/2.0 || c1_transform.translation.y >= HEIGHT/2.0) {c1_velocity.y *= -1.0} 

        if (c2_transform.translation.x <= -WIDTH/2.0 || c2_transform.translation.x >= WIDTH/2.0) {c2_velocity.x *= -1.0} 
        if (c2_transform.translation.y <= -HEIGHT/2.0 || c2_transform.translation.y >= HEIGHT/2.0) {c2_velocity.y *= -1.0} 
    }
}
