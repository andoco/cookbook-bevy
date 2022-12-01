use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(handle_connected)
        .add_system(handle_event)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 0.5, 0.),
        ..default()
    });

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
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    for event in events.iter() {
        info!("{:?}", event);

        match event.event_type {
            GamepadEventType::AxisChanged(GamepadAxisType::LeftStickX, value) => {
                let angle = 10. * value.to_radians();
                camera_query.single_mut().rotate_y(angle);
            }
            GamepadEventType::AxisChanged(GamepadAxisType::LeftStickY, value) => {
                let angle = 10. * value.to_radians();
                camera_query.single_mut().rotate_x(angle);
            }
            _ => {}
        }
    }
}
