use std::sync::OnceLock;
use serde::Deserialize;

const DEFAULT_CONFIG: &str = r#"
[network]
server_addr = "127.0.0.1:8080"

[mock]
enabled = true
"#;

#[derive(Deserialize)]
struct Config {
    pub network: Network,
    pub mock: Option<Mock>,
}

#[derive(Deserialize)]
pub struct Network {
    pub server_addr: String,
}

#[derive(Deserialize)]
pub struct Mock {
    pub enabled: bool,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

fn config_path() -> std::path::PathBuf {
    // Look next to the executable first, then fall back to cwd
    if let Ok(exe) = std::env::current_exe() {
        let p = exe.with_file_name("Config.toml");
        if p.exists() { return p; }
    }
    std::path::PathBuf::from("Config.toml")
}

fn load() -> &'static Config {
    CONFIG.get_or_init(|| {
        let text = std::fs::read_to_string(config_path())
            .unwrap_or_else(|_| DEFAULT_CONFIG.to_string());
        toml::from_str(&text).unwrap_or_else(|_| {
            toml::from_str(DEFAULT_CONFIG).expect("embedded default config is valid")
        })
    })
}

pub fn server_addr() -> &'static str {
    &load().network.server_addr
}

pub fn mock_enabled() -> bool {
    load().mock.as_ref().map(|m| m.enabled).unwrap_or(true)
}
