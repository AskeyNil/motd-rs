mod command;
mod components;
mod tools;

use clap::Parser;
use components::{
    component::Component, disk::Disk, memory::Memory, network::Network, system::System,
};

#[derive(Parser)]
#[command(version)]
struct Cli {
    /// Sets a custom config file
    #[arg(
        short,
        long,
        value_name = "FILE",
        help = "Sets a custom config files",
        default_value = "config.toml"
    )]
    config: std::path::PathBuf,
}

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    system: System,
    // docker: Docker,
    network: Network,
    disk: Disk,
    memory: Memory,
    // last_login: LastLogin,
    // service_status: ServiceStatus,
}

fn main() {
    let cli = Cli::parse();
    let context = std::fs::read_to_string(&cli.config).unwrap();
    let config: Config = toml::from_str(&context).unwrap();
    let v: Vec<&dyn Component> = vec![
        &config.system,
        &config.network,
        &config.memory,
        &config.disk,
    ];

    let max_width = v.iter().map(|s| s.width()).max().unwrap();

    for component in v {
        component.print(max_width);
    }
}
