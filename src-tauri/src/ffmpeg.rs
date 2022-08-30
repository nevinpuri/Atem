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