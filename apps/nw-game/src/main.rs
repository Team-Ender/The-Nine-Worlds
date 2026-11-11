use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};

mod menus;
mod states;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::new()
        .insert_resource(nw_renderer::PlayerPhysics::new()?)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "The Nine Worlds".into(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            primary_cursor_options: Some(CursorOptions {
                visible: true,
                grab_mode: CursorGrabMode::None,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((nw_renderer::NineWorldsPlugin, menus::MenuPlugin))
        .add_systems(
            OnEnter(states::GameState::Singleplayer),
            nw_renderer::setup_scene,
        )
        .configure_sets(
            Update,
            nw_renderer::GameControlSet.run_if(in_state(states::GameState::Singleplayer)),
        )
        .run();
    Ok(())
}
