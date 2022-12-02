use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(handle_connected)
        .add_system(handle_event)
        .add_system(move_camera)
        .run();
}

#[derive(Component)]
struct Speed(Vec2);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0., 0.5, 0.),
            ..default()
        })
        .insert(Speed(Vec2::ZERO));

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane { size: 50. }.into()),
        material: materials.add(Color::GREEN.into()),
        ..default()
    });

    // red cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Cube::default().into()),
        material: materials.add(Color::RED.into()),
        transform: Transform::from_xyz(0., 0.5, -5.),
        ..default()
    });
}

fn handle_connected(gamepads: Res<Gamepads>) {
    if !gamepads.is_changed() {
        return;
    }

    info!("{:?}", gamepads);
}

fn handle_event(
    mut events: EventReader<GamepadEvent>,
    mut camera_query: Query<&mut Speed, With<Camera>>,
) {
    for event in events.iter() {
        info!("{:?}", event);

        match event.event_type {
            GamepadEventType::AxisChanged(GamepadAxisType::LeftStickX, value) => {
                camera_query.single_mut().0.x = value;
            }
            GamepadEventType::AxisChanged(GamepadAxisType::LeftStickY, value) => {
                camera_query.single_mut().0.y = -value;
            }
            _ => {}
        }
    }
}

fn move_camera(mut camera_query: Query<(&Speed, &mut Transform), With<Camera>>) {
    if let Ok((speed, mut tx)) = camera_query.get_single_mut() {
        tx.rotate_y(-speed.0.x.to_radians());
        let dir = tx.local_z();
        tx.translation += dir * speed.0.y;
    }
}
