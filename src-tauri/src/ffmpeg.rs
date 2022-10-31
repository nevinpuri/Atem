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

use crate::process::get_download_link;

#[derive(Serialize, Deserialize)]

/// file path is the full path inluding the video name, and output_dir is only the output dir
pub struct OutFile {
    pub full_path: String,
    pub explorer_dir: String,
}

pub struct FFmpegProcess {
    ffmpeg_path: String,
}

impl FFmpegProcess {
    pub fn new(base_dir: &Path) -> Self {
        FFmpegProcess {
            ffmpeg_path: "undefined".to_string(),
        }
    }

    pub fn compress(file: &Path, output: &Path) {}

    pub fn get_ffmpeg_path(path: &Path) -> PathBuf {
        let ffmpeg_path = path.join("ffmpeg/");

        if !ffmpeg_path.exists() {
            panic!("ffmpeg not installed");
        }

        ffmpeg_path
    }
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

pub async fn download_file(path: &Path, link: &str) -> reqwest::Result<File> {
    // let download_link = get_download_link()?;

    let response = reqwest::get(link).await?;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("file to download, {}", fname);
        let fname = path.join(fname);
        println!("will be located under {:#?}", fname);
        File::create(fname).expect("Failed to create file")
    };

    let content = response.text().await?;
    copy(&mut content.as_bytes(), &mut dest).expect("Failed to copy downloaded bytes to file");

    Ok(dest)
}

// todo: refactor to return io error instead of panic
pub fn extract_zip(zip_file: File) -> Result<(), Error> {
    let mut archive = zip::ZipArchive::new(zip_file).expect("Failed to open zip archive");

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        if (*file.name()).ends_with("/") {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }

            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
                }
            }
        }
    }

    Ok(())
}

pub fn get_ff_path(exe_name: &str) -> PathBuf {
    let cur_dir = match env::current_dir() {
        Ok(dir) => {
            let exe: &str = match consts::OS {
                "windows" => ".exe",
                _ => "",
            };

            dir.join(format!("{}/{}{}", exe_name, exe_name, exe))
        }
        Err(e) => {
            panic!("Failed getting exe name dir")
        }
    };

    cur_dir
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

// copy ffmpeg-adsf to ffmpeg
pub fn get_duration(input: &str, ffprobe_path: &Path) -> f32 {
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
pub fn get_original_audio_rate(input: &str, ffprobe_path: &Path) -> f32 {
    let output = Command::new_sidecar("ffprobe")
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
        .expect("Failed to run ffprobe to get original audio rate")
        .stdout;

    let arate = remove_whitespace(&output);

    if arate == "N/A" {
        return 0.00;
    }

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

pub fn convert_first(input: &str, video_bitrate: f32, ffmpeg_path: &Path) {
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
    ffmpeg_path: &Path,
    output: &str,
) {
    let output = Command::new_sidecar("ffmpeg")
        .expect("failed to get ffmpeg sidecar")
        .args([
            "-i",
            input,
            "-c:v",
            "libx264",
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

pub fn get_output(input: &str) -> OutFile {
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

    // let mut split: Vec<&str> = input.split(".").collect();
    // split.pop(); // remove file extension

    // let len = &split.len();

    // let file_name = split[len - 1];

    // let formatted = format!("{}-8m", file_name);

    // split[len - 1] = &formatted;

    // let joined = split.join(".") + ".mp4";

    OutFile::new(output_path, vid_dir.to_string())
    // output_path
}
