use bevy::{
    app::AppExit,
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};

use crate::states::GameState;

pub struct MenuPlugin;

#[derive(Resource)]
struct MultiplayerConfig {
    server_ip: String,
}

impl Default for MultiplayerConfig {
    fn default() -> Self {
        Self {
            server_ip: "127.0.0.1".to_owned(),
        }
    }
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .init_resource::<MultiplayerConfig>()
            .add_systems(Startup, setup_menu_camera)
            .add_systems(Update, toggle_pause)
            .add_systems(
                EguiPrimaryContextPass,
                title_screen_ui.run_if(in_state(GameState::TitleScreen)),
            )
            .add_systems(
                EguiPrimaryContextPass,
                pause_menu_ui.run_if(in_state(GameState::Paused)),
            )
            .add_systems(
                EguiPrimaryContextPass,
                multiplayer_lobby_ui.run_if(in_state(GameState::MultiplayerLobby)),
            )
            .add_systems(OnEnter(GameState::Singleplayer), lock_cursor)
            .add_systems(OnEnter(GameState::Paused), unlock_cursor)
            .add_systems(OnEnter(GameState::TitleScreen), unlock_cursor);
    }
}

fn setup_menu_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
    ));
}

fn title_screen_ui(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) -> Result {
    let context = contexts.ctx_mut()?;
    context
        .layer_painter(egui::LayerId::background())
        .rect_filled(
            context.viewport_rect(),
            0.0,
            egui::Color32::from_rgb(12, 17, 26),
        );

    egui::Window::new("The Nine Worlds")
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .resizable(false)
        .show(context, |ui| {
            ui.heading("The Nine Worlds");
            ui.label("A Norse-themed RPG");
            ui.add_space(20.0);

            if ui.button("Singleplayer").clicked() {
                next_state.set(GameState::Singleplayer);
            }
            if ui.button("Multiplayer").clicked() {
                next_state.set(GameState::MultiplayerLobby);
            }
            if ui.button("Quit").clicked() {
                exit.write(AppExit::Success);
            }
        });

    Ok(())
}

fn pause_menu_ui(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
) -> Result {
    egui::Window::new("Game Paused")
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .resizable(false)
        .show(contexts.ctx_mut()?, |ui| {
            ui.heading("Paused");
            if ui.button("Resume").clicked() {
                next_state.set(GameState::Singleplayer);
            }
            if ui.button("Quit to Title").clicked() {
                next_state.set(GameState::TitleScreen);
            }
        });

    Ok(())
}

fn toggle_pause(
    keys: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::Singleplayer | GameState::InGameMultiplayer => {
                next_state.set(GameState::Paused)
            }
            GameState::Paused => next_state.set(GameState::Singleplayer),
            GameState::TitleScreen | GameState::SettingsMenu | GameState::MultiplayerLobby => {}
        }
    }
}

fn lock_cursor(mut cursor_options: Query<&mut CursorOptions, With<PrimaryWindow>>) {
    for mut cursor in &mut cursor_options {
        cursor.visible = false;
        cursor.grab_mode = CursorGrabMode::Locked;
    }
}

fn unlock_cursor(mut cursor_options: Query<&mut CursorOptions, With<PrimaryWindow>>) {
    for mut cursor in &mut cursor_options {
        cursor.visible = true;
        cursor.grab_mode = CursorGrabMode::None;
    }
}

// 2. Multiplayer Lobby (Hosting & Connecting to remote servers)
fn multiplayer_lobby_ui(
    mut contexts: EguiContexts,
    mut config: ResMut<MultiplayerConfig>,
    mut next_state: ResMut<NextState<GameState>>,
) -> Result {
    let context = contexts.ctx_mut()?;
    egui::Window::new("Multiplayer Lobby")
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .resizable(false)
        .show(context, |ui| {
            ui.heading("Multiplayer Setup");
            ui.add_space(10.0);

            ui.label("Server IP Address:");
            ui.text_edit_singleline(&mut config.server_ip);
            ui.add_space(10.0);

            ui.add_enabled(
                false,
                egui::Button::new(format!("Connect to {} (Coming Soon)", config.server_ip)),
            );
            ui.add_enabled(false, egui::Button::new("Host Server (Coming Soon)"));

            ui.add_space(10.0);
            if ui.button("Back").clicked() {
                next_state.set(GameState::TitleScreen);
            }
        });
    Ok(())
}
