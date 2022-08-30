use compressor::ffmpeg::{get_duration, get_original_audio_rate, get_target_size, is_minsize, get_target_video_rate, convert_first, convert_out};

pub mod ffmpeg;

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri:command]
fn convert_video(input: &str, target_size: f32) ->  bool {
    let output = format!("{}-8m.mp4", input).as_str();
    let duration = get_duration(input);
    let audio_rate = get_original_audio_rate(input);
    let min_size = get_target_size(audio_rate, duration);

    if !is_minsize(min_size, size) {
        return false;
    }

    let target_bitrate = get_target_video_rate(size, duration, audio_rate);
    convert_first(input, target_bitrate, true);
    convert_out(input, target_bitrate, audio_bitrate, output);
    true
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
