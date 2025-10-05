use bardic_core::Commands;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::process::{Command, Stdio};
use std::{thread, time};

const SOCKET_PATH: &str = "/tmp/bardic.sock";

pub fn ensure_daemon_running() -> std::io::Result<()> {
    if is_daemon_running()? {
        println!("Daemon is already running");
        return Ok(());
    }

    Command::new("bardic-daemon")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .spawn()?;

    for _ in 1..10 {
        if is_daemon_running()? {
            println!("Daemon successfully started");
            return Ok(());
        }

        thread::sleep(time::Duration::from_millis(100));
    }

    Err(std::io::Error::other("Failed to start daemon"))
}

pub fn is_daemon_running() -> std::io::Result<bool> {
    match UnixStream::connect(SOCKET_PATH) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

pub fn send_command(cmd: &Commands) -> std::io::Result<()> {
    let mut stream = UnixStream::connect(SOCKET_PATH)?;
    let json_message =
        serde_json::to_string(cmd).expect("Command serialization failed unexpectedly");
    let message = format!("{json_message}\n");

    stream.write_all(message.as_bytes())?;
    stream.flush()?;
    stream.shutdown(std::net::Shutdown::Write)?;

    let mut response = String::new();
    let mut reader = BufReader::new(&stream);

    reader.read_line(&mut response)?;

    if !response.is_empty() {
        println!("{response}");
    }

    Ok(())
}
