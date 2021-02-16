use rodio::Source;
use std::time::Duration;

fn main() {
    play(100, 200);
    play(200, 200);
    play(400, 300);

}

fn play(freq: u32, millis: u64) { 
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    let source = rodio::source::SineWave::new(freq)
        .take_duration(Duration::from_millis(millis));

    sink.append(source);
    sink.play();
    sink.sleep_until_end();
}