use compressor::ffmpeg::get_duration;

#[test]
fn test_duration() {
    assert_eq!(get_duration("/home/nevin/Desktop/ffmpeg/teaser.mkv"), 30.033);
    // get_duration("/home/nevin/Desktop/ffmpeg/teaser.mkv");
}