mod config;

use zbus::{interface, fdo, Connection};
use std::collections::HashMap;
use std::error::Error;
use std::future::pending as future;
use std::process::Command;

fn uri_to_path(uri: &str) -> String {
    if uri.starts_with("file://") {
        let path = &uri[7..];
        if path.starts_with('/') {
            path.to_string()
        } else {
            let first_slash = path.find('/');
            if let Some(pos) = first_slash {
                format!("/{}", &path[pos + 1..])
            } else {
                path.to_string()
            }
        }
    } else {
        uri.to_string()
    }
}
struct GnomeTerminalProxy;

#[interface(name = "org.freedesktop.Application")]
impl GnomeTerminalProxy {
    fn open(&mut self, uris: Vec<String>, _options: HashMap<String, zbus::zvariant::Value>) -> fdo::Result<()> {
        if let Some(uri_to_open) = uris.first() {
            let local_path = uri_to_path(uri_to_open);

            if !local_path.is_empty() {
                let is_directory_hint = local_path.ends_with('/') || !local_path.contains('.');
                let target_path = if is_directory_hint {
                    local_path.clone()
                } else {
                    std::path::Path::new(&local_path)
                        .parent()
                        .and_then(std::path::Path::to_str)
                        .unwrap_or(".")
                        .to_string()
                };
                let full_command = config::load().unwrap().terminal_command.replace("{path}", &target_path);
                if let Some(parts) = shlex::split(&full_command) {
                    if parts.is_empty() {
                        eprintln!("  Error: Command is empty after splitting.");
                        return Ok(());
                    }
                    let mut command = Command::new(&parts[0]);
                    if parts.len() > 1 {
                        command.args(&parts[1..]);
                    }
                    command.spawn().expect("Could not spawn terminal");
                } else {
                    eprintln!("  Error: Could not parse the terminal command string.");
                }

            } else {
                eprintln!("  Empty or invalid URI for launching terminal.");
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = config::load()?;
    let (bus_name_str, object_path_str) = match config.gnome_service_to_mock.as_str() {
        "ptyxis" => ("org.gnome.Ptyxis", "/org/gnome/Ptyxis"),
        "kgx" => ("org.gnome.Console", "/org/gnome/Console"),
        other => {
            eprintln!("Invalid value for 'gnomeServiceToMock' in config: '{}'", other);
            std::process::exit(1);
        }
    };


    let connection = Connection::session().await?;

    eprintln!("Registering object path: {}", object_path_str);
    connection
        .object_server()
        .at(object_path_str, GnomeTerminalProxy {})
        .await?;

    eprintln!("Requesting D-Bus name: {}", bus_name_str);
    connection.request_name(bus_name_str).await?;
    eprintln!("Running... Press Ctrl+C to exit.");

    future::<()>().await;

    Ok(())
}