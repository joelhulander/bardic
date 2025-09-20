use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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

#[derive(Subcommand)]
enum VolumeCommands {
    Up,
    Down,
}

#[derive(Subcommand)]
enum SpotifyCommands {
    Login,
    Logout,
    WhoAmI,
    Play,
    Pause,
    Stop,
}

#[derive(Subcommand)]
enum DaemonCommands {
    Start,
    Stop,
    Status,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Play { song } => {
            println!("Subcommand Play used. song is: {song:?}");
        }
        Commands::Pause => {
            println!("Subcommand Pause used");
        }
        Commands::Stop => {
            println!("Subcommand Stop used");
        }
        Commands::Next => {
            println!("Subcommand Next used");
        }
        Commands::Previous => {
            println!("Subcommand Previous used");
        }
        Commands::Volume(volume_command) => {
            println!("Subcommand Volume used");
            match volume_command {
                VolumeCommands::Up => {
                    println!("Up used");
                }
                VolumeCommands::Down => {
                    println!("Down used");
                }
            }
        }
        Commands::Seek => {
            println!("Subcommand Seek used");
        }
        Commands::Spotify(_) => {
            println!("Subcommand Spotify used");
        }
        Commands::Status => {
            println!("Subcommand Status used");
        }
        Commands::NowPlaying => {
            println!("Subcommand NowPlaying used");
        }
        Commands::Daemon(daemon_command) => {
            println!("Subcommand Daemon used");
            match daemon_command {
                DaemonCommands::Start => {
                    println!("Start used");
                }
                DaemonCommands::Stop => {
                    println!("Stop used");
                }
                DaemonCommands::Status => {
                    println!("Status used")
                }
            }
        }
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
