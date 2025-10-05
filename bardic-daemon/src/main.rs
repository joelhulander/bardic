use bardic_core::playback::{LocalPlayer, PlaybackState};
use bardic_core::{Commands, DaemonCommands};
use serde_json::from_str;
use std::fs::*;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

const SOCKET_PATH: &str = "/tmp/bardic.sock";

struct Daemon {
    audio_player: LocalPlayer,
    shutdown: Arc<AtomicBool>,
}

impl Daemon {
    fn new(shutdown: Arc<AtomicBool>) -> Self {
        remove_file(SOCKET_PATH).ok();

        Self {
            audio_player: LocalPlayer::new(),
            shutdown,
        }
    }

    fn run(&self) -> std::io::Result<()> {
        let listener = UnixListener::bind(SOCKET_PATH)?;

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    // Connection successfully established
                    if self.handle_client(stream).is_ok() {}
                }
                Err(_) => {
                    break;
                }
            }
            if self.shutdown.load(Ordering::Relaxed) {
                break;
            }
        }

        Ok(())
    }

    fn handle_client(&self, mut stream: UnixStream) -> std::io::Result<()> {
        let mut message = String::new();

        let mut reader = BufReader::new(&stream);
        reader.read_line(&mut message)?;
        let command: Commands = from_str(message.trim())?;

        match command {
            Commands::Play { song } => {
                if let Some(song) = song {
                    let response = format!("Playing {}", song.display());
                    stream.write_all(response.as_bytes())?;
                    self.audio_player.play(song)?;
                } else if let PlaybackState::Stopped = self.audio_player.state() {
                    stream.write_all(b"No song is currently playing")?;
                } else {
                    self.audio_player.resume();
                    stream.write_all(b"Resuming playback")?;
                }
            }
            Commands::Pause => {
                self.audio_player.pause();
            }
            Commands::Stop => {
                self.audio_player.stop();
            }
            Commands::Daemon(DaemonCommands::Stop) => {
                stream.write_all(b"Stopping daemon...")?;
                self.shutdown.store(true, Ordering::Relaxed);
            }
            _ => println!("Other commands"),
        }

        Ok(())
    }
}

impl Drop for Daemon {
    fn drop(&mut self) {
        if let Err(e) = remove_file(SOCKET_PATH) {
            eprintln!("Failed to remove temporary file: {e}");
        }
    }
}

fn main() -> std::io::Result<()> {
    let shutdown = Arc::new(AtomicBool::new(false));
    let daemon = Daemon::new(shutdown.clone());

    ctrlc::set_handler(move || {
        println!("Received ctrl+c");
        shutdown.store(true, Ordering::Relaxed);
    })
    .expect("Error setting Ctrl-C handler");

    daemon.run()?;

    Ok(())
}
