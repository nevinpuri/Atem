use directories::{self, UserDirs};
use reqwest;
use serde::{Deserialize, Serialize};
use std::env::consts;
use std::fs::{self, create_dir_all, File};
use std::io::{self, copy, Error};
use std::path::Path;
use std::path::PathBuf;
use std::{env, path};
use tauri::api::process::Command;

#[derive(Serialize, Deserialize)]
/// file path is the full path inluding the video name, and output_dir is only the output dir
pub struct OutFile {
    pub full_path: String,
    pub explorer_dir: String,
}

impl OutFile {
    pub fn new(file_path: String, output_dir: String) -> Self {
        OutFile {
            full_path: file_path,
            explorer_dir: output_dir,
        }
    }

    pub fn empty() -> Self {
        OutFile {
            full_path: "".to_string(),
            explorer_dir: "".to_string(),
        }
    }
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

// copy ffmpeg-adsf to ffmpeg
pub fn get_duration(input: &str) -> f32 {
    let output = Command::new_sidecar("ffprobe")
        .expect("failed to find ffprobe sidecar")
        .args([
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "csv=p=0",
            input,
        ])
        // TODO: write custom error handler
        .output()
        .expect("Failed to run ffprobe to get duration")
        .stdout;

    let duration = remove_whitespace(&output);

    let parsed: f32 = duration.parse().unwrap();

    parsed
}

/// Returns in kb
pub fn get_original_audio_rate(input: &str) -> f32 {
    let out = Command::new_sidecar("ffprobe")
        .expect("failed to find ffprobe sidecar")
        .args([
            "-v",
            "error",
            "-select_streams",
            "a:0",
            "-show_entries",
            "stream=bit_rate",
            "-of",
            "csv=p=0",
            input,
        ])
        .output()
        .expect("Failed to run ffprobe to get original audio rate");

    let output = out.stdout;

    let arate = remove_whitespace(&output);

    if arate == "N/A" {
        return 0.00;
    }

    println!("arate {}", arate);

    let parsed: f32 = arate
        .parse::<f32>()
        .expect("Failed to parse original audio rate")
        / 1024.00;

    println!("arate: {}", arate);

    parsed
    // use 7.8
}

pub fn get_target_size(audio_rate: f32, duration: f32) -> f32 {
    let size = (audio_rate * duration) / 8192.00;
    size
}

pub fn is_minsize(min_size: f32, size: f32) -> bool {
    return min_size < size;
}

/// returns in kib/s
pub fn get_target_video_rate(size: f32, duration: f32, audio_rate: f32) -> f32 {
    let size = (size * 8192.00) / (1.048576 * duration) - audio_rate;
    size
}

pub fn convert_first(input: &str, video_bitrate: f32) {
    let temp_dir = env::temp_dir();
    let nul = if env::consts::OS == "windows" {
        "nul"
    } else {
        "/dev/null"
    };

    // make 1280:-1 conditional if video is already smaller than that
    let output = Command::new_sidecar("ffmpeg")
        .expect("failed to get ffmpeg sidecar")
        .args([
            "-y",
            "-i",
            input,
            "-c:v",
            "libx264",
            "-passlogfile",
            temp_dir.to_str().expect("Failed to convert temp dir to string"),
            "-filter:v",
            "scale=1280:-1",
            "-b:v",
            format!("{}k", video_bitrate).as_str(),
            "-pass",
            "1",
            "-an",
            "-f",
            "mp4",
            nul,
        ])
        .output()
        .expect("Failed first conversion")
        .stderr;

    println!("{}", &output);
}

pub fn convert_out(
    input: &str,
    video_bitrate: f32,
    audio_bitrate: f32,
    output: &str,
) {
    let temp_dir = env::temp_dir();
    let output = Command::new_sidecar("ffmpeg")
        .expect("failed to get ffmpeg sidecar")
        .args([
            "-i",
            input,
            "-c:v",
            "libx264",
            "-passlogfile",
            temp_dir.to_str().expect("Failed to convert temp dir to string"),
            "-filter:v",
            "scale=1280:-1",
            "-b:v",
            format!("{}k", video_bitrate).as_str(),
            "-pass",
            "2",
            "-c:a",
            "aac",
            "-b:a",
            format!("{}k", audio_bitrate).as_str(),
            output,
        ])
        .output()
        .expect("Failed first conversion")
        .stdout;
}

pub fn get_output(input: &str) -> String {
    let file_path = Path::new(input);
    let user_dirs = UserDirs::new().expect("Failed to find user dirs");

    let vid_dir = match user_dirs.video_dir() {
        Some(vid_dir) => vid_dir.as_os_str().to_str().unwrap(),
        _ => {
            // if video dir fails, use the parent dir of the clip
            match file_path.parent() {
                Some(dir) => dir.as_os_str().to_str().unwrap(),
                // use current dir
                _ => ".",
            }
        }
    };

    let file_name = match file_path.file_stem() {
        Some(name) => name.to_str().unwrap(),
        _ => {
            panic!("No file name")
        }
    };

    let file_out = format!("{}-8m.mp4", file_name);
    let output_path = Path::new(vid_dir)
        .join(file_out)
        .as_os_str()
        .to_str()
        .unwrap()
        .to_string();

    output_path
}
