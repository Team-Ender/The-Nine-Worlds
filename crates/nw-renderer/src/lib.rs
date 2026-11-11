use bevy::{app::AppExit, input::mouse::AccumulatedMouseMotion, prelude::*};
use bevy_egui::EguiPlugin;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GameControlSet;

pub struct NineWorldsPlugin;

impl Plugin for NineWorldsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .insert_resource(CameraLook::default())
            .insert_resource(PhysicsClock::default())
            .insert_resource(PhysicsSettings::load_or_default())
            .add_systems(Update, control_camera.in_set(GameControlSet));
    }
}

#[derive(Resource)]
pub struct PlayerPhysics(nw_physics::FirstPersonPhysics);

impl PlayerPhysics {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self(nw_physics::FirstPersonPhysics::new()?))
    }
}

#[derive(Component)]
struct PlayerCamera;

#[derive(Component)]
struct AvatarPart(Vec3);

#[derive(Resource)]
struct CameraLook {
    yaw: f32,
    pitch: f32,
}

impl Default for CameraLook {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.25,
        }
    }
}

use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct PhysicsSettings {
    pub target_hz: f32,
    pub min_hz: f32,
    pub max_hz: f32,
}

impl Default for PhysicsSettings {
    fn default() -> Self {
        Self {
            target_hz: 60.0,
            min_hz: 30.0,
            max_hz: 480.0,
        }
    }
}

// Portable path helper placed next to the executable
pub fn get_settings_path() -> PathBuf {
    let mut path = env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
    path.pop(); // Strip the executable name to get the folder
    path.push("game.json");
    path
}

impl PhysicsSettings {
    pub fn load_or_default() -> Self {
        let path = get_settings_path();
        if path.exists() {
            if let Ok(data) = fs::read_to_string(&path) {
                if let Ok(settings) = serde_json::from_str(&data) {
                    return settings;
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        let path = get_settings_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        if let Ok(data) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, data);
        }
    }
}

#[derive(Resource, Default)]
struct PhysicsClock {
    accumulator: f32,
}

pub fn setup_scene(
    mut initialized: Local<bool>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if *initialized {
        return;
    }
    *initialized = true;

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 3.8, 13.0).looking_at(Vec3::new(0.0, 1.3, 8.0), Vec3::Y),
        PlayerCamera,
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 12_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    spawn_mesh(
        &mut commands,
        &mut meshes,
        &mut materials,
        Plane3d::default().mesh().size(100.0, 100.0).into(),
        Vec3::new(0.0, -0.01, 0.0),
        Color::srgb(0.25, 0.75, 0.95),
    );
    spawn_mesh(
        &mut commands,
        &mut meshes,
        &mut materials,
        Cuboid::new(2.0, 2.0, 2.0).into(),
        Vec3::new(-4.0, 1.0, 1.5),
        Color::srgb(0.15, 0.45, 1.0),
    );
    spawn_mesh(
        &mut commands,
        &mut meshes,
        &mut materials,
        Cone::new(1.0, 2.0).mesh().resolution(4).build(),
        Vec3::new(-1.6, 1.0, 1.5),
        Color::srgb(1.0, 0.35, 0.15),
    );
    spawn_mesh(
        &mut commands,
        &mut meshes,
        &mut materials,
        Sphere::new(1.0).into(),
        Vec3::new(0.8, 1.0, 1.5),
        Color::srgb(0.2, 0.9, 0.45),
    );
    spawn_mesh(
        &mut commands,
        &mut meshes,
        &mut materials,
        Cylinder::new(1.0, 2.0).into(),
        Vec3::new(3.2, 1.0, 1.5),
        Color::srgb(0.85, 0.7, 0.2),
    );
    spawn_mesh(
        &mut commands,
        &mut meshes,
        &mut materials,
        Cone::new(1.0, 2.0).into(),
        Vec3::new(5.6, 1.0, 1.5),
        Color::srgb(0.75, 0.35, 0.95),
    );

