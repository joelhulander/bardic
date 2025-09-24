use rodio::{Decoder, OutputStream, Sink};
use std::cell::RefCell;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub struct LocalPlayer {
    _stream: OutputStream,
    sink: Sink,
    current_track: RefCell<Option<Track>>,
}

struct Track {
    path: PathBuf,
}

pub enum PlaybackState {
    Playing,
    Paused,
    Stopped,
}

impl LocalPlayer {
    pub fn new() -> Self {
        let _stream =
            rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
        let sink = rodio::Sink::connect_new(_stream.mixer());

        Self {
            _stream,
            sink,
            current_track: RefCell::new(None),
        }
    }

    pub fn play(&self, file_path: PathBuf) {
        let file = BufReader::new(File::open(&file_path).unwrap());
        let source = Decoder::new(BufReader::new(file)).unwrap();

        let current_track = match &*self.current_track.borrow() {
            Some(song) => {
                if song.path == file_path {
                    self.resume();
                    return;
                } else {
                    self.sink.stop();
                }
                Some(Track { path: file_path })
            }
            None => Some(Track { path: file_path }),
        };

        *self.current_track.borrow_mut() = current_track;

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

    pub fn next(&self) {
        self.sink.skip_one();
    }

    pub fn state(&self) -> PlaybackState {
        if self.sink.empty() {
            PlaybackState::Stopped
        } else if self.sink.is_paused() {
            PlaybackState::Paused
        } else {
            PlaybackState::Playing
        }
    }
}

impl Default for LocalPlayer {
    fn default() -> Self {
        Self::new()
    }
}
