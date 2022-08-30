use std::process::Command;
use std::str::from_utf8;

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
    let nul = if unix {
        "/dev/null"
    } else {
        "nul"
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