    spawn_avatar_part(
        &mut commands,
        &mut meshes,
        &mut materials,
        Cylinder::new(0.3, 0.75).into(),
        Vec3::new(0.0, 0.05, 0.0),
        Color::srgb(0.12, 0.32, 0.72),
    );
    spawn_avatar_part(
        &mut commands,
        &mut meshes,
        &mut materials,
        Sphere::new(0.22).into(),
        Vec3::new(0.0, 0.72, 0.0),
        Color::srgb(0.88, 0.65, 0.48),
    );
    for side in [-1.0, 1.0] {
        spawn_avatar_part(
            &mut commands,
            &mut meshes,
            &mut materials,
            Cylinder::new(0.1, 0.65).into(),
            Vec3::new(side * 0.4, 0.08, 0.0),
            Color::srgb(0.12, 0.32, 0.72),
        );
        spawn_avatar_part(
            &mut commands,
            &mut meshes,
            &mut materials,
            Cylinder::new(0.13, 0.65).into(),
            Vec3::new(side * 0.16, -0.57, 0.0),
            Color::srgb(0.16, 0.2, 0.28),
        );
    }
}

fn spawn_mesh(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    mesh: Mesh,
    position: Vec3,
    color: Color,
) {
    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: color,
            ..default()
        })),
        Transform::from_translation(position),
    ));
}

fn spawn_avatar_part(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    mesh: Mesh,
    offset: Vec3,
    color: Color,
) {
    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: color,
            ..default()
        })),
        Transform::default(),
        AvatarPart(offset),
    ));
}

fn control_camera(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut look: ResMut<CameraLook>,
    mut physics: ResMut<PlayerPhysics>,
    mut physics_clock: ResMut<PhysicsClock>,
    mut camera: Query<&mut Transform, With<PlayerCamera>>,
    mut avatar_parts: Query<(&AvatarPart, &mut Transform), Without<PlayerCamera>>,
    mut exit: MessageWriter<AppExit>,
) {
    const MOUSE_SENSITIVITY: f32 = 0.0025;
    const FIXED_STEP: f32 = 1.0 / 60.0;
    const CAMERA_DISTANCE: f32 = 5.0;

    let Ok(mut transform) = camera.single_mut() else {
        return;
    };

    look.yaw += mouse_motion.delta.x * MOUSE_SENSITIVITY;
    look.pitch = (look.pitch - mouse_motion.delta.y * MOUSE_SENSITIVITY).clamp(-1.57, 1.57);

    let forward = Vec3::new(look.yaw.sin(), 0.0, -look.yaw.cos());
    let right = Vec3::new(look.yaw.cos(), 0.0, look.yaw.sin());
    let mut movement = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        movement += forward;
    }
    if keys.pressed(KeyCode::KeyS) {
        movement -= forward;
    }
    if keys.pressed(KeyCode::KeyD) {
        movement += right;
    }
    if keys.pressed(KeyCode::KeyA) {
        movement -= right;
    }

    if movement != Vec3::ZERO {
        movement = movement.normalize() * 5.0;
    }

    if let Err(error) = physics
        .0
        .set_horizontal_velocity(glam::Vec3::new(movement.x, movement.y, movement.z))
    {
        error!("Could not set player velocity: {error}");
        exit.write(AppExit::error());
        return;
    }

    if keys.just_pressed(KeyCode::Space) {
        if let Err(error) = physics.0.jump() {
            error!("Could not jump: {error}");
            exit.write(AppExit::error());
            return;
        }
    }

    physics_clock.accumulator += time.delta_secs().min(0.1);
    while physics_clock.accumulator >= FIXED_STEP {
        if let Err(error) = physics.0.step(FIXED_STEP) {
            error!("Could not step player physics: {error}");
            exit.write(AppExit::error());
            return;
        }
        physics_clock.accumulator -= FIXED_STEP;
    }

    match physics.0.player_position() {
        Ok(position) => {
            let player_position = Vec3::from_array(position.to_array());
            let facing = Quat::from_rotation_y(-look.yaw);
            for (part, mut part_transform) in &mut avatar_parts {
                part_transform.translation = player_position + facing * part.0;
                part_transform.rotation = facing;
            }

            let camera_target = player_position + Vec3::new(0.0, 0.3, 0.0);
            let camera_rotation = Quat::from_euler(EulerRot::YXZ, -look.yaw, look.pitch, 0.0);
            transform.translation =
                camera_target + camera_rotation * Vec3::new(0.0, 0.0, CAMERA_DISTANCE);
            transform.look_at(camera_target, Vec3::Y);
        }
        Err(error) => {
            error!("Could not read player position: {error}");
            exit.write(AppExit::error());
        }
    }
}
