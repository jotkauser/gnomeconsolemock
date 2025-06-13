use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Error;
use std::path::PathBuf;
use std::process;
use json_comments::StripComments;
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub gnome_service_to_mock: String,
    pub terminal_command: String,
}

fn generate_default_config(path: &PathBuf) -> Result<Config, Error> {
    let config_dir = path.parent().unwrap();
    if !config_dir.exists() {
        fs::create_dir_all(config_dir)?;
    }

    let pretty_config_content = format!(
        r#"{{
  // GNOME D-Bus service to mock. Can be ptyxis or kgx
  "gnomeServiceToMock": "kgx",

  // The command used to launch your terminal.
  // The `{{path}}` placeholder will be replaced with the directory to open.
  "terminalCommand": "kitty -d {{path}}"
}}"#,
    );

    fs::write(path, pretty_config_content)?;
    eprintln!("Created default config file: {}", path.display());
    eprintln!("Please edit it to your liking.");
    process::exit(0);
}

pub fn load() -> Result<Config, Error> {
    let config_dir = dirs::config_dir().unwrap();
    let path = config_dir.join("gnomeconsolemock");
    let config_file = path.join("config.jsonc");
    if !config_file.exists() {
        return generate_default_config(&config_file);
    }
    let file_content = fs::read_to_string(&config_file)?;
    let stripped = StripComments::new(file_content.as_bytes());
    let config: Config = serde_json::from_reader(stripped)?;

    Ok(config)
    
}