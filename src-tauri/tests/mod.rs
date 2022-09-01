use compressor::ffmpeg::*;

#[test]
fn test_utils() {
    let input = "/home/nevin/Desktop/ffmpeg/teaser.mkv";
    assert_eq!(get_duration(&input), 30.033);
    assert_eq!(get_original_audio_rate(&input), 0.00);
    // get_duration("/home/nevin/Desktop/ffmpeg/teaser.mkv");
}

#[test]
fn test_target_size() {
    let input = "/home/nevin/Desktop/ffmpeg/teaser.mkv";
    let duration = get_duration(input);
    let audio_rate = get_original_audio_rate(input);
    let target_size = get_target_size(audio_rate, duration);
    println!("target size: {}", target_size);
}

// #[test]
fn test_convert() {
    let input = "/home/nevin/Desktop/ffmpeg/teaser.mkv";
    let target_size = 7.8;
    let duration = get_duration(input);
    let audio_rate = get_original_audio_rate(input);
    let min_size = get_target_size(audio_rate, duration);

    if !is_minsize(min_size, target_size) {
        println!("Target size too small");
        return;
    }

    let target_bitrate = get_target_video_rate(target_size, duration, audio_rate);
    convert_first(input, target_bitrate, true);
    convert_out(input, target_bitrate, audio_rate, "/home/nevin/Desktop/ffmpeg/teaser-8m.mp4");
    // write to file and reread that file every second
    println!("done converting");
}


#[test]
fn test_file_format() {
    let input = "/home/nevin/Desktop/video3.mkv";
    // assert_eq!(format_input(input), "/home/nevin/Videos/video3-8m.mp4");
}