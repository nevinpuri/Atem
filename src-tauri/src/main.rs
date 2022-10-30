use compressor::{ffmpeg::{get_duration, get_original_audio_rate, get_target_size, is_minsize, get_target_video_rate, convert_first, convert_out, get_output_dir, FileInfo, get_ffmpeg_path, extract_zip, download_file}, process::get_download_link};
use tauri::{api::{process::Command, dialog::message}, Manager};
use std::{env, path::Path};

pub mod ffmpeg;
pub mod process;

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command(async)]
async fn setup(base_path: &str) {
    let ffmpeg_path = Path::new(base_path).join("ffmpeg/");
    let ffmpeg_url = get_download_link().expect("Failed to get download link").ffmpeg;
    println!("got download link {}", ffmpeg_url);
    println!("downloading ffmpeg zip to {:#?}", &ffmpeg_path);
    let zip = download_file(&ffmpeg_path, &ffmpeg_url).await.expect("Failed to download ffmpeg");
    println!("extracting ffmpeg");
    extract_zip(zip).expect("Failed to extract ffmpeg");
    println!("done")
}

#[tauri::command(async)]
fn open_file_explorer(path: &str, window: tauri::Window) {
    let label = window.label();
    let parent_window = window.get_window(label).unwrap();
    match env::consts::OS {
        "windows" => {
            Command::new("explorer")
            .args(["/select,", path]) // The comma after select is not a typo
            .spawn()
            .unwrap();
        },
        "macos" => {
            Command::new( "open" )
            .args(["-R", path]) // i don't have a mac so not 100% sure
            .spawn()
            .unwrap();
        },
        _ => {
            tauri::async_runtime::spawn(async move {
                message(Some(&parent_window), "Unsupported OS", "Opening a file browser is unsupported on linux");
            });
        }
    }
}

#[tauri::command(async)]
fn convert_video(input: &str, target_size: f32, base_dir: &str) -> FileInfo {
    let path = Path::new(&base_dir).join("ffmpeg");
    let ffmpeg_path = get_ffmpeg_path(&path);

    let output = get_output_dir(input);

    let duration = get_duration(input);
    let audio_rate = get_original_audio_rate(input);
    let min_size = get_target_size(audio_rate, duration);

    if !is_minsize(min_size, target_size) {
        return FileInfo::empty();
    }

    let target_bitrate = get_target_video_rate(target_size, duration, audio_rate);
    convert_first(input, target_bitrate, true);
    convert_out(input, target_bitrate, audio_rate, &output.file_path);

    return output;
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_file_explorer])
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![convert_video])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
