use bardic_core::Commands;
use bardic_core::playback::{LocalPlayer, PlaybackState};
use serde_json::from_str;
use std::fs::*;
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};

struct Daemon {
    audio_player: LocalPlayer,
}

impl Daemon {
    fn new() -> Self {
        remove_file("/tmp/bardic").ok();

        Self {
            audio_player: LocalPlayer::new(),
        }
    }
    fn run(&self) -> std::io::Result<()> {
        let listener = UnixListener::bind("/tmp/bardic")?;

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    // Connection successfully established
                    self.handle_client(stream)?;
                }
                Err(_) => {
                    break;
                }
            }
        }

        Ok(())
    }
    fn handle_client(&self, mut stream: UnixStream) -> std::io::Result<()> {
        let mut message = String::new();
        stream.read_to_string(&mut message)?;
        let command: Commands = from_str(message.as_str())?;

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
            _ => println!("Other commands"),
        }
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let daemon = Daemon::new();

    daemon.run()?;

    Ok(())
}
