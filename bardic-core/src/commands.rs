use std::path::PathBuf;
use clap::Subcommand;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Subcommand)]
pub enum Commands {
    Play {
        song: Option<PathBuf>,
    },
    Pause,
    Stop,
    Next,
    Previous,
    #[command(subcommand)]
    Volume(VolumeCommands),
    Seek,
    #[command(subcommand)]
    Spotify(SpotifyCommands),
    Status,
    NowPlaying,
    #[command(subcommand)]
    Daemon(DaemonCommands),
}

#[derive(Subcommand, Serialize, Deserialize)]
pub enum VolumeCommands {
    Up,
    Down,
}

#[derive(Subcommand, Serialize, Deserialize)]
pub enum SpotifyCommands {
    Login,
    Logout,
    WhoAmI,
    Play,
    Pause,
    Stop,
}

#[derive(Subcommand, Serialize, Deserialize)]
pub enum DaemonCommands {
    Start,
    Stop,
    Status,
}

