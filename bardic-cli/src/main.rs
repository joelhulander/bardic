use bardic_core::Commands;
use clap::Parser;
use std::io::Write;
use std::os::unix::net::UnixStream;

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

    let json_message =
        serde_json::to_string(&daemon_command).expect("Command serialization failed unexpectedly");
    let message = format!("{json_message}\n");

    let mut stream = UnixStream::connect("/tmp/mysocket")?;
    stream.write_all(message.as_bytes())?;

    Ok(())
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
