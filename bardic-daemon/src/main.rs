use bardic_core::Commands;
use bardic_core::playback::LocalPlayer;
use serde_json::from_str;
use std::fs::*;
use std::io::Read;
use std::os::unix::net::{UnixListener, UnixStream};

struct Daemon {
    audio_player: LocalPlayer,
}

impl Daemon {
    fn new() -> Self {
        remove_file("/tmp/mysocket").ok();

        Self {
            audio_player: LocalPlayer::new(),
        }
    }
    fn run(&self) -> std::io::Result<()> {
        let listener = UnixListener::bind("/tmp/mysocket")?;

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
                    self.audio_player.play(song)?;
                } else {
                    self.audio_player.resume();
                };
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
