use clap::Subcommand;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Subcommand, Debug)]
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

#[derive(Serialize, Deserialize, Subcommand, Debug)]
pub enum VolumeCommands {
    Up,
    Down,
}

#[derive(Serialize, Deserialize, Subcommand, Debug)]
pub enum SpotifyCommands {
    Login,
    Logout,
    WhoAmI,
    Play,
    Pause,
    Stop,
}

#[derive(Serialize, Deserialize, Subcommand, Debug)]
pub enum DaemonCommands {
    Start,
    Stop,
    Status,
}
