#![windows_subsystem = "windows"]
use atem::{
    ffmpeg::{
        convert_first, convert_out, get_duration,
        get_original_audio_rate, get_output, get_target_size, get_target_video_rate, is_minsize,
    },
};
use std::env;
use tauri::{
    api::{dialog::message, process::Command},
    Manager,
};

pub mod ffmpeg;

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command(async)]
fn open_file_explorer(path: &str, window: tauri::Window) {
    let label = window.label();
    let parent_window = window.get_window(label).unwrap();
    println!("{}", path);
    match env::consts::OS {
        "windows" => {
            Command::new("explorer")
                .args(["/select,", path]) // The comma after select is not a typo
                .spawn()
                .unwrap();
        },
        "macos" => {
            Command::new("open")
                .args(["-R", path]) // i don't have a mac so not 100% sure
                .spawn()
                .unwrap();
        }
        _ => {
            tauri::async_runtime::spawn(async move {
                message(
                    Some(&parent_window),
                    "Unsupported OS",
                    "Opening a file browser is unsupported on linux",
                );
            });
        }
    }
}

#[tauri::command(async)]
fn convert_video(input: &str, target_size: f32) -> String {
    let output = get_output(input);

    let duration = get_duration(input);
    let audio_rate = get_original_audio_rate(input);
    let min_size = get_target_size(audio_rate, duration);

    if !is_minsize(min_size, target_size) {
        println!("{min_size}");
        return "".to_string();
    }

    let target_bitrate = get_target_video_rate(target_size, duration, audio_rate);
    convert_first(input, target_bitrate);
    convert_out(
        input,
        target_bitrate,
        audio_rate,
        &output,
    );

    return output;
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            open_file_explorer,
            convert_video
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
