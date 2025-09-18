use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

pub struct AudioPlayer {
    _stream: OutputStream,
    sink: Sink,
}

impl AudioPlayer {
    pub fn new() -> Self {
        let _stream =
            rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
        let sink = rodio::Sink::connect_new(_stream.mixer());

        Self { _stream, sink }
    }

    pub fn play(&self, file_path: &str) {
        let file = BufReader::new(File::open(file_path).unwrap());
        let source = Decoder::new(BufReader::new(file)).unwrap();
        self.sink.append(source);
    }

    pub fn pause(&self) {
        self.sink.pause();
    }

    pub fn stop(&self) {
        self.sink.stop();
    }

    pub fn resume(&self) {
        self.sink.play();
    }

    pub fn queue_song(&self, file_path: &str) {
        self.play(file_path);
    }

    pub fn next(&self) {
        self.sink.skip_one();
    }
}

impl Default for AudioPlayer {
    fn default() -> Self {
        Self::new()
    }
}
