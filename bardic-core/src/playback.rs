use rodio::{Decoder, OutputStream, Sink};
use std::cell::RefCell;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub struct LocalPlayer {
    _stream: RefCell<Option<OutputStream>>,
    sink: RefCell<Option<Sink>>,
    current_track: RefCell<Option<Track>>,
}

pub struct Track {
    path: PathBuf,
}

pub enum PlaybackState {
    Playing,
    Paused,
    Stopped,
}

impl LocalPlayer {
    pub fn new() -> Self {
        Self {
            _stream: RefCell::new(None),
            sink: RefCell::new(None),
            current_track: RefCell::new(None),
        }
    }

    pub fn play(&self, file_path: PathBuf) -> std::io::Result<()> {
        self.ensure_sink()?;
        let file = BufReader::new(File::open(&file_path)?);
        let source = Decoder::new(BufReader::new(file))
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        let current_track = match &*self.current_track.borrow() {
            Some(song) => {
                if song.path == file_path {
                    self.resume();
                    return Ok(());
                } else {
                    self.sink.borrow().as_ref().unwrap().stop();
                }
                Some(Track { path: file_path })
            }
            None => Some(Track { path: file_path }),
        };

        *self.current_track.borrow_mut() = current_track;

        self.sink.borrow().as_ref().unwrap().append(source);

        Ok(())
    }

    pub fn pause(&self) {
        if let Some(sink) = self.sink.borrow().as_ref() {
            sink.pause();
        }
    }

    pub fn stop(&self) {
        *self._stream.borrow_mut() = None;
        *self.sink.borrow_mut() = None;
        *self.current_track().borrow_mut() = None;
    }

    pub fn resume(&self) {
        if let Some(sink) = self.sink.borrow().as_ref() {
            sink.play();
        }
    }

    pub fn next(&self) {
        if let Some(sink) = self.sink.borrow().as_ref() {
            sink.skip_one();
        }
    }

    pub fn state(&self) -> PlaybackState {
        match self.sink.borrow().as_ref() {
            Some(sink) => {
                if sink.empty() {
                    PlaybackState::Stopped
                } else if sink.is_paused() {
                    PlaybackState::Paused
                } else {
                    PlaybackState::Playing
                }
            }
            None => PlaybackState::Stopped,
        }
    }

    pub fn current_track(&self) -> &RefCell<Option<Track>> {
        &self.current_track
    }

    fn ensure_sink(&self) -> std::io::Result<()> {
        if self.sink.borrow().is_some() {
            return Ok(());
        }

        let stream =
            rodio::OutputStreamBuilder::open_default_stream().map_err(std::io::Error::other)?;
        let sink = rodio::Sink::connect_new(stream.mixer());

        *self._stream.borrow_mut() = Some(stream);
        *self.sink.borrow_mut() = Some(sink);
        Ok(())
    }
}

impl Default for LocalPlayer {
    fn default() -> Self {
        Self::new()
    }
}
