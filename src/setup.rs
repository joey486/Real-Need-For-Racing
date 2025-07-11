use bevy::prelude::*;
use bevy::asset::AssetServer;
use crate::components::*;
use crate::ui::spawn_game_over_ui;
use bevy_kira_audio::{Audio, AudioControl};

use bevy::window::PrimaryWindow;
use windows::core::PCWSTR;

#[cfg(target_os = "windows")]
use bevy::winit::WinitWindows;
#[cfg(target_os = "windows")]




pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Road background
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.2, 0.2, 0.2),
                custom_size: Some(Vec2::new(800.0, 600.0)),
                ..default()
            },
            ..default()
        },
        Road,
    ));

    let line_texture = asset_server.load("white_line.png");
    for x in [-200.0, 0.0, 200.0] {
        super::systems::road::spawn_road_lines(&mut commands, line_texture.clone(), x, 10);
    }

    // Player with collision bounds
    let player_car_type = CarType::Player;
    let player_collision_bounds = player_car_type.collision_bounds();

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(player_car_type.asset_path()),
            transform: Transform {
                translation: Vec3::new(0.0, -200.0, 10.0),
                scale: Vec3::splat(0.1),
                ..default()
            },
            ..default()
        },
        Player,
        Velocity { speed: 150.0 },
        player_car_type,
        CollisionBounds {
            size: player_collision_bounds,
        },
    ));

    // UI
    spawn_game_over_ui(commands);
}

pub fn play_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let music = asset_server.load("audio/background_audio.ogg");
    audio.play(music).looped();
}

#[cfg(target_os = "windows")]
pub fn set_windows_titlebar_icon(
    window_query: Query<Entity, With<PrimaryWindow>>,
    winit_windows: NonSend<WinitWindows>,
) {
    use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use windows::Win32::Foundation::*;
    use windows::Win32::UI::WindowsAndMessaging::*;

    fn to_wide(s: &str) -> Vec<u16> {
        OsStr::new(s).encode_wide().chain(once(0)).collect()
    }

    if let Ok(window_entity) = window_query.get_single() {
        if let Some(winit_window) = winit_windows.get_window(window_entity) {
            if let RawWindowHandle::Win32(handle) = winit_window.raw_window_handle() {
                let hwnd = HWND(handle.hwnd as isize);
                let icon_path = to_wide("assets/icon.ico");

                unsafe {
                    match LoadImageW(
                        None,
                        PCWSTR(icon_path.as_ptr()),
                        IMAGE_ICON,
                        32,
                        32,
                        LR_LOADFROMFILE,
                    ) {
                        Ok(hicon) => {
                            SendMessageW(hwnd, WM_SETICON, WPARAM(ICON_BIG as usize), LPARAM(hicon.0 as isize));
                            SendMessageW(hwnd, WM_SETICON, WPARAM(ICON_SMALL as usize), LPARAM(hicon.0 as isize));
                        }
                        Err(e) => {
                            eprintln!("Failed to load icon: {:?}", e);
                        }
                    }
                }
            }
        }
    }
}