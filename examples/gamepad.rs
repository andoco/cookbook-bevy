use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(handle_connected)
        .add_system(handle_event)
        .run();
}

fn handle_connected(gamepads: Res<Gamepads>) {
    if !gamepads.is_changed() {
        return;
    }

    info!("{:?}", gamepads);
}

fn handle_event(mut events: EventReader<GamepadEvent>) {
    for event in events.iter() {
        info!("{:?}", event);
    }
}
