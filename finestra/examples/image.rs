// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use finestra::*;

#[derive(Default)]
struct Application;

#[cfg(target_os = "macos")]
fn get_image_file() -> std::path::PathBuf {
    use std::fs::read_dir;

    let directory = "/System/Library/Desktop Pictures";
    for entry in read_dir(directory).unwrap().flatten() {
        if entry.file_name().to_string_lossy().ends_with(".heic") {
            return entry.path();
        }
    }

    panic!("Failed to find a macOS picture")
}

#[cfg(target_os = "windows")]
fn get_image_file() -> String {
    panic!("Failed to find a Windows wallpaper")
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn get_image_file() -> String {
    panic!("Failed to find an image")
}

impl AppDelegate<AppState> for Application {
    fn make_content_view(&mut self, state: &mut AppState, _: Window) -> impl finestra::View<Self, AppState>  {
        _ = state;
        ImageView::new()
            .with(Image::with_contents_of_file(get_image_file()))
    }
}

#[derive(Debug, Default)]
struct AppState {
}

fn main() {
    App::with_state(Application, AppState::default())
        .run();
}
