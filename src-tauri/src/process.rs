use std::env::consts;

pub struct DownloadLink {
    pub ffmpeg: String,
    pub ffprobe: String
}

pub fn get_download_link() -> Option<DownloadLink> {
    let os: &str = match consts::OS {
        "windows" => "win",
        "linux" => "linux",
        "macos" => "osx",
        _ => {
            return None;
        }
    };

    let arch: &str = match consts::ARCH {
        "x86" => "32",
        "x86_64" => "64",
        "aarch64" => "arm-64",
        _ => {
            return None;
        }
    };

    if os == "macos" && arch == "arm-64" {
        return None;
    }

    let download_link_ffmpeg = format!("https://github.com/ffbinaries/ffbinaries-prebuilt/releases/download/v4.4.1/ffmpeg-4.4.1-{}-{}.zip", os, arch);
    let download_link_ffprobe = format!("https://github.com/ffbinaries/ffbinaries-prebuilt/releases/download/v4.4.1/ffprobe-4.4.1-{}-{}.zip", os, arch);

    Some(DownloadLink {
        ffmpeg: download_link_ffmpeg,
        ffprobe: download_link_ffprobe
    })
}