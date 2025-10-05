use bardic_core::{Commands, DaemonCommands};
use bardic_ipc::{ensure_daemon_running, is_daemon_running, send_command};
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let daemon_command = match cli.command {
        Commands::Play { song } => {
            if let Some(song_path) = song {
                let abs_file_path = song_path.canonicalize()?;
                Commands::Play {
                    song: Some(abs_file_path),
                }
            } else {
                Commands::Play { song }
            }
        }
        Commands::Pause => Commands::Pause,
        Commands::Stop => Commands::Stop,
        Commands::Next => Commands::Next,
        Commands::Previous => Commands::Previous,
        Commands::Volume(volume_command) => Commands::Volume(volume_command),
        Commands::Seek => Commands::Seek,
        Commands::Spotify(spotify_commands) => Commands::Spotify(spotify_commands),
        Commands::Status => Commands::Status,
        Commands::NowPlaying => Commands::NowPlaying,
        Commands::Daemon(daemon_command) => Commands::Daemon(daemon_command),
    };

    match &daemon_command {
        Commands::Daemon(DaemonCommands::Start) => {
            ensure_daemon_running()?;
        }
        Commands::Daemon(DaemonCommands::Stop) => {
            if is_daemon_running()? {
                send_command(&daemon_command)?;
            } else {
                println!("Daemon is already stopped");
            }
        }
        Commands::Daemon(DaemonCommands::Status) => {
            if is_daemon_running()? {
                println!("Running");
            } else {
                println!("Stopped");
            }
        }
        _ => {
            if !is_daemon_running()? {
                println!("Daemon is not running");
                return Ok(());
            }

            send_command(&daemon_command)?;
        }
    }

    Ok(())
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
