use std::fs::{create_dir_all, File, self};
use std::io::{copy, self};
use reqwest::Result;
use std::{process::Command, path::Path};
use std::str::from_utf8;
use std::env;
use directories::{self, UserDirs};
use serde::{Serialize, Deserialize};

use crate::process::get_download_link;

#[derive(Serialize, Deserialize)]
pub struct FileInfo {
    pub file_path: String,
    pub output_dir: String
}

impl FileInfo {
    pub fn new(file_path: String, output_dir: String) -> Self {
        FileInfo {
            file_path,
            output_dir
        }
    }

    pub fn empty() -> Self {
        FileInfo {
            file_path: "".to_string(),
            output_dir: "".to_string()
        }
    }
}

pub async fn download_ffmpeg(path: &Path) -> reqwest::Result<File> {
    let download_link = get_download_link().expect("Failed to get valid download link").ffmpeg;
    let response = reqwest::get(download_link).await?;

    let mut dest = {
        let fname = response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() {None} else {Some(name)})
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
pub fn extract_zip(zip_file: File) {
    let mut archive = zip::ZipArchive::new(zip_file).expect("Failed to open zip archive");

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).expect("Failed to get file at index");
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        if (*file.name()).ends_with("/") {
            fs::create_dir_all(&outpath).expect("Failed to create directory for extracting files");
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).expect("Failed creating directory for file parent");
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

/// downloads ffmpeg and creates the path if needed
pub fn get_ffmpeg_path(path: &Path) -> &Path {
    if !path.exists() {
        create_dir_all(path).expect("Failed to create ffmpeg path");
    }

    panic!("Not implemented");
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

pub fn get_duration(input: &str) -> f32 {
    let output = Command::new("ffprobe")
        .args(
        ["-v", "error",
        "-show_entries", "format=duration",
        "-of", "csv=p=0",
        input])
        // TODO: write custom error handler
        .output()
        .expect("Failed to run ffprobe to get duration")
        .stdout;

        let duration = match from_utf8(&output) {
            Ok(value) => {
                remove_whitespace(value)
            },
            Err(e) => {
                eprintln!("Error, {}", e);
                panic!("Failed");
            }
        };

        let parsed: f32 = duration.parse().unwrap();

        parsed
}

/// Returns in kb
pub fn get_original_audio_rate(input: &str) -> f32 {
    let output = Command::new("ffprobe")
        .args(
        ["-v", "error",
        "-select_streams", "a:0",
        "-show_entries", "stream=bit_rate",
        "-of", "csv=p=0",
        input])
        .output()
        .expect("Failed to run ffprobe to get original audio rate")
        .stdout;
    
    let arate = match from_utf8(&output) {
        Ok(value) => remove_whitespace(value),
        Err(e) => {
            eprintln!("{}", e);
            panic!("Failed")
        }
    };

    if arate == "N/A" {
        return 0.00;
    }

    let parsed: f32 = arate.parse::<f32>().expect("Failed to parse original audio rate") / 1024.00;

    println!("arate: {}", arate);

    parsed
    // use 7.8
}

pub fn get_target_size(audio_rate: f32, duration: f32) -> f32 {
    let size = (audio_rate * duration) / 8192.00;
    size
}

pub fn is_minsize(min_size: f32, size: f32) -> bool {
    return min_size < size
}

/// returns in kib/s
pub fn get_target_video_rate(size: f32, duration: f32, audio_rate: f32) -> f32 {
    let size = (size * 8192.00) / (1.048576 * duration) - audio_rate;
    size
}

pub fn convert_first(input: &str, video_bitrate: f32, unix: bool) {
    let nul = if env::consts::OS == "windows" {
        "nul"
    } else {
        "/dev/null"
    };

    let output = Command::new("ffmpeg")
    .args([
        "-y",
        "-i", input,
        "-c:v", "libx264",
        "-b:v", format!("{}k", video_bitrate).as_str(),
        "-pass", "1",
        "-an",
        "-f", "mp4",
        nul
    ])
    .output()
    .expect("Failed first conversion")
    .stderr;

    println!("{}", from_utf8(&output).unwrap());
}

pub fn convert_out(input: &str, video_bitrate: f32, audio_bitrate: f32, output: &str) {
    let output = Command::new("ffmpeg")
    .args([
        "-i", input,
        "-c:v", "libx264",
        "-b:v", format!("{}k", video_bitrate).as_str(),
        "-pass", "2",
        "-c:a", "aac",
        "-b:a", format!("{}k", audio_bitrate).as_str(),
        output
    ])
    .output()
    .expect("Failed first conversion")
    .stdout;
}

pub fn format_input(input: &str) -> FileInfo {
    let file_path = Path::new(input);
    let user_dirs = UserDirs::new().expect("Failed to find user dirs");

    let vid_dir = match user_dirs.video_dir() {
        Some(vid_dir) => vid_dir.as_os_str().to_str().unwrap(),
        _ => {
            // if video dir fails, use the parent dir of the clip
            match file_path.parent() {
                Some(dir) => {
                    dir.as_os_str().to_str().unwrap()
                },
                // use current dir
                _ => "."
            }
        }
    };

    let file_name = match file_path.file_stem() {
        Some(name) => {
            name.to_str().unwrap()
        },
        _ => {
            panic!("No file name")
        }
    };

    let file_out = format!("{}-8m.mp4", file_name);
    let output_path = Path::new(vid_dir).join(file_out).as_os_str().to_str().unwrap().to_string();

    // let mut split: Vec<&str> = input.split(".").collect();
    // split.pop(); // remove file extension

    // let len = &split.len();

    // let file_name = split[len - 1];

    // let formatted = format!("{}-8m", file_name);

    // split[len - 1] = &formatted;

    // let joined = split.join(".") + ".mp4";

    FileInfo::new(output_path, vid_dir.to_string())
    // output_path

}