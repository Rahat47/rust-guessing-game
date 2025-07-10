use rodio::{Decoder, OutputStream, source::Source};
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::Duration;

fn play_sound(file_path: &str) {
    // Create a separate thread for audio playback
    let file_path_owned = file_path.to_owned();
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let file = BufReader::new(File::open(file_path_owned).unwrap());
        let source = Decoder::new(file).unwrap();
        let _ = stream_handle.play_raw(source.convert_samples());

        // Keep the thread alive while the sound is playing
        // This is a simple way to handle it; more robust solutions might be needed for complex scenarios
        thread::sleep(Duration::from_millis(2000)); // Adjust duration as needed
    });
}

pub fn play_start_sound() {
    play_sound("assets/sounds/start.wav");
}

pub fn play_guess_sound() {
    play_sound("assets/sounds/guess.wav");
}

pub fn play_win_sound() {
    play_sound("assets/sounds/win.wav");
}

pub fn play_game_over_sound() {
    play_sound("assets/sounds/game_over.wav");
}
